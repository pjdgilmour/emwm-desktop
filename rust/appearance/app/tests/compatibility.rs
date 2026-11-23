// SPDX-License-Identifier: GPL-3.0-only
use anyhow::Result;
use emwm_appearance::{
    files::ini_value,
    images, lightdm,
    settings::{default_effects, Settings},
    text,
};
use image::{Rgb, RgbImage, Rgba, RgbaImage};
use serde_json::json;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::Command;

fn write(root: &Path, name: &str, data: &str) -> Result<PathBuf> {
    let p = root.join(name);
    fs::create_dir_all(p.parent().unwrap())?;
    fs::write(&p, data)?;
    Ok(p)
}

#[test]
fn lightdm_only_changes_selected_keys_with_backups_and_safe_destinations() -> Result<()> {
    let tmp = tempfile::tempdir()?;
    let root = tmp.path();
    write(
        root,
        "etc/X11/default-display-manager",
        "/usr/sbin/lightdm\n",
    )?;
    write(
        root,
        "usr/share/xgreeters/lightdm-greeter.desktop",
        "[Desktop Entry]\nExec=lightdm-gtk-greeter\n",
    )?;
    write(
        root,
        "etc/lightdm/lightdm.conf",
        "[Seat:*]\ngreeter-session=lightdm-greeter\n[Seat:private]\ngreeter-session=slick\n",
    )?;
    assert_eq!(lightdm::detect(root, "seat0")?["supported"], true);
    assert_eq!(lightdm::detect(root, "private")?["supported"], false);
    write(root, "usr/share/themes/Test Theme/gtk-3.0/gtk.css", "")?;
    write(
        root,
        "usr/share/icons/Test Icons/index.theme",
        "[Icon Theme]\nDirectories=test\n",
    )?;
    fs::create_dir_all(root.join("usr/share/icons/Test Cursor/cursors"))?;
    let old="# keep\n[greeter]\nbackground=/old.jpg\nindicators=~clock\n[monitor: HDMI-1]\nbackground=/other.png\n[monitor: disabled]\nbackground=#skip\n";
    let config = write(root, "etc/lightdm/lightdm-gtk-greeter.conf", old)?;
    let backup = lightdm::theme(root, "Test Theme", "Test Icons", "Test Cursor", 24)?;
    assert_eq!(
        fs::read_to_string(backup.join("lightdm-gtk-greeter.conf-0"))?,
        old
    );
    let themed = fs::read_to_string(&config)?;
    assert!(themed.contains("icon-theme-name=Test Icons\n"));
    assert!(themed.contains("# keep\n"));
    assert!(themed.contains("indicators=~clock\n"));
    assert!(lightdm::theme(root, "../escape", "Test Icons", "Test Cursor", 24).is_err());
    assert_eq!(fs::read_to_string(&config)?, themed);

    let image = images::png(RgbImage::from_pixel(3, 2, Rgb([42, 103, 170])))?;
    assert!(lightdm::wallpaper(root, b"broken", "zoom").is_err());
    let backup = lightdm::wallpaper(root, &image, "tile")?;
    assert_eq!(
        fs::read_to_string(backup.join("lightdm-gtk-greeter.conf-0"))?,
        themed
    );
    let text = fs::read_to_string(&config)?;
    let val = ini_value(&text, "greeter", "background", "");
    let installed = root.join(val.strip_prefix("#stretched:/").unwrap());
    assert_eq!(
        fs::metadata(&installed)?.permissions().mode() & 0o777,
        0o644
    );
    let decoded = image::open(&installed)?.into_rgb8();
    assert_eq!(decoded.as_raw(), &[42, 103, 170].repeat(6));
    assert_eq!(ini_value(&text, "monitor: HDMI-1", "background", ""), val);
    assert_eq!(
        ini_value(&text, "monitor: disabled", "background", ""),
        "#skip"
    );
    let dir = installed.parent().unwrap();
    fs::rename(dir, root.join("outside"))?;
    symlink(root.join("outside"), dir)?;
    assert!(lightdm::wallpaper(root, &image, "zoom").is_err());
    assert_eq!(fs::read_to_string(config)?, text);
    Ok(())
}

#[test]
fn native_wallpaper_geometry_and_alpha() -> Result<()> {
    let img = RgbaImage::from_fn(2, 1, |x, _| {
        if x == 0 {
            Rgba([255, 0, 0, 255])
        } else {
            Rgba([0, 255, 0, 128])
        }
    });
    let centered = images::compose(img.clone(), "center", 4, 3, "#0000ff")?;
    assert_eq!(centered.get_pixel(1, 1).0, [255, 0, 0]);
    assert_eq!(centered.get_pixel(2, 1).0, [0, 128, 127]);
    assert_eq!(centered.get_pixel(0, 0).0, [0, 0, 255]);
    let tiled = images::compose(img, "tile", 5, 3, "#0000ff")?;
    assert_eq!(tiled.get_pixel(1, 2).0, [0, 128, 127]);
    let img = RgbaImage::from_pixel(8, 4, Rgba([255, 0, 0, 255]));
    for mode in ["fit", "stretch", "zoom"] {
        let out = images::compose(img.clone(), mode, 8, 8, "#0000ff")?;
        assert_eq!(out.get_pixel(3, 3).0, [255, 0, 0]);
        assert_eq!(
            out.get_pixel(0, 0).0,
            if mode == "fit" {
                [0, 0, 255]
            } else {
                [255, 0, 0]
            }
        );
    }
    for (w, h) in [(0, 1), (16385, 1), (10000, 10000)] {
        assert!(images::dimensions(w, h).is_err());
    }
    Ok(())
}

fn files_in(dir: &Path) -> Result<Vec<(PathBuf, Vec<u8>)>> {
    fn visit(base: &Path, path: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) -> Result<()> {
        for entry in fs::read_dir(path)? {
            let e = entry?;
            let path = e.path();
            if e.file_type()?.is_dir() {
                visit(base, &path, out)?;
            } else if e.file_type()?.is_file() {
                out.push((path.strip_prefix(base)?.to_owned(), fs::read(path)?));
            }
        }
        Ok(())
    }
    let mut out = vec![];
    visit(dir, dir, &mut out)?;
    out.sort();
    Ok(out)
}

#[test]
fn native_apply_matches_python_files_and_reads_old_preferences() -> Result<()> {
    let tmp = tempfile::tempdir()?;
    let home = tmp.path().join("home");
    let data = tmp.path().join("data");
    let state = tmp.path().join("state");
    let init = || -> Result<()> {
        write(&home, ".themes/Test Theme/gtk-3.0/gtk.css", "")?;
        write(
            &home,
            ".icons/Test Icons/index.theme",
            "[Icon Theme]\nDirectories=16x16/apps\n",
        )?;
        fs::create_dir_all(home.join(".icons/Test Cursor/cursors"))?;
        write(&home, ".Xresources", "! keep\nXTerm*faceSize: 14\n")?;
        write(&home, ".Xprofile", "export GTK_THEME=Old\n")?;
        write(
            &home,
            ".config/gtk-3.0/settings.ini",
            "# keep\n[Settings]\ngtk-font-name=Serif 12\n",
        )?;
        write(
            &home,
            ".config/qt6ct/qt6ct.conf",
            "[Appearance]\nstyle=Fusion\n",
        )?;
        write(
            &home,
            ".config/emwm-desktop/compositor.conf",
            "[compositor]\nfuture=42\n",
        )?;
        write(
            &home,
            ".config/emwm-desktop/desktop.conf",
            "[desktop]\n# keep\nfuture=42\n",
        )?;
        Ok(())
    };
    init()?;
    let model = Settings {
        config: home.join(".config"),
        data: data.clone(),
        home: home.clone(),
        state: state.clone(),
        directory: home.join(".config/emwm-desktop/appearance"),
    };
    let mut colors = json!({});
    for (k, v) in text::FIELDS
        .into_iter()
        .zip(text::palette("classic").unwrap())
    {
        colors[k] = json!(v);
    }
    let values = json!({"theme":"Test Theme","icons":"Test Icons","cursor":"Test Cursor","size":32,"palette":"custom","colors":colors,
        "gradient":{"mode":"diagonal","active_start":"#AABBCC"}});
    let mut effects = default_effects();
    effects["effect"] = json!("twist");
    effects["duration_ms"] = json!(350);
    effects["shadow_opacity"] = json!(0.65);
    let desktop = json!({"monitor":"DP-4","anchor":"right"});
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../apps/emwm-appearance");
    // The reference and the port use identical absolute configuration paths.
    let cmd=Command::new("/usr/bin/python3").args(["-c","import json,sys;from core import Settings;v,e,d=map(json.loads,sys.argv[1:]);Settings().apply(v,compositor=e,desktop=d)"])
        .args([values.to_string(),effects.to_string(),desktop.to_string()]).env("HOME",&home)
        .env("XDG_CONFIG_HOME",model.config.as_path()).env("XDG_STATE_HOME",&state).env("XDG_DATA_HOME",&data)
        .env("PYTHONPATH",src).env("PYTHONDONTWRITEBYTECODE","1").env("RUST_ANYTHING_UNIT_GRADIENT","original").output()?;
    assert!(
        cmd.status.success(),
        "{}",
        String::from_utf8_lossy(&cmd.stderr)
    );
    let expected = files_in(&home)?;
    let old = model.current()?;
    assert_eq!(old["gradient"]["active_start"], "#aabbcc");
    assert_eq!(model.desktop()?, desktop);
    assert_eq!(model.effects()?, effects);
    fs::remove_dir_all(&home)?;
    init()?;
    model.apply(&values, &effects, &desktop)?;
    assert_eq!(model.current()?, old);
    let actual = files_in(&home)?;
    assert_eq!(actual.len(), expected.len());
    for ((p, a), (q, b)) in actual.iter().zip(expected) {
        assert_eq!(p, &q);
        assert_eq!(
            String::from_utf8_lossy(a),
            String::from_utf8_lossy(&b),
            "{}",
            p.display()
        );
    }
    model.apply(&values, &effects, &desktop)?;
    assert_eq!(files_in(&home)?, actual);
    Ok(())
}

#[test]
fn resized_translucent_wallpapers_match_pillow_pixels() -> Result<()> {
    let script = r#"from PIL import Image
import sys
from wallpaper import compose
iw,ih,w,h=map(int,sys.argv[1:5]);mode=sys.argv[5]
img=Image.frombytes('RGBA',(iw,ih),bytes((i*79+11)%256 for i in range(iw*ih*4)))
sys.stdout.buffer.write(compose(img,mode,w,h,'#2c508e').tobytes())"#;
    let py = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../apps/emwm-appearance");
    for (iw, ih, w, h) in [
        (9, 8, 5, 12),
        (7, 11, 18, 7),
        (12, 17, 6, 4),
        (31, 27, 11, 5),
        (4, 4, 4, 4),
    ] {
        let pixels: Vec<_> = (0..iw * ih * 4)
            .map(|i| ((i * 79 + 11) % 256) as u8)
            .collect();
        let img = RgbaImage::from_raw(iw, ih, pixels).unwrap();
        for mode in ["stretch", "zoom", "fit", "tile", "center"] {
            let expected = Command::new("/usr/bin/python3")
                .args(["-c", script])
                .args([iw, ih, w, h].map(|n| n.to_string()))
                .arg(mode)
                .env("PYTHONPATH", &py)
                .env("RUST_ANYTHING_UNIT_APPEARANCE_WALLPAPER", "original")
                .output()?;
            assert!(
                expected.status.success(),
                "{}",
                String::from_utf8_lossy(&expected.stderr)
            );
            assert_eq!(
                images::compose(img.clone(), mode, w, h, "#2c508e")?.into_raw(),
                expected.stdout,
                "{mode} {iw}×{ih} → {w}×{h}"
            );
        }
    }
    Ok(())
}
