// SPDX-License-Identifier: MIT
use std::{
    ffi::{c_char, c_ulong},
    mem::{offset_of, size_of},
    ptr::null_mut,
};
use x11::xt;
#[repr(C)]
pub struct Resources {
    pub locking: u8,
    pub suspend: u8,
    pub shade: u8,
    pub blank_on_lock: u8,
    pub numlock: *mut c_char,
    pub image: *mut c_char,
    pub color: c_ulong,
    pub wm: *mut c_char,
    pub launcher: *mut c_char,
    pub lock_timeout: u32,
    pub unlock_timeout: u32,
    pub blank_timeout: u32,
    pub primary: u32,
    pub shutdown: u8,
    pub reboot: u8,
    pub lock_on_suspend: u8,
    pub silent: u8,
}
#[repr(C)]
struct Resource {
    name: *const c_char,
    class: *const c_char,
    kind: *const c_char,
    size: u32,
    offset: u32,
    default_kind: *const c_char,
    default: usize,
}
impl Resources {
    pub fn read(w: xt::Widget) -> Self {
        // SAFETY: each field is an integer or a nullable raw pointer. Xt fills
        // each resource from its descriptor or its default before returning.
        let mut out: Self = unsafe { std::mem::zeroed() };
        macro_rules! res {
            ($name:literal,$class:literal,$kind:literal,$ty:ty,$field:ident,$default_type:literal,$default:expr) => {
                Resource {
                    name: $name.as_ptr(),
                    class: $class.as_ptr(),
                    kind: $kind.as_ptr(),
                    size: size_of::<$ty>() as u32,
                    offset: offset_of!(Resources, $field) as u32,
                    default_kind: $default_type.as_ptr(),
                    default: $default as usize,
                }
            };
        }
        let mut table = [
            res!(
                c"enableShade",
                c"EnableShade",
                c"Boolean",
                u8,
                shade,
                c"Immediate",
                1usize
            ),
            res!(
                c"enableLocking",
                c"EnableLocking",
                c"Boolean",
                u8,
                locking,
                c"Immediate",
                1usize
            ),
            res!(
                c"enableSuspend",
                c"EnableSuspend",
                c"Boolean",
                u8,
                suspend,
                c"Immediate",
                1usize
            ),
            res!(
                c"blankOnLock",
                c"BlankOnLock",
                c"Boolean",
                u8,
                blank_on_lock,
                c"Immediate",
                1usize
            ),
            res!(
                c"numLockState",
                c"NumLockState",
                c"String",
                *mut c_char,
                numlock,
                c"Immediate",
                c"on".as_ptr()
            ),
            res!(
                c"lockBackgroundImage",
                c"LockBackgroundImage",
                c"String",
                *mut c_char,
                image,
                c"Immediate",
                0usize
            ),
            res!(
                c"lockBackgroundColor",
                c"LockBackgroundColor",
                c"Pixel",
                c_ulong,
                color,
                c"String",
                c"#4C719E".as_ptr()
            ),
            res!(
                c"launcher",
                c"Launcher",
                c"String",
                *mut c_char,
                launcher,
                c"Immediate",
                c"/usr/lib/emwm-desktop/bin/xmtoolbox".as_ptr()
            ),
            res!(
                c"windowManager",
                c"WindowManager",
                c"String",
                *mut c_char,
                wm,
                c"Immediate",
                c"/usr/lib/emwm-desktop/bin/emwm".as_ptr()
            ),
            res!(
                c"blankTimeout",
                c"BlankTimeout",
                c"Int",
                u32,
                blank_timeout,
                c"Immediate",
                400usize
            ),
            res!(
                c"lockTimeout",
                c"LockTimeout",
                c"Int",
                u32,
                lock_timeout,
                c"Immediate",
                600usize
            ),
            res!(
                c"unlockScreenTimeout",
                c"UnlockScreenTimeout",
                c"Int",
                u32,
                unlock_timeout,
                c"Immediate",
                8usize
            ),
            res!(
                c"primaryXineramaScreen",
                c"PrimaryXineramaScreen",
                c"Int",
                u32,
                primary,
                c"Immediate",
                0usize
            ),
            res!(
                c"lockOnSuspend",
                c"LockOnSuspend",
                c"Boolean",
                u8,
                lock_on_suspend,
                c"Immediate",
                0usize
            ),
            res!(
                c"showShutdown",
                c"ShowShutdown",
                c"Boolean",
                u8,
                shutdown,
                c"Immediate",
                1usize
            ),
            res!(
                c"showReboot",
                c"ShowReboot",
                c"Boolean",
                u8,
                reboot,
                c"Immediate",
                1usize
            ),
            res!(
                c"silent",
                c"Silent",
                c"Boolean",
                u8,
                silent,
                c"Immediate",
                0usize
            ),
        ];
        // SAFETY: the Xt descriptors address fields in this repr(C) value with their exact size.
        unsafe {
            xt::XtGetApplicationResources(
                w,
                (&mut out as *mut Self).cast(),
                table.as_mut_ptr().cast(),
                table.len() as u32,
                null_mut(),
                0,
            );
        }
        out
    }
}
