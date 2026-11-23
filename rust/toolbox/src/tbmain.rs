// Copyright (C) 2018–2026 alx@fastestcode.org. SPDX-License-Identifier: MIT
// Ported from emwm-utils tbmain.c through C2Rust 0.22.1. Xt runs these callbacks
// on its application thread, retaining the original Motif widget and resource ABI.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_mut,
    unused_variables,
    unused_assignments
)]
use crate::applications::tb_xdg_menu_create;
use crate::commands::{exec_command, expand_env_vars, get_login, print_version, rsignal};
#[repr(C)]
pub struct _XGC {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _XDisplay {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _XPrivate {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _XrmHashBucketRec {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _WidgetRec {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _WidgetClassRec {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _XtAppStruct {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _TranslationData {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct __XmStringRec {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct __XmParseMappingRec {
    _opaque: [u8; 0],
}
extern "C" {
    fn malloc(__size: size_t) -> *mut libc::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut libc::c_void;
    fn realloc(__ptr: *mut libc::c_void, __size: size_t) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(__status: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(__stream: *mut FILE, __format: *const libc::c_char, ...) -> libc::c_int;
    fn sprintf(__s: *mut libc::c_char, __format: *const libc::c_char, ...) -> libc::c_int;
    fn snprintf(
        __s: *mut libc::c_char,
        __maxlen: size_t,
        __format: *const libc::c_char,
        ...
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
    fn _exit(__status: libc::c_int) -> !;
    fn setsid() -> __pid_t;
    fn vfork() -> libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
    fn XInternAtom(_: *mut Display, _: *const libc::c_char, _: libc::c_int) -> Atom;
    fn XCreatePixmapFromBitmapData(
        _: *mut Display,
        _: Drawable,
        _: *mut libc::c_char,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: libc::c_uint,
    ) -> Pixmap;
    fn XStringToKeysym(_: *const libc::c_char) -> KeySym;
    fn XDefaultScreenOfDisplay(_: *mut Display) -> *mut Screen;
    fn XScreenNumberOfScreen(_: *mut Screen) -> libc::c_int;
    fn XSetErrorHandler(_: XErrorHandler) -> XErrorHandler;
    fn XAllocNamedColor(
        _: *mut Display,
        _: Colormap,
        _: *const libc::c_char,
        _: *mut XColor,
        _: *mut XColor,
    ) -> libc::c_int;
    fn XChangeProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: Atom,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_uchar,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XFlush(_: *mut Display) -> libc::c_int;
    fn XFree(_: *mut libc::c_void) -> libc::c_int;
    fn XGetWindowProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: libc::c_long,
        _: libc::c_long,
        _: libc::c_int,
        _: Atom,
        _: *mut Atom,
        _: *mut libc::c_int,
        _: *mut libc::c_ulong,
        _: *mut libc::c_ulong,
        _: *mut *mut libc::c_uchar,
    ) -> libc::c_int;
    fn XGetWindowAttributes(_: *mut Display, _: Window, _: *mut XWindowAttributes) -> libc::c_int;
    fn XGrabKey(
        _: *mut Display,
        _: libc::c_int,
        _: libc::c_uint,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XKeysymToKeycode(_: *mut Display, _: KeySym) -> KeyCode;
    fn XMapWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XQueryTree(
        _: *mut Display,
        _: Window,
        _: *mut Window,
        _: *mut Window,
        _: *mut *mut Window,
        _: *mut libc::c_uint,
    ) -> libc::c_int;
    fn XRaiseWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XSelectInput(_: *mut Display, _: Window, _: libc::c_long) -> libc::c_int;
    fn XSendEvent(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_long,
        _: *mut XEvent,
    ) -> libc::c_int;
    fn XSetInputFocus(_: *mut Display, _: Window, _: libc::c_int, _: Time) -> libc::c_int;
    fn XSync(_: *mut Display, _: libc::c_int) -> libc::c_int;
    fn XTranslateCoordinates(
        _: *mut Display,
        _: Window,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut Window,
    ) -> libc::c_int;
    fn memcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn memmove(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strcpy(__dest: *mut libc::c_char, __src: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(__s: *mut libc::c_char, __delim: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(__s: *const libc::c_char) -> size_t;
    fn strerror(__errnum: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    pub(super) fn XtManageChild(_: Widget);
    pub(super) fn XtUnmanageChild(_: Widget);
    fn XtDispatchEvent(_: *mut XEvent) -> Boolean;
    fn XtIsManaged(_: Widget) -> Boolean;
    fn XtParseTranslationTable(_: *const libc::c_char) -> XtTranslations;
    fn XtOverrideTranslations(_: Widget, _: XtTranslations);
    fn XtAppProcessEvent(_: XtAppContext, _: XtInputMask);
    fn XtLastTimestampProcessed(_: *mut Display) -> Time;
    fn XtAppAddTimeOut(
        _: XtAppContext,
        _: libc::c_ulong,
        _: XtTimerCallbackProc,
        _: XtPointer,
    ) -> XtIntervalId;
    fn XtRemoveTimeOut(_: XtIntervalId);
    fn XtAppAddSignal(_: XtAppContext, _: XtSignalCallbackProc, _: XtPointer) -> XtSignalId;
    fn XtNoticeSignal(_: XtSignalId);
    fn XtAppNextEvent(_: XtAppContext, _: *mut XEvent);
    fn XtRealizeWidget(_: Widget);
    pub(super) fn XtDestroyWidget(_: Widget);
    fn XtDisplay(_: Widget) -> *mut Display;
    fn XtScreen(_: Widget) -> *mut Screen;
    fn XtWindow(_: Widget) -> Window;
    pub(super) fn XtAddCallback(_: Widget, _: *const libc::c_char, _: XtCallbackProc, _: XtPointer);
    fn XtCreateWidget(
        _: *const libc::c_char,
        _: WidgetClass,
        _: Widget,
        _: ArgList,
        _: Cardinal,
    ) -> Widget;
    fn XtToolkitInitialize();
    fn XtSetLanguageProc(_: XtAppContext, _: XtLanguageProc, _: XtPointer) -> XtLanguageProc;
    fn XtVaAppInitialize(
        _: *mut XtAppContext,
        _: *const libc::c_char,
        _: XrmOptionDescList,
        _: Cardinal,
        _: *mut libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut String_0,
        ...
    ) -> Widget;
    fn XtDestroyApplicationContext(_: XtAppContext);
    fn XtGetApplicationResources(
        _: Widget,
        _: XtPointer,
        _: XtResourceList,
        _: Cardinal,
        _: ArgList,
        _: Cardinal,
    );
    pub(super) fn XtSetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtVaSetValues(_: Widget, ...);
    fn XtMalloc(_: Cardinal) -> *mut libc::c_char;
    fn XtFree(_: *mut libc::c_char);
    fn XmUpdateDisplay(w: Widget);
    pub(super) fn XmStringCreateLocalized(text: String_0) -> XmString;
    pub(super) fn XmStringFree(string: XmString);
    fn XmStringUnparse(
        string: XmString,
        tag: XmStringTag,
        tag_type: XmTextType,
        output_type: XmTextType,
        parse_table: XmParseTable,
        parse_count: Cardinal,
        parse_model: XmParseModel,
    ) -> XtPointer;
    fn XmProcessTraversal(w: Widget, dir: XmTraversalDirection) -> Boolean;
    fn XmCreateRowColumn(p: Widget, name: *mut libc::c_char, al: ArgList, ac: Cardinal) -> Widget;
    pub(super) fn XmCreatePulldownMenu(
        p: Widget,
        name: *mut libc::c_char,
        al: ArgList,
        ac: Cardinal,
    ) -> Widget;
    fn XmVaCreateManagedRowColumn(parent: Widget, name: *mut libc::c_char, ...) -> Widget;
    pub(super) fn XmCreatePushButtonGadget(
        parent: Widget,
        name: *mut libc::c_char,
        arglist: ArgList,
        argcount: Cardinal,
    ) -> Widget;
    pub(super) fn XmCreateCascadeButtonGadget(
        parent: Widget,
        name: *mut libc::c_char,
        al: ArgList,
        ac: Cardinal,
    ) -> Widget;
    pub(super) fn XmCreateSeparatorGadget(
        parent: Widget,
        name: *mut libc::c_char,
        arglist: ArgList,
        argcount: Cardinal,
    ) -> Widget;
    fn XmCreateFrame(
        parent: Widget,
        name: *mut libc::c_char,
        arglist: ArgList,
        argcount: Cardinal,
    ) -> Widget;
    fn XmVaCreateManagedFrame(parent: Widget, name: *mut libc::c_char, ...) -> Widget;
    fn XmCreateLabelGadget(
        parent: Widget,
        name: *mut libc::c_char,
        arglist: *mut Arg,
        argCount: Cardinal,
    ) -> Widget;
    fn XmSelectionBoxGetChild(sb: Widget, which: libc::c_uchar) -> Widget;
    fn XmCreatePromptDialog(
        ds_p: Widget,
        name: String_0,
        sb_args: ArgList,
        sb_n: Cardinal,
    ) -> Widget;
    fn XmTextFieldGetString(w: Widget) -> *mut libc::c_char;
    fn XmTextFieldSetSelection(
        w: Widget,
        first: XmTextPosition,
        last: XmTextPosition,
        sel_time: Time,
    );
    fn XmCreateMessageDialog(
        parent: Widget,
        name: *mut libc::c_char,
        al: ArgList,
        ac: Cardinal,
    ) -> Widget;
    pub(super) fn XmMessageBoxGetChild(widget: Widget, child: libc::c_uchar) -> Widget;
    fn XRRQueryExtension(
        dpy: *mut Display,
        event_base_return: *mut libc::c_int,
        error_base_return: *mut libc::c_int,
    ) -> libc::c_int;
    fn XRRQueryVersion(
        dpy: *mut Display,
        major_version_return: *mut libc::c_int,
        minor_version_return: *mut libc::c_int,
    ) -> libc::c_int;
    fn XRRSelectInput(dpy: *mut Display, window: Window, mask: libc::c_int);
    fn XRRGetMonitors(
        dpy: *mut Display,
        window: Window,
        get_active: libc::c_int,
        nmonitors: *mut libc::c_int,
    ) -> *mut XRRMonitorInfo;
    fn XRRFreeMonitors(monitors: *mut XRRMonitorInfo);
    fn __errno_location() -> *mut libc::c_int;
    fn tb_parse_config(filename: *const libc::c_char, ent_root: *mut *mut tb_entry) -> libc::c_int;
    fn tb_parser_error_string() -> *mut libc::c_char;
    fn TrayCreate(parent: Widget, shell: Widget) -> Widget;
    fn TrayStart();

    static mut switcherWidgetClass: WidgetClass;
    fn SwitcherSetActiveWorkspace(_: Widget, _: libc::c_ushort);
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __caddr_t = *mut libc::c_char;
pub type pid_t = __pid_t;
pub type caddr_t = __caddr_t;
pub type time_t = __time_t;
pub type FILE = libc::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type uint32_t = __uint32_t;
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Pixmap = XID;
pub type Colormap = XID;
pub type KeySym = XID;
pub type KeyCode = libc::c_uchar;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWindowAttributes {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub depth: libc::c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: libc::c_int,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub colormap: Colormap,
    pub map_installed: libc::c_int,
    pub map_state: libc::c_int,
    pub all_event_masks: libc::c_long,
    pub your_event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub screen: *mut Screen,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColor {
    pub pixel: libc::c_ulong,
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub flags: libc::c_char,
    pub pad: libc::c_char,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
pub type XEvent = _XEvent;
pub type XErrorHandler =
    Option<unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int>;
pub type XrmOptionKind = libc::c_uint;
pub const XrmoptionSkipNArgs: XrmOptionKind = 7;
pub const XrmoptionSkipLine: XrmOptionKind = 6;
pub const XrmoptionSkipArg: XrmOptionKind = 5;
pub const XrmoptionResArg: XrmOptionKind = 4;
pub const XrmoptionSepArg: XrmOptionKind = 3;
pub const XrmoptionStickyArg: XrmOptionKind = 2;
pub const XrmoptionIsArg: XrmOptionKind = 1;
pub const XrmoptionNoArg: XrmOptionKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XrmOptionDescRec {
    pub option: *mut libc::c_char,
    pub specifier: *mut libc::c_char,
    pub argKind: XrmOptionKind,
    pub value: XPointer,
}
pub type XrmOptionDescList = *mut XrmOptionDescRec;
pub type String_0 = *mut libc::c_char;
pub type Widget = *mut _WidgetRec;
pub type WidgetClass = *mut _WidgetClassRec;
pub type Cardinal = libc::c_uint;
pub type XtAppContext = *mut _XtAppStruct;
pub type XtIntervalId = libc::c_ulong;
pub type XtSignalId = libc::c_ulong;
pub type Boolean = libc::c_char;
pub type XtArgVal = libc::c_long;
pub type XtPointer = *mut libc::c_void;
pub type XtTranslations = *mut _TranslationData;
pub type XtInputMask = libc::c_ulong;
pub type XtTimerCallbackProc = Option<unsafe extern "C" fn(XtPointer, *mut XtIntervalId) -> ()>;
pub type XtSignalCallbackProc = Option<unsafe extern "C" fn(XtPointer, *mut XtSignalId) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Arg {
    pub name: String_0,
    pub value: XtArgVal,
}
pub type ArgList = *mut Arg;
pub type XtCallbackProc = Option<unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtCallbackRec {
    pub callback: XtCallbackProc,
    pub closure: XtPointer,
}
pub type XtCallbackRec = _XtCallbackRec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtResource {
    pub resource_name: String_0,
    pub resource_class: String_0,
    pub resource_type: String_0,
    pub resource_size: Cardinal,
    pub resource_offset: Cardinal,
    pub default_type: String_0,
    pub default_addr: XtPointer,
}
pub type XtResource = _XtResource;
pub type XtResourceList = *mut _XtResource;
pub type XtLanguageProc =
    Option<unsafe extern "C" fn(*mut Display, String_0, XtPointer) -> String_0>;
pub type XmString = *mut __XmStringRec;
pub type XmStringTag = *mut libc::c_char;
pub type XmTextType = libc::c_uint;
pub const XmNO_TEXT: XmTextType = 3;
pub const XmWIDECHAR_TEXT: XmTextType = 2;
pub const XmMULTIBYTE_TEXT: XmTextType = 1;
pub const XmCHARSET_TEXT: XmTextType = 0;
pub type XmParseModel = libc::c_uint;
pub const XmOUTPUT_BOTH: XmParseModel = 4;
pub const XmOUTPUT_END: XmParseModel = 3;
pub const XmOUTPUT_BEGINNING: XmParseModel = 2;
pub const XmOUTPUT_BETWEEN: XmParseModel = 1;
pub const XmOUTPUT_ALL: XmParseModel = 0;
pub type XmParseMapping = *mut __XmParseMappingRec;
pub type XmParseTable = *mut XmParseMapping;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const XmHORIZONTAL: C2RustUnnamed_2 = 2;
pub const XmVERTICAL: C2RustUnnamed_2 = 1;
pub const XmNO_ORIENTATION: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const XmMENU_OPTION: C2RustUnnamed_3 = 4;
pub const XmMENU_POPUP: C2RustUnnamed_3 = 3;
pub const XmMENU_PULLDOWN: C2RustUnnamed_3 = 2;
pub const XmMENU_BAR: C2RustUnnamed_3 = 1;
pub const XmWORK_AREA: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const XmPACK_NONE: C2RustUnnamed_4 = 3;
pub const XmPACK_COLUMN: C2RustUnnamed_4 = 2;
pub const XmPACK_TIGHT: C2RustUnnamed_4 = 1;
pub const XmNO_PACKING: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const XmALIGNMENT_UNSPECIFIED: C2RustUnnamed_5 = 3;
pub const XmALIGNMENT_END: C2RustUnnamed_5 = 2;
pub const XmALIGNMENT_CENTER: C2RustUnnamed_5 = 1;
pub const XmALIGNMENT_BEGINNING: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const XmCR_PROTOCOLS: C2RustUnnamed_6 = 6666;
pub const XmCR_LEAVE_CHILD: C2RustUnnamed_6 = 86;
pub const XmCR_ENTER_CHILD: C2RustUnnamed_6 = 85;
pub const XmCR_VERIFY_TEXT_FAILED: C2RustUnnamed_6 = 84;
pub const XmCR_VERIFY_TEXT: C2RustUnnamed_6 = 83;
pub const XmCR_UPDATE_TEXT: C2RustUnnamed_6 = 82;
pub const XmCR_UPDATE_SHELL: C2RustUnnamed_6 = 81;
pub const XmCR_PDM_EXIT_ERROR: C2RustUnnamed_6 = 80;
pub const XmCR_PDM_CANCEL: C2RustUnnamed_6 = 79;
pub const XmCR_PDM_OK: C2RustUnnamed_6 = 78;
pub const XmCR_PDM_START_PXAUTH: C2RustUnnamed_6 = 77;
pub const XmCR_PDM_START_VXAUTH: C2RustUnnamed_6 = 76;
pub const XmCR_PDM_START_ERROR: C2RustUnnamed_6 = 75;
pub const XmCR_PDM_UP: C2RustUnnamed_6 = 74;
pub const XmCR_PDM_NONE: C2RustUnnamed_6 = 73;
pub const XmCR_PAGE_SETUP: C2RustUnnamed_6 = 72;
pub const XmCR_END_JOB: C2RustUnnamed_6 = 71;
pub const XmCR_START_JOB: C2RustUnnamed_6 = 70;
pub const XmCR_MINOR_TAB: C2RustUnnamed_6 = 69;
pub const XmCR_MAJOR_TAB: C2RustUnnamed_6 = 68;
pub const XmCR_PAGE_SCROLLER_DECREMENT: C2RustUnnamed_6 = 67;
pub const XmCR_PAGE_SCROLLER_INCREMENT: C2RustUnnamed_6 = 66;
pub const XmCR_SPIN_LAST: C2RustUnnamed_6 = 65;
pub const XmCR_SPIN_FIRST: C2RustUnnamed_6 = 64;
pub const XmCR_SPIN_PRIOR: C2RustUnnamed_6 = 63;
pub const XmCR_SPIN_NEXT: C2RustUnnamed_6 = 62;
pub const XmCR_POST: C2RustUnnamed_6 = 61;
pub const XmCR_NO_RENDITION: C2RustUnnamed_6 = 60;
pub const XmCR_NO_FONT: C2RustUnnamed_6 = 59;
pub const XmCR_DRAG_START: C2RustUnnamed_6 = 58;
pub const XmCR_SELECT: C2RustUnnamed_6 = 57;
pub const XmCR_EXPANDED: C2RustUnnamed_6 = 56;
pub const XmCR_COLLAPSED: C2RustUnnamed_6 = 55;
pub const XmCR_REPOST: C2RustUnnamed_6 = 54;
pub const XmCR_FOCUS_MOVED: C2RustUnnamed_6 = 47;
pub const XmCR_OBSCURED_TRAVERSAL: C2RustUnnamed_6 = 46;
pub const XmCR_TEAR_OFF_DEACTIVATE: C2RustUnnamed_6 = 45;
pub const XmCR_TEAR_OFF_ACTIVATE: C2RustUnnamed_6 = 44;
pub const XmCR_CREATE: C2RustUnnamed_6 = 43;
pub const XmCR_LOSE_PRIMARY: C2RustUnnamed_6 = 42;
pub const XmCR_GAIN_PRIMARY: C2RustUnnamed_6 = 41;
pub const XmCR_INPUT: C2RustUnnamed_6 = 40;
pub const XmCR_RESIZE: C2RustUnnamed_6 = 39;
pub const XmCR_EXPOSE: C2RustUnnamed_6 = 38;
pub const XmCR_COMMAND_CHANGED: C2RustUnnamed_6 = 37;
pub const XmCR_COMMAND_ENTERED: C2RustUnnamed_6 = 36;
pub const XmCR_NO_MATCH: C2RustUnnamed_6 = 35;
pub const XmCR_APPLY: C2RustUnnamed_6 = 34;
pub const XmCR_CANCEL: C2RustUnnamed_6 = 32;
pub const XmCR_OK: C2RustUnnamed_6 = 31;
pub const XmCR_CASCADING: C2RustUnnamed_6 = 30;
pub const XmCR_CLIPBOARD_DATA_DELETE: C2RustUnnamed_6 = 29;
pub const XmCR_CLIPBOARD_DATA_REQUEST: C2RustUnnamed_6 = 28;
pub const XmCR_DEFAULT_ACTION: C2RustUnnamed_6 = 27;
pub const XmCR_BROWSE_SELECT: C2RustUnnamed_6 = 26;
pub const XmCR_EXTENDED_SELECT: C2RustUnnamed_6 = 25;
pub const XmCR_MULTIPLE_SELECT: C2RustUnnamed_6 = 24;
pub const XmCR_SINGLE_SELECT: C2RustUnnamed_6 = 23;
pub const XmCR_EXECUTE: C2RustUnnamed_6 = 22;
pub const XmCR_MOVING_INSERT_CURSOR: C2RustUnnamed_6 = 21;
pub const XmCR_MODIFYING_TEXT_VALUE: C2RustUnnamed_6 = 20;
pub const XmCR_LOSING_FOCUS: C2RustUnnamed_6 = 19;
pub const XmCR_FOCUS: C2RustUnnamed_6 = 18;
pub const XmCR_UNMAP: C2RustUnnamed_6 = 17;
pub const XmCR_MAP: C2RustUnnamed_6 = 16;
pub const XmCR_DISARM: C2RustUnnamed_6 = 12;
pub const XmCR_ARM: C2RustUnnamed_6 = 11;
pub const XmCR_ACTIVATE: C2RustUnnamed_6 = 10;
pub const XmCR_DRAG: C2RustUnnamed_6 = 9;
pub const XmCR_TO_BOTTOM: C2RustUnnamed_6 = 8;
pub const XmCR_TO_TOP: C2RustUnnamed_6 = 7;
pub const XmCR_PAGE_DECREMENT: C2RustUnnamed_6 = 6;
pub const XmCR_PAGE_INCREMENT: C2RustUnnamed_6 = 5;
pub const XmCR_DECREMENT: C2RustUnnamed_6 = 4;
pub const XmCR_INCREMENT: C2RustUnnamed_6 = 3;
pub const XmCR_VALUE_CHANGED: C2RustUnnamed_6 = 2;
pub const XmCR_HELP: C2RustUnnamed_6 = 1;
pub const XmCR_NONE: C2RustUnnamed_6 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmSelectionBoxCallbackStruct {
    pub reason: libc::c_int,
    pub event: *mut XEvent,
    pub value: XmString,
    pub length: libc::c_int,
}
pub type C2RustUnnamed_7 = libc::c_uint;
pub const XmSHADOW_OUT: C2RustUnnamed_7 = 8;
pub const XmSHADOW_IN: C2RustUnnamed_7 = 7;
pub type XmTextPosition = libc::c_long;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const XmDIALOG_DIR_LIST_LABEL: C2RustUnnamed_8 = 16;
pub const XmDIALOG_DIR_LIST: C2RustUnnamed_8 = 15;
pub const XmDIALOG_SEPARATOR: C2RustUnnamed_8 = 14;
pub const XmDIALOG_TEXT: C2RustUnnamed_8 = 13;
pub const XmDIALOG_SYMBOL_LABEL: C2RustUnnamed_8 = 12;
pub const XmDIALOG_SELECTION_LABEL: C2RustUnnamed_8 = 11;
pub const XmDIALOG_MESSAGE_LABEL: C2RustUnnamed_8 = 10;
pub const XmDIALOG_LIST_LABEL: C2RustUnnamed_8 = 9;
pub const XmDIALOG_LIST: C2RustUnnamed_8 = 8;
pub const XmDIALOG_HELP_BUTTON: C2RustUnnamed_8 = 7;
pub const XmDIALOG_FILTER_TEXT: C2RustUnnamed_8 = 6;
pub const XmDIALOG_FILTER_LABEL: C2RustUnnamed_8 = 5;
pub const XmDIALOG_OK_BUTTON: C2RustUnnamed_8 = 4;
pub const XmDIALOG_DEFAULT_BUTTON: C2RustUnnamed_8 = 3;
pub const XmDIALOG_CANCEL_BUTTON: C2RustUnnamed_8 = 2;
pub const XmDIALOG_APPLY_BUTTON: C2RustUnnamed_8 = 1;
pub const XmDIALOG_NONE: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const XmDIALOG_SYSTEM_MODAL: C2RustUnnamed_9 = 3;
pub const XmDIALOG_FULL_APPLICATION_MODAL: C2RustUnnamed_9 = 2;
pub const XmDIALOG_PRIMARY_APPLICATION_MODAL: C2RustUnnamed_9 = 1;
pub const XmDIALOG_MODELESS: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const XmDIALOG_WORKING: C2RustUnnamed_10 = 6;
pub const XmDIALOG_WARNING: C2RustUnnamed_10 = 5;
pub const XmDIALOG_QUESTION: C2RustUnnamed_10 = 4;
pub const XmDIALOG_MESSAGE: C2RustUnnamed_10 = 3;
pub const XmDIALOG_INFORMATION: C2RustUnnamed_10 = 2;
pub const XmDIALOG_ERROR: C2RustUnnamed_10 = 1;
pub const XmDIALOG_TEMPLATE: C2RustUnnamed_10 = 0;
pub type XmTraversalDirection = libc::c_uint;
pub const XmTRAVERSE_GLOBALLY_BACKWARD: XmTraversalDirection = 11;
pub const XmTRAVERSE_GLOBALLY_FORWARD: XmTraversalDirection = 10;
pub const XmTRAVERSE_RIGHT: XmTraversalDirection = 9;
pub const XmTRAVERSE_LEFT: XmTraversalDirection = 8;
pub const XmTRAVERSE_DOWN: XmTraversalDirection = 7;
pub const XmTRAVERSE_UP: XmTraversalDirection = 6;
pub const XmTRAVERSE_PREV_TAB_GROUP: XmTraversalDirection = 5;
pub const XmTRAVERSE_NEXT_TAB_GROUP: XmTraversalDirection = 4;
pub const XmTRAVERSE_HOME: XmTraversalDirection = 3;
pub const XmTRAVERSE_PREV: XmTraversalDirection = 2;
pub const XmTRAVERSE_NEXT: XmTraversalDirection = 1;
pub const XmTRAVERSE_CURRENT: XmTraversalDirection = 0;
pub type RROutput = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XRRMonitorInfo {
    pub name: Atom,
    pub primary: libc::c_int,
    pub automatic: libc::c_int,
    pub noutput: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub outputs: *mut RROutput,
}
pub type XRRMonitorInfo = _XRRMonitorInfo;
pub type tb_entry_type = libc::c_uint;
pub const TBE_SEPARATOR: tb_entry_type = 2;
pub const TBE_COMMAND: tb_entry_type = 1;
pub const TBE_CASCADE: tb_entry_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tb_entry {
    pub type_0: tb_entry_type,
    pub level: libc::c_int,
    pub title: *mut libc::c_char,
    pub mnemonic: libc::c_char,
    pub command: *mut libc::c_char,
    pub next: *mut tb_entry,
}
pub type sigfunc_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tb_resources {
    pub title: *mut libc::c_char,
    pub show_date_time: Boolean,
    pub system_tray: Boolean,
    pub date_time_fmt: *mut libc::c_char,
    pub rc_file: *mut libc::c_char,
    pub hotkey: *mut libc::c_char,
    pub horizontal: Boolean,
    pub separators: Boolean,
    pub switcher: Boolean,
    pub occupy_all: Boolean,
    pub applications_menu: Boolean,
    pub applications_title: *mut libc::c_char,
    pub applications_file: *mut libc::c_char,
}
static mut randr_event: libc::c_int = -(1 as libc::c_int);
static mut visible_timer: XtIntervalId = 0 as XtIntervalId;
unsafe extern "C" fn free_menu_command(
    mut widget: Widget,
    mut data: XtPointer,
    mut event: XtPointer,
) {
    XtFree(data as *mut libc::c_char);
}
#[no_mangle]
pub static mut app_res: tb_resources = tb_resources {
    title: ::core::ptr::null::<libc::c_char>() as *mut libc::c_char,
    show_date_time: 0,
    system_tray: 0,
    date_time_fmt: ::core::ptr::null::<libc::c_char>() as *mut libc::c_char,
    rc_file: ::core::ptr::null::<libc::c_char>() as *mut libc::c_char,
    hotkey: ::core::ptr::null::<libc::c_char>() as *mut libc::c_char,
    horizontal: 0,
    separators: 0,
    switcher: 0,
    occupy_all: 0,
    applications_menu: 0,
    applications_title: ::core::ptr::null::<libc::c_char>() as *mut libc::c_char,
    applications_file: ::core::ptr::null::<libc::c_char>() as *mut libc::c_char,
};
static mut xrdb_resources: [XtResource; 13] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<libc::c_char>(),
    resource_class: ::core::ptr::null_mut::<libc::c_char>(),
    resource_type: ::core::ptr::null_mut::<libc::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<libc::c_char>(),
    default_addr: ::core::ptr::null_mut::<libc::c_void>(),
}; 13];
static mut xrdb_options: [XrmOptionDescRec; 8] = [
    XrmOptionDescRec {
        option: c"-noapplications".as_ptr().cast_mut(),
        specifier: c"applicationsMenu".as_ptr().cast_mut(),
        argKind: XrmoptionNoArg,
        value: c"False".as_ptr() as XPointer,
    },
    XrmOptionDescRec {
        option: c"-applications".as_ptr().cast_mut(),
        specifier: c"applicationsMenu".as_ptr().cast_mut(),
        argKind: XrmoptionNoArg,
        value: c"True".as_ptr() as XPointer,
    },
    XrmOptionDescRec {
        option: c"-appmenu".as_ptr().cast_mut(),
        specifier: c"applicationsMenuFile".as_ptr().cast_mut(),
        argKind: XrmoptionSepArg,
        value: ::core::ptr::null::<libc::c_void>() as *mut libc::c_void as XPointer,
    },
    XrmOptionDescRec {
        option: c"-title".as_ptr().cast_mut(),
        specifier: c"title".as_ptr().cast_mut(),
        argKind: XrmoptionSepArg,
        value: ::core::ptr::null::<libc::c_void>() as *mut libc::c_void as XPointer,
    },
    XrmOptionDescRec {
        option: c"-rcfile".as_ptr().cast_mut(),
        specifier: c"rcFile".as_ptr().cast_mut(),
        argKind: XrmoptionSepArg,
        value: ::core::ptr::null::<libc::c_void>() as *mut libc::c_void as XPointer,
    },
    XrmOptionDescRec {
        option: c"-hotkey".as_ptr().cast_mut(),
        specifier: c"hotkey".as_ptr().cast_mut(),
        argKind: XrmoptionSepArg,
        value: ::core::ptr::null::<libc::c_void>() as *mut libc::c_void as XPointer,
    },
    XrmOptionDescRec {
        option: c"-horizontal".as_ptr().cast_mut(),
        specifier: c"horizontal".as_ptr().cast_mut(),
        argKind: XrmoptionNoArg,
        value: c"True".as_ptr() as XPointer,
    },
    XrmOptionDescRec {
        option: c"+horizontal".as_ptr().cast_mut(),
        specifier: c"horizontal".as_ptr().cast_mut(),
        argKind: XrmoptionNoArg,
        value: c"False".as_ptr() as XPointer,
    },
];
static mut fallback_res: [String_0; 5] = [
    c"XmToolbox.x: 8".as_ptr().cast_mut(),
    c"XmToolbox.y: 28".as_ptr().cast_mut(),
    c"XmToolbox.mwmDecorations: 58".as_ptr().cast_mut(),
    c"*mainFrame.shadowThickness: 1".as_ptr().cast_mut(),
    ::core::ptr::null::<libc::c_char>() as *mut libc::c_char,
];
#[no_mangle]
pub static mut app_context: XtAppContext = ::core::ptr::null::<_XtAppStruct>() as *mut _XtAppStruct;
#[no_mangle]
pub static mut wshell: Widget = ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec;
static mut xa_ndesks: Atom = 0 as Atom;
static mut xa_cdesk: Atom = 0 as Atom;
static mut xa_xmsm_mgr: Atom = 0 as Atom;
static mut xa_xmsm_pid: Atom = 0 as Atom;
static mut xa_xmsm_cmd: Atom = 0 as Atom;
static mut xa_xmsm_cfg: Atom = 0 as Atom;
static mut def_x_err_handler: Option<
    unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
> = None;
static mut xmsm_cmd_err: [libc::c_char; 55] = unsafe {
    ::core::mem::transmute::<[u8; 55], [libc::c_char; 55]>(
        *b"Cannot retrieve session manager PID.\nxmsm not running?\0",
    )
};
static mut wmain: Widget = ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec;
static mut wdtlabel: Widget = ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec;
static mut wdtframe: Widget = ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec;
static mut wmenu: Widget = ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec;
static mut wswitch: Widget = ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec;
static mut wgadsep: Widget = ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec;
static mut wgadrc: Widget = ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec;
static mut rc_file_path: String_0 = ::core::ptr::null::<libc::c_char>() as *mut libc::c_char;
static mut xt_sigusr1: XtSignalId = 0;
static mut hotkey_code: KeyCode = 0 as KeyCode;
#[no_mangle]
pub static mut hotkey_mods: libc::c_uint = 0 as libc::c_uint;
#[no_mangle]
pub static mut xmsm_cfg: libc::c_ulong = 0 as libc::c_ulong;
static mut sm_reqstat: Boolean = 0;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut dpy: *mut Display = ::core::ptr::null_mut::<Display>();
    let mut root_window: Window = 0;
    let mut wframe: Widget = ::core::ptr::null_mut::<_WidgetRec>();
    let mut root_event_mask: libc::c_int =
        ((1 as libc::c_long) << 22 as libc::c_int) as libc::c_int;
    let mut retries: libc::c_int = 0;
    rsignal(
        10 as libc::c_int,
        Some(sigusr_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    rsignal(
        12 as libc::c_int,
        Some(sigusr_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    rsignal(
        17 as libc::c_int,
        Some(sigchld_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    XtSetLanguageProc(
        ::core::ptr::null_mut::<_XtAppStruct>(),
        None,
        ::core::ptr::null_mut::<libc::c_void>(),
    );
    XtToolkitInitialize();
    wshell = XtVaAppInitialize(
        &raw mut app_context,
        c"XmToolbox".as_ptr(),
        &raw mut xrdb_options as XrmOptionDescList,
        (::core::mem::size_of::<[XrmOptionDescRec; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<XrmOptionDescRec>() as usize) as Cardinal,
        &raw mut argc,
        argv,
        &raw mut fallback_res as *mut String_0,
        c"iconName".as_ptr().cast_mut(),
        c"Toolbox".as_ptr(),
        c"allowShellResize".as_ptr().cast_mut(),
        1 as libc::c_int,
        c"mwmFunctions".as_ptr().cast_mut() as *mut libc::c_char,
        (1 as libc::c_long) << 2 as libc::c_int | (1 as libc::c_long) << 3 as libc::c_int,
        c"mappedWhenManaged".as_ptr().cast_mut(),
        0 as libc::c_int,
        ::core::ptr::null_mut::<libc::c_void>(),
    );
    dpy = XtDisplay(wshell);
    root_window = (*XtScreen(wshell)).root;
    if argc > 1 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        while i < argc {
            if strcmp(c"-version".as_ptr(), *argv.offset(i as isize)) == 0 {
                print_version(c"xmtoolbox".as_ptr());
                XtDestroyApplicationContext(app_context);
                return 0 as libc::c_int;
            }
            i += 1;
        }
    }
    XtGetApplicationResources(
        wshell,
        &raw mut app_res as XtPointer,
        &raw mut xrdb_resources as XtResourceList,
        (::core::mem::size_of::<[XtResource; 13]>() as usize)
            .wrapping_div(::core::mem::size_of::<XtResource>() as usize) as Cardinal,
        ::core::ptr::null_mut::<Arg>(),
        0 as Cardinal,
    );
    if !getenv(c"EMWM_DESKTOP_TRAY".as_ptr()).is_null()
        && strcmp(getenv(c"EMWM_DESKTOP_TRAY".as_ptr()), c"0".as_ptr()) == 0
    {
        app_res.system_tray = 0 as Boolean;
    }
    retries = 12 as libc::c_int;
    loop {
        xa_xmsm_mgr = XInternAtom(dpy, c"_XM_SESSION_MANAGER".as_ptr(), 1 as libc::c_int);
        if !(xa_xmsm_mgr == 0 as Atom && {
            let fresh4 = retries;
            retries = retries - 1;
            fresh4 != 0
        }) {
            break;
        }
        usleep(250000 as libc::c_int as __useconds_t);
    }
    xa_xmsm_pid = XInternAtom(dpy, c"_XM_SESSION_MANAGER_PID".as_ptr(), 1 as libc::c_int);
    xa_xmsm_cmd = XInternAtom(dpy, c"_XM_SESSION_MANAGER_CMD".as_ptr(), 1 as libc::c_int);
    xa_xmsm_cfg = XInternAtom(dpy, c"_XM_SESSION_MANAGER_CFG".as_ptr(), 1 as libc::c_int);
    retries = 12 as libc::c_int;
    loop {
        xa_ndesks = XInternAtom(
            dpy,
            c"_NET_NUMBER_OF_DESKTOPS".as_ptr(),
            i32::from(retries != 0),
        );
        if !(xa_ndesks == 0 as Atom && {
            let fresh5 = retries;
            retries = retries - 1;
            fresh5 != 0
        }) {
            break;
        }
        usleep(250000 as libc::c_int as __useconds_t);
    }
    xa_cdesk = XInternAtom(dpy, c"_NET_CURRENT_DESKTOP".as_ptr(), 0 as libc::c_int);
    if get_xmsm_config(&raw mut xmsm_cfg) == 0 {
        message_dialog(0 as Boolean, &raw const xmsm_cmd_err as *const libc::c_char);
    }
    if app_res.title.is_null() {
        let mut title: *mut libc::c_char = ::core::ptr::null_mut::<libc::c_char>();
        let mut login: *mut libc::c_char = ::core::ptr::null_mut::<libc::c_char>();
        let mut host: [libc::c_char; 256] = ::core::mem::transmute::<
            [u8; 256],
            [libc::c_char; 256],
        >(
            *b"localhost\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        login = get_login();
        if !login.is_null() {
            gethostname(&raw mut host as *mut libc::c_char, 255 as size_t);
            title = malloc(
                strlen(login)
                    .wrapping_add(strlen(&raw mut host as *mut libc::c_char))
                    .wrapping_add(2 as size_t),
            ) as *mut libc::c_char;
            if title.is_null() {
                perror(c"malloc".as_ptr());
                return 1 as libc::c_int;
            }
            sprintf(
                title,
                c"%s@%s".as_ptr(),
                login,
                &raw mut host as *mut libc::c_char,
            );
            XtVaSetValues(
                wshell,
                c"title".as_ptr().cast_mut(),
                title,
                ::core::ptr::null_mut::<libc::c_void>(),
            );
            free(title as *mut libc::c_void);
        }
    }
    wframe = XmVaCreateManagedFrame(
        wshell,
        c"mainFrame".as_ptr().cast_mut(),
        c"shadowType".as_ptr().cast_mut() as *mut libc::c_char,
        XmSHADOW_OUT as libc::c_int,
        ::core::ptr::null_mut::<libc::c_void>(),
    );
    wmain = XmVaCreateManagedRowColumn(
        wframe,
        c"main".as_ptr().cast_mut(),
        c"marginWidth".as_ptr().cast_mut() as *mut libc::c_char,
        0 as libc::c_int,
        c"marginHeight".as_ptr().cast_mut() as *mut libc::c_char,
        0 as libc::c_int,
        c"spacing".as_ptr().cast_mut() as *mut libc::c_char,
        0 as libc::c_int,
        c"orientation".as_ptr().cast_mut(),
        if app_res.horizontal as libc::c_int != 0 {
            XmHORIZONTAL as libc::c_int
        } else {
            XmVERTICAL as libc::c_int
        },
        ::core::ptr::null_mut::<libc::c_void>(),
    );
    rc_file_path = (if !app_res.rc_file.is_null() {
        app_res.rc_file
    } else {
        find_rc_file()
    }) as String_0;
    if !rc_file_path.is_null() {
        if access(rc_file_path as *const libc::c_char, 4 as libc::c_int) == -(1 as libc::c_int) {
            message_dialog(0 as Boolean, c"Cannot access RC file. Exiting!".as_ptr());
            perror(rc_file_path as *const libc::c_char);
            return 1 as libc::c_int;
        }
    } else if app_res.applications_menu == 0 {
        message_dialog(
            0 as Boolean,
            c"RC file not found, nor specified. Exiting!".as_ptr(),
        );
        fprintf(
            stderr,
            c"%s not found, nor specified.\n".as_ptr(),
            c"toolboxrc".as_ptr(),
        );
        return 1 as libc::c_int;
    }
    if construct_menu() == 0 {
        return 1 as libc::c_int;
    }
    create_utility_widgets(wmain);
    XtRealizeWidget(wshell);
    if app_res.system_tray != 0 {
        TrayStart();
    }
    if app_res.occupy_all != 0 {
        set_ws_presence(wshell);
    }
    set_icon(wshell);
    if setup_hotkeys() != 0 {
        root_event_mask = (root_event_mask as libc::c_long
            | (1 as libc::c_long) << 0 as libc::c_int) as libc::c_int;
    }
    XMapWindow(XtDisplay(wshell), XtWindow(wshell));
    let mut error: libc::c_int = 0;
    let mut major: libc::c_int = 1 as libc::c_int;
    let mut minor: libc::c_int = 5 as libc::c_int;
    if XRRQueryExtension(dpy, &raw mut randr_event, &raw mut error) != 0
        && XRRQueryVersion(dpy, &raw mut major, &raw mut minor) != 0
        && (major > 1 as libc::c_int || minor >= 5 as libc::c_int)
    {
        XRRSelectInput(
            dpy,
            root_window,
            ((1 as libc::c_long) << 0 as libc::c_int
                | (1 as libc::c_long) << 1 as libc::c_int
                | (1 as libc::c_long) << 2 as libc::c_int) as libc::c_int,
        );
        visible_timer = XtAppAddTimeOut(
            app_context,
            800 as libc::c_ulong,
            Some(ensure_visible as unsafe extern "C" fn(XtPointer, *mut XtIntervalId) -> ()),
            ::core::ptr::null_mut::<libc::c_void>(),
        );
    } else {
        randr_event = -(1 as libc::c_int);
    }
    if XtIsManaged(wswitch) != 0 {
        XmProcessTraversal(wswitch, XmTRAVERSE_CURRENT);
    }
    XSelectInput(
        XtDisplay(wshell),
        root_window,
        root_event_mask as libc::c_long,
    );
    xt_sigusr1 = XtAppAddSignal(
        app_context,
        Some(xt_sigusr1_handler as unsafe extern "C" fn(XtPointer, *mut XtSignalId) -> ()),
        ::core::ptr::null_mut::<libc::c_void>(),
    );
    loop {
        let mut evt: XEvent = _XEvent { type_0: 0 };
        XtAppNextEvent(app_context, &raw mut evt);
        if randr_event >= 0 as libc::c_int
            && (evt.type_0 == randr_event + 0 as libc::c_int
                || evt.type_0 == randr_event + 1 as libc::c_int)
        {
            if visible_timer != 0 {
                XtRemoveTimeOut(visible_timer);
            }
            visible_timer = XtAppAddTimeOut(
                app_context,
                400 as libc::c_ulong,
                Some(ensure_visible as unsafe extern "C" fn(XtPointer, *mut XtIntervalId) -> ()),
                ::core::ptr::null_mut::<libc::c_void>(),
            );
        } else if evt.xany.window == root_window {
            handle_root_event(&raw mut evt);
        } else {
            XtDispatchEvent(&raw mut evt);
        }
    }
}
unsafe extern "C" fn ensure_visible(mut unused: XtPointer, mut id: *mut XtIntervalId) {
    let mut dpy: *mut Display = XtDisplay(wshell);
    let mut client: Window = XtWindow(wshell);
    let mut root: Window = (*XtScreen(wshell)).root;
    let mut frame: Window = client;
    let mut r: Window = 0;
    let mut parent: Window = 0;
    let mut children: *mut Window = ::core::ptr::null_mut::<Window>();
    let mut child: Window = 0;
    let mut count: libc::c_uint = 0;
    let mut attr: XWindowAttributes = XWindowAttributes {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        depth: 0,
        visual: ::core::ptr::null_mut::<Visual>(),
        root: 0,
        class: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        colormap: 0,
        map_installed: 0,
        map_state: 0,
        all_event_masks: 0,
        your_event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        screen: ::core::ptr::null_mut::<Screen>(),
    };
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut best: libc::c_int = 0 as libc::c_int;
    let mut area_best: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut monitors: *mut XRRMonitorInfo = ::core::ptr::null_mut::<XRRMonitorInfo>();
    visible_timer = 0 as XtIntervalId;
    if XGetWindowAttributes(dpy, client, &raw mut attr) == 0 || attr.map_state != 2 as libc::c_int {
        return;
    }
    while XQueryTree(
        dpy,
        frame,
        &raw mut r,
        &raw mut parent,
        &raw mut children,
        &raw mut count,
    ) != 0
    {
        if !children.is_null() {
            XFree(children as *mut libc::c_void);
        }
        if parent == root || parent == 0 as Window {
            break;
        }
        frame = parent;
    }
    if XGetWindowAttributes(dpy, frame, &raw mut attr) == 0 {
        return;
    }
    XTranslateCoordinates(
        dpy,
        client,
        root,
        0 as libc::c_int,
        0 as libc::c_int,
        &raw mut cx,
        &raw mut cy,
        &raw mut child,
    );
    monitors = XRRGetMonitors(dpy, root, 1 as libc::c_int, &raw mut n);
    if monitors.is_null() || n < 1 as libc::c_int {
        if !monitors.is_null() {
            XRRFreeMonitors(monitors);
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < n {
        let mut left: libc::c_int = if attr.x > (*monitors.offset(i as isize)).x {
            attr.x
        } else {
            (*monitors.offset(i as isize)).x
        };
        let mut top: libc::c_int = if attr.y > (*monitors.offset(i as isize)).y {
            attr.y
        } else {
            (*monitors.offset(i as isize)).y
        };
        let mut right: libc::c_int = if attr.x + attr.width
            < (*monitors.offset(i as isize)).x + (*monitors.offset(i as isize)).width
        {
            attr.x + attr.width
        } else {
            (*monitors.offset(i as isize)).x + (*monitors.offset(i as isize)).width
        };
        let mut bottom: libc::c_int = if attr.y + attr.height
            < (*monitors.offset(i as isize)).y + (*monitors.offset(i as isize)).height
        {
            attr.y + attr.height
        } else {
            (*monitors.offset(i as isize)).y + (*monitors.offset(i as isize)).height
        };
        let mut area: libc::c_int = if right > left && bottom > top {
            (right - left) * (bottom - top)
        } else {
            0 as libc::c_int
        };
        if area > area_best || area == area_best && (*monitors.offset(i as isize)).primary != 0 {
            best = i;
            area_best = area;
        }
        i += 1;
    }
    nx = attr.x;
    ny = attr.y;
    if nx + attr.width
        > (*monitors.offset(best as isize)).x + (*monitors.offset(best as isize)).width
    {
        nx = (*monitors.offset(best as isize)).x + (*monitors.offset(best as isize)).width
            - attr.width;
    }
    if ny + attr.height
        > (*monitors.offset(best as isize)).y + (*monitors.offset(best as isize)).height
    {
        ny = (*monitors.offset(best as isize)).y + (*monitors.offset(best as isize)).height
            - attr.height;
    }
    if nx < (*monitors.offset(best as isize)).x {
        nx = (*monitors.offset(best as isize)).x;
    }
    if ny < (*monitors.offset(best as isize)).y {
        ny = (*monitors.offset(best as isize)).y;
    }
    if nx != attr.x || ny != attr.y {
        let mut event: XEvent = _XEvent {
            type_0: 0 as libc::c_int,
        };
        event.xclient.type_0 = 33 as libc::c_int;
        event.xclient.window = client;
        event.xclient.message_type =
            XInternAtom(dpy, c"_NET_MOVERESIZE_WINDOW".as_ptr(), 0 as libc::c_int);
        event.xclient.format = 32 as libc::c_int;
        event.xclient.data.l[0] = (10 as libc::c_int
            | (3 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 12 as libc::c_int)
            as libc::c_long;
        event.xclient.data.l[1] = (nx + cx - attr.x) as libc::c_long;
        event.xclient.data.l[2] = (ny + cy - attr.y) as libc::c_long;
        XSendEvent(
            dpy,
            root,
            0 as libc::c_int,
            (1 as libc::c_long) << 20 as libc::c_int | (1 as libc::c_long) << 19 as libc::c_int,
            &raw mut event,
        );
        XFlush(dpy);
    }
    XRRFreeMonitors(monitors);
}
unsafe extern "C" fn set_icon(mut wshell_0: Widget) {
    let mut image: Pixmap = 0;
    let mut mask: Pixmap = 0;
    let mut root: Window = 0;
    let mut dpy: *mut Display = XtDisplay(wshell_0);
    let mut depth: libc::c_int = 0;
    let mut screen: libc::c_int = 0;
    let mut pscreen: *mut Screen = ::core::ptr::null_mut::<Screen>();
    let mut cmap: Colormap = 0;
    let mut bg_color: XColor = XColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    let mut fg_color: XColor = XColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    let mut tmp: XColor = XColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    static mut toolbox_xbm_bits: [libc::c_uchar; 288] = [
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfb as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x8b as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xdf as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x5b as libc::c_int as libc::c_uchar,
        0x4a as libc::c_int as libc::c_uchar,
        0xf9 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x5b as libc::c_int as libc::c_uchar,
        0x4a as libc::c_int as libc::c_uchar,
        0xf9 as libc::c_int as libc::c_uchar,
        0xdf as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfb as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfb as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xdf as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xb as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x1b as libc::c_int as libc::c_uchar,
        0x4d as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x6b as libc::c_int as libc::c_uchar,
        0xb7 as libc::c_int as libc::c_uchar,
        0x1d as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xef as libc::c_int as libc::c_uchar,
        0x7f as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xdf as libc::c_int as libc::c_uchar,
        0x6d as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x8f as libc::c_int as libc::c_uchar,
        0x48 as libc::c_int as libc::c_uchar,
        0xfa as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x5 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x29 as libc::c_int as libc::c_uchar,
        0x33 as libc::c_int as libc::c_uchar,
        0xc3 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x5d as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xb as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x66 as libc::c_int as libc::c_uchar,
        0x6 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xbb as libc::c_int as libc::c_uchar,
        0xde as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x5 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xa5 as libc::c_int as libc::c_uchar,
        0x4a as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xb5 as libc::c_int as libc::c_uchar,
        0x5e as libc::c_int as libc::c_uchar,
        0xc5 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xb as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x4b as libc::c_int as libc::c_uchar,
        0x95 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x6b as libc::c_int as libc::c_uchar,
        0xbf as libc::c_int as libc::c_uchar,
        0xb as libc::c_int as libc::c_uchar,
        0x5 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0xa as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x9d as libc::c_int as libc::c_uchar,
        0xdf as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x2b as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0xd7 as libc::c_int as libc::c_uchar,
        0x4 as libc::c_int as libc::c_uchar,
        0xfc as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xbb as libc::c_int as libc::c_uchar,
        0xab as libc::c_int as libc::c_uchar,
        0x4 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x13 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x6b as libc::c_int as libc::c_uchar,
        0x91 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfb as libc::c_int as libc::c_uchar,
        0x77 as libc::c_int as libc::c_uchar,
        0xa as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
    ];
    static mut toolbox_m_xbm_bits: [libc::c_uchar; 288] = [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x7f as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x7f as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x7f as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x7f as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x7f as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xf8 as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x7f as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    pscreen = XDefaultScreenOfDisplay(dpy);
    screen = XScreenNumberOfScreen(pscreen);
    root = (*pscreen).root;
    depth = (*(*(dpy as _XPrivDisplay)).screens.offset(screen as isize)).root_depth;
    cmap = (*(*(dpy as _XPrivDisplay)).screens.offset(screen as isize)).cmap;
    fg_color.pixel = (*(*(dpy as _XPrivDisplay)).screens.offset(screen as isize)).black_pixel;
    bg_color.pixel = (*(*(dpy as _XPrivDisplay)).screens.offset(screen as isize)).white_pixel;
    XAllocNamedColor(
        dpy,
        cmap,
        c"LightBlue".as_ptr(),
        &raw mut bg_color,
        &raw mut tmp,
    );
    image = XCreatePixmapFromBitmapData(
        dpy,
        root as Drawable,
        &raw mut toolbox_xbm_bits as *mut libc::c_uchar as *mut libc::c_char,
        48 as libc::c_uint,
        48 as libc::c_uint,
        fg_color.pixel,
        bg_color.pixel,
        depth as libc::c_uint,
    );
    mask = XCreatePixmapFromBitmapData(
        dpy,
        root as Drawable,
        &raw mut toolbox_m_xbm_bits as *mut libc::c_uchar as *mut libc::c_char,
        48 as libc::c_uint,
        48 as libc::c_uint,
        1 as libc::c_ulong,
        0 as libc::c_ulong,
        1 as libc::c_uint,
    );
    XtVaSetValues(
        wshell_0,
        c"iconPixmap".as_ptr().cast_mut(),
        image,
        c"iconMask".as_ptr().cast_mut(),
        mask,
        ::core::ptr::null_mut::<libc::c_void>(),
    );
}
unsafe extern "C" fn set_ws_presence(mut wshell_0: Widget) {
    let mut dpy: *mut Display = XtDisplay(wshell_0);
    let mut wps_atom: Atom =
        XInternAtom(dpy, c"_MWM_WORKSPACE_PRESENCE".as_ptr(), 0 as libc::c_int);
    let mut all_atom: Atom = XInternAtom(dpy, c"all".as_ptr(), 0 as libc::c_int);
    XChangeProperty(
        dpy,
        XtWindow(wshell_0),
        wps_atom,
        wps_atom,
        32 as libc::c_int,
        0 as libc::c_int,
        &raw mut all_atom as *mut libc::c_uchar,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn setup_hotkeys() -> Boolean {
    let mut root_window: Window = 0;
    let mut buf: *mut libc::c_char = ::core::ptr::null_mut::<libc::c_char>();
    let mut token: *mut libc::c_char = ::core::ptr::null_mut::<libc::c_char>();
    let mut key_sym: KeySym = 0 as KeySym;
    static mut def_x_err_handler_0: Option<
        unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
    > = None;
    if app_res.hotkey.is_null() || strcasecmp(app_res.hotkey, c"none".as_ptr()) == 0 {
        return 0 as Boolean;
    }
    hotkey_code = 0 as KeyCode;
    hotkey_mods = 0 as libc::c_uint;
    buf = strdup(app_res.hotkey);
    token = strtok(buf, c" \t+".as_ptr());
    if !token.is_null() {
        while !token.is_null() {
            if strcasecmp(token, c"alt".as_ptr()) == 0 {
                hotkey_mods |= ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
            } else if strcasecmp(token, c"ctrl".as_ptr()) == 0
                || strcasecmp(token, c"control".as_ptr()) == 0
            {
                hotkey_mods |= ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
            } else if strcasecmp(token, c"shift".as_ptr()) == 0 {
                hotkey_mods |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            } else {
                key_sym = XStringToKeysym(token);
                break;
            }
            token = strtok(::core::ptr::null_mut::<libc::c_char>(), c" \t+".as_ptr());
        }
    } else {
        key_sym = XStringToKeysym(buf);
    }
    free(buf as *mut libc::c_void);
    if key_sym == 0 as KeySym {
        fputs(c"Invalid hotkey specification\n".as_ptr(), stderr);
        return 0 as Boolean;
    }
    hotkey_code = XKeysymToKeycode(XtDisplay(wshell), key_sym);
    root_window = (*XtScreen(wshell)).root;
    XSync(XtDisplay(wshell), 0 as libc::c_int);
    def_x_err_handler_0 = XSetErrorHandler(Some(
        xgrabkey_err_handler as unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
    ))
        as Option<unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int>;
    XGrabKey(
        XtDisplay(wshell),
        hotkey_code as libc::c_int,
        hotkey_mods,
        root_window,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    XGrabKey(
        XtDisplay(wshell),
        hotkey_code as libc::c_int,
        hotkey_mods | ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint,
        root_window,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    XGrabKey(
        XtDisplay(wshell),
        hotkey_code as libc::c_int,
        hotkey_mods | ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
        root_window,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    XGrabKey(
        XtDisplay(wshell),
        hotkey_code as libc::c_int,
        hotkey_mods
            | ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
            | ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint,
        root_window,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    XSync(XtDisplay(wshell), 0 as libc::c_int);
    XSetErrorHandler(def_x_err_handler_0 as XErrorHandler);
    return 1 as Boolean;
}
unsafe extern "C" fn xgrabkey_err_handler(
    mut dpy: *mut Display,
    mut evt: *mut XErrorEvent,
) -> libc::c_int {
    if (*evt).error_code as libc::c_int == 10 as libc::c_int {
        fputs(
            c"Cannot setup hotkey. Specified key code is used by another application.\n".as_ptr(),
            stderr,
        );
        return 0 as libc::c_int;
    }
    exit(1 as libc::c_int);
}
unsafe extern "C" fn handle_root_event(mut evt: *mut XEvent) {
    if (*evt).type_0 == 3 as libc::c_int {
        let mut e: *mut XKeyEvent = evt as *mut XKeyEvent;
        if (*e).keycode == hotkey_code as libc::c_uint
            && ((*e).state & hotkey_mods != 0 || hotkey_mods == 0)
        {
            raise_and_focus(wshell);
        }
    } else if (*evt).type_0 == 28 as libc::c_int {
        let mut e_0: *mut XPropertyEvent = evt as *mut XPropertyEvent;
        if ((*e_0).atom == xa_cdesk || (*e_0).atom == xa_ndesks)
            && app_res.switcher as libc::c_int != 0
        {
            let mut nws: libc::c_ushort = 0;
            let mut iws: libc::c_ushort = 0;
            if get_ws_info(&raw mut nws, &raw mut iws) != 0 {
                if (*e_0).atom == xa_ndesks {
                    let mut args: [Arg; 4] = [Arg {
                        name: ::core::ptr::null_mut::<libc::c_char>(),
                        value: 0,
                    }; 4];
                    let mut n: libc::c_int = 0 as libc::c_int;
                    args[n as usize].name = c"numberOfWorkspaces".as_ptr().cast_mut();
                    args[n as usize].value = nws as XtArgVal;
                    n += 1;
                    args[n as usize].name = c"activeWorkspace".as_ptr().cast_mut();
                    args[n as usize].value = iws as XtArgVal;
                    n += 1;
                    if app_res.horizontal != 0 {
                        args[n as usize].name = c"columns".as_ptr().cast_mut();
                        args[n as usize].value = nws as XtArgVal;
                        n += 1;
                    }
                    if nws as libc::c_int > 1 as libc::c_int {
                        XtManageChild(wswitch);
                        XtManageChild(wgadrc);
                        if app_res.separators != 0 {
                            XtManageChild(wgadsep);
                        }
                        XmProcessTraversal(wswitch, XmTRAVERSE_CURRENT);
                    } else {
                        XtUnmanageChild(wswitch);
                        if app_res.show_date_time == 0 && app_res.system_tray == 0 {
                            XtUnmanageChild(wgadrc);
                            XtUnmanageChild(wgadsep);
                        }
                    }
                    XtSetValues(wswitch, &raw mut args as ArgList, n as Cardinal);
                } else if (*e_0).atom == xa_cdesk {
                    SwitcherSetActiveWorkspace(wswitch, iws);
                }
            } else {
                fputs(
                    c"Failed to retrieve workspace information.\n".as_ptr(),
                    stderr,
                );
                XtUnmanageChild(wswitch);
                if app_res.show_date_time == 0 && app_res.system_tray == 0 {
                    XtUnmanageChild(wgadrc);
                    XtUnmanageChild(wgadsep);
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn raise_and_focus(mut w: Widget) {
    static mut XaNET_ACTIVE_WINDOW: Atom = 0 as Atom;
    static mut XaWM_STATE: Atom = 0 as Atom;
    static mut XaWM_CHANGE_STATE: Atom = 0 as Atom;
    let mut ret_type: Atom = 0;
    let mut ret_fmt: libc::c_int = 0;
    let mut ret_items: libc::c_ulong = 0;
    let mut ret_bytes: libc::c_ulong = 0;
    let mut state: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut evt: XClientMessageEvent = XClientMessageEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: ::core::ptr::null_mut::<Display>(),
        window: 0,
        message_type: 0,
        format: 0,
        data: C2RustUnnamed_1 { b: [0; 20] },
    };
    let mut dpy: *mut Display = XtDisplay(wshell);
    if XaWM_STATE == 0 as Atom {
        XaWM_STATE = XInternAtom(dpy, c"WM_STATE".as_ptr(), 1 as libc::c_int);
        XaWM_CHANGE_STATE = XInternAtom(dpy, c"WM_CHANGE_STATE".as_ptr(), 1 as libc::c_int);
        XaNET_ACTIVE_WINDOW = XInternAtom(dpy, c"_NET_ACTIVE_WINDOW".as_ptr(), 1 as libc::c_int);
    }
    if XaWM_STATE == 0 as Atom {
        return;
    }
    if XGetWindowProperty(
        dpy,
        XtWindow(w),
        XaWM_STATE,
        0 as libc::c_long,
        1 as libc::c_long,
        0 as libc::c_int,
        XaWM_STATE,
        &raw mut ret_type,
        &raw mut ret_fmt,
        &raw mut ret_items,
        &raw mut ret_bytes,
        &raw mut state as *mut *mut libc::c_uchar,
    ) != 0 as libc::c_int
    {
        return;
    }
    if ret_type == XaWM_STATE && ret_fmt != 0 && *state == 3 as uint32_t {
        evt.type_0 = 33 as libc::c_int;
        evt.send_event = 1 as libc::c_int;
        evt.message_type = XaWM_CHANGE_STATE;
        evt.display = dpy;
        evt.window = XtWindow(w);
        evt.format = 32 as libc::c_int;
        evt.data.l[0] = 1 as libc::c_long;
        XSendEvent(
            dpy,
            (*XtScreen(wshell)).root,
            1 as libc::c_int,
            (1 as libc::c_long) << 19 as libc::c_int | (1 as libc::c_long) << 20 as libc::c_int,
            &raw mut evt as *mut XEvent,
        );
    } else if XaNET_ACTIVE_WINDOW != 0 {
        evt.type_0 = 33 as libc::c_int;
        evt.send_event = 1 as libc::c_int;
        evt.serial = 0 as libc::c_ulong;
        evt.display = dpy;
        evt.window = XtWindow(w);
        evt.message_type = XaNET_ACTIVE_WINDOW;
        evt.format = 32 as libc::c_int;
        XSendEvent(
            dpy,
            (*XtScreen(wshell)).root,
            0 as libc::c_int,
            (1 as libc::c_long) << 19 as libc::c_int | (1 as libc::c_long) << 20 as libc::c_int,
            &raw mut evt as *mut XEvent,
        );
    } else {
        XRaiseWindow(dpy, XtWindow(w));
        XSync(dpy, 0 as libc::c_int);
        XSetInputFocus(dpy, XtWindow(w), 2 as libc::c_int, 0 as Time);
    }
    XFree(state as *mut libc::c_char as *mut libc::c_void);
}
unsafe extern "C" fn construct_menu() -> Boolean {
    let mut args: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<libc::c_char>(),
        value: 0,
    }; 10];
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut wlevel: *mut Widget = ::core::ptr::null_mut::<Widget>();
    let mut nlevels: libc::c_uint = 1 as libc::c_uint;
    let mut entries: *mut tb_entry = ::core::ptr::null_mut::<tb_entry>();
    let mut cur: *mut tb_entry = ::core::ptr::null_mut::<tb_entry>();
    let mut err: libc::c_int = 0;
    if !rc_file_path.is_null() && {
        err = tb_parse_config(rc_file_path as *const libc::c_char, &raw mut entries);
        err != 0
    } {
        report_rcfile_error(
            rc_file_path as *const libc::c_char,
            if !tb_parser_error_string().is_null() {
                tb_parser_error_string()
            } else {
                strerror(err)
            },
        );
        return 0 as Boolean;
    }
    if entries.is_null() && app_res.applications_menu == 0 {
        report_rcfile_error(
            rc_file_path as *const libc::c_char,
            c"File doesn't seem to contain any entries.".as_ptr(),
        );
        return 0 as Boolean;
    }
    cur = entries;
    while !cur.is_null() {
        nlevels = if (*cur).level as libc::c_uint > nlevels {
            (*cur).level as libc::c_uint
        } else {
            nlevels
        };
        cur = (*cur).next;
    }
    if !wmenu.is_null() {
        XtUnmanageChild(wmenu);
        XtDestroyWidget(wmenu);
    }
    n = 0 as libc::c_int;
    args[n as usize].name = c"shadowThickness".as_ptr().cast_mut();
    args[n as usize].value = 0 as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"spacing".as_ptr().cast_mut();
    args[n as usize].value = 1 as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"marginWidth".as_ptr().cast_mut();
    args[n as usize].value = 0 as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"orientation".as_ptr().cast_mut();
    args[n as usize].value = (if app_res.horizontal as libc::c_int != 0 {
        XmHORIZONTAL as libc::c_int
    } else {
        XmVERTICAL as libc::c_int
    }) as XtArgVal;
    n += 1;
    args[n as usize].name = c"packing".as_ptr().cast_mut();
    args[n as usize].value = (if app_res.horizontal as libc::c_int != 0 {
        XmPACK_TIGHT as libc::c_int
    } else {
        XmPACK_COLUMN as libc::c_int
    }) as XtArgVal;
    n += 1;
    args[n as usize].name = c"rowColumnType".as_ptr().cast_mut();
    args[n as usize].value = XmMENU_BAR as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"positionIndex".as_ptr().cast_mut();
    args[n as usize].value = 0 as libc::c_int as XtArgVal;
    n += 1;
    wmenu = XmCreateRowColumn(
        wmain,
        c"menu".as_ptr().cast_mut(),
        &raw mut args as ArgList,
        n as Cardinal,
    );
    if app_res.applications_menu != 0 {
        tb_xdg_menu_create(wmenu, app_res.applications_title, app_res.applications_file);
    }
    wlevel = calloc(
        nlevels.wrapping_add(1 as libc::c_uint) as size_t,
        ::core::mem::size_of::<Widget>() as size_t,
    ) as *mut Widget;
    if wlevel.is_null() {
        perror(c"malloc".as_ptr());
        return 0 as Boolean;
    }
    cur = entries;
    let ref mut fresh0 = *wlevel.offset(0 as libc::c_int as isize);
    *fresh0 = wmenu;
    while !cur.is_null() {
        let mut w: Widget = ::core::ptr::null_mut::<_WidgetRec>();
        let mut title: XmString = ::core::ptr::null_mut::<__XmStringRec>();
        if (*cur).type_0 as libc::c_uint == TBE_CASCADE as libc::c_int as libc::c_uint
            && !(*cur).next.is_null()
        {
            let mut new_pulldown: Widget = ::core::ptr::null_mut::<_WidgetRec>();
            let mut new_cascade: Widget = ::core::ptr::null_mut::<_WidgetRec>();
            new_pulldown = XmCreatePulldownMenu(
                *wlevel.offset((*cur).level as isize),
                c"commandPulldown".as_ptr().cast_mut(),
                ::core::ptr::null_mut::<Arg>(),
                0 as Cardinal,
            );
            title = XmStringCreateLocalized((*cur).title as String_0);
            n = 0 as libc::c_int;
            args[n as usize].name = c"labelString".as_ptr().cast_mut();
            args[n as usize].value = title as XtArgVal;
            n += 1;
            args[n as usize].name = c"mnemonic".as_ptr().cast_mut();
            args[n as usize].value = (*cur).mnemonic as KeySym as XtArgVal;
            n += 1;
            args[n as usize].name = c"subMenuId".as_ptr().cast_mut();
            args[n as usize].value = new_pulldown as XtArgVal;
            n += 1;
            new_cascade = XmCreateCascadeButtonGadget(
                *wlevel.offset((*cur).level as isize),
                c"cascadeButton".as_ptr().cast_mut(),
                &raw mut args as ArgList,
                n as Cardinal,
            );
            XmStringFree(title);
            let ref mut fresh1 = *wlevel.offset((*(*cur).next).level as isize);
            *fresh1 = new_pulldown;
            XtManageChild(new_cascade);
        } else if (*cur).type_0 as libc::c_uint == TBE_COMMAND as libc::c_int as libc::c_uint {
            let mut command: *mut libc::c_char = if !(*cur).command.is_null() {
                strcpy(
                    XtMalloc((strlen((*cur).command) as Cardinal).wrapping_add(1 as Cardinal)),
                    (*cur).command,
                )
            } else {
                ::core::ptr::null_mut::<libc::c_char>()
            };
            let mut push_callback: [XtCallbackRec; 2] = [
                _XtCallbackRec {
                    callback: ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()>,
                        XtCallbackProc,
                    >(Some(
                        menu_command_cb as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> (),
                    )),
                    closure: command as XtPointer,
                },
                _XtCallbackRec {
                    callback: ::core::mem::transmute::<*mut libc::c_void, XtCallbackProc>(
                        ::core::ptr::null_mut::<libc::c_void>(),
                    ),
                    closure: ::core::ptr::null_mut::<libc::c_void>(),
                },
            ];
            title = XmStringCreateLocalized((*cur).title as String_0);
            n = 0 as libc::c_int;
            args[n as usize].name = c"labelString".as_ptr().cast_mut();
            args[n as usize].value = title as XtArgVal;
            n += 1;
            if (*cur).mnemonic != 0 {
                args[n as usize].name = c"mnemonic".as_ptr().cast_mut();
                args[n as usize].value = (*cur).mnemonic as KeySym as XtArgVal;
                n += 1;
            }
            args[n as usize].name = c"activateCallback".as_ptr().cast_mut();
            args[n as usize].value = &raw mut push_callback as *mut XtCallbackRec as XtArgVal;
            n += 1;
            w = XmCreatePushButtonGadget(
                *wlevel.offset((*cur).level as isize),
                c"menuButton".as_ptr().cast_mut(),
                &raw mut args as ArgList,
                n as Cardinal,
            );
            XtAddCallback(
                w,
                c"destroyCallback".as_ptr().cast_mut(),
                Some(free_menu_command as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()),
                command as XtPointer,
            );
            XmStringFree(title);
            XtManageChild(w);
        } else if (*cur).type_0 as libc::c_uint == TBE_SEPARATOR as libc::c_int as libc::c_uint {
            w = XmCreateSeparatorGadget(
                *wlevel.offset((*cur).level as isize),
                c"separator".as_ptr().cast_mut(),
                ::core::ptr::null_mut::<Arg>(),
                0 as Cardinal,
            );
            XtManageChild(w);
        }
        cur = (*cur).next;
    }
    free(wlevel as *mut libc::c_void);
    XtManageChild(wmenu);
    return 1 as Boolean;
}
unsafe extern "C" fn report_rcfile_error(
    mut rc_file: *const libc::c_char,
    mut err_desc: *const libc::c_char,
) {
    let mut buffer: *mut libc::c_char = ::core::ptr::null_mut::<libc::c_char>();
    let mut err_msg: [libc::c_char; 29] =
        ::core::mem::transmute::<[u8; 29], [libc::c_char; 29]>(*b"Error while parsing RC file:\0");
    let mut msg_len: size_t = 0;
    msg_len = strlen(&raw mut err_msg as *mut libc::c_char)
        .wrapping_add(strlen(err_desc))
        .wrapping_add(strlen(rc_file))
        .wrapping_add(10 as size_t);
    buffer = malloc(msg_len) as *mut libc::c_char;
    if buffer.is_null() {
        perror(c"malloc".as_ptr());
        return;
    }
    sprintf(
        buffer,
        c"%s %s\n%s.".as_ptr(),
        &raw mut err_msg as *mut libc::c_char,
        rc_file,
        err_desc,
    );
    message_dialog(0 as Boolean, buffer);
    free(buffer as *mut libc::c_void);
}
unsafe extern "C" fn create_utility_widgets(mut wparent: Widget) {
    let mut cbr: [XtCallbackRec; 2] = [
        _XtCallbackRec {
            callback: None,
            closure: ::core::ptr::null_mut::<libc::c_void>(),
        },
        _XtCallbackRec {
            callback: None,
            closure: ::core::ptr::null_mut::<libc::c_void>(),
        },
    ];
    let mut wmenu_0: Widget = ::core::ptr::null_mut::<_WidgetRec>();
    let mut wpulldown: Widget = ::core::ptr::null_mut::<_WidgetRec>();
    let mut wcascade: Widget = ::core::ptr::null_mut::<_WidgetRec>();
    let mut footer: Widget = ::core::ptr::null_mut::<_WidgetRec>();
    let mut w: Widget = ::core::ptr::null_mut::<_WidgetRec>();
    let mut title: XmString = ::core::ptr::null_mut::<__XmStringRec>();
    let mut nws: libc::c_ushort = 0 as libc::c_ushort;
    let mut iws: libc::c_ushort = 0 as libc::c_ushort;
    let mut args: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<libc::c_char>(),
        value: 0,
    }; 10];
    let mut n: libc::c_int = 0;
    args[0].name = c"orientation".as_ptr().cast_mut();
    args[0].value = (if app_res.horizontal as libc::c_int != 0 {
        XmVERTICAL as libc::c_int
    } else {
        XmHORIZONTAL as libc::c_int
    }) as XtArgVal;
    w = XmCreateSeparatorGadget(
        wparent,
        c"separator".as_ptr().cast_mut(),
        &raw mut args as ArgList,
        1 as Cardinal,
    );
    if app_res.separators != 0 {
        XtManageChild(w);
    }
    n = 0 as libc::c_int;
    args[n as usize].name = c"shadowThickness".as_ptr().cast_mut();
    args[n as usize].value = 0 as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"spacing".as_ptr().cast_mut();
    args[n as usize].value = 1 as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"marginWidth".as_ptr().cast_mut();
    args[n as usize].value = 0 as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"orientation".as_ptr().cast_mut();
    args[n as usize].value = (if app_res.horizontal as libc::c_int != 0 {
        XmHORIZONTAL as libc::c_int
    } else {
        XmVERTICAL as libc::c_int
    }) as XtArgVal;
    n += 1;
    args[n as usize].name = c"rowColumnType".as_ptr().cast_mut();
    args[n as usize].value = XmMENU_BAR as libc::c_int as XtArgVal;
    n += 1;
    wmenu_0 = XmCreateRowColumn(
        wparent,
        c"menu".as_ptr().cast_mut(),
        &raw mut args as ArgList,
        n as Cardinal,
    );
    wpulldown = XmCreatePulldownMenu(
        wmenu_0,
        c"sessionPulldown".as_ptr().cast_mut(),
        ::core::ptr::null_mut::<Arg>(),
        0 as Cardinal,
    );
    title = XmStringCreateLocalized(c"Session".as_ptr().cast_mut());
    n = 0 as libc::c_int;
    args[n as usize].name = c"labelString".as_ptr().cast_mut();
    args[n as usize].value = title as XtArgVal;
    n += 1;
    args[n as usize].name = c"mnemonic".as_ptr().cast_mut();
    args[n as usize].value = 'S' as i32 as KeySym as XtArgVal;
    n += 1;
    args[n as usize].name = c"subMenuId".as_ptr().cast_mut();
    args[n as usize].value = wpulldown as XtArgVal;
    n += 1;
    wcascade = XmCreateCascadeButtonGadget(
        wmenu_0,
        c"session".as_ptr().cast_mut(),
        &raw mut args as ArgList,
        n as Cardinal,
    );
    XmStringFree(title);
    XtManageChild(wcascade);
    n = 0 as libc::c_int;
    cbr[0].callback =
        Some(exec_cb as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()) as XtCallbackProc;
    title = XmStringCreateLocalized(c"Execute...".as_ptr().cast_mut());
    args[n as usize].name = c"labelString".as_ptr().cast_mut();
    args[n as usize].value = title as XtArgVal;
    n += 1;
    args[n as usize].name = c"mnemonic".as_ptr().cast_mut();
    args[n as usize].value = 'E' as i32 as KeySym as XtArgVal;
    n += 1;
    args[n as usize].name = c"activateCallback".as_ptr().cast_mut();
    args[n as usize].value = &raw mut cbr as *mut XtCallbackRec as XtArgVal;
    n += 1;
    w = XmCreatePushButtonGadget(
        wpulldown,
        c"execute".as_ptr().cast_mut(),
        &raw mut args as ArgList,
        n as Cardinal,
    );
    XmStringFree(title);
    XtManageChild(w);
    w = XmCreateSeparatorGadget(
        wpulldown,
        c"separator".as_ptr().cast_mut(),
        ::core::ptr::null_mut::<Arg>(),
        0 as Cardinal,
    );
    XtManageChild(w);
    if xmsm_cfg & 0x2 as libc::c_ulong != 0 {
        n = 0 as libc::c_int;
        cbr[0].callback = Some(lock_cb as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ())
            as XtCallbackProc;
        title = XmStringCreateLocalized(c"Lock".as_ptr().cast_mut());
        args[n as usize].name = c"labelString".as_ptr().cast_mut();
        args[n as usize].value = title as XtArgVal;
        n += 1;
        args[n as usize].name = c"mnemonic".as_ptr().cast_mut();
        args[n as usize].value = 'L' as i32 as KeySym as XtArgVal;
        n += 1;
        args[n as usize].name = c"activateCallback".as_ptr().cast_mut();
        args[n as usize].value = &raw mut cbr as *mut XtCallbackRec as XtArgVal;
        n += 1;
        w = XmCreatePushButtonGadget(
            wpulldown,
            c"lock".as_ptr().cast_mut(),
            &raw mut args as ArgList,
            n as Cardinal,
        );
        XmStringFree(title);
        XtManageChild(w);
    }
    n = 0 as libc::c_int;
    cbr[0].callback = Some(logout_cb as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ())
        as XtCallbackProc;
    title = XmStringCreateLocalized(c"Logout...".as_ptr().cast_mut());
    args[n as usize].name = c"labelString".as_ptr().cast_mut();
    args[n as usize].value = title as XtArgVal;
    n += 1;
    args[n as usize].name = c"mnemonic".as_ptr().cast_mut();
    args[n as usize].value = 'o' as i32 as KeySym as XtArgVal;
    n += 1;
    args[n as usize].name = c"activateCallback".as_ptr().cast_mut();
    args[n as usize].value = &raw mut cbr as *mut XtCallbackRec as XtArgVal;
    n += 1;
    w = XmCreatePushButtonGadget(
        wpulldown,
        c"logout".as_ptr().cast_mut(),
        &raw mut args as ArgList,
        n as Cardinal,
    );
    XmStringFree(title);
    XtManageChild(w);
    if xmsm_cfg & 0x1 as libc::c_ulong != 0 {
        n = 0 as libc::c_int;
        cbr[0].callback =
            Some(suspend_cb as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ())
                as XtCallbackProc;
        title = XmStringCreateLocalized(c"Suspend".as_ptr().cast_mut());
        args[n as usize].name = c"labelString".as_ptr().cast_mut();
        args[n as usize].value = title as XtArgVal;
        n += 1;
        args[n as usize].name = c"mnemonic".as_ptr().cast_mut();
        args[n as usize].value = 'S' as i32 as KeySym as XtArgVal;
        n += 1;
        args[n as usize].name = c"activateCallback".as_ptr().cast_mut();
        args[n as usize].value = &raw mut cbr as *mut XtCallbackRec as XtArgVal;
        n += 1;
        w = XmCreatePushButtonGadget(
            wpulldown,
            c"suspend".as_ptr().cast_mut(),
            &raw mut args as ArgList,
            n as Cardinal,
        );
        XmStringFree(title);
        XtManageChild(w);
    }
    XtManageChild(wmenu_0);
    args[0].name = c"orientation".as_ptr().cast_mut();
    args[0].value = (if app_res.horizontal as libc::c_int != 0 {
        XmVERTICAL as libc::c_int
    } else {
        XmHORIZONTAL as libc::c_int
    }) as XtArgVal;
    wgadsep = XmCreateSeparatorGadget(
        wparent,
        c"separator".as_ptr().cast_mut(),
        &raw mut args as ArgList,
        1 as Cardinal,
    );
    n = 0 as libc::c_int;
    args[n as usize].name = c"marginWidth".as_ptr().cast_mut();
    args[n as usize].value = 3 as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"marginHeight".as_ptr().cast_mut();
    args[n as usize].value = 3 as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"spacing".as_ptr().cast_mut();
    args[n as usize].value = 3 as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"packing".as_ptr().cast_mut();
    args[n as usize].value = XmPACK_TIGHT as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"orientation".as_ptr().cast_mut();
    args[n as usize].value = (if app_res.horizontal as libc::c_int != 0 {
        XmHORIZONTAL as libc::c_int
    } else {
        XmVERTICAL as libc::c_int
    }) as XtArgVal;
    n += 1;
    wgadrc = XmCreateRowColumn(
        wparent,
        c"gadgets".as_ptr().cast_mut(),
        &raw mut args as ArgList,
        n as Cardinal,
    );
    n = 0 as libc::c_int;
    if get_ws_info(&raw mut nws, &raw mut iws) != 0 {
        args[n as usize].name = c"numberOfWorkspaces".as_ptr().cast_mut();
        args[n as usize].value = nws as XtArgVal;
        n += 1;
        args[n as usize].name = c"activeWorkspace".as_ptr().cast_mut();
        args[n as usize].value = iws as XtArgVal;
        n += 1;
    }
    cbr[0].callback = Some(ws_change_cb as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ())
        as XtCallbackProc;
    args[n as usize].name = c"valueChangedCallback".as_ptr().cast_mut();
    args[n as usize].value = &raw mut cbr as XtArgVal;
    n += 1;
    wswitch = XtCreateWidget(
        c"workspaceSwitcher".as_ptr(),
        switcherWidgetClass,
        wgadrc,
        &raw mut args as ArgList,
        n as Cardinal,
    );
    if app_res.switcher as libc::c_int != 0 && nws as libc::c_int > 1 as libc::c_int {
        XtManageChild(wswitch);
        if app_res.separators != 0 {
            XtManageChild(wgadsep);
        }
    }
    footer = XmVaCreateManagedRowColumn(
        wgadrc,
        c"status".as_ptr().cast_mut(),
        c"orientation".as_ptr().cast_mut(),
        XmVERTICAL as libc::c_int,
        c"marginWidth".as_ptr().cast_mut() as *mut libc::c_char,
        0 as libc::c_int,
        c"marginHeight".as_ptr().cast_mut() as *mut libc::c_char,
        0 as libc::c_int,
        c"spacing".as_ptr().cast_mut() as *mut libc::c_char,
        3 as libc::c_int,
        ::core::ptr::null_mut::<libc::c_void>(),
    );
    n = 0 as libc::c_int;
    args[n as usize].name = c"shadowType".as_ptr().cast_mut();
    args[n as usize].value = XmSHADOW_IN as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"shadowThickness".as_ptr().cast_mut();
    args[n as usize].value = 1 as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"marginWidth".as_ptr().cast_mut();
    args[n as usize].value = 2 as libc::c_int as XtArgVal;
    n += 1;
    args[n as usize].name = c"marginHeight".as_ptr().cast_mut();
    args[n as usize].value = 2 as libc::c_int as XtArgVal;
    n += 1;
    wdtframe = XmCreateFrame(
        footer,
        c"dateTimeFrame".as_ptr().cast_mut(),
        &raw mut args as ArgList,
        n as Cardinal,
    );
    n = 0 as libc::c_int;
    args[n as usize].name = c"alignment".as_ptr().cast_mut();
    args[n as usize].value = XmALIGNMENT_CENTER as libc::c_int as XtArgVal;
    n += 1;
    wdtlabel = XmCreateLabelGadget(
        wdtframe,
        c"dateTime".as_ptr().cast_mut(),
        &raw mut args as *mut Arg,
        n as Cardinal,
    );
    if app_res.show_date_time != 0 {
        XtManageChild(wdtlabel);
        XtManageChild(wdtframe);
        if app_res.separators != 0 {
            XtManageChild(wgadsep);
        }
        time_update_cb(
            ::core::ptr::null_mut::<libc::c_void>(),
            ::core::ptr::null_mut::<XtIntervalId>(),
        );
    }
    if app_res.system_tray != 0 {
        TrayCreate(footer, wshell);
        if app_res.separators != 0 {
            XtManageChild(wgadsep);
        }
    }
    if app_res.show_date_time == 0 && app_res.system_tray == 0 {
        XtUnmanageChild(footer);
    }
    if XtIsManaged(wswitch) as libc::c_int != 0
        || app_res.show_date_time as libc::c_int != 0
        || app_res.system_tray as libc::c_int != 0
    {
        XtManageChild(wgadrc);
    }
}
fn find_rc_file() -> *mut libc::c_char {
    use std::ffi::{CString, OsString};
    use std::os::unix::ffi::OsStringExt;

    let Some(home) = std::env::var_os("HOME") else {
        eprintln!("HOME is not set!");
        return std::ptr::null_mut();
    };
    let mut personal = home;
    personal.push("/.toolboxrc");
    let lang = std::env::var_os("LANG");
    let mut candidates = vec![personal];
    for base in [
        option_env!("EMWM_RCDIR").unwrap_or("/usr/share/emwm-desktop"),
        "/etc/X11",
        "/usr/lib/X11",
        "/usr/local/lib/X11",
    ] {
        if let Some(locale) = &lang {
            let mut localized = OsString::from(base);
            localized.push("/");
            localized.push(locale);
            localized.push("/toolboxrc");
            candidates.push(localized);
        }
        candidates.push(OsString::from(format!("{base}/toolboxrc")));
    }
    for candidate in candidates {
        let Ok(path) = CString::new(candidate.into_vec()) else {
            continue;
        };
        // SAFETY: access reads the CString during this call. Xt's caller holds
        // the allocated path while reloading the same menu throughout the session.
        unsafe {
            if libc::access(path.as_ptr(), libc::R_OK) == 0 {
                return libc::strdup(path.as_ptr());
            }
        }
    }
    std::ptr::null_mut()
}
unsafe extern "C" fn time_update_cb(mut client_data: XtPointer, mut id: *mut XtIntervalId) {
    let mut args: [Arg; 2] = [Arg {
        name: ::core::ptr::null_mut::<libc::c_char>(),
        value: 0,
    }; 2];
    let mut time_str: [libc::c_char; 256] = [0; 256];
    let mut secs: time_t = 0;
    let mut the_time: *mut tm = ::core::ptr::null_mut::<tm>();
    let mut xm_str: XmString = ::core::ptr::null_mut::<__XmStringRec>();
    static mut init: Boolean = 1 as Boolean;
    time(&raw mut secs);
    the_time = localtime(&raw mut secs);
    strftime(
        &raw mut time_str as *mut libc::c_char,
        255 as size_t,
        app_res.date_time_fmt,
        the_time,
    );
    xm_str = XmStringCreateLocalized(&raw mut time_str as String_0);
    args[0].name = c"labelString".as_ptr().cast_mut();
    args[0].value = xm_str as XtArgVal;
    XtSetValues(wdtlabel, &raw mut args as ArgList, 1 as Cardinal);
    if init != 0 {
        args[0].name = c"recomputeSize".as_ptr().cast_mut();
        args[0].value = 0 as libc::c_int as XtArgVal;
        XtSetValues(wdtlabel, &raw mut args as ArgList, 1 as Cardinal);
        init = 0 as Boolean;
    }
    XmStringFree(xm_str);
    XtAppAddTimeOut(
        app_context,
        (60000 as libc::c_int - (*the_time).tm_sec * 1000 as libc::c_int) as libc::c_ulong,
        Some(time_update_cb as unsafe extern "C" fn(XtPointer, *mut XtIntervalId) -> ()),
        ::core::ptr::null_mut::<libc::c_void>(),
    );
}
unsafe extern "C" fn xt_sigusr1_handler(mut client_data: XtPointer, mut id: *mut XtSignalId) {
    construct_menu();
}
unsafe extern "C" fn exec_cb(mut w: Widget, mut client_data: XtPointer, mut call_data: XtPointer) {
    static mut wdlg: Widget = ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec;
    static mut wtext: Widget = ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec;
    let mut args: [Arg; 5] = [Arg {
        name: ::core::ptr::null_mut::<libc::c_char>(),
        value: 0,
    }; 5];
    let mut n: libc::c_int = 0 as libc::c_int;
    if wdlg.is_null() {
        let mut xm_title: XmString = ::core::ptr::null_mut::<__XmStringRec>();
        let mut xm_prompt: XmString = ::core::ptr::null_mut::<__XmStringRec>();
        let mut callback: [XtCallbackRec; 2] = [
            _XtCallbackRec {
                callback: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()>,
                    XtCallbackProc,
                >(Some(
                    exec_dialog_cb as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> (),
                )),
                closure: ::core::ptr::null_mut::<libc::c_void>(),
            },
            _XtCallbackRec {
                callback: ::core::mem::transmute::<*mut libc::c_void, XtCallbackProc>(
                    ::core::ptr::null_mut::<libc::c_void>(),
                ),
                closure: ::core::ptr::null_mut::<libc::c_void>(),
            },
        ];
        let mut alt_tt_src: [libc::c_char; 161] = ::core::mem::transmute::<
            [u8; 161],
            [libc::c_char; 161],
        >(
            *b":s <Key>osfEndLine: end-of-line(extend)\n:s <Key>osfBeginLine: beginning-of-line(extend)\n:<Key>osfEndLine: end-of-line()\n:<Key>osfBeginLine: beginning-of-line()\n\0",
        );
        let mut alt_tt: XtTranslations = ::core::ptr::null_mut::<_TranslationData>();
        n = 0 as libc::c_int;
        xm_title = XmStringCreateLocalized(c"Toolbox".as_ptr().cast_mut());
        xm_prompt = XmStringCreateLocalized(c"Specify a command".as_ptr().cast_mut());
        args[n as usize].name = c"dialogTitle".as_ptr().cast_mut();
        args[n as usize].value = xm_title as XtArgVal;
        n += 1;
        args[n as usize].name = c"okCallback".as_ptr().cast_mut();
        args[n as usize].value = &raw mut callback as *mut XtCallbackRec as XtArgVal;
        n += 1;
        args[n as usize].name = c"cancelCallback".as_ptr().cast_mut();
        args[n as usize].value = &raw mut callback as *mut XtCallbackRec as XtArgVal;
        n += 1;
        args[n as usize].name = c"selectionLabelString".as_ptr().cast_mut();
        args[n as usize].value = xm_prompt as XtArgVal;
        n += 1;
        wdlg = XmCreatePromptDialog(
            wshell,
            c"promptDialog".as_ptr().cast_mut(),
            &raw mut args as ArgList,
            n as Cardinal,
        );
        XmStringFree(xm_title);
        XmStringFree(xm_prompt);
        wtext = XmSelectionBoxGetChild(wdlg, XmDIALOG_TEXT as libc::c_int as libc::c_uchar);
        alt_tt = XtParseTranslationTable(&raw mut alt_tt_src as *mut libc::c_char);
        if !alt_tt.is_null() {
            XtOverrideTranslations(wtext, alt_tt);
        }
        XtUnmanageChild(XmSelectionBoxGetChild(
            wdlg,
            XmDIALOG_HELP_BUTTON as libc::c_int as libc::c_uchar,
        ));
    } else {
        let mut text: *mut libc::c_char = ::core::ptr::null_mut::<libc::c_char>();
        let mut len: size_t = 0;
        text = XmTextFieldGetString(wtext);
        len = strlen(text);
        if len != 0 {
            XmTextFieldSetSelection(
                wtext,
                0 as XmTextPosition,
                len as XmTextPosition,
                XtLastTimestampProcessed(XtDisplay(wtext)),
            );
        }
        XtFree(text);
    }
    XtManageChild(wdlg);
}
unsafe extern "C" fn exec_dialog_cb(
    mut w: Widget,
    mut client_data: XtPointer,
    mut call_data: XtPointer,
) {
    let mut command: *mut libc::c_char = ::core::ptr::null_mut::<libc::c_char>();
    let mut exp_cmd: *mut libc::c_char = ::core::ptr::null_mut::<libc::c_char>();
    let mut errval: libc::c_int = 0;
    let mut cbs: *mut XmSelectionBoxCallbackStruct = call_data as *mut XmSelectionBoxCallbackStruct;
    if (*cbs).reason == XmCR_CANCEL as libc::c_int {
        return;
    }
    command = XmStringUnparse(
        (*cbs).value,
        ::core::ptr::null_mut::<libc::c_char>(),
        XmCHARSET_TEXT,
        XmCHARSET_TEXT,
        ::core::ptr::null_mut::<XmParseMapping>(),
        0 as Cardinal,
        XmOUTPUT_ALL,
    ) as *mut libc::c_char;
    if command.is_null() {
        return;
    }
    if strlen(command) == 0 {
        XtFree(command);
        return;
    }
    errval = expand_env_vars(command, &raw mut exp_cmd);
    if errval != 0 {
        report_exec_error(c"Failed to parse command string".as_ptr(), command, errval);
        XtFree(command);
        return;
    }
    XtFree(command);
    errval = exec_command(exp_cmd);
    if errval != 0 {
        report_exec_error(c"Error executing command".as_ptr(), exp_cmd, errval);
    }
    free(exp_cmd as *mut libc::c_void);
}
unsafe extern "C" fn message_dialog(
    mut confirm: Boolean,
    mut message_str: *const libc::c_char,
) -> Boolean {
    let mut wdlg: Widget = ::core::ptr::null_mut::<_WidgetRec>();
    let mut xm_message_str: XmString = ::core::ptr::null_mut::<__XmStringRec>();
    let mut args: [Arg; 8] = [Arg {
        name: ::core::ptr::null_mut::<libc::c_char>(),
        value: 0,
    }; 8];
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut result: libc::c_int = -(1 as libc::c_int);
    let mut xm_title: XmString = ::core::ptr::null_mut::<__XmStringRec>();
    let mut callback: [XtCallbackRec; 2] = [
        _XtCallbackRec {
            callback: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()>,
                XtCallbackProc,
            >(Some(
                message_dialog_cb as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> (),
            )),
            closure: &raw mut result as XtPointer,
        },
        _XtCallbackRec {
            callback: ::core::mem::transmute::<*mut libc::c_void, XtCallbackProc>(
                ::core::ptr::null_mut::<libc::c_void>(),
            ),
            closure: ::core::ptr::null_mut::<libc::c_void>(),
        },
    ];
    xm_message_str = XmStringCreateLocalized(message_str as String_0);
    xm_title = XmStringCreateLocalized(c"Toolbox".as_ptr().cast_mut());
    args[n as usize].name = c"dialogTitle".as_ptr().cast_mut();
    args[n as usize].value = xm_title as XtArgVal;
    n += 1;
    args[n as usize].name = c"okCallback".as_ptr().cast_mut();
    args[n as usize].value = &raw mut callback as *mut XtCallbackRec as XtArgVal;
    n += 1;
    args[n as usize].name = c"cancelCallback".as_ptr().cast_mut();
    args[n as usize].value = &raw mut callback as *mut XtCallbackRec as XtArgVal;
    n += 1;
    args[n as usize].name = c"messageString".as_ptr().cast_mut();
    args[n as usize].value = xm_message_str as XtArgVal;
    n += 1;
    args[n as usize].name = c"dialogStyle".as_ptr().cast_mut();
    args[n as usize].value = XmDIALOG_PRIMARY_APPLICATION_MODAL as libc::c_int as XtArgVal;
    n += 1;
    wdlg = XmCreateMessageDialog(
        wshell,
        c"messageDialog".as_ptr().cast_mut(),
        &raw mut args as ArgList,
        n as Cardinal,
    );
    n = 0 as libc::c_int;
    args[n as usize].name = c"dialogType".as_ptr().cast_mut();
    args[n as usize].value = (if confirm as libc::c_int != 0 {
        XmDIALOG_QUESTION as libc::c_int
    } else {
        XmDIALOG_INFORMATION as libc::c_int
    }) as XtArgVal;
    n += 1;
    args[n as usize].name = c"defaultButtonType".as_ptr().cast_mut();
    args[n as usize].value = (if confirm as libc::c_int != 0 {
        XmDIALOG_CANCEL_BUTTON as libc::c_int
    } else {
        XmDIALOG_OK_BUTTON as libc::c_int
    }) as XtArgVal;
    n += 1;
    XtSetValues(wdlg, &raw mut args as ArgList, n as Cardinal);
    XmStringFree(xm_title);
    XmStringFree(xm_message_str);
    if confirm == 0 {
        XtUnmanageChild(XmMessageBoxGetChild(
            wdlg,
            XmDIALOG_CANCEL_BUTTON as libc::c_int as libc::c_uchar,
        ));
    }
    XtUnmanageChild(XmMessageBoxGetChild(
        wdlg,
        XmDIALOG_HELP_BUTTON as libc::c_int as libc::c_uchar,
    ));
    XtManageChild(wdlg);
    while XtIsManaged(wdlg) as libc::c_int != 0 && result == -(1 as libc::c_int) {
        XtAppProcessEvent(app_context, 1 as XtInputMask);
    }
    if result == -(1 as libc::c_int) {
        result = 0 as libc::c_int;
    }
    let display = XtDisplay(wdlg);
    XtDestroyWidget(wdlg);
    XSync(display, 0);
    XmUpdateDisplay(wshell);
    return result as Boolean;
}
unsafe extern "C" fn message_dialog_cb(
    mut w: Widget,
    mut client_data: XtPointer,
    mut call_data: XtPointer,
) {
    // SAFETY: message_dialog supplies its live result integer and Motif supplies the callback.
    let cbs = &*call_data.cast::<XmSelectionBoxCallbackStruct>();
    *client_data.cast::<libc::c_int>() = i32::from(cbs.reason == XmCR_OK as libc::c_int);
}

unsafe fn report_exec_error(
    message: *const libc::c_char,
    command: *const libc::c_char,
    code: libc::c_int,
) {
    use std::ffi::CStr;
    // SAFETY: command and message are C strings retained throughout the modal callback.
    let message = CStr::from_ptr(message).to_bytes();
    let command = CStr::from_ptr(command).to_bytes();
    let reason = CStr::from_ptr(libc::strerror(code)).to_bytes();
    let text = [message, b" '", command, b"'.\n", reason, b".\0"].concat();
    message_dialog(0, text.as_ptr().cast());
}
unsafe extern "C" fn menu_command_cb(
    mut w: Widget,
    mut client_data: XtPointer,
    mut call_data: XtPointer,
) {
    let mut errval: libc::c_int = 0;
    let mut cmd: *mut libc::c_char = client_data as *mut libc::c_char;
    let mut exp_cmd: *mut libc::c_char = ::core::ptr::null_mut::<libc::c_char>();
    errval = expand_env_vars(cmd, &raw mut exp_cmd);
    if errval != 0 {
        report_exec_error(c"Failed to parse command string".as_ptr(), cmd, errval);
        return;
    }
    errval = exec_command(exp_cmd);
    if errval != 0 {
        report_exec_error(c"Error executing command".as_ptr(), exp_cmd, errval);
    }
    free(exp_cmd as *mut libc::c_void);
}
unsafe extern "C" fn send_xmsm_cmd(mut command: *const libc::c_char) -> Boolean {
    let mut dpy: *mut Display = XtDisplay(wshell);
    let mut root: Window = (*XtScreen(wshell)).root;
    let mut shell: Window = 0;
    let mut ret_type: Atom = 0;
    let mut ret_format: libc::c_int = 0;
    let mut ret_items: libc::c_ulong = 0;
    let mut left_items: libc::c_ulong = 0;
    let mut prop_data: *mut libc::c_uchar = ::core::ptr::null_mut::<libc::c_uchar>();
    if xa_xmsm_mgr == 0 as Atom || xa_xmsm_pid == 0 as Atom {
        return 0 as Boolean;
    }
    XGetWindowProperty(
        dpy,
        root,
        xa_xmsm_mgr,
        0 as libc::c_long,
        ::core::mem::size_of::<Window>() as libc::c_long,
        0 as libc::c_int,
        33 as libc::c_int as Atom,
        &raw mut ret_type,
        &raw mut ret_format,
        &raw mut ret_items,
        &raw mut left_items,
        &raw mut prop_data,
    );
    if ret_type == 33 as libc::c_int as Atom {
        shell = *(prop_data as *mut Window);
        XFree(prop_data as *mut libc::c_void);
        def_x_err_handler = XSetErrorHandler(Some(
            local_x_err_handler
                as unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
        ))
            as Option<unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int>;
        sm_reqstat = 1 as Boolean;
        XGetWindowProperty(
            dpy,
            shell,
            xa_xmsm_pid,
            0 as libc::c_long,
            ::core::mem::size_of::<pid_t>() as libc::c_long,
            0 as libc::c_int,
            19 as libc::c_int as Atom,
            &raw mut ret_type,
            &raw mut ret_format,
            &raw mut ret_items,
            &raw mut left_items,
            &raw mut prop_data,
        );
        if ret_items != 0 {
            XFree(prop_data as *mut libc::c_void);
        }
        if ret_type == 19 as libc::c_int as Atom {
            let mut evt: XClientMessageEvent = XClientMessageEvent {
                type_0: 33 as libc::c_int,
                serial: 0 as libc::c_ulong,
                send_event: 1 as libc::c_int,
                display: dpy,
                window: 0 as Window,
                message_type: xa_xmsm_cmd,
                format: 8 as libc::c_int,
                data: C2RustUnnamed_1 { b: [0; 20] },
            };
            memcpy(
                &raw mut evt.data.b as *mut libc::c_char as *mut libc::c_void,
                command as *const libc::c_void,
                strlen(command).wrapping_add(1 as size_t),
            );
            XSendEvent(
                dpy,
                shell,
                1 as libc::c_int,
                0 as libc::c_long,
                &raw mut evt as *mut XEvent,
            );
            XSync(dpy, 0 as libc::c_int);
            XSetErrorHandler(def_x_err_handler as XErrorHandler);
            return sm_reqstat;
        }
        XSync(dpy, 0 as libc::c_int);
        XSetErrorHandler(def_x_err_handler as XErrorHandler);
    }
    return 0 as Boolean;
}
unsafe extern "C" fn get_xmsm_config(mut flags: *mut libc::c_ulong) -> Boolean {
    let mut dpy: *mut Display = XtDisplay(wshell);
    let mut root: Window = (*XtScreen(wshell)).root;
    let mut ret_type: Atom = 0;
    let mut ret_format: libc::c_int = 0;
    let mut ret_items: libc::c_ulong = 0;
    let mut left_items: libc::c_ulong = 0;
    let mut prop_data: *mut libc::c_uchar = ::core::ptr::null_mut::<libc::c_uchar>();
    if xa_xmsm_cfg == 0 as Atom {
        return 0 as Boolean;
    }
    XGetWindowProperty(
        dpy,
        root,
        xa_xmsm_cfg,
        0 as libc::c_long,
        ::core::mem::size_of::<libc::c_ulong>() as libc::c_long,
        0 as libc::c_int,
        19 as libc::c_int as Atom,
        &raw mut ret_type,
        &raw mut ret_format,
        &raw mut ret_items,
        &raw mut left_items,
        &raw mut prop_data,
    );
    if ret_items != 0 {
        *flags = *prop_data as libc::c_ulong;
        XFree(prop_data as *mut libc::c_void);
        return 1 as Boolean;
    }
    return 0 as Boolean;
}
unsafe extern "C" fn local_x_err_handler(
    mut dpy: *mut Display,
    mut evt: *mut XErrorEvent,
) -> libc::c_int {
    if (*evt).error_code as libc::c_int == 3 as libc::c_int {
        sm_reqstat = 0 as Boolean;
        return 0 as libc::c_int;
    }
    return def_x_err_handler.expect("non-null function pointer")(dpy, evt);
}
unsafe extern "C" fn get_ws_info(
    mut ws_count: *mut libc::c_ushort,
    mut iactive: *mut libc::c_ushort,
) -> Boolean {
    let mut success: Boolean = 1 as Boolean;
    let mut dpy: *mut Display = XtDisplay(wshell);
    let mut root: Window = (*XtScreen(wshell)).root;
    let mut ret_type: Atom = 0;
    let mut ret_format: libc::c_int = 0;
    let mut ret_items: libc::c_ulong = 0;
    let mut left_items: libc::c_ulong = 0;
    let mut prop_data: *mut libc::c_uchar = ::core::ptr::null_mut::<libc::c_uchar>();
    if xa_ndesks == 0 as Atom || xa_cdesk == 0 as Atom {
        return 0 as Boolean;
    }
    XGetWindowProperty(
        dpy,
        root,
        xa_ndesks,
        0 as libc::c_long,
        ::core::mem::size_of::<libc::c_ulong>() as libc::c_long,
        0 as libc::c_int,
        6 as libc::c_int as Atom,
        &raw mut ret_type,
        &raw mut ret_format,
        &raw mut ret_items,
        &raw mut left_items,
        &raw mut prop_data,
    );
    if ret_items != 0 {
        *ws_count = *prop_data as libc::c_ushort;
        XFree(prop_data as *mut libc::c_void);
    } else {
        ws_count = ::core::ptr::null_mut::<libc::c_ushort>();
        success = 0 as Boolean;
    }
    XGetWindowProperty(
        dpy,
        root,
        xa_cdesk,
        0 as libc::c_long,
        ::core::mem::size_of::<libc::c_ulong>() as libc::c_long,
        0 as libc::c_int,
        6 as libc::c_int as Atom,
        &raw mut ret_type,
        &raw mut ret_format,
        &raw mut ret_items,
        &raw mut left_items,
        &raw mut prop_data,
    );
    if ret_items != 0 {
        *iactive = *prop_data as libc::c_ushort;
        XFree(prop_data as *mut libc::c_void);
    } else {
        *iactive = 0 as libc::c_ushort;
        success = 0 as Boolean;
    }
    return success;
}
unsafe extern "C" fn ws_change_cb(
    mut w: Widget,
    mut client_data: XtPointer,
    mut call_data: XtPointer,
) {
    let mut dpy: *mut Display = XtDisplay(wshell);
    let mut root_wnd: Window = (*XtScreen(wshell)).root;
    let mut index: *mut libc::c_short = call_data as *mut libc::c_short;
    let mut evt: XClientMessageEvent = XClientMessageEvent {
        type_0: 33 as libc::c_int,
        serial: 0,
        send_event: 0,
        display: dpy,
        window: root_wnd,
        message_type: xa_cdesk,
        format: 32 as libc::c_int,
        data: C2RustUnnamed_1 { b: [0; 20] },
    };
    evt.data.l[0] = *index as libc::c_long;
    evt.data.l[1] = 0 as libc::c_long;
    XSendEvent(
        dpy,
        root_wnd,
        0 as libc::c_int,
        (1 as libc::c_long) << 20 as libc::c_int | (1 as libc::c_long) << 19 as libc::c_int,
        &raw mut evt as *mut XEvent,
    );
}
unsafe extern "C" fn suspend_cb(
    mut w: Widget,
    mut client_data: XtPointer,
    mut call_data: XtPointer,
) {
    if send_xmsm_cmd(c"SUSPEND".as_ptr()) == 0 {
        message_dialog(0 as Boolean, &raw const xmsm_cmd_err as *const libc::c_char);
    }
}
unsafe extern "C" fn lock_cb(mut w: Widget, mut client_data: XtPointer, mut call_data: XtPointer) {
    if send_xmsm_cmd(c"LOCK".as_ptr()) == 0 {
        message_dialog(0 as Boolean, &raw const xmsm_cmd_err as *const libc::c_char);
    }
}
unsafe extern "C" fn logout_cb(
    mut w: Widget,
    mut client_data: XtPointer,
    mut call_data: XtPointer,
) {
    if send_xmsm_cmd(c"LOGOUT".as_ptr()) == 0 {
        message_dialog(0 as Boolean, &raw const xmsm_cmd_err as *const libc::c_char);
    }
}
unsafe extern "C" fn sigchld_handler(mut sig: libc::c_int) {
    let mut status: libc::c_int = 0;
    waitpid(-(1 as __pid_t), &raw mut status, 1 as libc::c_int);
}
unsafe extern "C" fn sigusr_handler(mut sig: libc::c_int) {
    if sig == 10 as libc::c_int {
        XtNoticeSignal(xt_sigusr1);
    }
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args_os()
        .map(|arg| {
            ::std::ffi::CString::new(std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()))
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut libc::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut libc::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as libc::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
unsafe extern "C" fn run_static_initializers() {
    xrdb_resources = [
        _XtResource {
            resource_name: c"systemTray".as_ptr().cast_mut(),
            resource_class: c"SystemTray".as_ptr().cast_mut(),
            resource_type: c"Boolean".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, system_tray) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: 1 as libc::c_int as XtPointer,
        },
        _XtResource {
            resource_name: c"applicationsMenu".as_ptr().cast_mut(),
            resource_class: c"ApplicationsMenu".as_ptr().cast_mut(),
            resource_type: c"Boolean".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, applications_menu) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: 1 as libc::c_int as XtPointer,
        },
        _XtResource {
            resource_name: c"applicationsMenuTitle".as_ptr().cast_mut(),
            resource_class: c"ApplicationsMenuTitle".as_ptr().cast_mut(),
            resource_type: c"String".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, applications_title) as Cardinal,
            default_type: c"String".as_ptr().cast_mut(),
            default_addr: c"Applications".as_ptr() as XtPointer,
        },
        _XtResource {
            resource_name: c"applicationsMenuFile".as_ptr().cast_mut(),
            resource_class: c"ApplicationsMenuFile".as_ptr().cast_mut(),
            resource_type: c"String".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, applications_file) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: ::core::ptr::null_mut::<libc::c_void>(),
        },
        _XtResource {
            resource_name: c"title".as_ptr().cast_mut(),
            resource_class: c"Title".as_ptr().cast_mut(),
            resource_type: c"String".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, title) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: ::core::ptr::null_mut::<libc::c_void>(),
        },
        _XtResource {
            resource_name: c"dateTimeDisplay".as_ptr().cast_mut(),
            resource_class: c"DateTimeDisplay".as_ptr().cast_mut(),
            resource_type: c"Boolean".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, show_date_time) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: 1 as libc::c_int as XtPointer,
        },
        _XtResource {
            resource_name: c"dateTimeFormat".as_ptr().cast_mut(),
            resource_class: c"DateTimeFormat".as_ptr().cast_mut(),
            resource_type: c"String".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, date_time_fmt) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: c"%m/%d %l:%M %p".as_ptr() as XtPointer,
        },
        _XtResource {
            resource_name: c"rcFile".as_ptr().cast_mut(),
            resource_class: c"RcFile".as_ptr().cast_mut(),
            resource_type: c"String".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, rc_file) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: ::core::ptr::null_mut::<libc::c_void>(),
        },
        _XtResource {
            resource_name: c"hotkey".as_ptr().cast_mut(),
            resource_class: c"Hotkey".as_ptr().cast_mut(),
            resource_type: c"String".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<String_0>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, hotkey) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: ::core::ptr::null_mut::<libc::c_void>(),
        },
        _XtResource {
            resource_name: c"horizontal".as_ptr().cast_mut(),
            resource_class: c"Horizontal".as_ptr().cast_mut(),
            resource_type: c"Boolean".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, horizontal) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: ::core::ptr::null_mut::<libc::c_void>(),
        },
        _XtResource {
            resource_name: c"separators".as_ptr().cast_mut(),
            resource_class: c"Separators".as_ptr().cast_mut(),
            resource_type: c"Boolean".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, separators) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: 1 as libc::c_int as XtPointer,
        },
        _XtResource {
            resource_name: c"workspaceSwitcher".as_ptr().cast_mut(),
            resource_class: c"WorkspaceSwitcher".as_ptr().cast_mut(),
            resource_type: c"Boolean".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, switcher) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: 1 as libc::c_int as XtPointer,
        },
        _XtResource {
            resource_name: c"occupyAllWorkspaces".as_ptr().cast_mut(),
            resource_class: c"OccupyAllWorkspaces".as_ptr().cast_mut(),
            resource_type: c"Boolean".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<Boolean>() as Cardinal,
            resource_offset: std::mem::offset_of!(tb_resources, occupy_all) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: 1 as libc::c_int as XtPointer,
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

#[cfg(feature = "test-probes")]
#[path = "probe.rs"]
mod probe;
