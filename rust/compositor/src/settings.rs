// SPDX-License-Identifier: GPL-3.0-only
use crate::effects::Effect;
use std::{ffi::CString, fs, path::Path};
#[derive(Clone, Debug)]
pub struct Settings {
    pub backend: String,
    pub effect: Effect,
    pub duration: i32,
    pub fade: i32,
    pub radius: i32,
    pub offset: i32,
    pub fps: i32,
    pub shadow_opacity: f64,
    pub shadows: bool,
    pub vsync: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            backend: "auto".into(),
            effect: Effect::Zoom,
            duration: 220,
            fade: 120,
            radius: 16,
            offset: 6,
            fps: 60,
            shadow_opacity: 0.45,
            shadows: true,
            vsync: true,
        }
    }
}
pub fn backend(name: &str) -> bool {
    matches!(name, "auto" | "glx" | "xrender")
}
impl Settings {
    pub fn read(path: &Path, backend_override: &str) -> Self {
        let mut settings = Self::default();
        if let Ok(data) = fs::read(path) {
            for line in String::from_utf8_lossy(&data).lines() {
                let trim = |c| matches!(c, ' ' | '\t' | '\r' | '\n');
                let line = line.trim_matches(trim);
                if line.starts_with(['#', '[']) {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim_matches(trim);
                    if settings.apply(key, value.trim_matches(trim)).is_none() {
                        eprintln!("emwm-compositor: ignoring invalid setting {key}");
                    }
                }
            }
        }
        if !backend_override.is_empty() {
            settings.backend = backend_override.to_owned();
        }
        settings
    }
    fn apply(&mut self, key: &str, v: &str) -> Option<()> {
        // stoi and stod accept a numeric prefix, including C hex floats for opacity.
        let int = |low, high| {
            let digits = v.trim_start_matches([' ', '\t', '\x0b', '\x0c', '\r', '\n']);
            let sign = usize::from(digits.starts_with(['+', '-']));
            let end = sign
                + digits.as_bytes()[sign..]
                    .iter()
                    .take_while(|b| b.is_ascii_digit())
                    .count();
            digits[..end]
                .parse::<i32>()
                .ok()
                .map(|n| n.clamp(low, high))
        };
        match key {
            "backend" if backend(v) => self.backend = v.into(),
            "effect" => match v {
                "none" => self.effect = Effect::Disabled,
                "zoom" => self.effect = Effect::Zoom,
                "twist" => self.effect = Effect::Twist,
                "flip" => self.effect = Effect::Flip,
                _ => {}
            },
            "duration_ms" => self.duration = int(50, 2000)?,
            "fade_ms" => self.fade = int(0, 1000)?,
            "shadow_radius" => self.radius = int(1, 64)?,
            "shadow_offset" => self.offset = int(-32, 64)?,
            "max_fps" => self.fps = int(15, 240)?,
            "shadow_opacity" => {
                let s = CString::new(v.split('\0').next()?).ok()?;
                let mut end = std::ptr::null_mut();
                // SAFETY: s is NUL terminated and libc writes the end pointer in this thread.
                let n = unsafe {
                    *libc::__errno_location() = 0;
                    let n = libc::strtod(s.as_ptr(), &mut end);
                    if *libc::__errno_location() == libc::ERANGE {
                        return None;
                    }
                    n
                };
                if std::ptr::eq(end, s.as_ptr()) {
                    return None;
                }
                if n.is_finite() {
                    self.shadow_opacity = n.clamp(0., 1.);
                }
            }
            "shadows" => self.shadows = matches!(v, "true" | "1"),
            "vsync" => self.vsync = matches!(v, "true" | "1"),
            _ => {}
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reload_preserves_numeric_prefixes_and_limits() {
        let mut settings = Settings::default();
        settings.apply("duration_ms", "232ms").unwrap();
        assert_eq!(settings.duration, 232);
        assert!(settings.apply("duration_ms", "2147483649").is_none());
        assert_eq!(settings.duration, 232);
        settings.apply("shadow_offset", "-52").unwrap();
        assert_eq!(settings.offset, -32);
        settings.apply("shadow_opacity", "0x1p-2").unwrap();
        assert_eq!(settings.shadow_opacity, 0.25);
        settings.apply("shadow_opacity", "nan").unwrap();
        assert_eq!(settings.shadow_opacity, 0.25);
        settings.apply("effect", "flip").unwrap();
        settings.apply("effect", "other").unwrap();
        assert_eq!(settings.effect, Effect::Flip);
    }
}
