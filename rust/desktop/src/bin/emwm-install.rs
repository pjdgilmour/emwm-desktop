// SPDX-License-Identifier: GPL-3.0-only
//! Build the private Debian package out of pinned, isolated source trees.
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    env,
    error::Error,
    ffi::OsString,
    fs, io,
    os::unix::fs::{symlink, PermissionsExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use tempfile::Builder;

#[path = "../installer/deps.rs"]
mod deps;
#[path = "../installer/vendor.rs"]
mod vendor;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

macro_rules! cmd {
    ($program:expr $(, $arg:expr)* $(,)?) => {{
        let mut command = Command::new($program);
        $(command.arg($arg);)*
        command
    }};
}
fn run(cmd: &mut Command) -> Result<()> {
    println!("+ {cmd:?}");
    let status = cmd.status()?;
    if !status.success() {
        return Err(format!("{cmd:?}: {status}").into());
    }
    Ok(())
}
fn capture(cmd: &mut Command) -> Result<String> {
    let output = cmd.stderr(Stdio::inherit()).output()?;
    if !output.status.success() {
        return Err(format!("{cmd:?}: {}", output.status).into());
    }
    Ok(String::from_utf8(output.stdout)?)
}
fn copy(src: &Path, dst: &Path, mode: u32) -> Result<()> {
    fs::create_dir_all(dst.parent().ok_or("Destino inválido")?)?;
    fs::copy(src, dst)?;
    fs::set_permissions(dst, fs::Permissions::from_mode(mode))?;
    Ok(())
}
fn digest(path: &Path) -> Result<String> {
    let mut hash = Sha256::new();
    io::copy(&mut fs::File::open(path)?, &mut hash)?;
    Ok(format!("{:x}", hash.finalize()))
}

#[derive(Default)]
struct Options {
    build_only: bool,
    deps_only: bool,
    tests: bool,
    yes: bool,
    sources: Option<PathBuf>,
    jobs: usize,
}
const HELP: &str = include_str!("../install-help.txt");

fn parse(args: impl IntoIterator<Item = OsString>) -> std::result::Result<Options, String> {
    let mut opt = Options {
        jobs: std::thread::available_parallelism()
            .map_or(1, usize::from)
            .min(4),
        ..Options::default()
    };
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        let arg = arg.to_string_lossy();
        let (name, inline) = arg
            .split_once('=')
            .map_or((arg.as_ref(), None), |(n, v)| (n, Some(v)));
        // argparse accepted unique prefixes of the documented long options.
        let names = [
            "--help",
            "--build-only",
            "--dependencies-only",
            "--sources",
            "--test",
            "--yes",
            "--jobs",
        ];
        let name = if name.starts_with("--") && name.len() > 2 {
            names
                .iter()
                .copied()
                .find(|known| known.starts_with(name))
                .unwrap_or(name)
        } else {
            name
        };
        let mut value = || {
            inline
                .map(OsString::from)
                .or_else(|| {
                    args.next().filter(|v| {
                        let s = v.to_string_lossy();
                        !s.starts_with('-') || s == "-" || s.parse::<i64>().is_ok()
                    })
                })
                .ok_or_else(|| format!("argument {name}: expected one argument"))
        };
        if inline.is_some() && !matches!(name, "--sources" | "--jobs") {
            return Err(format!(
                "argument {name}: ignored explicit argument '{}'",
                inline.unwrap_or_default()
            ));
        }
        match name {
            "--build-only" => opt.build_only = true,
            "--dependencies-only" => opt.deps_only = true,
            "--test" => opt.tests = true,
            "--yes" => opt.yes = true,
            "--sources" => opt.sources = Some(PathBuf::from(value()?)),
            "--jobs" => {
                let n = value()?;
                opt.jobs = n
                    .to_string_lossy()
                    .parse::<i64>()
                    .map_err(|_| {
                        format!(
                            "argument --jobs: invalid int value: '{}'",
                            n.to_string_lossy()
                        )
                    })?
                    .try_into()
                    .unwrap_or(0);
            }
            "-h" | "--help" => {
                print!("{HELP}");
                std::process::exit(0);
            }
            _ => return Err(format!("unrecognized arguments: {arg}")),
        }
    }
    if !(1..=128).contains(&opt.jobs) {
        return Err("--jobs deve estar entre 1 e 128".into());
    }
    if opt.build_only && opt.deps_only {
        return Err("--build-only e --dependencies-only são mutuamente exclusivos".into());
    }
    Ok(opt)
}

struct Build {
    root: PathBuf,
    work: PathBuf,
    stage: PathBuf,
    jobs: String,
}
const PREFIX: &str = "usr/lib/emwm-desktop";
const DATA: &str = "usr/share/emwm-desktop";
impl Build {
    fn new(root: &Path, work: &Path, jobs: usize) -> Self {
        Self {
            root: root.into(),
            work: work.into(),
            stage: work.join("debian/emwm-desktop"),
            jobs: format!("-j{jobs}"),
        }
    }
    fn source(&self, name: &str, spec: &Value, local: &Option<PathBuf>) -> Result<()> {
        let cache = self.root.join("build/cache");
        fs::create_dir_all(&cache)?;
        let field = |key: &str| {
            spec[key]
                .as_str()
                .ok_or_else(|| format!("sources.lock.json: {name}.{key}"))
        };
        let dest = self.work.join(name);
        fs::create_dir(&dest)?;
        if let Some(revision) = spec["revision"].as_str() {
            if revision.len() != 40 || !revision.bytes().all(|v| v.is_ascii_hexdigit()) {
                return Err(format!("Revisão inválida em sources.lock.json: {name}").into());
            }
            let repo = if let Some(dir) = local {
                dir.join(name)
            } else {
                let repo = cache.join(format!("{name}.git"));
                if !repo.is_dir() {
                    run(&mut cmd!("git", "init", "--bare", &repo))?;
                }
                run(&mut cmd!(
                    "git",
                    "-C",
                    &repo,
                    "fetch",
                    "--depth=1",
                    field("url")?,
                    revision
                ))?;
                repo
            };
            run(&mut cmd!(
                "git",
                "-C",
                &repo,
                "cat-file",
                "-e",
                format!("{revision}^{{commit}}")
            ))?;
            let archive = self.work.join(format!("{name}.tar"));
            run(&mut cmd!(
                "git", "-C", &repo, "archive", "-o", &archive, revision
            ))?;
            run(&mut cmd!("tar", "-xf", archive, "-C", &dest))?;
        } else {
            let archive = cache.join(field("archive")?);
            if !archive.exists() {
                let tmp = tempfile::NamedTempFile::new_in(&cache)?;
                run(&mut cmd!(
                    "curl",
                    "--fail",
                    "--location",
                    "--max-time",
                    "60",
                    "--output",
                    tmp.path(),
                    field("url")?
                ))?;
                if digest(tmp.path())? != field("sha256")? {
                    return Err(format!("Checksum inválido para {name}").into());
                }
                tmp.persist(&archive)?;
            }
            if digest(&archive)? != field("sha256")? {
                return Err(format!("Checksum inválido para {name}").into());
            }
            run(&mut cmd!(
                "tar",
                "-xf",
                &archive,
                "--strip-components=1",
                "-C",
                &dest,
                field("directory")?
            ))?;
        }
        let mut patches: Vec<_> =
            fs::read_dir(self.root.join("patches").join(name))?.collect::<io::Result<_>>()?;
        patches.sort_by_key(|p| p.file_name());
        for p in patches {
            if p.path().extension().is_some_and(|v| v == "patch") {
                run(cmd!(
                    "patch",
                    "--batch",
                    "--forward",
                    "--fuzz=0",
                    "-p1",
                    "-i",
                    p.path()
                )
                .current_dir(&dest))?;
            }
        }
        Ok(())
    }
    fn source_archive(&self) -> Result<PathBuf> {
        let tar = self.work.join("sources.tar");
        run(&mut cmd!(
            "tar",
            "-cf",
            &tar,
            "-C",
            &self.work,
            "emwm",
            "emwm-utils",
            "pcmanfm"
        ))?;
        let project = self.work.join("project.tar");
        run(cmd!(
            "tar",
            "-cf",
            &project,
            "--exclude=__pycache__",
            "--exclude=target",
            "--exclude=rust/dependency-sources",
            "--exclude=.rust-anything",
            "--exclude=*.so",
            "--exclude=*.o",
            "--exclude=build",
            "--transform=flags=r;s|^|emwm-desktop/|",
            "-C",
            &self.root
        )
        .args([
            "apps",
            "third_party",
            "scripts",
            "session",
            "config",
            "packaging",
            "patches",
            "tests",
            "rust",
            "install.sh",
            "VERSION",
            "sources.lock.json",
            "README.md",
            "THIRD_PARTY.md",
            "LICENSE",
        ]))?;
        run(&mut cmd!("tar", "--concatenate", "--file", &tar, &project))?;
        // Build from exactly the application sources included in the package,
        // even if a developer edits the checkout during the build.
        run(&mut cmd!("tar", "-xf", &project, "-C", &self.work))?;
        let snapshot = self.work.join("emwm-desktop");
        vendor::append(
            &snapshot,
            &self.work,
            &tar,
            &[
                "rust/desktop/Cargo.toml",
                "rust/tellmwm/Cargo.toml",
                "rust/pcmanfm-app/Cargo.toml",
                "rust/wm/Cargo.toml",
                "rust/xmsm/Cargo.toml",
                "rust/toolbox/Cargo.toml",
                "rust/emwm/Cargo.toml",
                "rust/emwm-utils/Cargo.toml",
                "rust/tray/Cargo.toml",
                "rust/compositor/Cargo.toml",
                "rust/wallpaper/Cargo.toml",
                "rust/appearance/app/Cargo.toml",
            ],
        )?;
        run(&mut cmd!("xz", "-T2", "-f", &tar))?;
        Ok(snapshot)
    }
    fn build_projects(&self) -> Result<()> {
        for name in ["emwm", "emwm-utils"] {
            let src = self.work.join(name).join("src");
            symlink("../mf/Makefile.Linux", src.join("Makefile"))?;
            let mut make = cmd!(
                "make",
                "-C",
                &src,
                &self.jobs,
                "PREFIX=/usr/lib/emwm-desktop",
                "RCDIR=/usr/share/emwm-desktop",
                "APPLRESDIR=/usr/share/emwm-desktop/app-defaults"
            );
            if name == "emwm-utils" {
                make.arg("CPPFLAGS=-DSUSPEND_CMD='\"/usr/bin/systemctl suspend\"'");
            }
            run(&mut make)?;
        }
        let port_target = self.work.join("upstream-build");
        for name in ["pcmanfm-app", "tellmwm"] {
            let manifest = self.root.join("rust").join(name).join("Cargo.toml");
            run(&mut cmd!(
                "cargo",
                "build",
                "--release",
                "--locked",
                "--manifest-path",
                &manifest,
                "--target-dir",
                &port_target
            ))?;
        }
        for (name, dir) in [
            ("emwm-wallpaper-tool", "wallpaper-build"),
            ("emwm-compositor", "compositor-build"),
        ] {
            let out = self.work.join(dir);
            run(&mut cmd!(
                "cmake",
                "-S",
                self.root.join("apps").join(name),
                "-B",
                &out,
                "-DCMAKE_BUILD_TYPE=Release"
            ))?;
            run(&mut cmd!("cmake", "--build", &out, &self.jobs))?;
            run(&mut cmd!(
                "ctest",
                "--test-dir",
                &out,
                "--output-on-failure"
            ))?;
        }
        let manifest = self.root.join("rust/desktop/Cargo.toml");
        let target = self.work.join("desktop-build");
        run(&mut cmd!(
            "cargo",
            "build",
            "--release",
            "--locked",
            "--manifest-path",
            &manifest,
            "--target-dir",
            &target,
            "--bin",
            "emwm-session",
            "--bin",
            "xmsession",
            "--bin",
            "emwm-desktop-icons"
        ))?;
        run(&mut cmd!(
            "cargo",
            "test",
            "--locked",
            "--manifest-path",
            &manifest,
            "--target-dir",
            &target
        ))?;
        let appearance = self.work.join("appearance-reference");
        let app_manifest = self.root.join("rust/appearance/app/Cargo.toml");
        let app_target = self.work.join("appearance-build");
        run(&mut cmd!(
            "cargo",
            "build",
            "--release",
            "--locked",
            "--manifest-path",
            &app_manifest,
            "--target-dir",
            &app_target
        ))?;
        run(&mut cmd!(
            "cargo",
            "test",
            "--release",
            "--locked",
            "--manifest-path",
            &app_manifest,
            "--target-dir",
            &app_target
        ))?;
        run(&mut cmd!(
            "sh",
            self.root.join("scripts/build-gradient.sh"),
            &app_target,
            &appearance
        ))?;
        run(&mut cmd!(
            "sh",
            self.root.join("rust/appearance/build.sh"),
            &app_target,
            &appearance
        ))?;
        run(cmd!(
            "python3",
            "-m",
            "unittest",
            "discover",
            "-s",
            self.root.join("apps/emwm-appearance/tests"),
            "-v"
        )
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .env("PYTHONPATH", &appearance)
        .env("RUST_ANYTHING_MODE", "new"))?;
        Ok(())
    }
    fn x11_tests(&self) -> Result<()> {
        let w = &self.work;
        let tests = self.root.join("tests");
        let wm = w.join("emwm/src/emwm");
        let compositor = w.join("compositor-build/emwm-compositor");
        let fm = w.join("upstream-build/release/pcmanfm");
        let tell = w.join("upstream-build/release/tellmwm");
        let utils = w.join("emwm-utils");
        let wallpaper = w.join("wallpaper-build");
        let search = env::join_paths(
            [wallpaper.clone(), w.join("upstream-build/release")]
                .into_iter()
                .chain(env::split_paths(&env::var_os("PATH").unwrap_or_default())),
        )?;
        let icons = w.join("desktop-build/release/emwm-desktop-icons");
        for backend in ["xrender", "glx"] {
            run(&mut cmd!(
                "xvfb-run",
                "-a",
                w.join("compositor-build/test-render-effects"),
                backend
            ))?;
        }
        run(cmd!("make", "test").current_dir(w.join("emwm")))?;
        run(&mut cmd!("sh", tests.join("tellmwm.sh"), &wm, &tell))?;
        run(cmd!("make", "test-tiling").current_dir(w.join("emwm")))?;
        run(cmd!("make", "test-argb")
            .current_dir(w.join("emwm"))
            .env("EMWM_COMPOSITOR", &compositor))?;
        run(&mut cmd!(
            "python3",
            self.root.join("rust/pcmanfm-app/tests/files.py"),
            &fm,
            &fm
        ))?;
        for name in ["native-desktop.py", "desktop-drag.py"] {
            run(cmd!(
                "python3",
                tests.join(name),
                "--wm",
                &wm,
                "--compositor",
                &compositor,
                "--file-manager",
                &fm
            )
            .env("EMWM_TEST_ICONS_BINARY", &icons)
            .env("PATH", &search))?;
        }
        run(cmd!(
            "python3",
            tests.join("desktop-monitors.py"),
            "--wm",
            &wm,
            "--file-manager",
            &fm,
            "--tellmwm",
            &tell
        )
        .env("EMWM_TEST_ICONS_BINARY", &icons)
        .env("PATH", &search))?;
        run(&mut cmd!(
            "python3",
            tests.join("monitor-layout.py"),
            "--wm",
            &wm,
            "--wallpaper",
            wallpaper.join("emwm-wallpaper-tool"),
            "--toolbox",
            utils.join("src/xmtoolbox")
        ))?;
        run(&mut cmd!(
            "python3",
            tests.join("title-gradient.py"),
            "--wm",
            &wm,
            "--compositor",
            &compositor
        ))?;
        run(cmd!("make", "test-x11").current_dir(&utils))?;
        run(&mut cmd!("sh", tests.join("run-toolchest-tray.sh"), &utils))?;
        run(cmd!("cmake", "--build", &wallpaper, "--target", "test-x11")
            .env("EMWM_BINARY", &wm)
            .env("PATH", &search))?;
        run(cmd!(
            "sh",
            self.root.join("apps/emwm-appearance/tests/run-x11.sh")
        )
        .env("PYTHONPATH", self.work.join("appearance-reference"))
        .env("RUST_ANYTHING_MODE", "new")
        .env("PYTHONDONTWRITEBYTECODE", "1"))?;
        run(&mut cmd!(
            "xvfb-run",
            "-a",
            "dbus-run-session",
            "--",
            "/usr/bin/python3",
            self.root.join("apps/emwm-appearance/tests/native-smoke.py"),
            self.work.join("appearance-build/release/emwm-appearance")
        ))
    }
    fn stage_files(&self) -> Result<()> {
        let bin = self.stage.join(PREFIX).join("bin");
        let public_bin = self.stage.join("usr/bin");
        fs::create_dir_all(&public_bin)?;
        let data = self.stage.join(DATA);
        for (src, name) in [
            ("emwm/src/emwm", "emwm"),
            ("emwm-utils/src/xmtoolbox", "xmtoolbox"),
            ("emwm-utils/src/xmtray", "xmtray"),
            ("emwm-utils/src/xmsm", "xmsm"),
            ("desktop-build/release/xmsession", "xmsession"),
            ("upstream-build/release/tellmwm", "tellmwm"),
            ("upstream-build/release/pcmanfm", "pcmanfm"),
            ("wallpaper-build/emwm-wallpaper-tool", "emwm-wallpaper-tool"),
            ("compositor-build/emwm-compositor", "emwm-compositor"),
            ("desktop-build/release/emwm-session", "emwm-session"),
            (
                "desktop-build/release/emwm-desktop-icons",
                "emwm-desktop-icons",
            ),
        ] {
            copy(
                &self.work.join(src),
                &bin.join(name),
                if name == "xmsm" { 0o4755 } else { 0o755 },
            )?;
        }
        for (src, dest) in [
            ("emwm/src/Emwm.ad", "app-defaults/Emwm"),
            ("emwm-utils/src/XmSm.ad", "app-defaults/XmSm"),
            ("emwm-utils/src/XmToolbox.ad", "app-defaults/XmToolbox"),
            ("emwm/src/system.emwmrc", "system.emwmrc"),
            (
                "emwm-utils/src/emwm-applications.menu",
                "emwm-applications.menu",
            ),
        ] {
            copy(&self.work.join(src), &data.join(dest), 0o644)?;
        }
        for item in fs::read_dir(self.work.join("emwm-utils/src/desktop-directories"))? {
            let f = item?;
            if f.path().extension().is_some_and(|ext| ext == "directory") {
                copy(
                    &f.path(),
                    &data.join("desktop-directories").join(f.file_name()),
                    0o644,
                )?;
            }
        }
        for item in fs::read_dir(self.root.join("config"))? {
            let f = item?;
            copy(&f.path(), &data.join(f.file_name()), 0o644)?;
        }
        let appearance_dir = self.stage.join(PREFIX).join("appearance");
        for name in ["emwm-appearance", "emwm-appearance-login"] {
            copy(
                &self.work.join("appearance-build/release").join(name),
                &appearance_dir.join(name),
                0o755,
            )?;
        }
        symlink("../appearance/emwm-appearance", bin.join("emwm-appearance"))?;
        symlink(
            "../lib/emwm-desktop/appearance/emwm-appearance",
            public_bin.join("emwm-appearance"),
        )?;
        for (src, dest, mode) in [
            (
                "apps/emwm-appearance/emwm-appearance.desktop",
                "usr/share/applications/emwm-appearance.desktop",
                0o644,
            ),
            (
                "packaging/emwm-desktop.desktop",
                "usr/share/xsessions/emwm-desktop.desktop",
                0o644,
            ),
            (
                "packaging/emwm-wallpaper-tool.desktop",
                "usr/share/applications/emwm-desktop-wallpaper.desktop",
                0o644,
            ),
            ("packaging/postinst", "DEBIAN/postinst", 0o755),
            ("packaging/postrm", "DEBIAN/postrm", 0o755),
        ] {
            copy(&self.root.join(src), &self.stage.join(dest), mode)?;
        }
        for src in [
            "emwm/src/emwm.1",
            "emwm/src/emwmrc.4",
            "emwm-utils/src/xmtoolbox.1",
            "emwm-utils/src/xmtray.1",
            "emwm-utils/src/xmsm.1",
            "emwm-desktop/third_party/tellmwm/tellmwm.1",
        ] {
            let p = Path::new(src);
            let ext = p
                .extension()
                .ok_or("Página de manual sem seção")?
                .to_string_lossy();
            copy(
                &self.work.join(src),
                &data
                    .join(format!("man/man{ext}"))
                    .join(p.file_name().ok_or("Manual inválido")?),
                0o644,
            )?;
        }
        symlink(
            "../lib/emwm-desktop/bin/emwm-session",
            public_bin.join("emwm-desktop-session"),
        )?;
        let doc = self.stage.join("usr/share/doc/emwm-desktop");
        for (src, dest) in [
            ("README.md", "README.md"),
            ("THIRD_PARTY.md", "copyright"),
            ("LICENSE", "licenses/emwm-desktop-GPL-3"),
            (
                "apps/emwm-compositor/COPYING.WindowMaker",
                "licenses/WindowMaker-GPL-2",
            ),
            ("apps/emwm-compositor/README.md", "compositor.md"),
            ("sources.lock.json", "sources.lock.json"),
        ] {
            copy(&self.root.join(src), &doc.join(dest), 0o644)?;
        }
        for (src, dest) in [
            ("emwm/COPYING", "licenses/emwm-COPYING"),
            ("emwm-utils/COPYING", "licenses/emwm-utils-COPYING"),
            ("pcmanfm/COPYING", "licenses/pcmanfm-COPYING"),
            (
                "emwm-desktop/third_party/tellmwm/COPYING",
                "licenses/tellmwm-COPYING",
            ),
            ("sources.tar.xz", "sources.tar.xz"),
        ] {
            copy(&self.work.join(src), &doc.join(dest), 0o644)?;
        }
        Ok(())
    }
    fn package(&self, version: &str, dist: &Path) -> Result<PathBuf> {
        use std::io::Read;
        let source_control="Source: emwm-desktop\nSection: x11\nPriority: optional\nMaintainer: pjdgilmour <paulogilmour@hotmail.com>\nStandards-Version: 4.7.0\n\nPackage: emwm-desktop\nArchitecture: any\nDescription: Classic Motif desktop\n";
        fs::write(self.work.join("debian/control"), source_control)?;
        let mut deps_cmd = cmd!("dpkg-shlibdeps", "-O");
        let mut size = 0;
        let mut dirs = vec![self.stage.clone()];
        while let Some(dir) = dirs.pop() {
            fs::set_permissions(&dir, fs::Permissions::from_mode(0o755))?;
            for item in fs::read_dir(dir)? {
                let f = item?;
                let path = f.path();
                let meta = f.metadata()?;
                if meta.is_dir() {
                    dirs.push(path);
                    continue;
                }
                size += meta.len();
                let mut magic = [0; 4];
                if path.starts_with(self.stage.join(PREFIX))
                    && fs::File::open(&path)?.read(&mut magic)? == 4
                    && magic == *b"\x7fELF"
                {
                    let mut arg = OsString::from("-e");
                    arg.push(path);
                    deps_cmd.arg(arg);
                }
            }
        }
        let output = capture(deps_cmd.current_dir(&self.work))?;
        let abi = output
            .lines()
            .find_map(|l| l.strip_prefix("shlibs:Depends="))
            .ok_or("dpkg-shlibdeps: faltam dependências ABI")?;
        let arch = capture(&mut cmd!("dpkg", "--print-architecture"))?;
        let arch = arch.trim();
        let runtime = deps::RUNTIME.join(", ");
        let kb = size / 1024;
        let control=format!("Package: emwm-desktop\nVersion: {version}\nArchitecture: {arch}\nSection: x11\nPriority: optional\nMaintainer: pjdgilmour <paulogilmour@hotmail.com>\nInstalled-Size: {kb}\nDepends: {abi}, {runtime}\nRecommends: lightdm | x-display-manager, network-manager-gnome, pipewire-audio, pavucontrol\nHomepage: https://github.com/pjdgilmour/emwm-desktop\nDescription: Classic Motif desktop with XDG menus and system tray\n Enhanced Motif Window Manager, Toolchest, xmtray and persistent wallpapers.\n Installed as a separate EMWM Desktop session with private executables.\n");
        fs::write(self.stage.join("DEBIAN/control"), control)?;
        fs::set_permissions(
            self.stage.join("DEBIAN/control"),
            fs::Permissions::from_mode(0o644),
        )?;
        fs::create_dir_all(dist)?;
        let name = format!("emwm-desktop_{version}_{arch}.deb");
        let package = dist.join(&name);
        let temp = Builder::new().prefix(".package-").tempdir_in(dist)?;
        let built = temp.path().join(&name);
        run(&mut cmd!(
            "dpkg-deb",
            "--root-owner-group",
            "--build",
            &self.stage,
            &built
        ))?;
        let checksum = format!("{}  {name}\n", digest(&built)?);
        run(&mut cmd!("dpkg-deb", "--info", &built))?;
        let sum = temp.path().join("sha256");
        fs::write(&sum, checksum)?;
        // Serialize publication of the package and its sidecar across simultaneous builds.
        use std::os::fd::AsRawFd;
        let directory = fs::File::open(dist)?;
        // SAFETY: the directory file stays open through both renames.
        if unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_EX) } != 0 {
            return Err(io::Error::last_os_error().into());
        }
        fs::rename(built, &package)?;
        fs::rename(sum, package.with_extension("deb.sha256"))?;
        Ok(package)
    }
}

fn install(options: Options) -> Result<()> {
    let os = fs::read_to_string("/etc/os-release")?;
    let attr = |field: &str| {
        os.lines()
            .find_map(|l| l.strip_prefix(field))
            .map(|v| v.trim_matches('"'))
    };
    if attr("ID=") != Some("debian") || attr("VERSION_ID=") != Some("13") {
        return Err("Esta versão do instalador foi preparada para Debian 13 (trixie).".into());
    }
    // SAFETY: geteuid takes no arguments and has no preconditions.
    if unsafe { libc::geteuid() } == 0 {
        return Err("Execute como usuário normal. O instalador usa sudo somente para apt.".into());
    }
    let apt_install = || {
        let mut cmd = cmd!("sudo", "apt-get", "install");
        if options.yes {
            cmd.arg("-y");
        }
        cmd
    };
    if !options.build_only {
        run(&mut cmd!("sudo", "apt-get", "update"))?;
        let mut cmd = apt_install();
        cmd.args(deps::BUILD_DEPS);
        if options.tests {
            cmd.args(deps::TEST_DEPS);
        }
        run(&mut cmd)?;
    }
    if options.deps_only {
        return Ok(());
    }
    let root = env::var_os("EMWM_BUILD_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .canonicalize()?;
    let version = fs::read_to_string(root.join("VERSION"))?;
    let version = version.trim();
    if !version.as_bytes().first().is_some_and(u8::is_ascii_digit)
        || !version
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b".+~:-".contains(&b))
    {
        return Err("VERSION inválida.".into());
    }
    let specs: Value = serde_json::from_reader(fs::File::open(root.join("sources.lock.json"))?)?;
    let base = root.join("build");
    fs::create_dir_all(&base)?;
    let work = Builder::new().prefix("package-").tempdir_in(&base)?;
    let mut build = Build::new(&root, work.path(), options.jobs);
    let sources = options.sources.map(fs::canonicalize).transpose()?;
    for name in ["emwm", "emwm-utils", "pcmanfm"] {
        build.source(name, &specs[name], &sources)?;
    }
    build.root = build.source_archive()?;
    build.build_projects()?;
    if options.tests {
        build.x11_tests()?;
    }
    build.stage_files()?;
    let package = build.package(version, &root.join("dist"))?;
    if options.tests {
        run(cmd!(
            "python3",
            "-m",
            "unittest",
            "discover",
            "-s",
            build.root.join("tests"),
            "-v"
        )
        .env(
            "EMWM_TEST_SESSION_BINARY",
            build.work.join("desktop-build/release/emwm-session"),
        )
        .env("EMWM_TEST_PACKAGE", &package))?;
        run(&mut cmd!(
            "sh",
            build.root.join("tests/smoke-session.sh"),
            &package
        ))?;
    }
    if options.build_only {
        println!("\nPacote gerado, sem instalação: {}", package.display());
    } else {
        run(apt_install().arg(&package))?;
        println!(
            "\nInstalado. Selecione EMWM Desktop na tela de login. A sessão atual foi preservada."
        );
    }
    Ok(())
}
fn main() {
    let opts = parse(env::args_os().skip(1)).unwrap_or_else(|e| {
        eprintln!(
            "{}\ninstall.py: error: {e}",
            HELP.split("\n\n").next().unwrap_or("install.py")
        );
        std::process::exit(2)
    });
    if let Err(e) = install(opts) {
        eprintln!("Erro: {e}");
        std::process::exit(1);
    }
}
