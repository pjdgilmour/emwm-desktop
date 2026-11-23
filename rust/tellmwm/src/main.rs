// Port of tellmwm 1.2 by alx@fastestcode.org, https://fastestcode.org/emwm.html
mod display;
use display::{Display, Id};
use std::io::{self, Write};
use std::os::unix::ffi::OsStringExt;
use std::process::ExitCode;
use x11::xlib::XA_ATOM;

const VERSION: &str = "tellmwm v1.2 - http://fastestcode.org/emwm.html";
const USAGE: &str = "\nUsage: tellmwm [<command> <arguments>]
Commands and arguments (see man 1 tellmwm for more info):
\tbackdrop <workspace ID> [-f <color> -b <color>] [<image name> | none]
\tsetws    <workspace ID>
\taddws    [<title>]
\tnamews   <workspace ID> <title>
\tdelws    <workspace ID>
If no command is specified, current configuration will be printed.";
type Failure = &'static str;

struct Workspace {
    name: Vec<u8>,
    title: Vec<u8>,
    bg: Id,
    fg: Id,
    image: Option<Vec<u8>>,
}
fn number(bytes: &[u8]) -> Id {
    let Ok(value) = std::ffi::CString::new(bytes) else {
        return 0;
    };
    // SAFETY: Xlib supplies NUL-free text fields. strtoul with base 0 preserves
    // the original handling of signed, octal and hexadecimal workspace properties.
    unsafe { libc::strtoul(value.as_ptr(), std::ptr::null_mut(), 0) }
}
impl Workspace {
    fn read(dpy: &Display, wm: Id, id: Id) -> Option<Self> {
        let name = dpy.name(id)?;
        let key = [b"_MWM_WORKSPACE_INFO_".as_slice(), &name].concat();
        let fields = dpy.text_list(wm, dpy.atom(&key)?)?;
        if fields.len() < 7 {
            return None;
        }
        Some(Self {
            name,
            title: fields[0].clone(),
            fg: number(&fields[3]),
            bg: number(&fields[2]),
            image: dpy.name(number(&fields[4])),
        })
    }
}

fn print_info(dpy: &Display, wm: Id, out: &mut impl Write) -> Option<()> {
    for id in dpy.words(wm, b"_MWM_WORKSPACE_LIST", Some(XA_ATOM), 256)? {
        let ws = Workspace::read(dpy, wm, id)?;
        for part in [
            b"Workspace \"".as_slice(),
            &ws.name,
            b"\"\n\tTitle     : ",
            &ws.title,
        ] {
            out.write_all(part).ok()?;
        }
        write!(
            out,
            "\n\tForeground: #{:06x}\n\tBackground: #{:06x}\n\tBackdrop  : ",
            ws.fg, ws.bg
        )
        .ok()?;
        out.write_all(ws.image.as_deref().unwrap_or(b"(none)"))
            .ok()?;
        out.write_all(b"\n\n").ok()?;
    }
    let current = *dpy
        .words(wm, b"_MWM_WORKSPACE_CURRENT", Some(XA_ATOM), 1)?
        .first()?;
    out.write_all(b"Current Workspace: ").ok()?;
    out.write_all(&dpy.name(current)?).ok()?;
    out.write_all(b"\n").ok()
}

fn quoted(mut cmd: Vec<u8>, arg: &[u8]) -> Vec<u8> {
    cmd.extend_from_slice(b" \"");
    cmd.extend_from_slice(arg);
    cmd.push(b'"');
    cmd
}
fn backdrop(dpy: &Display, wm: Id, args: &[Vec<u8>]) -> Result<Vec<u8>, Failure> {
    if args.len() < 2 {
        return Err("Command requires a workspace ID and color and/or image parameters");
    }
    let id = dpy
        .atom(&args[0])
        .ok_or("Invalid workspace ID specified!")?;
    let mut ws = Workspace::read(dpy, wm, id).ok_or("Failed to retrieve workspace info!")?;
    let mut image = None;
    let mut words = args[1..].iter();
    while let Some(word) = words.next() {
        match word.as_slice() {
            b"-f" | b"-b" => {
                let fg = word == b"-f";
                let name = words.next().ok_or(if fg {
                    "Missing parameter for -f"
                } else {
                    "Missing parameter for -b"
                })?;
                if let Some(pixel) = dpy.color(name) {
                    if fg {
                        ws.fg = pixel;
                    } else {
                        ws.bg = pixel;
                    }
                } else {
                    let _ = io::stderr().write_all(
                        &[b"Cannot allocate named color '".as_slice(), name, b"'\n"].concat(),
                    );
                }
            }
            file if image.is_none() => image = Some(file),
            other => {
                let _ = io::stderr()
                    .write_all(&[b"Ambiguous parameter '".as_slice(), other, b"'\n"].concat());
                return Err("");
            }
        }
    }
    let file = image.or(ws.image.as_deref()).unwrap_or(b"(null)");
    Ok(quoted(
        format!("BACKDROP {id:#x} {:#x} {:#x}", ws.fg, ws.bg).into_bytes(),
        file,
    ))
}

fn run(dpy: &Display, args: &[Vec<u8>]) -> Result<(), Failure> {
    let first = args.first().map(Vec::as_slice);
    match first {
        Some(b"-v") => {
            println!("{VERSION}");
            return Ok(());
        }
        Some(b"-h" | b"help") => {
            println!("{VERSION}\n{USAGE}");
            return Ok(());
        }
        Some(b"backdrop" | b"setws" | b"delws" | b"namews" | b"addws") | None => {}
        _ => {
            eprintln!("Invalid command specified");
            return Ok(());
        }
    }
    let wm = dpy
        .words(dpy.root(), b"_MOTIF_WM_INFO", None, 2)
        .and_then(|words| words.get(1).copied())
        .filter(|&id| id != 0)
        .ok_or(if first.is_none() {
            ""
        } else {
            "Failed to retrieve workspace manager info!"
        })?;
    let Some(command) = first else {
        return print_info(dpy, wm, &mut io::stdout().lock()).ok_or("");
    };
    let args = &args[1..];
    let request = match command {
        b"backdrop" => backdrop(dpy, wm, args)?,
        b"addws" => {
            let base = b"ADDWKSPC".to_vec();
            match args.first() {
                Some(title) => quoted(base, title),
                None => base,
            }
        }
        _ => {
            if command == b"namews" && args.len() < 2 {
                return Err("Command requires workspace ID and name arguments");
            }
            let name = args.first().ok_or("Command requires a workspace ID")?;
            let id = dpy.atom(name).ok_or("Invalid workspace ID specified!")?;
            match command {
                b"setws" => format!("WRKSPACE {id:#x}").into_bytes(),
                b"delws" => format!("DELWKSPC {id:#x}").into_bytes(),
                _ => quoted(format!("NAMWKSPC {id:#x}").into_bytes(), &args[1]),
            }
        }
    };
    dpy.send(wm, request)
        .then_some(())
        .ok_or("Failed to send WSM command")
}
fn main() -> ExitCode {
    let args = std::env::args_os()
        .skip(1)
        .map(|v| v.into_vec())
        .collect::<Vec<_>>();
    match Display::open()
        .ok_or("Can't open display")
        .and_then(|dpy| run(&dpy, &args))
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            if !msg.is_empty() {
                eprintln!("{msg}");
            }
            ExitCode::FAILURE
        }
    }
}
