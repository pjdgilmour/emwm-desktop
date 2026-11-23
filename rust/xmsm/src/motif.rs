// SPDX-License-Identifier: MIT
//! Intrinsics and Motif resources are passed by value in native longs.
use std::ffi::{c_char, c_long, c_void, CStr};
use x11::xt::{self, Widget, WidgetClass};
pub use xt::Widget as W;
pub type Callback = unsafe extern "C" fn(Widget, *mut c_void, *mut c_void);
#[expect(
    improper_ctypes,
    reason = "x11-rs models opaque Xt handles as pointers to empty records"
)]
extern "C" {
    pub static topLevelShellWidgetClass: WidgetClass;
}
#[link(name = "Xm")]
#[expect(
    improper_ctypes,
    reason = "x11-rs models opaque Xt handles as pointers to empty records"
)]
extern "C" {
    pub static xmDrawingAreaWidgetClass: WidgetClass;
    pub static xmFrameWidgetClass: WidgetClass;
    pub static xmRowColumnWidgetClass: WidgetClass;
    pub static xmLabelGadgetClass: WidgetClass;
    pub static xmTextFieldWidgetClass: WidgetClass;
    pub static xmDialogShellWidgetClass: WidgetClass;
    pub static xmFormWidgetClass: WidgetClass;
    pub static xmSeparatorGadgetClass: WidgetClass;
    pub static xmToggleButtonGadgetClass: WidgetClass;
    pub static xmPushButtonGadgetClass: WidgetClass;
    fn XmStringCreateLocalized(text: *const c_char) -> *mut c_void;
    fn XmStringFree(s: *mut c_void);
    pub fn XmToggleButtonGadgetGetState(w: Widget) -> u8;
    pub fn XmTextFieldSetString(w: Widget, s: *const c_char);
    pub fn XmTextFieldGetLastPosition(w: Widget) -> c_long;
    pub fn XmProcessTraversal(w: Widget, direction: i32) -> u8;
    pub fn XmMessageBoxGetChild(w: Widget, which: u8) -> Widget;
    pub fn XmCreateErrorDialog(
        parent: Widget,
        name: *const c_char,
        args: *mut xt::Arg,
        n: u32,
    ) -> Widget;
    pub fn XmGetColors(
        screen: *mut x11::xlib::Screen,
        cm: std::ffi::c_ulong,
        bg: std::ffi::c_ulong,
        fg: *mut std::ffi::c_ulong,
        top: *mut std::ffi::c_ulong,
        bot: *mut std::ffi::c_ulong,
        sel: *mut std::ffi::c_ulong,
    );
    pub fn XmGetPixmap(
        screen: *mut x11::xlib::Screen,
        name: *const c_char,
        fg: std::ffi::c_ulong,
        bg: std::ffi::c_ulong,
    ) -> std::ffi::c_ulong;
    pub fn XmAddProtocolCallback(
        w: Widget,
        property: std::ffi::c_ulong,
        protocol: std::ffi::c_ulong,
        cb: Callback,
        data: *mut c_void,
    );
    pub fn XmAddProtocols(
        w: Widget,
        property: std::ffi::c_ulong,
        atoms: *mut std::ffi::c_ulong,
        n: u32,
    );
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Arg {
    name: *const c_char,
    value: c_long,
}
pub fn arg(name: &'static CStr, value: c_long) -> Arg {
    Arg {
        name: name.as_ptr(),
        value,
    }
}
#[macro_export]
macro_rules! args { ($($name:literal => $value:expr),* $(,)?) => {[$($crate::motif::arg($name,$value as std::ffi::c_long)),*]}; }
pub struct Label(*mut c_void);
impl Label {
    pub fn new(s: &CStr) -> Self {
        // SAFETY: XmString creates an independent copy.
        Self(unsafe { XmStringCreateLocalized(s.as_ptr()) })
    }
    pub fn ptr(&self) -> *mut c_void {
        self.0
    }
}
impl Drop for Label {
    fn drop(&mut self) {
        unsafe { XmStringFree(self.0) }
    }
}
// SAFETY: all callers supply a live Xt widget and valid resource lists whose
// C layout matches Arg in Intrinsic.h. Resource arrays are copied by Xt.
pub unsafe fn child(parent: Widget, name: &CStr, class: WidgetClass, attrs: &mut [Arg]) -> Widget {
    xt::XtCreateManagedWidget(
        name.as_ptr(),
        class,
        parent,
        attrs.as_mut_ptr().cast(),
        attrs.len() as u32,
    )
}
pub unsafe fn popup(parent: Widget, name: &CStr, class: WidgetClass, attrs: &mut [Arg]) -> Widget {
    xt::XtCreatePopupShell(
        name.as_ptr(),
        class,
        parent,
        attrs.as_mut_ptr().cast(),
        attrs.len() as u32,
    )
}
pub unsafe fn set(w: Widget, attrs: &mut [Arg]) {
    xt::XtSetValues(w, attrs.as_mut_ptr().cast(), attrs.len() as u32);
}
pub unsafe fn dims(w: Widget) -> (u16, u16) {
    let (mut width, mut height) = (0u16, 0u16);
    let mut attrs = args![c"width"=>&mut width as *mut u16,c"height"=>&mut height as *mut u16];
    xt::XtGetValues(w, attrs.as_mut_ptr().cast(), 2);
    (width, height)
}
pub unsafe fn text(w: Widget, s: &CStr) {
    let s = Label::new(s);
    set(w, &mut args![c"labelString"=>s.ptr()]);
}
pub fn label_child(
    parent: Widget,
    name: &CStr,
    class: WidgetClass,
    title: &CStr,
    attrs: &mut Vec<Arg>,
) -> Widget {
    let s = Label::new(title);
    attrs.push(arg(c"labelString", s.ptr() as c_long));
    unsafe { child(parent, name, class, attrs) }
}
