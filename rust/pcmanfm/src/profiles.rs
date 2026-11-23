// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright 2010–2012 Hong Jen Yee, 2012–2017 Andriy Grytsenko, 2024 Ingo Brückl.
//! PCManFM desktop-item persistence. Group and path names are Unix bytes.
use super::{position, Grid};
use glib_sys as glib;
use std::collections::HashSet;
use std::ffi::{c_char, CStr, CString, OsStr};
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::ptr::{null_mut, NonNull};
use std::slice;

// GLib permits non-UTF-8 group names. Keep them as CStr across its key-file API.
pub struct Profile(NonNull<glib::GKeyFile>);

impl Drop for Profile {
    fn drop(&mut self) {
        // SAFETY: Profile owns the original GLib reference.
        unsafe { glib::g_key_file_unref(self.0.as_ptr()) };
    }
}

impl Profile {
    fn parse(bytes: &[u8]) -> Option<Self> {
        // SAFETY: GLib returns an owned key file and copies from the counted input buffer.
        unsafe {
            let key = Self(NonNull::new(glib::g_key_file_new())?);
            (glib::g_key_file_load_from_data(
                key.0.as_ptr(),
                bytes.as_ptr().cast(),
                bytes.len(),
                glib::G_KEY_FILE_NONE,
                null_mut(),
            ) != 0)
                .then_some(key)
        }
    }

    fn load(path: &CStr) -> Option<Self> {
        Self::parse(&std::fs::read(OsStr::from_bytes(path.to_bytes())).ok()?)
    }

    fn bytes(&self) -> Vec<u8> {
        let mut len = 0;
        // SAFETY: GLib allocates len initialized bytes and one final NUL. Copy before freeing.
        unsafe {
            let data = glib::g_key_file_to_data(self.0.as_ptr(), &mut len, null_mut());
            let bytes = if len == 0 {
                Vec::new()
            } else {
                slice::from_raw_parts(data.cast(), len).to_owned()
            };
            glib::g_free(data.cast());
            bytes
        }
    }

    fn groups(&self) -> Vec<CString> {
        let mut count = 0;
        // SAFETY: the returned string vector contains count initialized, non-null group names.
        unsafe {
            let ptr = glib::g_key_file_get_groups(self.0.as_ptr(), &mut count);
            let groups = if count == 0 {
                Vec::new()
            } else {
                slice::from_raw_parts(ptr, count)
                    .iter()
                    .map(|&s| CStr::from_ptr(s).to_owned())
                    .collect()
            };
            glib::g_strfreev(ptr);
            groups
        }
    }

    fn has(&self, group: &CStr) -> bool {
        // SAFETY: key and group are borrowed for the duration of the call.
        unsafe { glib::g_key_file_has_group(self.0.as_ptr(), group.as_ptr()) != 0 }
    }

    fn integer(&self, group: &CStr, name: &CStr) -> i32 {
        // SAFETY: all pointers are valid, ignoring errors reproduces GLib's default of zero.
        unsafe {
            glib::g_key_file_get_integer(self.0.as_ptr(), group.as_ptr(), name.as_ptr(), null_mut())
        }
    }

    fn set(&self, group: &CStr, name: &CStr, value: i32) {
        // SAFETY: GLib copies the borrowed names if needed; this key file is thread-local.
        unsafe {
            glib::g_key_file_set_integer(self.0.as_ptr(), group.as_ptr(), name.as_ptr(), value)
        }
    }

    fn anchor(&self) -> (i32, i32) {
        // SAFETY: both keys are static C strings; missing or invalid keys give zero.
        let right = unsafe {
            glib::g_key_file_get_boolean(
                self.0.as_ptr(),
                c"*".as_ptr(),
                c"emwm_anchor_right".as_ptr(),
                null_mut(),
            )
        };
        (right, self.integer(c"*", c"emwm_workarea_width"))
    }

    fn remove_settings(&self) {
        // SAFETY: an absent settings group is allowed.
        unsafe {
            glib::g_key_file_remove_group(self.0.as_ptr(), c"*".as_ptr(), null_mut());
        }
    }

    fn read_position(&self, group: &CStr, layout: bool, grid: Grid) -> SavedPoint {
        if !self.has(group) {
            return SavedPoint::default();
        }
        let mut x = self.integer(group, c"x");
        if layout {
            let (right, width) = self.anchor();
            x = position::anchor_x(
                x,
                width,
                right,
                grid.work.x,
                grid.work.width,
                grid.cell_width,
                grid.right,
            );
        }
        SavedPoint {
            found: 1,
            x,
            y: self.integer(group, c"y"),
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct SavedPoint {
    found: i32,
    x: i32,
    y: i32,
}

/// # Safety
/// The caller provides a NUL-terminated path, and frees a non-null result exactly once.
#[no_mangle]
pub unsafe extern "C" fn emwm_profile_load(path: *const c_char) -> *mut Profile {
    // SAFETY: the filename remains valid throughout the file read.
    Profile::load(unsafe { CStr::from_ptr(path) })
        .map_or(null_mut(), |p| Box::into_raw(Box::new(p)))
}

/// # Safety
/// The pointer is null or the result of a matching emwm_profile_load.
#[no_mangle]
pub unsafe extern "C" fn emwm_profile_free(profile: *mut Profile) {
    if !profile.is_null() {
        // SAFETY: this transfers ownership back from the C caller.
        drop(unsafe { Box::from_raw(profile) });
    }
}

/// # Safety
/// Non-null pointers are borrowed for the call; name is NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn emwm_profile_position(
    profile: *const Profile,
    name: *const c_char,
    layout: i32,
    grid: Grid,
) -> SavedPoint {
    if profile.is_null() || name.is_null() {
        return SavedPoint::default();
    }
    // SAFETY: desktop owns the loaded profile and the item's filename.
    unsafe { (&*profile).read_position(CStr::from_ptr(name), layout != 0, grid) }
}

fn save(path: &CStr, data: &[u8]) {
    // SAFETY: GLib reads exactly the indicated number of bytes, and performs its atomic write.
    unsafe {
        glib::g_file_set_contents(
            path.as_ptr(),
            data.as_ptr(),
            data.len() as isize,
            null_mut(),
        );
    }
}

/// # Safety
/// All inputs are NUL-terminated. C owns the result and frees it with g_free.
#[no_mangle]
pub unsafe extern "C" fn emwm_named_profile(
    directory: *const c_char,
    name: *const c_char,
    legacy: *const c_char,
) -> *mut c_char {
    // SAFETY: these strings and GLib's allocated escaped result are valid until copied.
    unsafe {
        let dir = CStr::from_ptr(directory).to_bytes();
        let escaped = glib::g_uri_escape_string(name, std::ptr::null(), 0);
        let mut named = [
            dir,
            b"/desktop-output-",
            CStr::from_ptr(escaped).to_bytes(),
            b".conf",
        ]
        .concat();
        glib::g_free(escaped.cast());
        named.push(0);
        let path = CStr::from_bytes_with_nul_unchecked(&named);
        if glib::g_file_test(path.as_ptr(), glib::G_FILE_TEST_EXISTS) == 0 {
            if let Ok(data) = std::fs::read(OsStr::from_bytes(CStr::from_ptr(legacy).to_bytes())) {
                save(path, &data);
            }
        }
        glib::g_strdup(path.as_ptr())
    }
}

#[repr(C)]
pub struct FixedItem {
    basename: *const c_char,
    x: i32,
    y: i32,
}

#[repr(C)]
pub struct DesktopState {
    config: *const c_char,
    configured: i32,
    layout: i32,
    active: i32,
    loaded: i32,
    grid: Grid,
    fixed: *const FixedItem,
    fixed_count: usize,
    present: *const *const c_char,
    present_count: usize,
}

fn append_anchor(buf: &mut Vec<u8>, right: i32, width: i32) {
    let _ = write!(
        buf,
        "emwm_anchor_right={}\nemwm_workarea_width={}\n",
        right != 0,
        width
    );
}

fn append_item(buf: &mut Vec<u8>, basename: &[u8], x: i32, y: i32) {
    buf.push(b'[');
    for &byte in basename {
        match byte {
            b'\r' => buf.extend_from_slice(b"\\r"),
            b'\n' => buf.extend_from_slice(b"\\n"),
            b'\\' | b'[' | b']' => {
                buf.push(b'\\');
                buf.push(byte);
            }
            _ => buf.push(byte),
        }
    }
    let _ = write!(buf, "]\nx={x}\ny={y}\n\n");
}

fn merge_absent(
    saved: &Profile,
    current: &Profile,
    present: &HashSet<&[u8]>,
    loaded: bool,
    grid: Grid,
) {
    for group in saved.groups() {
        let name = group.to_bytes();
        if name == b"*" || current.has(&group) || present.contains(name) {
            continue;
        }
        if loaded && !name.windows(3).any(|w| w == b"://") {
            continue;
        }
        let point = saved.read_position(&group, true, grid);
        current.set(&group, c"x", point.x);
        current.set(&group, c"y", point.y);
    }
}

/// # Safety
/// All C strings and arrays in state are borrowed and valid until this call returns.
/// An array with count zero may be null.
#[no_mangle]
pub unsafe extern "C" fn emwm_save_positions(path: *const c_char, state: &DesktopState) {
    // SAFETY: the GTK caller holds the config buffer, model and fixed list while serializing.
    let (path, mut buf) = unsafe {
        (
            CStr::from_ptr(path),
            CStr::from_ptr(state.config).to_bytes().to_vec(),
        )
    };
    let layout = state.layout != 0;
    let active = state.active != 0;
    let saved = if layout { Profile::load(path) } else { None };
    if state.configured != 0 {
        if layout && active {
            append_anchor(&mut buf, state.grid.right, state.grid.work.width);
        }
        buf.push(b'\n');
    }
    if let Some(saved) = saved.as_ref().filter(|_| !active) {
        let (right, width) = saved.anchor();
        append_anchor(&mut buf, right, width);
        buf.push(b'\n');
        saved.remove_settings();
        buf.extend_from_slice(&saved.bytes());
    }
    if state.fixed_count > 0 {
        // SAFETY: fixed_count measures the contiguous records, each basename comes from libfm.
        for item in unsafe { slice::from_raw_parts(state.fixed, state.fixed_count) } {
            // SAFETY: the model owns this NUL-terminated basename.
            append_item(
                &mut buf,
                unsafe { CStr::from_ptr(item.basename) }.to_bytes(),
                item.x,
                item.y,
            );
        }
    }
    if let Some(saved) = saved.as_ref().filter(|_| active) {
        if let Some(current) = Profile::parse(&buf) {
            let mut present = HashSet::new();
            if state.present_count > 0 {
                // SAFETY: present_count measures the array of libfm names owned by the model.
                for &name in unsafe { slice::from_raw_parts(state.present, state.present_count) } {
                    // SAFETY: the caller provided a live, non-null libfm name.
                    present.insert(unsafe { CStr::from_ptr(name) }.to_bytes());
                }
            }
            merge_absent(saved, &current, &present, state.loaded != 0, state.grid);
            buf = current.bytes();
        }
    }
    save(path, &buf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rect;

    fn grid() -> Grid {
        Grid {
            work: Rect {
                x: 0,
                y: 0,
                width: 1000,
                height: 600,
            },
            x_margin: 2,
            y_margin: 2,
            cell_width: 100,
            cell_height: 84,
            right: 1,
        }
    }

    #[test]
    fn positions_follow_anchor_and_keep_non_utf8_filenames() {
        let p = Profile::parse(
            b"[*]\nemwm_anchor_right=true\nemwm_workarea_width=800\n[na\xffme]\nx=600\ny=10\n",
        )
        .unwrap();
        let group = c"na\xffme";
        let pos = p.read_position(group, true, grid());
        assert_eq!((pos.found, pos.x, pos.y), (1, 800, 10));
        let pos = p.read_position(group, false, grid());
        assert_eq!((pos.x, pos.y), (600, 10));
    }

    #[test]
    fn loaded_models_restore_only_absent_remote_groups() {
        let previous = Profile::parse(
            b"[*]\n[trash:///]\nx=8\ny=3\n[volume:///a]\nx=5\ny=7\n[removed]\nx=9\n[fixed]\nx=2\n",
        )
        .unwrap();
        let current = Profile::parse(b"[*]\n[fixed]\nx=15\ny=20\n").unwrap();
        let present = HashSet::from([&b"volume:///a"[..]]);
        merge_absent(&previous, &current, &present, true, grid());
        assert_eq!(current.integer(c"trash:///", c"x"), 892);
        assert!(!current.has(c"volume:///a"));
        assert!(!current.has(c"removed"));
        assert_eq!(current.integer(c"fixed", c"x"), 15);
        merge_absent(&previous, &current, &present, false, grid());
        assert!(current.has(c"removed"));
    }

    #[test]
    fn legacy_group_escaping_matches_pcmanfm() {
        let mut buf = Vec::new();
        append_item(&mut buf, b"A\xff\r\n[]\\", -3, 2);
        assert_eq!(buf, b"[A\xff\\r\\n\\[\\]\\\\]\nx=-3\ny=2\n\n");
    }
    #[test]
    fn empty_key_file_and_item_tables_round_trip() {
        let p = Profile::parse(b"").unwrap();
        assert!(p.groups().is_empty());
        assert!(p.bytes().is_empty());
        // SAFETY: missing readings use a null handle; no ownership is transferred.
        unsafe {
            assert_eq!(
                emwm_profile_position(std::ptr::null(), std::ptr::null(), 1, grid()).found,
                0
            );
            emwm_profile_free(null_mut());
        }
    }

    #[test]
    fn output_profile_migrates_once_and_keeps_positions_on_inactive_monitors() {
        use std::fs;
        let tmp = std::env::temp_dir().join(format!("emwm-pcmanfm-profile-{}", std::process::id()));
        fs::create_dir(&tmp).unwrap();
        let directory = CString::new(tmp.as_os_str().as_bytes()).unwrap();
        let original =
            b"[*]\nemwm_anchor_right=true\nemwm_workarea_width=800\n[icon]\nx=123\ny=456\n";
        let legacy = tmp.join("desktop-items-0.conf");
        fs::write(&legacy, original).unwrap();
        let legacy_c = CString::new(legacy.as_os_str().as_bytes()).unwrap();
        let named = tmp.join("desktop-output-HDMI%20%2F%202.conf");
        let config = c"[*]\nwallpaper=new\n";
        let state = DesktopState {
            config: config.as_ptr(),
            configured: 1,
            layout: 1,
            active: 0,
            loaded: 0,
            grid: grid(),
            fixed: std::ptr::null(),
            fixed_count: 0,
            present: std::ptr::null(),
            present_count: 0,
        };
        // SAFETY: all C strings are valid, the empty arrays have zero counts and
        // both returned filenames are freed with the same GLib allocator.
        unsafe {
            let path =
                emwm_named_profile(directory.as_ptr(), c"HDMI / 2".as_ptr(), legacy_c.as_ptr());
            assert_eq!(
                CStr::from_ptr(path).to_bytes(),
                named.as_os_str().as_bytes()
            );
            assert_eq!(fs::read(&named).unwrap(), original);
            emwm_save_positions(path, &state);
            let profile = emwm_profile_load(path);
            assert!(!profile.is_null());
            let point = emwm_profile_position(profile, c"icon".as_ptr(), 0, grid());
            assert_eq!((point.found, point.x, point.y), (1, 123, 456));
            emwm_profile_free(profile);
            glib::g_free(path.cast());

            let migrated = fs::read(&named).unwrap();
            assert!(migrated.windows(13).any(|s| s == b"wallpaper=new"));
            let again =
                emwm_named_profile(directory.as_ptr(), c"HDMI / 2".as_ptr(), legacy_c.as_ptr());
            assert_eq!(fs::read(&named).unwrap(), migrated);
            glib::g_free(again.cast());
        }
        assert_eq!(fs::read(&legacy).unwrap(), original);
        fs::remove_dir_all(tmp).unwrap();
    }
}
