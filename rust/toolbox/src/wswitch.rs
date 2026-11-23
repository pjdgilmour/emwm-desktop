// Copyright (C) 2018–2026 alx@fastestcode.org. SPDX-License-Identifier: MIT
// Ported from emwm-utils wswitch.c through C2Rust 0.22.1. Xt runs these callbacks
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
#[repr(C)]
pub struct _XGC {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _XDisplay {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _XRegion {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _TranslationData {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _XtStateRec {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _XtEventRec {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _XtAppStruct {
    _opaque: [u8; 0],
}
use crate::tbmain::{__XmStringRec, XmStringCreateLocalized, XmStringFree};
#[repr(C)]
pub struct __XmRenderTableRec {
    _opaque: [u8; 0],
}
extern "C" {
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    fn sprintf(__s: *mut libc::c_char, __format: *const libc::c_char, ...) -> libc::c_int;
    fn XFillRectangle(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn XQueryColor(_: *mut Display, _: Colormap, _: *mut XColor) -> libc::c_int;
    fn XSetForeground(_: *mut Display, _: GC, _: libc::c_ulong) -> libc::c_int;
    fn XtMakeResizeRequest(
        _: Widget,
        _: Dimension,
        _: Dimension,
        _: *mut Dimension,
        _: *mut Dimension,
    ) -> XtGeometryResult;
    pub(super) fn XtDisplayOfObject(_: Widget) -> *mut Display;
    fn XtWindowOfObject(_: Widget) -> Window;
    fn XtCallCallbackList(_: Widget, _: XtCallbackList, _: XtPointer);
    fn XtWidgetToApplicationContext(_: Widget) -> XtAppContext;
    fn XtAppWarning(_: XtAppContext, _: *const libc::c_char);
    fn XtMalloc(_: Cardinal) -> *mut libc::c_char;
    fn XtFree(_: *mut libc::c_char);
    fn XtGetGC(_: Widget, _: XtGCMask, _: *mut XGCValues) -> GC;
    fn XtAllocateGC(
        _: Widget,
        _: Cardinal,
        _: XtGCMask,
        _: *mut XGCValues,
        _: XtGCMask,
        _: XtGCMask,
    ) -> GC;
    fn XtReleaseGC(_: Widget, _: GC);
    fn XmStringDraw(
        d: *mut Display,
        w: Window,
        fontlist: XmFontList,
        string: XmString,
        gc: GC,
        x: Position,
        y: Position,
        width: Dimension,
        align: libc::c_uchar,
        lay_dir: libc::c_uchar,
        clip: *mut XRectangle,
    );
    static mut _XtInheritTranslations: libc::c_int;
    fn _XtInherit();
    fn XmeGetDefaultRenderTable(w: Widget, fontListType: libc::c_uchar) -> XmFontList;
    fn XmeReplyToQueryGeometry(
        widget: Widget,
        intended: *mut XtWidgetGeometry,
        desired: *mut XtWidgetGeometry,
    ) -> XtGeometryResult;
    fn XmRenderTableGetDefaultFontExtents(
        rendertable: XmRenderTable,
        height: *mut libc::c_int,
        ascent: *mut libc::c_int,
        descent: *mut libc::c_int,
    );
    static mut xmPrimitiveClassRec: XmPrimitiveClassRec;
    fn XmeDrawShadows(
        display: *mut Display,
        d: Drawable,
        top_gc: GC,
        bottom_gc: GC,
        x: Position,
        y: Position,
        width: Dimension,
        height: Dimension,
        shad_thick: Dimension,
        shad_type: libc::c_uint,
    );
}
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: libc::c_int,
    pub plane_mask: libc::c_ulong,
    pub foreground: libc::c_ulong,
    pub background: libc::c_ulong,
    pub line_width: libc::c_int,
    pub line_style: libc::c_int,
    pub cap_style: libc::c_int,
    pub join_style: libc::c_int,
    pub fill_style: libc::c_int,
    pub fill_rule: libc::c_int,
    pub arc_mode: libc::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: libc::c_int,
    pub ts_y_origin: libc::c_int,
    pub font: Font,
    pub subwindow_mode: libc::c_int,
    pub graphics_exposures: libc::c_int,
    pub clip_x_origin: libc::c_int,
    pub clip_y_origin: libc::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: libc::c_int,
    pub dashes: libc::c_char,
}
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
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: libc::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: libc::c_ulong,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRectangle {
    pub x: libc::c_short,
    pub y: libc::c_short,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
}
pub type Display = _XDisplay;
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
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub type Region = *mut _XRegion;
pub type XrmQuark = libc::c_int;
pub type XrmName = XrmQuark;
pub type XrmClass = XrmQuark;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XrmValue {
    pub size: libc::c_uint,
    pub addr: XPointer,
}
pub type String_0 = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WidgetRec {
    pub core: CorePart,
}
pub type CorePart = _CorePart;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CorePart {
    pub self_0: Widget,
    pub widget_class: WidgetClass,
    pub parent: Widget,
    pub xrm_name: XrmName,
    pub being_destroyed: Boolean,
    pub destroy_callbacks: XtCallbackList,
    pub constraints: XtPointer,
    pub x: Position,
    pub y: Position,
    pub width: Dimension,
    pub height: Dimension,
    pub border_width: Dimension,
    pub managed: Boolean,
    pub sensitive: Boolean,
    pub ancestor_sensitive: Boolean,
    pub event_table: XtEventTable,
    pub tm: XtTMRec,
    pub accelerators: XtTranslations,
    pub border_pixel: Pixel,
    pub border_pixmap: Pixmap,
    pub popup_list: WidgetList,
    pub num_popups: Cardinal,
    pub name: String_0,
    pub screen: *mut Screen,
    pub colormap: Colormap,
    pub window: Window,
    pub depth: Cardinal,
    pub background_pixel: Pixel,
    pub background_pixmap: Pixmap,
    pub visible: Boolean,
    pub mapped_when_managed: Boolean,
}
pub type Boolean = libc::c_char;
pub type Pixel = libc::c_ulong;
pub type Cardinal = libc::c_uint;
pub type WidgetList = *mut Widget;
pub type Widget = *mut _WidgetRec;
pub type XtTranslations = *mut _TranslationData;
pub type XtTMRec = _XtTMRec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtTMRec {
    pub translations: XtTranslations,
    pub proc_table: XtBoundActions,
    pub current_state: *mut _XtStateRec,
    pub lastEventTime: libc::c_ulong,
}
pub type XtBoundActions = *mut XtActionProc;
pub type XtActionProc =
    Option<unsafe extern "C" fn(Widget, *mut XEvent, *mut String_0, *mut Cardinal) -> ()>;
pub type XtEventTable = *mut _XtEventRec;
pub type Dimension = libc::c_ushort;
pub type Position = libc::c_short;
pub type XtPointer = *mut libc::c_void;
pub type XtCallbackList = *mut _XtCallbackRec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtCallbackRec {
    pub callback: XtCallbackProc,
    pub closure: XtPointer,
}
pub type XtCallbackProc = Option<unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()>;
pub type WidgetClass = *mut _WidgetClassRec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _WidgetClassRec {
    pub core_class: CoreClassPart,
}
pub type CoreClassPart = _CoreClassPart;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CoreClassPart {
    pub superclass: WidgetClass,
    pub class_name: String_0,
    pub widget_size: Cardinal,
    pub class_initialize: XtProc,
    pub class_part_initialize: XtWidgetClassProc,
    pub class_inited: XtEnum,
    pub initialize: XtInitProc,
    pub initialize_hook: XtArgsProc,
    pub realize: XtRealizeProc,
    pub actions: XtActionList,
    pub num_actions: Cardinal,
    pub resources: XtResourceList,
    pub num_resources: Cardinal,
    pub xrm_class: XrmClass,
    pub compress_motion: Boolean,
    pub compress_exposure: XtEnum,
    pub compress_enterleave: Boolean,
    pub visible_interest: Boolean,
    pub destroy: XtWidgetProc,
    pub resize: XtWidgetProc,
    pub expose: XtExposeProc,
    pub set_values: XtSetValuesFunc,
    pub set_values_hook: XtArgsFunc,
    pub set_values_almost: XtAlmostProc,
    pub get_values_hook: XtArgsProc,
    pub accept_focus: XtAcceptFocusProc,
    pub version: XtVersionType,
    pub callback_private: XtPointer,
    pub tm_table: String_0,
    pub query_geometry: XtGeometryHandler,
    pub display_accelerator: XtStringProc,
    pub extension: XtPointer,
}
pub type XtStringProc = Option<unsafe extern "C" fn(Widget, String_0) -> ()>;
pub type XtGeometryHandler = Option<
    unsafe extern "C" fn(Widget, *mut XtWidgetGeometry, *mut XtWidgetGeometry) -> XtGeometryResult,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XtWidgetGeometry {
    pub request_mode: XtGeometryMask,
    pub x: Position,
    pub y: Position,
    pub width: Dimension,
    pub height: Dimension,
    pub border_width: Dimension,
    pub sibling: Widget,
    pub stack_mode: libc::c_int,
}
pub type XtGeometryMask = libc::c_uint;
pub type XtGeometryResult = libc::c_uint;
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub type XtVersionType = libc::c_ulong;
pub type XtAcceptFocusProc = Option<unsafe extern "C" fn(Widget, *mut Time) -> Boolean>;
pub type XtArgsProc = Option<unsafe extern "C" fn(Widget, ArgList, *mut Cardinal) -> ()>;
pub type ArgList = *mut C2RustUnnamed_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub name: String_0,
    pub value: XtArgVal,
}
pub type XtArgVal = libc::c_long;
pub type XtAlmostProc = Option<
    unsafe extern "C" fn(Widget, Widget, *mut XtWidgetGeometry, *mut XtWidgetGeometry) -> (),
>;
pub type XtArgsFunc = Option<unsafe extern "C" fn(Widget, ArgList, *mut Cardinal) -> Boolean>;
pub type XtSetValuesFunc =
    Option<unsafe extern "C" fn(Widget, Widget, Widget, ArgList, *mut Cardinal) -> Boolean>;
pub type XtExposeProc = Option<unsafe extern "C" fn(Widget, *mut XEvent, Region) -> ()>;
pub type XtWidgetProc = Option<unsafe extern "C" fn(Widget) -> ()>;
pub type XtEnum = libc::c_uchar;
pub type XtResourceList = *mut _XtResource;
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
pub type XtActionList = *mut _XtActionsRec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XtActionsRec {
    pub string: String_0,
    pub proc_0: XtActionProc,
}
pub type XtRealizeProc =
    Option<unsafe extern "C" fn(Widget, *mut XtValueMask, *mut XSetWindowAttributes) -> ()>;
pub type XtValueMask = libc::c_ulong;
pub type XtInitProc = Option<unsafe extern "C" fn(Widget, Widget, ArgList, *mut Cardinal) -> ()>;
pub type XtWidgetClassProc = Option<unsafe extern "C" fn(WidgetClass) -> ()>;
pub type XtProc = Option<unsafe extern "C" fn() -> ()>;
pub type XtAppContext = *mut _XtAppStruct;
pub type XtGCMask = libc::c_ulong;
pub type XtActionsRec = _XtActionsRec;
pub type XtResource = _XtResource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmPrimitiveClassRec {
    pub core_class: CoreClassPart,
    pub primitive_class: XmPrimitiveClassPart,
}
pub type XmPrimitiveClassPart = _XmPrimitiveClassPart;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmPrimitiveClassPart {
    pub border_highlight: XtWidgetProc,
    pub border_unhighlight: XtWidgetProc,
    pub translations: String_0,
    pub arm_and_activate: XtActionProc,
    pub syn_resources: *mut XmSyntheticResource,
    pub num_syn_resources: libc::c_int,
    pub extension: XtPointer,
}
pub type XmSyntheticResource = _XmSyntheticResource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmSyntheticResource {
    pub resource_name: String_0,
    pub resource_size: Cardinal,
    pub resource_offset: Cardinal,
    pub export_proc: XmExportProc,
    pub import_proc: XmImportProc,
}
pub type XmImportProc =
    Option<unsafe extern "C" fn(Widget, libc::c_int, *mut XtArgVal) -> XmImportOperator>;
pub type XmImportOperator = libc::c_uint;
pub const XmSYNTHETIC_LOAD: XmImportOperator = 1;
pub const XmSYNTHETIC_NONE: XmImportOperator = 0;
pub type XmExportProc = Option<unsafe extern "C" fn(Widget, libc::c_int, *mut XtArgVal) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmPrimitiveRec {
    pub core: CorePart,
    pub primitive: XmPrimitivePart,
}
pub type XmPrimitivePart = _XmPrimitivePart;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmPrimitivePart {
    pub foreground: Pixel,
    pub shadow_thickness: Dimension,
    pub top_shadow_color: Pixel,
    pub top_shadow_pixmap: Pixmap,
    pub bottom_shadow_color: Pixel,
    pub bottom_shadow_pixmap: Pixmap,
    pub highlight_thickness: Dimension,
    pub highlight_color: Pixel,
    pub highlight_pixmap: Pixmap,
    pub help_callback: XtCallbackList,
    pub user_data: XtPointer,
    pub traversal_on: Boolean,
    pub highlight_on_enter: Boolean,
    pub have_traversal: Boolean,
    pub unit_type: libc::c_uchar,
    pub navigation_type: XmNavigationType,
    pub highlight_drawn: Boolean,
    pub highlighted: Boolean,
    pub highlight_GC: GC,
    pub top_shadow_GC: GC,
    pub bottom_shadow_GC: GC,
    pub convert_callback: XtCallbackList,
    pub popup_handler_callback: XtCallbackList,
    pub layout_direction: XmDirection,
    pub tool_tip_string: XmString,
}
pub type XmString = *mut __XmStringRec;
pub type XmDirection = libc::c_uchar;
pub type XmNavigationType = libc::c_uchar;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const XmSTRING_DIRECTION_DEFAULT: C2RustUnnamed_1 = 255;
pub const XmSTRING_DIRECTION_UNSET: C2RustUnnamed_1 = 3;
pub const XmSTRING_DIRECTION_R_TO_L: C2RustUnnamed_1 = 1;
pub const XmSTRING_DIRECTION_L_TO_R: C2RustUnnamed_1 = 0;
pub type XmFontList = *mut *mut __XmRenderTableRec;
pub type XmRenderTable = *mut *mut __XmRenderTableRec;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const XmALIGNMENT_UNSPECIFIED: C2RustUnnamed_2 = 3;
pub const XmALIGNMENT_END: C2RustUnnamed_2 = 2;
pub const XmALIGNMENT_CENTER: C2RustUnnamed_2 = 1;
pub const XmALIGNMENT_BEGINNING: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const XmSHADOW_OUT: C2RustUnnamed_3 = 8;
pub const XmSHADOW_IN: C2RustUnnamed_3 = 7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmBaselineMargins {
    pub get_or_set: libc::c_uchar,
    pub margin_top: Dimension,
    pub margin_bottom: Dimension,
    pub shadow: Dimension,
    pub highlight: Dimension,
    pub text_height: Dimension,
    pub margin_height: Dimension,
}
pub type XmBaselineMargins = _XmBaselineMargins;
pub type XmWidgetBaselineProc =
    Option<unsafe extern "C" fn(Widget, *mut *mut Dimension, *mut libc::c_int) -> Boolean>;
pub type XmWidgetDisplayRectProc = Option<unsafe extern "C" fn(Widget, *mut XRectangle) -> Boolean>;
pub type XmWidgetMarginsProc = Option<unsafe extern "C" fn(Widget, *mut XmBaselineMargins) -> ()>;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const XmTEXT_RENDER_TABLE: C2RustUnnamed_4 = 3;
pub const XmBUTTON_RENDER_TABLE: C2RustUnnamed_4 = 2;
pub const XmLABEL_RENDER_TABLE: C2RustUnnamed_4 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XmPrimitiveClassExtRec {
    pub next_extension: XtPointer,
    pub record_type: XrmQuark,
    pub version: libc::c_long,
    pub record_size: Cardinal,
    pub widget_baseline: XmWidgetBaselineProc,
    pub widget_display_rect: XmWidgetDisplayRectProc,
    pub widget_margins: XmWidgetMarginsProc,
}
pub type XmPrimitiveClassExtRec = _XmPrimitiveClassExtRec;
pub type XmPrimitiveClassRec = _XmPrimitiveClassRec;
pub type XmPrimitiveRec = _XmPrimitiveRec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct button_rec {
    pub x: Position,
    pub y: Position,
    pub width: Dimension,
    pub height: Dimension,
    pub label: XmString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct switcher_part {
    pub text_rt: XmRenderTable,
    pub fg_gc: GC,
    pub bg_gc: GC,
    pub fg_pixel: Pixel,
    pub bg_pixel: Pixel,
    pub sfg_pixel: Pixel,
    pub buttons: *mut button_rec,
    pub nbuttons: libc::c_short,
    pub iactive: libc::c_short,
    pub ncols: libc::c_short,
    pub hmargin: Dimension,
    pub vmargin: Dimension,
    pub font_height: Dimension,
    pub ws_change_cb: XtCallbackList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct switcher_rec {
    pub core: CorePart,
    pub primitive: XmPrimitivePart,
    pub switcher: switcher_part,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct switcher_class_part {
    pub extension: XtPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct switcher_class_rec {
    pub core: CoreClassPart,
    pub primitive: XmPrimitiveClassPart,
    pub switcher: switcher_class_part,
}
static mut resources: [XtResource; 7] = [_XtResource {
    resource_name: ::core::ptr::null_mut::<libc::c_char>(),
    resource_class: ::core::ptr::null_mut::<libc::c_char>(),
    resource_type: ::core::ptr::null_mut::<libc::c_char>(),
    resource_size: 0,
    resource_offset: 0,
    default_type: ::core::ptr::null_mut::<libc::c_char>(),
    default_addr: ::core::ptr::null_mut::<libc::c_void>(),
}; 7];
static mut translations: [libc::c_char; 345] = unsafe {
    ::core::mem::transmute::<
        [u8; 345],
        [libc::c_char; 345],
    >(
        *b"<Btn1Down>: PrimaryButton()\n<Btn2Down>: PrimaryButton()\n<Btn3Down>: PrimaryButton()\n<Key>1: SelectWorkspace(0)\n<Key>2: SelectWorkspace(1)\n<Key>3: SelectWorkspace(2)\n<Key>4: SelectWorkspace(3)\n<Key>5: SelectWorkspace(4)\n<Key>6: SelectWorkspace(5)\n<Key>7: SelectWorkspace(6)\n<Key>8: SelectWorkspace(7)\n<FocusIn>: FocusIn()\n<FocusOut>: FocusOut()\n\0",
    )
};
static mut actions: [XtActionsRec; 2] = {
    [
        _XtActionsRec {
            string: c"PrimaryButton".as_ptr().cast_mut(),
            proc_0: Some(
                mouse_input
                    as unsafe extern "C" fn(
                        Widget,
                        *mut XEvent,
                        *mut String_0,
                        *mut Cardinal,
                    ) -> (),
            ),
        },
        _XtActionsRec {
            string: c"SelectWorkspace".as_ptr().cast_mut(),
            proc_0: Some(
                select_workspace
                    as unsafe extern "C" fn(
                        Widget,
                        *mut XEvent,
                        *mut String_0,
                        *mut Cardinal,
                    ) -> (),
            ),
        },
    ]
};
static mut primClassExtRec: XmPrimitiveClassExtRec = {
    _XmPrimitiveClassExtRec {
        next_extension: ::core::ptr::null::<libc::c_void>() as *mut libc::c_void,
        record_type: 0 as libc::c_int,
        version: 1 as libc::c_long,
        record_size: ::core::mem::size_of::<XmPrimitiveClassExtRec>() as Cardinal,
        widget_baseline: None,
        widget_display_rect: Some(
            widget_display_rect as unsafe extern "C" fn(Widget, *mut XRectangle) -> Boolean,
        ),
        widget_margins: None,
    }
};
static mut class_rec_def: switcher_class_rec = switcher_class_rec {
    core: _CoreClassPart {
        superclass: ::core::ptr::null::<_WidgetClassRec>() as *mut _WidgetClassRec,
        class_name: ::core::ptr::null::<libc::c_char>() as *mut libc::c_char,
        widget_size: 0,
        class_initialize: None,
        class_part_initialize: None,
        class_inited: 0,
        initialize: None,
        initialize_hook: None,
        realize: None,
        actions: ::core::ptr::null::<_XtActionsRec>() as *mut _XtActionsRec,
        num_actions: 0,
        resources: ::core::ptr::null::<_XtResource>() as *mut _XtResource,
        num_resources: 0,
        xrm_class: 0,
        compress_motion: 0,
        compress_exposure: 0,
        compress_enterleave: 0,
        visible_interest: 0,
        destroy: None,
        resize: None,
        expose: None,
        set_values: None,
        set_values_hook: None,
        set_values_almost: None,
        get_values_hook: None,
        accept_focus: None,
        version: 0,
        callback_private: ::core::ptr::null::<libc::c_void>() as *mut libc::c_void,
        tm_table: ::core::ptr::null::<libc::c_char>() as *mut libc::c_char,
        query_geometry: None,
        display_accelerator: None,
        extension: ::core::ptr::null::<libc::c_void>() as *mut libc::c_void,
    },
    primitive: _XmPrimitiveClassPart {
        border_highlight: None,
        border_unhighlight: None,
        translations: ::core::ptr::null::<libc::c_char>() as *mut libc::c_char,
        arm_and_activate: None,
        syn_resources: ::core::ptr::null::<XmSyntheticResource>() as *mut XmSyntheticResource,
        num_syn_resources: 0,
        extension: ::core::ptr::null::<libc::c_void>() as *mut libc::c_void,
    },
    switcher: switcher_class_part {
        extension: ::core::ptr::null_mut::<libc::c_void>(),
    },
};
#[no_mangle]
pub static mut switcherWidgetClass: WidgetClass =
    &raw const class_rec_def as *mut switcher_class_rec as WidgetClass;
unsafe extern "C" fn draw(mut w: Widget, mut erase: Boolean) {
    let mut dpy: *mut Display = XtDisplayOfObject(w);
    let mut wnd: Window = XtWindowOfObject(w);
    let mut prim: *mut XmPrimitivePart = &raw mut (*(w as *mut XmPrimitiveRec)).primitive;
    let mut sw: *mut switcher_part = &raw mut (*(w as *mut switcher_rec)).switcher;
    let mut shadow: Dimension = (*prim).shadow_thickness;
    let mut width: libc::c_int = (*(w as *mut switcher_rec)).core.width as libc::c_int;
    let mut height: libc::c_int = (*(w as *mut switcher_rec)).core.height as libc::c_int;
    let mut i: libc::c_short = 0;
    if width < shadow as libc::c_int * 2 as libc::c_int
        || height < shadow as libc::c_int * 2 as libc::c_int
    {
        return;
    }
    if erase != 0 {
        XFillRectangle(
            dpy,
            wnd as Drawable,
            (*sw).bg_gc,
            0 as libc::c_int,
            0 as libc::c_int,
            ((*(w as *mut switcher_rec)).core.width as libc::c_int - 1 as libc::c_int)
                as libc::c_uint,
            ((*(w as *mut switcher_rec)).core.height as libc::c_int - 1 as libc::c_int)
                as libc::c_uint,
        );
    }
    XmeDrawShadows(
        dpy,
        wnd as Drawable,
        (*prim).top_shadow_GC,
        (*prim).bottom_shadow_GC,
        0 as Position,
        0 as Position,
        width as Dimension,
        height as Dimension,
        shadow,
        XmSHADOW_IN as libc::c_int as libc::c_uint,
    );
    if (*sw).buttons.is_null() {
        XmeDrawShadows(
            dpy,
            wnd as Drawable,
            (*prim).top_shadow_GC,
            (*prim).bottom_shadow_GC,
            1 as Position,
            1 as Position,
            (width - 2 as libc::c_int) as Dimension,
            (height - 2 as libc::c_int) as Dimension,
            shadow,
            XmSHADOW_OUT as libc::c_int as libc::c_uint,
        );
        return;
    }
    i = 0 as libc::c_short;
    while (i as libc::c_int) < (*sw).nbuttons as libc::c_int {
        if i as libc::c_int == (*sw).iactive as libc::c_int {
            XFillRectangle(
                dpy,
                wnd as Drawable,
                (*prim).highlight_GC,
                (*(*sw).buttons.offset(i as isize)).x as libc::c_int,
                (*(*sw).buttons.offset(i as isize)).y as libc::c_int,
                (*(*sw).buttons.offset(i as isize)).width as libc::c_uint,
                (*(*sw).buttons.offset(i as isize)).height as libc::c_uint,
            );
            XmeDrawShadows(
                dpy,
                wnd as Drawable,
                (*prim).bottom_shadow_GC,
                (*prim).bottom_shadow_GC,
                (*(*sw).buttons.offset(i as isize)).x,
                (*(*sw).buttons.offset(i as isize)).y,
                (*(*sw).buttons.offset(i as isize)).width,
                (*(*sw).buttons.offset(i as isize)).height,
                shadow,
                XmSHADOW_IN as libc::c_int as libc::c_uint,
            );
            if (*sw).sfg_pixel != (*sw).fg_pixel {
                XSetForeground(dpy, (*sw).fg_gc, (*sw).sfg_pixel as libc::c_ulong);
            }
        } else {
            XmeDrawShadows(
                dpy,
                wnd as Drawable,
                (*prim).top_shadow_GC,
                (*prim).bottom_shadow_GC,
                (*(*sw).buttons.offset(i as isize)).x,
                (*(*sw).buttons.offset(i as isize)).y,
                (*(*sw).buttons.offset(i as isize)).width,
                (*(*sw).buttons.offset(i as isize)).height,
                shadow,
                XmSHADOW_OUT as libc::c_int as libc::c_uint,
            );
            if (*sw).sfg_pixel != (*sw).fg_pixel {
                XSetForeground(dpy, (*sw).fg_gc, (*sw).fg_pixel as libc::c_ulong);
            }
        }
        XmStringDraw(
            dpy,
            wnd,
            (*sw).text_rt as XmFontList,
            (*(*sw).buttons.offset(i as isize)).label,
            (*sw).fg_gc,
            (*(*sw).buttons.offset(i as isize)).x,
            ((*(*sw).buttons.offset(i as isize)).y as libc::c_int
                + (*sw).vmargin as libc::c_int
                + shadow as libc::c_int) as Position,
            (*(*sw).buttons.offset(i as isize)).width,
            XmALIGNMENT_CENTER as libc::c_int as libc::c_uchar,
            XmSTRING_DIRECTION_DEFAULT as libc::c_int as libc::c_uchar,
            ::core::ptr::null_mut::<XRectangle>(),
        );
        i += 1;
    }
}
unsafe extern "C" fn get_pref_dimensions(
    mut w: Widget,
    mut width: *mut Dimension,
    mut height: *mut Dimension,
) {
    let mut sw: *mut switcher_part = &raw mut (*(w as *mut switcher_rec)).switcher;
    let mut prim: *mut XmPrimitivePart = &raw mut (*(w as *mut XmPrimitiveRec)).primitive;
    let mut shadow: Dimension = (*prim).shadow_thickness;
    let mut bw: Dimension = (shadow as libc::c_int * 2 as libc::c_int
        + (*sw).font_height as libc::c_int
        + (*sw).hmargin as libc::c_int * 2 as libc::c_int) as Dimension;
    let mut bh: Dimension = (shadow as libc::c_int * 2 as libc::c_int
        + (*sw).font_height as libc::c_int
        + (*sw).vmargin as libc::c_int * 2 as libc::c_int) as Dimension;
    let mut rows: Dimension = (((*sw).nbuttons as libc::c_int
        + ((*sw).ncols as libc::c_int - 1 as libc::c_int))
        / (*sw).ncols as libc::c_int) as Dimension;
    if rows == 0 {
        rows = 1 as Dimension;
    }
    if !width.is_null() {
        *width = (shadow as libc::c_int * 2 as libc::c_int
            + bw as libc::c_int * (*sw).ncols as libc::c_int) as Dimension;
    }
    if !height.is_null() {
        *height = (shadow as libc::c_int * 2 as libc::c_int
            + bh as libc::c_int * rows as libc::c_int) as Dimension;
    }
}
unsafe extern "C" fn layout(mut w: Widget) {
    let mut prim: *mut XmPrimitivePart = &raw mut (*(w as *mut XmPrimitiveRec)).primitive;
    let mut sw: *mut switcher_part = &raw mut (*(w as *mut switcher_rec)).switcher;
    let mut c: libc::c_short = 0;
    let mut r: libc::c_short = 0;
    let mut i: libc::c_short = 0;
    let mut rows: libc::c_short = 0;
    let mut st: Dimension = (*prim).shadow_thickness;
    let mut cw: libc::c_int = (*(w as *mut switcher_rec)).core.width as libc::c_int
        - st as libc::c_int * 2 as libc::c_int;
    let mut ch: libc::c_int = (*(w as *mut switcher_rec)).core.height as libc::c_int
        - st as libc::c_int * 2 as libc::c_int;
    let mut bw_min: Dimension = (st as libc::c_int * 2 as libc::c_int
        + (*sw).font_height as libc::c_int
        + (*sw).hmargin as libc::c_int * 2 as libc::c_int)
        as Dimension;
    let mut bh_min: Dimension = (st as libc::c_int * 2 as libc::c_int
        + (*sw).font_height as libc::c_int
        + (*sw).vmargin as libc::c_int * 2 as libc::c_int)
        as Dimension;
    let mut bpc: Dimension = (if (*sw).nbuttons as libc::c_int > (*sw).ncols as libc::c_int {
        (*sw).ncols as libc::c_int
    } else {
        (*sw).nbuttons as libc::c_int
    }) as Dimension;
    if (*sw).nbuttons == 0 || cw < bw_min as libc::c_int || ch < bh_min as libc::c_int {
        return;
    }
    rows = (((*sw).nbuttons as libc::c_int + (bpc as libc::c_int - 1 as libc::c_int))
        / bpc as libc::c_int) as libc::c_short;
    i = 0 as libc::c_short;
    r = 0 as libc::c_short;
    while (r as libc::c_int) < rows as libc::c_int {
        let mut cx: Position = 0 as Position;
        let mut cy: Position = (r as libc::c_int * bh_min as libc::c_int) as Position;
        let mut bw: Dimension = bw_min;
        if (bw_min as libc::c_int * bpc as libc::c_int) < cw {
            bw = (bw_min as libc::c_int
                + (cw - bw_min as libc::c_int * bpc as libc::c_int) / bpc as libc::c_int)
                as Dimension;
        }
        c = 0 as libc::c_short;
        while (c as libc::c_int) < bpc as libc::c_int {
            cx = (bw as libc::c_int * c as libc::c_int) as Position;
            (*(*sw).buttons.offset(i as isize)).x =
                (cx as libc::c_int + st as libc::c_int) as Position;
            (*(*sw).buttons.offset(i as isize)).y =
                (cy as libc::c_int + st as libc::c_int) as Position;
            (*(*sw).buttons.offset(i as isize)).width = bw;
            (*(*sw).buttons.offset(i as isize)).height = bh_min;
            c += 1;
            i += 1;
        }
        if i as libc::c_int != 0 && cw % bw as libc::c_int != 0 {
            let ref mut fresh1 = (*(*sw)
                .buttons
                .offset((i as libc::c_int - 1 as libc::c_int) as isize))
            .width;
            *fresh1 = (*fresh1 as libc::c_int + cw % bw as libc::c_int) as Dimension;
        }
        bpc = (if ((*sw).nbuttons as libc::c_int - i as libc::c_int) < (*sw).ncols as libc::c_int {
            (*sw).nbuttons as libc::c_int - i as libc::c_int
        } else {
            (*sw).ncols as libc::c_int
        }) as Dimension;
        r += 1;
    }
}
unsafe extern "C" fn hit_test(mut w: Widget, mut x: Position, mut y: Position) -> libc::c_short {
    let mut sw: *mut switcher_part = &raw mut (*(w as *mut switcher_rec)).switcher;
    let mut i: libc::c_short = 0;
    i = 0 as libc::c_short;
    while (i as libc::c_int) < (*sw).nbuttons as libc::c_int {
        if x as libc::c_int > (*(*sw).buttons.offset(i as isize)).x as libc::c_int
            && y as libc::c_int > (*(*sw).buttons.offset(i as isize)).y as libc::c_int
            && (x as libc::c_int)
                < (*(*sw).buttons.offset(i as isize)).x as libc::c_int
                    + (*(*sw).buttons.offset(i as isize)).width as libc::c_int
            && (y as libc::c_int)
                < (*(*sw).buttons.offset(i as isize)).y as libc::c_int
                    + (*(*sw).buttons.offset(i as isize)).height as libc::c_int
        {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int) as libc::c_short;
}
unsafe extern "C" fn expose(mut w: Widget, mut evt: *mut XEvent, mut reg: Region) {
    draw(w, 0 as Boolean);
}
unsafe extern "C" fn resize(mut w: Widget) {
    layout(w);
}
unsafe extern "C" fn query_geometry(
    mut w: Widget,
    mut ig: *mut XtWidgetGeometry,
    mut pg: *mut XtWidgetGeometry,
) -> XtGeometryResult {
    let mut pref_width: Dimension = 0;
    let mut pref_height: Dimension = 0;
    get_pref_dimensions(w, &raw mut pref_width, &raw mut pref_height);
    (*pg).request_mode = ((1 as libc::c_int) << 2 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int) as XtGeometryMask;
    (*pg).width = pref_width;
    (*pg).height = pref_height;
    return XmeReplyToQueryGeometry(w, ig, pg);
}
unsafe extern "C" fn widget_display_rect(mut w: Widget, mut r: *mut XRectangle) -> Boolean {
    let mut prim: *mut XmPrimitivePart = &raw mut (*(w as *mut XmPrimitiveRec)).primitive;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    (*r).x = (*prim).shadow_thickness as libc::c_short;
    (*r).y = (*prim).shadow_thickness as libc::c_short;
    width = (*(w as *mut switcher_rec)).core.width as libc::c_int
        - (*prim).shadow_thickness as libc::c_int * 2 as libc::c_int;
    height = (*(w as *mut switcher_rec)).core.height as libc::c_int
        - (*prim).shadow_thickness as libc::c_int * 2 as libc::c_int;
    if width < 0 as libc::c_int || height < 0 as libc::c_int {
        width = 0 as libc::c_int;
        height = 0 as libc::c_int;
    }
    (*r).width = width as Dimension as libc::c_ushort;
    (*r).height = height as Dimension as libc::c_ushort;
    return 1 as Boolean;
}
unsafe extern "C" fn realize(
    mut w: Widget,
    mut mask: *mut XtValueMask,
    mut att: *mut XSetWindowAttributes,
) {
    Some(
        (*(*switcherWidgetClass).core_class.superclass)
            .core_class
            .realize
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(w, mask, att);
}
unsafe extern "C" fn initialize(
    mut wreq: Widget,
    mut wnew: Widget,
    mut init_args: ArgList,
    mut ninit_args: *mut Cardinal,
) {
    let mut p: *mut switcher_part = &raw mut (*(wnew as *mut switcher_rec)).switcher;
    let mut height: libc::c_int = 0;
    let mut ascent: libc::c_int = 0;
    let mut descent: libc::c_int = 0;
    let mut pref_width: Dimension = 0;
    let mut pref_height: Dimension = 0;
    XmRenderTableGetDefaultFontExtents(
        (*p).text_rt,
        &raw mut height,
        &raw mut ascent,
        &raw mut descent,
    );
    (*p).font_height = height as Dimension;
    if (*p).nbuttons as libc::c_int > 1 as libc::c_int {
        let mut i: libc::c_short = 0;
        (*p).buttons = XtMalloc(
            (::core::mem::size_of::<button_rec>() as usize).wrapping_mul((*p).nbuttons as usize)
                as Cardinal,
        ) as *mut button_rec;
        i = 0 as libc::c_short;
        while (i as libc::c_int) < (*p).nbuttons as libc::c_int {
            let mut sz: [libc::c_char; 7] = [0; 7];
            sprintf(
                &raw mut sz as *mut libc::c_char,
                c"%d".as_ptr(),
                i as libc::c_int + 1 as libc::c_int,
            );
            let ref mut fresh2 = (*(*p).buttons.offset(i as isize)).label;
            *fresh2 = XmStringCreateLocalized(&raw mut sz as String_0);
            i += 1;
        }
    } else {
        (*p).nbuttons = 0 as libc::c_short;
        (*p).buttons = ::core::ptr::null_mut::<button_rec>();
    }
    init_gcs(wnew);
    get_pref_dimensions(wnew, &raw mut pref_width, &raw mut pref_height);
    if (*(wreq as *mut switcher_rec)).core.width as libc::c_int == 0 as libc::c_int {
        (*(wnew as *mut switcher_rec)).core.width = pref_width;
    }
    if (*(wreq as *mut switcher_rec)).core.height as libc::c_int == 0 as libc::c_int {
        (*(wnew as *mut switcher_rec)).core.height = pref_height;
    }
    layout(wnew);
}
unsafe extern "C" fn init_gcs(mut w: Widget) {
    let mut r: *mut switcher_rec = w as *mut switcher_rec;
    let mut gcv: XGCValues = XGCValues {
        function: 0,
        plane_mask: 0,
        foreground: 0,
        background: 0,
        line_width: 0,
        line_style: 0,
        cap_style: 0,
        join_style: 0,
        fill_style: 0,
        fill_rule: 0,
        arc_mode: 0,
        tile: 0,
        stipple: 0,
        ts_x_origin: 0,
        ts_y_origin: 0,
        font: 0,
        subwindow_mode: 0,
        graphics_exposures: 0,
        clip_x_origin: 0,
        clip_y_origin: 0,
        clip_mask: 0,
        dash_offset: 0,
        dashes: 0,
    };
    let mut gc_mask: XtGCMask = 0;
    let mut xc: XColor = XColor {
        pixel: 0 as libc::c_ulong,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    (*r).switcher.fg_pixel = (*r).primitive.foreground;
    (*r).switcher.bg_pixel = (*r).core.background_pixel;
    gcv.function = 0x3 as libc::c_int;
    gcv.foreground = (*r).switcher.fg_pixel as libc::c_ulong;
    gcv.line_style = 1 as libc::c_int;
    gcv.dashes = 1 as libc::c_char;
    gc_mask = ((1 as libc::c_long) << 2 as libc::c_int
        | (1 as libc::c_long) << 0 as libc::c_int
        | (1 as libc::c_long) << 5 as libc::c_int
        | (1 as libc::c_long) << 21 as libc::c_int) as XtGCMask;
    (*r).switcher.fg_gc = XtAllocateGC(
        w,
        0 as Cardinal,
        gc_mask,
        &raw mut gcv,
        gc_mask
            | ((1 as libc::c_long) << 19 as libc::c_int) as XtGCMask
            | ((1 as libc::c_long) << 14 as libc::c_int) as XtGCMask,
        0 as XtGCMask,
    );
    gcv.foreground = (*r).core.background_pixel as libc::c_ulong;
    (*r).switcher.bg_gc = XtGetGC(
        w,
        ((1 as libc::c_long) << 2 as libc::c_int) as XtGCMask,
        &raw mut gcv,
    );
    xc.pixel = (*r).primitive.highlight_color as libc::c_ulong;
    XQueryColor(XtDisplayOfObject(w), (*r).core.colormap, &raw mut xc);
    xc.red = (xc.red as libc::c_int / 256 as libc::c_int) as libc::c_ushort;
    xc.green = (xc.green as libc::c_int / 256 as libc::c_int) as libc::c_ushort;
    xc.blue = (xc.blue as libc::c_int / 256 as libc::c_int) as libc::c_ushort;
    (*r).switcher.sfg_pixel = (if (xc.red as libc::c_uint)
        .wrapping_mul(xc.red as libc::c_uint)
        .wrapping_add((xc.green as libc::c_int * xc.green as libc::c_int) as libc::c_uint)
        .wrapping_add((xc.blue as libc::c_int * xc.blue as libc::c_int) as libc::c_uint)
        > 56000 as libc::c_uint
    {
        (*(*r).core.screen).black_pixel
    } else {
        (*(*r).core.screen).white_pixel
    }) as Pixel;
}
unsafe extern "C" fn set_values(
    mut wcur: Widget,
    mut wreq: Widget,
    mut wset: Widget,
    mut args: ArgList,
    mut nargs: *mut Cardinal,
) -> Boolean {
    let mut cur: *mut switcher_rec = wcur as *mut switcher_rec;
    let mut set: *mut switcher_rec = wset as *mut switcher_rec;
    if (*cur).primitive.foreground != (*set).primitive.foreground
        || (*cur).core.background_pixel != (*set).core.background_pixel
    {
        XtReleaseGC(wcur, (*set).switcher.fg_gc);
        XtReleaseGC(wcur, (*set).switcher.bg_gc);
        init_gcs(wset);
    }
    if (*cur).switcher.nbuttons as libc::c_int != (*set).switcher.nbuttons as libc::c_int {
        let mut width: Dimension = 0;
        let mut height: Dimension = 0;
        if !(*set).switcher.buttons.is_null() {
            XtFree((*set).switcher.buttons as *mut libc::c_char);
            (*set).switcher.buttons = ::core::ptr::null_mut::<button_rec>();
        }
        if (*set).switcher.nbuttons as libc::c_int > 1 as libc::c_int {
            let mut i: libc::c_short = 0;
            (*set).switcher.buttons = XtMalloc(
                (::core::mem::size_of::<button_rec>() as usize)
                    .wrapping_mul((*set).switcher.nbuttons as usize) as Cardinal,
            ) as *mut button_rec;
            i = 0 as libc::c_short;
            while (i as libc::c_int) < (*set).switcher.nbuttons as libc::c_int {
                let mut sz: [libc::c_char; 7] = [0; 7];
                sprintf(
                    &raw mut sz as *mut libc::c_char,
                    c"%d".as_ptr(),
                    i as libc::c_int + 1 as libc::c_int,
                );
                let ref mut fresh0 = (*(*set).switcher.buttons.offset(i as isize)).label;
                *fresh0 = XmStringCreateLocalized(&raw mut sz as String_0);
                i += 1;
            }
        } else {
            (*set).switcher.nbuttons = 0 as libc::c_short;
        }
        get_pref_dimensions(wset, &raw mut width, &raw mut height);
        if XtMakeResizeRequest(wset, width, height, &raw mut width, &raw mut height) as libc::c_uint
            == XtGeometryAlmost as libc::c_int as libc::c_uint
        {
            XtMakeResizeRequest(
                wset,
                width,
                height,
                ::core::ptr::null_mut::<Dimension>(),
                ::core::ptr::null_mut::<Dimension>(),
            );
        }
        layout(wset);
    }
    if (*cur).switcher.iactive as libc::c_int != (*set).switcher.iactive as libc::c_int {
        if (*set).switcher.iactive as libc::c_int >= (*set).switcher.nbuttons as libc::c_int {
            (*set).switcher.iactive = 0 as libc::c_short;
        }
    }
    return (if XtWindowOfObject(wset) != 0 as Window {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as Boolean;
}
unsafe extern "C" fn class_initialize() {}
unsafe extern "C" fn destroy(mut w: Widget) {
    let mut p: *mut switcher_part = &raw mut (*(w as *mut switcher_rec)).switcher;
    if !(*p).buttons.is_null() {
        let mut i: libc::c_short = 0;
        i = 0 as libc::c_short;
        while (i as libc::c_int) < (*p).nbuttons as libc::c_int {
            XmStringFree((*(*p).buttons.offset(i as isize)).label);
            i += 1;
        }
        XtFree((*p).buttons as *mut libc::c_char);
    }
    XtReleaseGC(w, (*p).fg_gc);
    XtReleaseGC(w, (*p).bg_gc);
}
unsafe extern "C" fn default_render_table(
    mut w: Widget,
    mut offset: libc::c_int,
    mut pv: *mut XrmValue,
) {
    static mut rt: XmRenderTable =
        ::core::ptr::null::<*mut __XmRenderTableRec>() as *mut *mut __XmRenderTableRec;
    rt = XmeGetDefaultRenderTable(w, XmLABEL_RENDER_TABLE as libc::c_int as libc::c_uchar)
        as XmRenderTable;
    (*pv).addr = &raw mut rt as XPointer;
    (*pv).size = ::core::mem::size_of::<XmRenderTable>() as libc::c_uint;
}
unsafe extern "C" fn mouse_input(
    mut w: Widget,
    mut evt: *mut XEvent,
    mut params: *mut String_0,
    mut nparams: *mut Cardinal,
) {
    let mut p: *mut switcher_part = &raw mut (*(w as *mut switcher_rec)).switcher;
    let mut i: libc::c_short = 0;
    if (*evt).type_0 != 4 as libc::c_int && (*evt).type_0 != 5 as libc::c_int {
        XtAppWarning(
            XtWidgetToApplicationContext(w),
            c"Wrong event type for the action PrimaryButton()".as_ptr(),
        );
        return;
    }
    i = hit_test(
        w,
        (*evt).xbutton.x as Position,
        (*evt).xbutton.y as Position,
    );
    if i as libc::c_int != -(1 as libc::c_int) && i as libc::c_int != (*p).iactive as libc::c_int {
        (*p).iactive = i;
        draw(w, 1 as Boolean);
        if !(*p).ws_change_cb.is_null() {
            XtCallCallbackList(w, (*p).ws_change_cb, &raw mut i as XtPointer);
        }
    }
}
unsafe extern "C" fn select_workspace(
    mut w: Widget,
    mut evt: *mut XEvent,
    mut params: *mut String_0,
    mut nparams: *mut Cardinal,
) {
    let mut p: *mut switcher_part = &raw mut (*(w as *mut switcher_rec)).switcher;
    let mut iws: libc::c_short = 0;
    if *nparams == 0 as Cardinal {
        XtAppWarning(
            XtWidgetToApplicationContext(w),
            c"Missing argument for the SelectWorkspace() action".as_ptr(),
        );
        return;
    }
    iws = strtol(
        *params.offset(0 as libc::c_int as isize) as *const libc::c_char,
        ::core::ptr::null_mut::<*mut libc::c_char>(),
        10 as libc::c_int,
    ) as libc::c_short;
    if (*p).nbuttons as libc::c_int != 0
        && iws as libc::c_int >= 0 as libc::c_int
        && (iws as libc::c_int) < (*p).nbuttons as libc::c_int
        && iws as libc::c_int != (*p).iactive as libc::c_int
    {
        (*p).iactive = iws;
        draw(w, 1 as Boolean);
        if !(*p).ws_change_cb.is_null() {
            XtCallCallbackList(w, (*p).ws_change_cb, &raw mut iws as XtPointer);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn SwitcherSetActiveWorkspace(mut w: Widget, mut iws: libc::c_ushort) {
    let mut p: *mut switcher_part = &raw mut (*(w as *mut switcher_rec)).switcher;
    if !(*p).buttons.is_null()
        && (iws as libc::c_int) < (*p).nbuttons as libc::c_int
        && iws as libc::c_int != (*p).iactive as libc::c_int
    {
        (*p).iactive = iws as libc::c_short;
        draw(w, 1 as Boolean);
    }
}
unsafe extern "C" fn run_static_initializers() {
    class_rec_def = switcher_class_rec {
        core: _CoreClassPart {
            superclass: &raw mut xmPrimitiveClassRec as WidgetClass,
            class_name: c"WorkspaceSwitcher".as_ptr().cast_mut(),
            widget_size: ::core::mem::size_of::<switcher_rec>() as Cardinal,
            class_initialize: Some(class_initialize as unsafe extern "C" fn() -> ()),
            class_part_initialize: None,
            class_inited: 0 as XtEnum,
            initialize: Some(
                initialize as unsafe extern "C" fn(Widget, Widget, ArgList, *mut Cardinal) -> (),
            ),
            initialize_hook: None,
            realize: Some(
                realize
                    as unsafe extern "C" fn(
                        Widget,
                        *mut XtValueMask,
                        *mut XSetWindowAttributes,
                    ) -> (),
            ),
            actions: &raw mut actions as XtActionList,
            num_actions: (::core::mem::size_of::<[XtActionsRec; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtActionsRec>() as usize)
                as Cardinal,
            resources: &raw mut resources as XtResourceList,
            num_resources: (::core::mem::size_of::<[XtResource; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<XtResource>() as usize)
                as Cardinal,
            xrm_class: 0 as libc::c_int,
            compress_motion: 1 as Boolean,
            compress_exposure: 3 as XtEnum,
            compress_enterleave: 1 as Boolean,
            visible_interest: 0 as Boolean,
            destroy: Some(destroy as unsafe extern "C" fn(Widget) -> ()),
            resize: Some(resize as unsafe extern "C" fn(Widget) -> ()),
            expose: Some(expose as unsafe extern "C" fn(Widget, *mut XEvent, Region) -> ()),
            set_values: Some(
                set_values
                    as unsafe extern "C" fn(
                        Widget,
                        Widget,
                        Widget,
                        ArgList,
                        *mut Cardinal,
                    ) -> Boolean,
            ),
            set_values_hook: None,
            set_values_almost: ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                XtAlmostProc,
            >(Some(_XtInherit as unsafe extern "C" fn() -> ())),
            get_values_hook: None,
            accept_focus: None,
            version: (11 as libc::c_int * 1000 as libc::c_int + 6 as libc::c_int) as XtVersionType,
            callback_private: ::core::ptr::null_mut::<libc::c_void>(),
            tm_table: &raw mut translations as String_0,
            query_geometry: Some(
                query_geometry
                    as unsafe extern "C" fn(
                        Widget,
                        *mut XtWidgetGeometry,
                        *mut XtWidgetGeometry,
                    ) -> XtGeometryResult,
            ),
            display_accelerator: None,
            extension: ::core::ptr::null_mut::<libc::c_void>(),
        },
        primitive: _XmPrimitiveClassPart {
            border_highlight: None,
            border_unhighlight: None,
            translations: &raw mut _XtInheritTranslations as String_0,
            arm_and_activate: None,
            syn_resources: ::core::ptr::null_mut::<XmSyntheticResource>(),
            num_syn_resources: 0 as libc::c_int,
            extension: &raw mut primClassExtRec as XtPointer,
        },
        switcher: switcher_class_part {
            extension: ::core::ptr::null_mut::<libc::c_void>(),
        },
    };
    resources = [
        _XtResource {
            resource_name: c"renderTable".as_ptr().cast_mut() as *mut libc::c_char,
            resource_class: c"RenderTable".as_ptr().cast_mut() as *mut libc::c_char,
            resource_type: c"RenderTable".as_ptr().cast_mut() as *mut libc::c_char,
            resource_size: ::core::mem::size_of::<XmRenderTable>() as Cardinal,
            resource_offset: std::mem::offset_of!(switcher_rec, switcher.text_rt) as Cardinal,
            default_type: c"CallProc".as_ptr().cast_mut(),
            default_addr: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(Widget, libc::c_int, *mut XrmValue) -> ()>,
                XtPointer,
            >(Some(
                default_render_table
                    as unsafe extern "C" fn(Widget, libc::c_int, *mut XrmValue) -> (),
            )),
        },
        _XtResource {
            resource_name: c"marginWidth".as_ptr().cast_mut() as *mut libc::c_char,
            resource_class: c"MarginWidth".as_ptr().cast_mut() as *mut libc::c_char,
            resource_type: c"Dimension".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<Dimension>() as Cardinal,
            resource_offset: std::mem::offset_of!(switcher_rec, switcher.hmargin) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: 2 as libc::c_int as XtPointer,
        },
        _XtResource {
            resource_name: c"marginHeight".as_ptr().cast_mut() as *mut libc::c_char,
            resource_class: c"MarginHeight".as_ptr().cast_mut() as *mut libc::c_char,
            resource_type: c"Dimension".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<Dimension>() as Cardinal,
            resource_offset: std::mem::offset_of!(switcher_rec, switcher.vmargin) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: 2 as libc::c_int as XtPointer,
        },
        _XtResource {
            resource_name: c"numberOfWorkspaces".as_ptr().cast_mut(),
            resource_class: c"NumberOfWorkspaces".as_ptr().cast_mut(),
            resource_type: c"Short".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<libc::c_short>() as Cardinal,
            resource_offset: std::mem::offset_of!(switcher_rec, switcher.nbuttons) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: ::core::ptr::null_mut::<libc::c_void>(),
        },
        _XtResource {
            resource_name: c"activeWorkspace".as_ptr().cast_mut(),
            resource_class: c"ActiveWorkspace".as_ptr().cast_mut(),
            resource_type: c"Short".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<libc::c_short>() as Cardinal,
            resource_offset: std::mem::offset_of!(switcher_rec, switcher.iactive) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: ::core::ptr::null_mut::<libc::c_void>(),
        },
        _XtResource {
            resource_name: c"columns".as_ptr().cast_mut() as *mut libc::c_char,
            resource_class: c"Columns".as_ptr().cast_mut() as *mut libc::c_char,
            resource_type: c"Short".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<libc::c_short>() as Cardinal,
            resource_offset: std::mem::offset_of!(switcher_rec, switcher.ncols) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: 4 as libc::c_int as XtPointer,
        },
        _XtResource {
            resource_name: c"valueChangedCallback".as_ptr().cast_mut() as *mut libc::c_char,
            resource_class: c"ValueChangedCallback".as_ptr().cast_mut() as *mut libc::c_char,
            resource_type: c"Callback".as_ptr().cast_mut(),
            resource_size: ::core::mem::size_of::<XtCallbackList>() as Cardinal,
            resource_offset: std::mem::offset_of!(switcher_rec, switcher.ws_change_cb) as Cardinal,
            default_type: c"Immediate".as_ptr().cast_mut(),
            default_addr: ::core::ptr::null_mut::<libc::c_void>(),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
