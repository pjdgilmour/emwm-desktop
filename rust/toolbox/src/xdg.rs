// SPDX-License-Identifier: MIT
//! Take a fresh Garcon snapshot each time Motif posts the menu. GIO handles
//! launch semantics (including field codes and D-Bus activation).
use gio::prelude::*;
use glib::translate::{from_glib_full, from_glib_none, IntoGlibPtr, ToGlibPtr};
use std::collections::HashSet;
use std::ffi::{c_char, c_int, c_void, CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr::null_mut;

type Object = *mut glib::gobject_ffi::GObject;
#[link(name = "garcon-1")]
extern "C" {
    fn garcon_set_environment(desktop: *const c_char);
    fn garcon_menu_new_for_path(path: *const c_char) -> Object;
    fn garcon_menu_load(
        menu: Object,
        cancel: *mut c_void,
        error: *mut *mut glib::ffi::GError,
    ) -> c_int;
    fn garcon_menu_get_elements(menu: Object) -> *mut glib::ffi::GList;
    fn garcon_menu_get_type() -> usize;
    fn garcon_menu_get_directory(menu: Object) -> Object;
    fn garcon_menu_directory_get_visible(directory: Object) -> c_int;
    fn garcon_menu_element_get_name(element: Object) -> *const c_char;
    fn garcon_menu_element_get_icon_name(element: Object) -> *const c_char;
    fn garcon_menu_separator_get_type() -> usize;
    fn garcon_menu_item_get_type() -> usize;
    fn garcon_menu_item_get_desktop_id(item: Object) -> *const c_char;
    fn garcon_menu_item_get_hidden(item: Object) -> c_int;
    fn garcon_menu_item_get_no_display(item: Object) -> c_int;
    fn garcon_menu_item_get_show_in_environment(item: Object) -> c_int;
    fn garcon_menu_item_get_file(item: Object) -> *mut gio::ffi::GFile;
    fn garcon_menu_item_cache_get_default() -> Object;
    fn garcon_menu_item_cache_invalidate(cache: Object);
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Kind {
    Menu,
    Application,
    Separator,
}

/// Layout is shared with emwm-utils' tbxdg.h. Box owns the record. Each string
/// has one CString owner, each app one GLib reference, and each array owns its
/// child records through the callback installed at allocation.
#[repr(C)]
pub(crate) struct Entry {
    pub(crate) kind: Kind,
    pub(crate) name: *mut c_char,
    icon: *mut c_char,
    desktop_id: *mut c_char,
    pub(crate) app: *mut gio::ffi::GDesktopAppInfo,
    pub(crate) children: *mut glib::ffi::GPtrArray,
}

impl Entry {
    fn new(kind: Kind) -> Box<Self> {
        Box::new(Self {
            kind,
            name: null_mut(),
            icon: null_mut(),
            desktop_id: null_mut(),
            app: null_mut(),
            children: null_mut(),
        })
    }
    unsafe fn from_element(kind: Kind, obj: Object) -> Box<Self> {
        // SAFETY: obj is a Garcon menu element retained by the current menu.
        let copy = |s: *const c_char| {
            if s.is_null() {
                null_mut()
            } else {
                CStr::from_ptr(s).to_owned().into_raw()
            }
        };
        let mut entry = Self::new(kind);
        entry.name = copy(garcon_menu_element_get_name(obj));
        entry.icon = copy(garcon_menu_element_get_icon_name(obj));
        if kind == Kind::Menu {
            entry.children = glib::ffi::g_ptr_array_new_with_free_func(Some(free_child));
        }
        entry
    }
    fn add(&mut self, child: Box<Entry>) {
        // SAFETY: only menus use add, and their array takes ownership of each Box.
        unsafe { glib::ffi::g_ptr_array_add(self.children, Box::into_raw(child).cast()) };
    }
    pub(crate) fn items(&self) -> &[*mut Entry] {
        // SAFETY: Garcon snapshots are immutable while read by Motif's Xt thread.
        unsafe {
            if self.children.is_null() || (*self.children).len == 0 {
                return &[];
            }
            std::slice::from_raw_parts((*self.children).pdata.cast(), (*self.children).len as usize)
        }
    }
}
impl Drop for Entry {
    fn drop(&mut self) {
        // SAFETY: all fields are uniquely owned here under the tb_xdg_free contract.
        unsafe {
            for p in [self.name, self.icon, self.desktop_id] {
                if !p.is_null() {
                    drop(CString::from_raw(p));
                }
            }
            if !self.app.is_null() {
                glib::gobject_ffi::g_object_unref(self.app.cast());
            }
            if !self.children.is_null() {
                glib::ffi::g_ptr_array_unref(self.children);
            }
        }
    }
}
unsafe extern "C" fn free_child(p: *mut c_void) {
    // SAFETY: the GPtrArray holds only Entry pointers transferred from Box.
    tb_xdg_free(p.cast());
}
#[no_mangle]
pub(crate) unsafe extern "C" fn tb_xdg_free(p: *mut Entry) {
    // SAFETY: C callers return the root obtained from tb_xdg_load exactly once.
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}

unsafe fn copy_menu(menu: Object, seen: &mut HashSet<CString>) -> Box<Entry> {
    // SAFETY: menu owns all elements. The list links are caller-owned, not the elements.
    let mut tree = Entry::from_element(Kind::Menu, menu);
    let head = garcon_menu_get_elements(menu);
    let mut node = head;
    let mut separator = false;
    while !node.is_null() {
        let obj = (*node).data.cast();
        node = (*node).next;
        let is = |ty| glib::gobject_ffi::g_type_check_instance_is_a(obj, ty) != 0;
        let obj = obj.cast();
        let item = if is(garcon_menu_separator_get_type()) {
            separator = !tree.items().is_empty();
            continue;
        } else if is(garcon_menu_get_type()) {
            let dir = garcon_menu_get_directory(obj);
            if !dir.is_null() && garcon_menu_directory_get_visible(dir) == 0 {
                continue;
            }
            let child = copy_menu(obj, seen);
            if child.items().is_empty() {
                continue;
            }
            child
        } else if is(garcon_menu_item_get_type()) {
            let id = garcon_menu_item_get_desktop_id(obj);
            if id.is_null()
                || seen.contains(CStr::from_ptr(id))
                || garcon_menu_item_get_hidden(obj) != 0
                || garcon_menu_item_get_no_display(obj) != 0
                || garcon_menu_item_get_show_in_environment(obj) == 0
            {
                continue;
            }
            let file: gio::File = from_glib_full(garcon_menu_item_get_file(obj));
            let Some(app) = file.path().and_then(gio::DesktopAppInfo::from_filename) else {
                continue;
            };
            let id = CStr::from_ptr(id).to_owned();
            let mut entry = Entry::from_element(Kind::Application, obj);
            entry.app = app.into_glib_ptr();
            entry.desktop_id = id.clone().into_raw();
            seen.insert(id);
            entry
        } else {
            continue;
        };
        if separator {
            tree.add(Entry::new(Kind::Separator));
            separator = false;
        }
        tree.add(item);
    }
    glib::ffi::g_list_free(head);
    tree
}

fn default_menu() -> PathBuf {
    let mut name = std::env::var_os("XDG_MENU_PREFIX").unwrap_or_default();
    name.push("applications.menu");
    std::iter::once(glib::user_config_dir())
        .chain(glib::system_config_dirs())
        .map(|d| d.join("menus").join(&name))
        .find(|path| path.is_file())
        .unwrap_or_else(|| {
            Path::new(option_env!("EMWM_RCDIR").unwrap_or("/usr/share/emwm-desktop"))
                .join("emwm-applications.menu")
        })
}

#[no_mangle]
pub(crate) unsafe extern "C" fn tb_xdg_load(
    filename: *const c_char,
    error: *mut *mut glib::ffi::GError,
) -> *mut Entry {
    // SAFETY: callers provide an optional NUL-terminated pathname and optional GError output.
    let path = if filename.is_null() || *filename == 0 {
        default_menu()
    } else {
        PathBuf::from(OsStr::from_bytes(CStr::from_ptr(filename).to_bytes()))
    };
    let desktop = std::env::var_os("XDG_CURRENT_DESKTOP")
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "EMWM".into());
    let Ok(desktop) = CString::new(desktop.as_bytes()) else {
        return null_mut();
    };
    garcon_set_environment(desktop.as_ptr());
    let menu: glib::Object = from_glib_full(garcon_menu_new_for_path(path.to_glib_none().0));
    let cache: glib::Object = from_glib_full(garcon_menu_item_cache_get_default());
    garcon_menu_item_cache_invalidate(cache.as_ptr());
    if garcon_menu_load(menu.as_ptr(), null_mut(), error) == 0 {
        return null_mut();
    }
    Box::into_raw(copy_menu(menu.as_ptr(), &mut HashSet::new()))
}

#[no_mangle]
pub(crate) unsafe extern "C" fn tb_xdg_launch(
    app: *mut gio::ffi::GDesktopAppInfo,
    display: *const c_char,
    error: *mut *mut glib::ffi::GError,
) -> c_int {
    // SAFETY: the menu retains app while launching. GIO copies the display into its context.
    let original: gio::DesktopAppInfo = from_glib_none(app);
    let filename = original.filename().unwrap_or_default();
    let status = if let Some(current) = gio::DesktopAppInfo::from_filename(&filename) {
        let context = gio::AppLaunchContext::new();
        if !display.is_null() {
            gio::ffi::g_app_launch_context_setenv(context.as_ptr(), c"DISPLAY".as_ptr(), display);
        }
        current.launch(&[], Some(&context))
    } else {
        Err(glib::Error::new(
            gio::IOErrorEnum::NotFound,
            &format!("Application is no longer available: {}", filename.display()),
        ))
    };
    match status {
        Ok(()) => 1,
        Err(e) => {
            if !error.is_null() {
                *error = e.into_glib_ptr();
            }
            0
        }
    }
}
