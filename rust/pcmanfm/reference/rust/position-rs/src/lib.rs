//! C ABI for `position.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `int anchor_x(int x, int old_width, int was_right, int origin, int width, int cell, int right)` (`position.c:9`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn anchor_x(
    x: std::ffi::c_int,
    old_width: std::ffi::c_int,
    was_right: std::ffi::c_int,
    origin: std::ffi::c_int,
    width: std::ffi::c_int,
    cell: std::ffi::c_int,
    right: std::ffi::c_int,
) -> std::ffi::c_int {
    port::anchor_x(x, old_width, was_right, origin, width, cell, right)
}

/// `anchor_x` as `rust_anything_anchor_x`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_anchor_x(
    x: std::ffi::c_int,
    old_width: std::ffi::c_int,
    was_right: std::ffi::c_int,
    origin: std::ffi::c_int,
    width: std::ffi::c_int,
    cell: std::ffi::c_int,
    right: std::ffi::c_int,
) -> std::ffi::c_int {
    port::anchor_x(x, old_width, was_right, origin, width, cell, right)
}

/// `int snap_axis(int value, int origin, int cell)` (`position.c:19`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn snap_axis(
    value: std::ffi::c_int,
    origin: std::ffi::c_int,
    cell: std::ffi::c_int,
) -> std::ffi::c_int {
    port::snap_axis(value, origin, cell)
}

/// `snap_axis` as `rust_anything_snap_axis`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_snap_axis(
    value: std::ffi::c_int,
    origin: std::ffi::c_int,
    cell: std::ffi::c_int,
) -> std::ffi::c_int {
    port::snap_axis(value, origin, cell)
}

/// `int drag_axis(int value, int origin, int width, int margin, int item)` (`position.c:27`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn drag_axis(
    value: std::ffi::c_int,
    origin: std::ffi::c_int,
    width: std::ffi::c_int,
    margin: std::ffi::c_int,
    item: std::ffi::c_int,
) -> std::ffi::c_int {
    port::drag_axis(value, origin, width, margin, item)
}

/// `drag_axis` as `rust_anything_drag_axis`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_drag_axis(
    value: std::ffi::c_int,
    origin: std::ffi::c_int,
    width: std::ffi::c_int,
    margin: std::ffi::c_int,
    item: std::ffi::c_int,
) -> std::ffi::c_int {
    port::drag_axis(value, origin, width, margin, item)
}
