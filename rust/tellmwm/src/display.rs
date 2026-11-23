// Based on tellmwm 1.2 by alx@fastestcode.org, https://fastestcode.org/emwm.html
use std::ffi::{CStr, CString};
use std::os::raw::c_ulong;
use std::{mem::MaybeUninit, ptr, slice};
use x11::xlib as x;

pub type Id = c_ulong;
pub struct Display(*mut x::Display);

impl Display {
    pub fn open() -> Option<Self> {
        // SAFETY: a null name tells Xlib to use DISPLAY.
        let dpy = unsafe { x::XOpenDisplay(ptr::null()) };
        if dpy.is_null() {
            None
        } else {
            Some(Self(dpy))
        }
    }

    pub fn root(&self) -> Id {
        // SAFETY: self owns an open display.
        unsafe { x::XDefaultRootWindow(self.0) }
    }

    pub fn atom(&self, name: &[u8]) -> Option<Id> {
        let name = CString::new(name).ok()?;
        // SAFETY: the display and NUL-terminated name live throughout the call.
        let id = unsafe { x::XInternAtom(self.0, name.as_ptr(), x::True) };
        (id != 0).then_some(id)
    }

    pub fn name(&self, id: Id) -> Option<Vec<u8>> {
        if id == 0 {
            return None;
        }
        // SAFETY: Xlib returns an owned NUL-terminated string or NULL.
        unsafe {
            let name = x::XGetAtomName(self.0, id);
            if name.is_null() {
                return None;
            }
            let bytes = CStr::from_ptr(name).to_bytes().to_vec();
            x::XFree(name.cast());
            Some(bytes)
        }
    }

    pub fn words(&self, window: Id, key: &[u8], kind: Option<Id>, limit: i64) -> Option<Vec<Id>> {
        let property = self.atom(key)?;
        let kind = kind.unwrap_or(property);
        let (mut actual, mut format, mut count, mut left) = (0, 0, 0, 0);
        let mut data = ptr::null_mut();
        // SAFETY: all output parameters point to live storage. For format 32,
        // Xlib expands each wire value to a native unsigned long.
        unsafe {
            let status = x::XGetWindowProperty(
                self.0,
                window,
                property,
                0,
                limit as _,
                x::False,
                kind,
                &mut actual,
                &mut format,
                &mut count,
                &mut left,
                &mut data,
            );
            let words = if status == 0 && actual == kind && format == 32 && !data.is_null() {
                Some(slice::from_raw_parts(data.cast::<Id>(), count as usize).to_vec())
            } else {
                None
            };
            if !data.is_null() {
                x::XFree(data.cast());
            }
            words
        }
    }

    pub fn text_list(&self, window: Id, property: Id) -> Option<Vec<Vec<u8>>> {
        let mut text = MaybeUninit::uninit();
        let mut list = ptr::null_mut();
        let mut count = 0;
        // SAFETY: XGetTextProperty initializes text on success. Xlib owns both
        // allocations until we copy the converted strings and release them.
        unsafe {
            if x::XGetTextProperty(self.0, window, text.as_mut_ptr(), property) == 0 {
                return None;
            }
            let text = text.assume_init();
            x::XmbTextPropertyToTextList(self.0, &text, &mut list, &mut count);
            let fields = if list.is_null() || count < 0 {
                None
            } else {
                Some(
                    slice::from_raw_parts(list, count as usize)
                        .iter()
                        .map(|&s| CStr::from_ptr(s).to_bytes().to_vec())
                        .collect(),
                )
            };
            if !list.is_null() {
                x::XFreeStringList(list);
            }
            x::XFree(text.value.cast());
            fields
        }
    }

    pub fn color(&self, name: &[u8]) -> Option<Id> {
        let name = CString::new(name).ok()?;
        let mut nearest = MaybeUninit::uninit();
        let mut exact = MaybeUninit::uninit();
        // SAFETY: successful allocation initializes nearest, using the open display's colormap.
        unsafe {
            let cmap = x::XDefaultColormap(self.0, x::XDefaultScreen(self.0));
            if x::XAllocNamedColor(
                self.0,
                cmap,
                name.as_ptr(),
                nearest.as_mut_ptr(),
                exact.as_mut_ptr(),
            ) == 0
            {
                None
            } else {
                Some(nearest.assume_init().pixel)
            }
        }
    }

    pub fn send(&self, wm: Id, mut request: Vec<u8>) -> bool {
        let Some(atom) = self.atom(b"_MWM_WM_REQUEST") else {
            return false;
        };
        request.push(0);
        let Ok(length) = request.len().try_into() else {
            return false;
        };
        // SAFETY: XChangeProperty copies length bytes from the request before returning.
        unsafe {
            x::XChangeProperty(
                self.0,
                wm,
                atom,
                x::XA_STRING,
                8,
                x::PropModeAppend,
                request.as_ptr(),
                length,
            )
        };
        true
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        // SAFETY: this uniquely owned display is closed once and flushes pending requests.
        unsafe {
            x::XCloseDisplay(self.0);
        }
    }
}
