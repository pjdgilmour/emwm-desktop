// SPDX-License-Identifier: MIT
use super::*;
use std::ffi::{CStr, CString};
use std::mem::{offset_of, size_of};
use std::os::unix::ffi::OsStringExt;

#[link(name = "Xm")]
extern "C" {
    static xmFrameWidgetClass: *mut c_void;
    static xmDrawingAreaWidgetClass: *mut c_void;
    fn XmAddProtocolCallback(
        w: *mut c_void,
        property: Atom,
        protocol: Atom,
        cb: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void),
        data: *mut c_void,
    );
}

// Intrinsic.h definitions are opaque in x11-rs. Xt copies Arg values into
// the appropriately typed widget resources. XtArgVal is a long on Linux.
#[repr(C)]
struct Arg {
    name: *const c_char,
    value: c_long,
}
fn arg(name: &'static CStr, value: c_long) -> Arg {
    Arg {
        name: name.as_ptr(),
        value,
    }
}
#[repr(C)]
struct Resource {
    name: *const c_char,
    class: *const c_char,
    kind: *const c_char,
    size: u32,
    offset: u32,
    default_type: *const c_char,
    value: *mut c_void,
}
#[repr(C)]
#[derive(Default)]
pub(crate) struct Settings {
    pub(crate) size: i32,
    pub(crate) spacing: i32,
    pub(crate) minimum_width: i32,
    pub(crate) occupy_all: i8,
}

fn resource(
    name: &'static CStr,
    class: &'static CStr,
    kind: &'static CStr,
    size: usize,
    offset: usize,
    default: usize,
) -> Resource {
    Resource {
        name: name.as_ptr(),
        class: class.as_ptr(),
        kind: kind.as_ptr(),
        size: size as u32,
        offset: offset as u32,
        default_type: c"Immediate".as_ptr(),
        value: default as *mut c_void,
    }
}

fn settings(shell: Widget, embedded: bool) -> Result<Settings, ()> {
    let minimum = if embedded { 1 } else { 160 };
    let mut schema = [
        resource(
            c"minimumWidth",
            c"MinimumWidth",
            c"Int",
            size_of::<i32>(),
            offset_of!(Settings, minimum_width),
            minimum,
        ),
        resource(
            c"iconSize",
            c"IconSize",
            c"Int",
            size_of::<i32>(),
            offset_of!(Settings, size),
            24,
        ),
        resource(
            c"spacing",
            c"Spacing",
            c"Int",
            size_of::<i32>(),
            offset_of!(Settings, spacing),
            4,
        ),
        resource(
            c"occupyAll",
            c"OccupyAll",
            c"Boolean",
            1,
            offset_of!(Settings, occupy_all),
            1,
        ),
    ];
    let mut res = Settings::default();
    // SAFETY: Xt writes resources at the specified repr(C) offsets into res.
    unsafe {
        let output = (&mut res as *mut Settings).cast();
        let fields = schema.as_mut_ptr().cast();
        if embedded {
            xt::XtGetSubresources(
                shell,
                output,
                c"tray".as_ptr(),
                c"Tray".as_ptr(),
                fields,
                4,
                null_mut(),
                0,
            );
        } else {
            xt::XtGetApplicationResources(shell, output, fields, 4, null_mut(), 0);
        }
    }
    if embedded {
        if !(8..=128).contains(&res.size) {
            res.size = 24;
        }
        if !(0..=64).contains(&res.spacing) {
            res.spacing = 4;
        }
        if !(1..=4096).contains(&res.minimum_width) {
            res.minimum_width = 1;
        }
    } else if !(8..=128).contains(&res.size)
        || !(0..=64).contains(&res.spacing)
        || !(32..=4096).contains(&res.minimum_width)
    {
        eprintln!("xmtray: iconSize must be 8..128; spacing 0..64; minimumWidth 32..4096");
        return Err(());
    }
    Ok(res)
}

pub(crate) struct Ui {
    pub shell: Widget,
    pub frame: Widget,
    pub area: Widget,
    pub embedded: bool,
    pub settings: Settings,
}

impl Ui {
    pub(crate) fn new(parent: Widget, shell: Widget, embedded: bool) -> Result<Self, ()> {
        let settings = settings(shell, embedded)?;
        let (name, name_area, mut attrs) = if embedded {
            (
                c"trayFrame",
                c"trayIcons",
                vec![
                    arg(c"shadowThickness", 0),
                    arg(c"marginWidth", 0),
                    arg(c"marginHeight", 0),
                ],
            )
        } else {
            (
                c"frame",
                c"icons",
                vec![arg(c"shadowType", 8), arg(c"shadowThickness", 2)],
            )
        };
        let size = i64::from(settings.size + if embedded { 6 } else { 12 }) as c_long;
        let mut dims = [
            arg(c"width", size),
            arg(c"height", size),
            arg(c"marginWidth", 0),
            arg(c"marginHeight", 0),
        ];
        // SAFETY: class symbols are supplied by Motif, both parents live in the
        // same Xt tree; callbacks receive events while their widgets are alive.
        unsafe {
            let frame = xt::XtCreateManagedWidget(
                name.as_ptr(),
                xmFrameWidgetClass.cast(),
                parent,
                attrs.as_mut_ptr().cast(),
                attrs.len() as u32,
            );
            let area = xt::XtCreateManagedWidget(
                name_area.as_ptr(),
                xmDrawingAreaWidgetClass.cast(),
                frame,
                dims.as_mut_ptr().cast(),
                dims.len() as u32,
            );
            xt::XtAddCallback(
                area,
                c"resizeCallback".as_ptr(),
                Some(resize_cb),
                null_mut(),
            );
            xt::XtAddEventHandler(
                area,
                (StructureNotifyMask
                    | SubstructureNotifyMask
                    | SubstructureRedirectMask
                    | PropertyChangeMask) as _,
                1,
                Some(events_cb),
                null_mut(),
            );
            xt::XtAddEventHandler(shell, FocusChangeMask as _, 0, Some(events_cb), null_mut());
            Ok(Self {
                shell,
                area,
                frame,
                settings,
                embedded,
            })
        }
    }
    pub(crate) fn width(&self) -> i32 {
        let mut width = 0u16;
        let mut value = arg(c"width", (&mut width as *mut u16) as c_long);
        // SAFETY: the Xt Dimension resource writes an unsigned short.
        unsafe { xt::XtGetValues(self.area, (&mut value as *mut Arg).cast(), 1) };
        i32::from(width)
    }
    pub(crate) fn background(&self) -> c_ulong {
        let mut pixel = 0;
        let mut value = arg(c"background", (&mut pixel as *mut c_ulong) as c_long);
        // SAFETY: Xt Pixel writes an unsigned long for this DrawingArea.
        unsafe { xt::XtGetValues(self.area, (&mut value as *mut Arg).cast(), 1) };
        pixel
    }
    pub(crate) fn set_grid_size(&self, width: i32, height: i32, show: bool) {
        let mut values = [arg(c"width", width as _), arg(c"height", height as _)];
        // SAFETY: these are live Motif widgets. Managing the frame may invoke
        // the resize callback synchronously; layout positions the icons afterwards.
        unsafe {
            if self.embedded {
                if show {
                    xt::XtManageChild(self.frame)
                } else {
                    xt::XtUnmanageChild(self.frame)
                }
            }
            xt::XtSetValues(self.area, values.as_mut_ptr().cast(), 2);
        }
    }
}

unsafe extern "C" fn close_cb(w: *mut c_void, _a: *mut c_void, _b: *mut c_void) {
    // SAFETY: Xt calls us for the live application shell.
    xt::XtAppSetExitFlag(xt::XtWidgetToApplicationContext(w.cast()));
}

pub(crate) fn run() -> i32 {
    let mut args: Vec<_> = std::env::args_os().map(|a| a.into_vec()).collect();
    if args.len() == 2 && args[1] == b"--help" {
        println!("Usage: xmtray [-display DISPLAY] [-geometry +X+Y] [-iconsize 24] [-spacing 4]\nA standalone Motif X11 system tray. Resources: XmTray.iconSize, spacing, occupyAll.");
        return 0;
    }
    for a in &mut args {
        a.push(0);
    }
    let mut argv: Vec<_> = args
        .iter_mut()
        .map(|s| s.as_mut_ptr().cast::<c_char>())
        .collect();
    let mut argc = argv.len() as i32;
    argv.push(null_mut());
    let option = |name: &CStr, spec: &CStr| XrmOptionDescRec {
        option: name.as_ptr().cast_mut(),
        specifier: spec.as_ptr().cast_mut(),
        argKind: 3,
        value: null_mut(),
    };
    let mut options = [
        option(c"-iconsize", c".iconSize"),
        option(c"-spacing", c".spacing"),
    ];
    let mut defaults = [
        c"XmTray.title: Bandeja".as_ptr().cast_mut(),
        c"XmTray.geometry: +8+360".as_ptr().cast_mut(),
        null_mut(),
    ];
    let mut app = null_mut();
    let mut attrs = [
        arg(c"allowShellResize", 1),
        arg(c"deleteResponse", 2),
        arg(c"mwmFunctions", 4 | 8 | 32),
        arg(c"mwmDecorations", 2 | 8 | 16 | 32),
    ];
    // SAFETY: shell owns the Xt display and context until the loop exits. Xt
    // may reorder argv pointers, whose underlying buffers remain in args.
    unsafe {
        xt::XtSetLanguageProc(null_mut(), None, null_mut());
        let shell = xt::XtAppInitialize(
            &mut app,
            c"XmTray".as_ptr(),
            options.as_mut_ptr(),
            2,
            &mut argc,
            argv.as_mut_ptr(),
            defaults.as_mut_ptr(),
            attrs.as_mut_ptr().cast(),
            4,
        );
        if argc > 1 {
            use std::io::Write;
            let mut err = std::io::stderr().lock();
            let _ = err.write_all(b"xmtray: unknown option: ");
            let _ = err.write_all(CStr::from_ptr(argv[1]).to_bytes());
            let _ = err.write_all(b"\n");
            return 1;
        }
        let Ok(ui) = Ui::new(shell, shell, false) else {
            return 1;
        };
        let display = xt::XtDisplay(shell);
        set_host(Host::new(ui));
        xt::XtRealizeWidget(shell);
        let claimed = HOST.with_borrow_mut(|slot| slot.as_mut().is_some_and(Host::claim));
        if claimed {
            with_host(|host| {
                if host.ui.settings.occupy_all != 0 {
                    let window = xt::XtWindow(shell);
                    let presence = host.atom(c"_MWM_WORKSPACE_PRESENCE");
                    host.property(window, presence, presence, host.atom(c"all"));
                    host.property(
                        window,
                        host.atom(c"_NET_WM_DESKTOP"),
                        XA_CARDINAL,
                        0xffffffff,
                    );
                }
                XmAddProtocolCallback(
                    shell.cast(),
                    host.atom(c"WM_PROTOCOLS"),
                    host.atom(c"WM_DELETE_WINDOW"),
                    close_cb,
                    null_mut(),
                );
                install_signals(app);
                host.layout();
            });
            xt::XtMapWidget(shell);
            xt::XtAppMainLoop(app);
            with_host(Host::release);
        }
        HOST.with_borrow_mut(Option::take);
        xt::XtDestroyWidget(shell);
        XCloseDisplay(display);
    }
    0
}

pub(crate) fn selection_name(screen: i32) -> CString {
    CString::new(format!("_NET_SYSTEM_TRAY_S{screen}")).expect("integer has no NUL")
}
