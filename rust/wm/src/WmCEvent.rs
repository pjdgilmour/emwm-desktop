/*
 * Motif
 *
 * Copyright (c) 1987-2012, The Open Group. All rights reserved.
 * Copyright (c) 2018-2026, alx@fastestcode.org
 *
 * These libraries and programs are free software; you can
 * redistribute them and/or modify them under the terms of the GNU
 * Lesser General Public License as published by the Free Software
 * Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * These libraries and programs are distributed in the hope that
 * they will be useful, but WITHOUT ANY WARRANTY; without even the
 * implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 * PURPOSE. See the GNU Lesser General Public License for more
 * details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with these librararies and programs; if not, write
 * to the Free Software Foundation, Inc., 51 Franklin Street, Fifth
 * Floor, Boston, MA 02110-1301 USA
*/
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_0;
use crate::abi::WmWorkspaceData_WmBackdrop as WmWorkspaceData;
use crate::abi::*;
extern "C" {
    fn XAllowEvents(_: *mut Display, _: ::core::ffi::c_int, _: Time) -> ::core::ffi::c_int;
    fn XCheckTypedWindowEvent(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: *mut XEvent,
    ) -> ::core::ffi::c_int;
    fn XDisplayHeight(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XDisplayWidth(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XFree(_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn XGetWindowAttributes(
        _: *mut Display,
        _: Window,
        _: *mut XWindowAttributes,
    ) -> ::core::ffi::c_int;
    fn XKillClient(_: *mut Display, _: XID) -> ::core::ffi::c_int;
    fn XQueryTree(
        _: *mut Display,
        _: Window,
        _: *mut Window,
        _: *mut Window,
        _: *mut *mut Window,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XSendEvent(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_long,
        _: *mut XEvent,
    ) -> ::core::ffi::c_int;
    fn XDeleteContext(_: *mut Display, _: XID, _: XContext) -> ::core::ffi::c_int;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
    fn XSaveContext(
        _: *mut Display,
        _: XID,
        _: XContext,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn XtWindowToWidget(_: *mut Display, _: Window) -> Widget;
    fn XmProcessTraversal(w: Widget, dir: XmTraversalDirection) -> Boolean;
    static mut wmGD: WmGlobalData;
    fn XRRUpdateConfiguration(event: *mut XEvent) -> ::core::ffi::c_int;
    fn FrameExposureProc(pcd: *mut ClientData);
    fn BaseWinExposureProc(pcd: *mut ClientData);
    fn SetFrameShape(pcd: *mut ClientData);
    fn PushGadgetIn(pcd: *mut ClientData, gadget: ::core::ffi::c_int);
    fn UpdateClientDecorations(pcd: *mut ClientData) -> Boolean;
    fn SetColormapFocus(pSD: *mut WmScreenData, pCD: *mut ClientData);
    fn WmInstallColormap(pSD: *mut WmScreenData, colormap: Colormap);
    fn ResetColormapData(pCD: *mut ClientData, pWindows: *mut Window, count: ::core::ffi::c_int);
    fn ProcessColormapList(pSD: *mut WmScreenData, pCD: *mut ClientData);
    fn CheckForButtonAction(
        buttonEvent: *mut XButtonEvent,
        context: Context,
        subContext: Context,
        pCD: *mut ClientData,
    ) -> Boolean;
    fn GetTimestamp() -> Time;
    fn HandleKeyPress(
        keyEvent: *mut XKeyEvent,
        keySpecs: *mut KeySpec,
        checkContext: Boolean,
        context: Context,
        onlyFirst: Boolean,
        pCD: *mut ClientData,
    ) -> Boolean;
    fn HandleWsEnterNotify(enterEvent: *mut XEnterWindowEvent);
    fn IdentifyEventContext(
        event: *mut XButtonEvent,
        pCD: *mut ClientData,
        pContext: *mut Context,
        pPartContext: *mut ::core::ffi::c_int,
    );
    fn ProcessClickBPress(
        buttonEvent: *mut XButtonEvent,
        pCD: *mut ClientData,
        context: Context,
        subContext: Context,
    );
    fn ProcessClickBRelease(
        buttonEvent: *mut XButtonEvent,
        pCD: *mut ClientData,
        context: Context,
        subContext: Context,
    );
    fn WmDispatchWsEvent(event: *mut XEvent) -> Boolean;
    fn WmGrabButton(
        display: *mut Display,
        button: ::core::ffi::c_uint,
        modifiers: ::core::ffi::c_uint,
        grab_window: Window,
        event_mask: ::core::ffi::c_uint,
        owner_events: ::core::ffi::c_int,
        pointer_mode: ::core::ffi::c_int,
        keyboard_mode: ::core::ffi::c_int,
        confine_to: Window,
        cursor: Cursor,
    );
    fn HandleWmClientMessage(clientEvent: *mut XClientMessageEvent);
    fn HandleMwmRequest(pSD: *mut WmScreenData, pev: *mut XEvent);
    fn PaintFeedbackWindow(pSD: *mut WmScreenData);
    fn F_Beep(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn Do_Lower(pCD: *mut ClientData, pStackEntry: *mut ClientListEntry, flags: ::core::ffi::c_int);
    fn Do_Focus_Key(pCD: *mut ClientData, focusTime: Time, flags: ::core::ffi::c_long);
    fn F_Pass_Key(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Kill(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Restore_And_Raise(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn Do_Raise(pCD: *mut ClientData, pStackEntry: *mut ClientListEntry, flags: ::core::ffi::c_int);
    fn F_Raise_Lower(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn IconExposureProc(pcd: *mut ClientData, clearFirst: Boolean);
    fn PaintActiveIconText(pcd: *mut ClientData, erase: Boolean);
    fn AutoResetKeyFocus(pcdNoFocus: *mut ClientData, focusTime: Time);
    fn SetKeyboardFocus(pCD: *mut ClientData, focusFlags: ::core::ffi::c_long);
    fn UnManageWindow(pCD: *mut ClientData);
    fn PostMenu(
        menuSpec: *mut MenuSpec,
        pCD: *mut ClientData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        button: ::core::ffi::c_uint,
        newContext: Context,
        flags: ::core::ffi::c_long,
        passedInEvent: *mut XEvent,
    );
    fn ProcessWmProtocols(pCD: *mut ClientData);
    fn ProcessMwmMessages(pCD: *mut ClientData);
    fn ProcessWmColormapWindows(pCD: *mut ClientData);
    fn SendConfigureNotify(pCD: *mut ClientData);
    fn DeleteClientWmTimers(pCD: *mut ClientData);
    fn HandleClientFrameMove(pcd: *mut ClientData, pev: *mut XEvent);
    fn HandleClientFrameResize(pcd: *mut ClientData, pev: *mut XEvent);
    fn ProcessNewConfiguration(
        pCD: *mut ClientData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        clientRequest: Boolean,
    );
    fn RecomputeMaxConfig(pCD: *mut ClientData);
    fn PlaceFrameOnScreen(
        pCD: *mut ClientData,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
    );
    fn ProcessMwmHints(pCD: *mut ClientData, firstTime: Boolean);
    fn ProcessWmHints(pCD: *mut ClientData, firstTime: Boolean);
    fn ProcessWmIconTitle(pCD: *mut ClientData, firstTime: Boolean);
    fn ProcessWmNormalHints(
        pCD: *mut ClientData,
        firstTime: Boolean,
        manageFlags: ::core::ffi::c_long,
    );
    fn ProcessWmTransientFor(pCD: *mut ClientData);
    fn ProcessWmWindowTitle(pCD: *mut ClientData, firstTime: Boolean);
    fn WmGetWindowAttributes(window: Window) -> Boolean;
    fn ProcessSmClientID(pCD: *mut ClientData);
    fn ProcessWmSaveHint(pCD: *mut ClientData);
    fn AddClientToList(pWS: *mut WmWorkspaceData, pCD: *mut ClientData, onTop: Boolean);
    fn DeleteClientFromList(pWS: *mut WmWorkspaceData, pCD: *mut ClientData);
    fn FindTransientTreeLeader(pcd: *mut ClientData) -> *mut ClientData;
    fn StackTransientWindow(pcd: *mut ClientData);
    fn SetClientState(pCD: *mut ClientData, newState: ::core::ffi::c_int, setTime: Time);
    fn ClientInWorkspace(pWS: *mut WmWorkspaceData, pCD: *mut ClientData) -> Boolean;
    fn SetClientWsIndex(pCD: *mut ClientData);
    fn HonorAbsentMapBehavior(pCD: *mut ClientData);
    fn HandleEwmhCPropertyNotify(_: *mut ClientData, _: *mut XPropertyEvent);
    fn HandleEwmhClientMessage(_: *mut ClientData, _: *mut XClientMessageEvent);
    fn HandleEwmhRootClientMessage(_: *mut WmScreenData, _: *mut XClientMessageEvent) -> Boolean;
    fn ConfigureEwmhFullScreen(_: *mut ClientData, _: Boolean);
    fn UpdateXineramaInfo();
    fn GetXineramaScreenFromLocation(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
    fn InitIconPlacement(pWS: *mut WmWorkspaceData);
    static mut buttonModifierMasks: [::core::ffi::c_uint; 0];
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
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
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CurrentTime: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ButtonReleaseMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const ButtonMotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 13 as ::core::ffi::c_int;
pub const StructureNotifyMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 17 as ::core::ffi::c_int;
pub const KeyPress: ::core::ffi::c_int = 2;
pub const ButtonPress: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ButtonRelease: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MotionNotify: ::core::ffi::c_int = 6;
pub const EnterNotify: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LeaveNotify: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const FocusIn: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const FocusOut: ::core::ffi::c_int = 10;
pub const Expose: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const GraphicsExpose: ::core::ffi::c_int = 13;
pub const NoExpose: ::core::ffi::c_int = 14;
pub const DestroyNotify: ::core::ffi::c_int = 17;
pub const UnmapNotify: ::core::ffi::c_int = 18;
pub const MapRequest: ::core::ffi::c_int = 20;
pub const ReparentNotify: ::core::ffi::c_int = 21;
pub const ConfigureNotify: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ConfigureRequest: ::core::ffi::c_int = 23;
pub const PropertyNotify: ::core::ffi::c_int = 28;
pub const ColormapNotify: ::core::ffi::c_int = 32;
pub const ClientMessage: ::core::ffi::c_int = 33;
pub const Button1Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const Button2Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
pub const Button1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Button2: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NotifyNormal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NotifyGrab: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NotifyUngrab: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NotifyWhileGrabbed: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NotifyInferior: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NotifyNonlinear: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NotifyNonlinearVirtual: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PropertyNewValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GrabModeSync: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GrabModeAsync: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AsyncPointer: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ReplayPointer: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AsyncKeyboard: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ReplayKeyboard: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CWX: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const CWY: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const CWWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const CWHeight: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const CWBorderWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const CWSibling: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const CWStackMode: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const NorthWestGravity: ::core::ffi::c_int = 1;
pub const NorthGravity: ::core::ffi::c_int = 2;
pub const NorthEastGravity: ::core::ffi::c_int = 3;
pub const WestGravity: ::core::ffi::c_int = 4;
pub const CenterGravity: ::core::ffi::c_int = 5;
pub const EastGravity: ::core::ffi::c_int = 6;
pub const SouthWestGravity: ::core::ffi::c_int = 7;
pub const SouthGravity: ::core::ffi::c_int = 8;
pub const SouthEastGravity: ::core::ffi::c_int = 9;
pub const StaticGravity: ::core::ffi::c_int = 10;
pub const Above: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Below: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TopIf: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BottomIf: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const Opposite: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NormalState: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IconicState: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ShapeBounding: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ShapeNotify: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_FUNC_MOVE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const MWM_FUNC_MINIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const MWM_FUNC_CLOSE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const MWM_DECOR_BORDER: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const MWM_DECOR_RESIZEH: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const MWM_DECOR_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XA_WM_COMMAND: Atom = 34;
pub const XA_WM_HINTS: Atom = 35;
pub const XA_WM_ICON_NAME: Atom = 37;
pub const XA_WM_NAME: Atom = 39;
pub const XA_WM_NORMAL_HINTS: Atom = 40;
pub const XA_WM_CLASS: Atom = 67;
pub const XA_WM_TRANSIENT_FOR: Atom = 68;
pub const SELECT_BUTTON: ::core::ffi::c_int = Button1;
pub const SELECT_BUTTON_MASK: ::core::ffi::c_int = Button1Mask;
pub const DMANIP_BUTTON: ::core::ffi::c_int = Button2;
pub const DMANIP_BUTTON_MASK: ::core::ffi::c_int = Button2Mask;
pub const NoButton: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ALWAYS_SET_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const REFRESH_LAST_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const CLIENT_AREA_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const SCREEN_SWITCH_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const POST_STICKY: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const WM_DECOR_BORDER: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const WM_DECOR_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const WM_DECOR_RESIZEH: ::core::ffi::c_long = WM_DECOR_BORDER | MWM_DECOR_RESIZEH;
pub const WM_DECOR_RESIZE: ::core::ffi::c_long = WM_DECOR_BORDER | MWM_DECOR_RESIZEH;
pub const F_CONTEXT_ICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const F_CONTEXT_NORMAL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const F_CONTEXT_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const F_CONTEXT_ICONBOX: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const F_CONTEXT_WINDOW: ::core::ffi::c_long = F_CONTEXT_NORMAL | F_CONTEXT_MAXIMIZE;
pub const FRAME_CLIENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FRAME_SYSTEM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FRAME_TITLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FRAME_MINIMIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FRAME_MAXIMIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FRAME_RESIZE_NW: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const FRAME_RESIZE_N: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const FRAME_RESIZE_NE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const FRAME_RESIZE_E: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const FRAME_RESIZE_SE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FRAME_RESIZE_S: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const FRAME_RESIZE_SW: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const FRAME_RESIZE_W: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FRAME_CLOSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const ICON_PART_ALL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ICONBOX_PART_IICON: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const ICONBOX_PART_WICON: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const F_SUBCONTEXT_I_ALL: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << ICON_PART_ALL;
pub const F_SUBCONTEXT_IB_IICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_IICON;
pub const F_SUBCONTEXT_IB_WICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_WICON;
pub const F_SUBCONTEXT_W_CLIENT: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << FRAME_CLIENT;
pub const F_SUBCONTEXT_W_SYSTEM: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << FRAME_SYSTEM;
pub const F_SUBCONTEXT_W_TITLE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << FRAME_TITLE;
pub const F_SUBCONTEXT_W_MINIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << FRAME_MINIMIZE;
pub const F_SUBCONTEXT_W_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << FRAME_MAXIMIZE;
pub const F_SUBCONTEXT_W_CLOSE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << FRAME_CLOSE;
pub const F_SUBCONTEXT_W_RESIZE_NW: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << FRAME_RESIZE_NW;
pub const F_SUBCONTEXT_W_RESIZE_N: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << FRAME_RESIZE_N;
pub const F_SUBCONTEXT_W_RESIZE_NE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << FRAME_RESIZE_NE;
pub const F_SUBCONTEXT_W_RESIZE_E: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << FRAME_RESIZE_E;
pub const F_SUBCONTEXT_W_RESIZE_SE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << FRAME_RESIZE_SE;
pub const F_SUBCONTEXT_W_RESIZE_S: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << FRAME_RESIZE_S;
pub const F_SUBCONTEXT_W_RESIZE_SW: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << FRAME_RESIZE_SW;
pub const F_SUBCONTEXT_W_RESIZE_W: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << FRAME_RESIZE_W;
pub const F_SUBCONTEXT_W_RBORDER: ::core::ffi::c_long = F_SUBCONTEXT_W_RESIZE_NW
    | F_SUBCONTEXT_W_RESIZE_N
    | F_SUBCONTEXT_W_RESIZE_NE
    | F_SUBCONTEXT_W_RESIZE_E
    | F_SUBCONTEXT_W_RESIZE_SE
    | F_SUBCONTEXT_W_RESIZE_S
    | F_SUBCONTEXT_W_RESIZE_SW
    | F_SUBCONTEXT_W_RESIZE_W;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAXIMIZED_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const UNSEEN_STATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CLIENT_DESTROYED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
pub const CLIENT_TERMINATING: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 10 as ::core::ffi::c_int;
pub const PROTOCOL_MWM_MESSAGES: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const AMAP_BEHAVIOR_IGNORE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CMAP_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_EXPLICIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MOVE_CLIENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RESIZE_CLIENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STACK_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RRScreenChangeNotify: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn SetupCButtonBindings(
    mut window: Window,
    mut buttonSpecs: *mut ButtonSpec,
) {
    let mut buttonSpec = ::core::ptr::null_mut::<ButtonSpec>();
    let mut eventMask: ::core::ffi::c_uint = 0;
    let mut grabState: ::core::ffi::c_uint = 0;
    buttonSpec = buttonSpecs;
    while !buttonSpec.is_null() {
        if (*buttonSpec).context & F_CONTEXT_WINDOW as Context != 0
            && (*buttonSpec).subContext & F_SUBCONTEXT_W_CLIENT as Context != 0
        {
            eventMask = (ButtonMotionMask | ButtonReleaseMask) as ::core::ffi::c_uint;
            if (*buttonSpec).eventType == ButtonRelease as ::core::ffi::c_uint {
                grabState = (*buttonSpec).state
                    & !*(&raw mut buttonModifierMasks as *mut ::core::ffi::c_uint)
                        .offset((*buttonSpec).button as isize);
            } else {
                grabState = (*buttonSpec).state;
            }
            WmGrabButton(
                wmGD.display,
                (*buttonSpec).button,
                grabState,
                window,
                False as ::core::ffi::c_uint,
                eventMask as ::core::ffi::c_int,
                GrabModeSync,
                GrabModeAsync,
                None_0 as Window,
                None_0 as Cursor,
            );
        }
        buttonSpec = (*buttonSpec).nextButtonSpec as *mut ButtonSpec;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WmDispatchClientEvent(mut event: *mut XEvent) -> Boolean {
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut cmap_window_data = ::core::ptr::null_mut::<*mut ClientData>();
    let mut dispatchEvent = False as Boolean;
    if XFindContext(
        wmGD.display,
        (*event).xany.window as XID,
        wmGD.windowContextType,
        &raw mut pCD as *mut XPointer,
    ) != 0
        && XFindContext(
            wmGD.display,
            (*event).xany.window as XID,
            wmGD.cmapWindowContextType,
            &raw mut cmap_window_data as *mut XPointer,
        ) != 0
    {
        if wmGD.queryScreen != 0 {
            DetermineActiveScreen(event);
        }
        return HandleEventsOnSpecialWindows(event);
    }
    if !cmap_window_data.is_null() {
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        while !(*cmap_window_data.offset(j as isize)).is_null() {
            if (*wmGD.pActiveSD).colormapFocus == *cmap_window_data.offset(j as isize) {
                pCD = *cmap_window_data.offset(j as isize);
                break;
            } else {
                j += 1;
            }
        }
        if pCD.is_null() {
            pCD = *cmap_window_data.offset(0 as ::core::ffi::c_int as isize);
        }
    }
    if !((*event).type_0 == FocusOut) {
        wmGD.pActiveSD = (*pCD).pSD;
        wmGD.queryScreen = False as Boolean;
    }
    SetClientWsIndex(pCD);
    if (*event).xany.window == (*pCD).client {
        return HandleEventsOnClientWindow(pCD, event);
    }
    match (*event).type_0 {
        ButtonPress => {
            dispatchEvent = HandleCButtonPress(pCD, event as *mut XButtonEvent);
        }
        ButtonRelease => {
            if !wmGD.menuActive.is_null() {
                dispatchEvent = True as Boolean;
            } else {
                HandleCButtonRelease(pCD, event as *mut XButtonEvent);
            }
        }
        KeyPress => {
            dispatchEvent = HandleCKeyPress(pCD, event as *mut XKeyEvent);
        }
        MotionNotify => {
            HandleCMotionNotify(pCD, event as *mut XMotionEvent);
        }
        Expose => {
            if (*event).xexpose.count == 0 as ::core::ffi::c_int {
                if (*event).xexpose.window
                    == (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin
                {
                    IconExposureProc(pCD, True as Boolean);
                    if !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
                        .pIconBox
                        .is_null()
                    {
                        dispatchEvent = True as Boolean;
                    }
                } else if (*event).xexpose.window == (*(*pCD).pSD).activeIconTextWin {
                    PaintActiveIconText(pCD, FALSE as Boolean);
                } else if (*pCD).clientFlags & CLIENT_DESTROYED == 0 {
                    if (*event).xexpose.window == (*pCD).clientFrameWin
                        || (*event).xexpose.window == (*pCD).clientTitleWin
                    {
                        FrameExposureProc(pCD);
                    }
                    if (*event).xexpose.window == (*pCD).clientBaseWin {
                        BaseWinExposureProc(pCD);
                    }
                }
            }
        }
        EnterNotify => {
            HandleCEnterNotify(pCD, event as *mut XEnterWindowEvent);
        }
        LeaveNotify => {
            HandleCLeaveNotify(pCD, event as *mut XLeaveWindowEvent);
        }
        FocusIn => {
            dispatchEvent = HandleCFocusIn(pCD, event as *mut XFocusChangeEvent);
        }
        FocusOut => {
            dispatchEvent = HandleCFocusOut(pCD, event as *mut XFocusChangeEvent);
        }
        DestroyNotify => {
            if (*(event as *mut XDestroyWindowEvent)).window == (*pCD).client {
                (*pCD).clientFlags |= CLIENT_DESTROYED;
                UnManageWindow(pCD);
            }
        }
        UnmapNotify => {
            if (*(event as *mut XUnmapEvent)).window == (*pCD).client {
                if (*pCD).wmUnmapCount != 0 {
                    (*pCD).wmUnmapCount -= 1;
                } else {
                    UnManageWindow(pCD);
                }
            }
        }
        MapRequest => {
            if ClientInWorkspace((*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData, pCD) == 0 {
                if (*pCD).absentMapBehavior == AMAP_BEHAVIOR_IGNORE {
                    SetClientState(pCD, NORMAL_STATE | UNSEEN_STATE, GetTimestamp());
                } else {
                    HonorAbsentMapBehavior(pCD);
                    SetClientState(pCD, NORMAL_STATE, GetTimestamp());
                }
            } else {
                SetClientState(pCD, NORMAL_STATE, GetTimestamp());
            }
        }
        ConfigureRequest => {
            HandleCConfigureRequest(pCD, event as *mut XConfigureRequestEvent);
        }
        ColormapNotify => {
            HandleCColormapNotify(pCD, event as *mut XColormapEvent);
        }
        ClientMessage => {
            HandleClientMessage(pCD, event as *mut XClientMessageEvent);
        }
        ReparentNotify => {
            if (*(event as *mut XReparentEvent)).window == (*pCD).client
                && (*(event as *mut XReparentEvent)).parent != (*pCD).clientBaseWin
            {
                UnManageWindow(pCD);
            }
        }
        _ => {}
    }
    return dispatchEvent;
}
#[no_mangle]
pub unsafe extern "C" fn HandleEventsOnSpecialWindows(mut pEvent: *mut XEvent) -> Boolean {
    let mut dispatchEvent = True as Boolean;
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    if wmGD.xrandr_present as ::core::ffi::c_int != 0
        && (*pEvent).type_0 == wmGD.xrandr_base_evt + RRScreenChangeNotify
    {
        HandleRRScreenChangeNotify(pEvent);
    }
    if (*pEvent).xany.window == (*wmGD.pActiveSD).rootWindow {
        if (*pEvent).type_0 == FocusIn {
            SetKeyboardFocus(NULL as *mut ClientData, REFRESH_LAST_FOCUS);
        }
    } else if (*pEvent).xany.window == (*wmGD.pActiveSD).feedbackWin {
        if (*pEvent).type_0 == Expose {
            if (*pEvent).xexpose.count == 0 as ::core::ffi::c_int {
                PaintFeedbackWindow(wmGD.pActiveSD);
            }
        }
        dispatchEvent = False as Boolean;
    } else if (*pEvent).xany.window == (*wmGD.pActiveSD).inputScreenWindow {
        if (*pEvent).type_0 == ButtonPress {
            F_Beep(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                NULL as *mut ClientData,
                NULL as *mut XEvent,
            );
        } else if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER
            && (*pEvent).type_0 == EnterNotify
        {
            HandleWsEnterNotify(pEvent as *mut XEnterWindowEvent);
        }
        dispatchEvent = False as Boolean;
    } else if XFindContext(
        wmGD.display,
        (*pEvent).xany.window as XID,
        wmGD.mwmWindowContextType,
        &raw mut pSD as *mut XPointer,
    ) == 0
    {
        if (*pEvent).type_0 == PropertyNotify
            && (*pEvent).xproperty.atom == wmGD.xa_MWM_WM_REQUEST
            && (*pEvent).xproperty.state == PropertyNewValue
        {
            HandleMwmRequest(pSD, pEvent);
        }
        if (*pEvent).type_0 == ClientMessage {
            HandleWmClientMessage(pEvent as *mut XClientMessageEvent);
        }
    } else {
        match (*pEvent).type_0 {
            ConfigureRequest => {
                if GetParentWindow((*pEvent).xconfigurerequest.window)
                    == (*wmGD.pActiveSD).rootWindow
                {
                    dispatchEvent = WmDispatchWsEvent(pEvent);
                }
            }
            MapRequest => {
                if GetParentWindow((*pEvent).xmaprequest.window) == (*wmGD.pActiveSD).rootWindow {
                    dispatchEvent = WmDispatchWsEvent(pEvent);
                }
            }
            ClientMessage => {
                if HandleEwmhRootClientMessage(wmGD.pActiveSD, &raw mut (*pEvent).xclient) != 0 {
                    dispatchEvent = False as Boolean;
                }
            }
            _ => {}
        }
    }
    return dispatchEvent;
}
#[no_mangle]
pub unsafe extern "C" fn HandleEventsOnClientWindow(
    mut pCD: *mut ClientData,
    mut pEvent: *mut XEvent,
) -> Boolean {
    let mut doXtDispatchEvent = True as Boolean;
    if (*pEvent).type_0 == wmGD.shapeEventBase + ShapeNotify {
        HandleCShapeNotify(pCD, pEvent as *mut XShapeEvent);
    } else {
        match (*pEvent).type_0 {
            ColormapNotify => {
                HandleCColormapNotify(pCD, pEvent as *mut XColormapEvent);
                doXtDispatchEvent = False as Boolean;
            }
            PropertyNotify => {
                HandleCPropertyNotify(pCD, pEvent as *mut XPropertyEvent);
                HandleEwmhCPropertyNotify(pCD, pEvent as *mut XPropertyEvent);
                doXtDispatchEvent = False as Boolean;
            }
            ClientMessage => {
                HandleClientMessage(pCD, pEvent as *mut XClientMessageEvent);
                HandleEwmhClientMessage(pCD, pEvent as *mut XClientMessageEvent);
            }
            _ => {}
        }
    }
    return doXtDispatchEvent;
}
#[no_mangle]
pub unsafe extern "C" fn HandleCPropertyNotify(
    mut pCD: *mut ClientData,
    mut propertyEvent: *mut XPropertyEvent,
) {
    match (*propertyEvent).atom {
        XA_WM_HINTS => {
            ProcessWmHints(pCD, FALSE as Boolean);
        }
        XA_WM_NORMAL_HINTS => {
            ProcessWmNormalHints(pCD, FALSE as Boolean, 0 as ::core::ffi::c_long);
        }
        XA_WM_NAME => {
            ProcessWmWindowTitle(pCD, FALSE as Boolean);
        }
        XA_WM_ICON_NAME => {
            ProcessWmIconTitle(pCD, FALSE as Boolean);
        }
        XA_WM_CLASS => {}
        XA_WM_COMMAND => {
            if (*pCD).clientFlags & CLIENT_TERMINATING != 0 {
                DeleteClientWmTimers(pCD);
                XKillClient(wmGD.display, (*pCD).client as XID);
            }
        }
        XA_WM_TRANSIENT_FOR => {
            DeleteClientFromList((*(*pCD).pSD).pActiveWS as *mut WmWorkspaceData, pCD);
            ProcessWmTransientFor(pCD);
            AddClientToList(
                (*(*pCD).pSD).pActiveWS as *mut WmWorkspaceData,
                pCD,
                True as Boolean,
            );
            if !(*pCD).transientLeader.is_null() {
                StackTransientWindow(pCD);
            }
        }
        _ => {
            if (*propertyEvent).atom == wmGD.xa_WM_PROTOCOLS {
                ProcessWmProtocols(pCD);
            } else if (*propertyEvent).atom == wmGD.xa_MWM_MESSAGES {
                if (*pCD).protocolFlags & PROTOCOL_MWM_MESSAGES != 0 {
                    ProcessMwmMessages(pCD);
                }
            } else if (*propertyEvent).atom == wmGD.xa_SM_CLIENT_ID {
                ProcessSmClientID(pCD);
            } else if (*propertyEvent).atom == wmGD.xa_WMSAVE_HINT {
                ProcessWmSaveHint(pCD);
            } else if (*propertyEvent).atom == wmGD.xa_WM_COLORMAP_WINDOWS {
                if (*propertyEvent).state == PropertyNewValue {
                    ProcessWmColormapWindows(pCD);
                } else {
                    ResetColormapData(
                        pCD,
                        ::core::ptr::null_mut::<Window>(),
                        0 as ::core::ffi::c_int,
                    );
                }
                if (*wmGD.pActiveSD).colormapFocus == pCD
                    && ((*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE)
                {
                    (*pCD).clientCmapFlagsInitialized = 0 as ::core::ffi::c_int;
                    ProcessColormapList(wmGD.pActiveSD, pCD);
                }
            } else if (*propertyEvent).atom == wmGD.xa_MWM_HINTS {
                let mut prev_decor = (*pCD).decor as ::core::ffi::c_ulong;
                let mut prev_func = (*pCD).clientFunctions as ::core::ffi::c_ulong;
                let mut decor_diff: ::core::ffi::c_ulong = 0;
                ProcessMwmHints(pCD, False as Boolean);
                decor_diff = prev_decor ^ (*pCD).decor as ::core::ffi::c_ulong;
                if decor_diff != 0 || prev_func != (*pCD).clientFunctions as ::core::ffi::c_ulong {
                    let mut full_screen = (*pCD).fullScreen;
                    if full_screen != 0 {
                        ConfigureEwmhFullScreen(pCD, False as Boolean);
                    }
                    if decor_diff & WM_DECOR_TITLE as ::core::ffi::c_ulong != 0
                        && ((*(*pCD).pSD).decoupleTitleAppearance as ::core::ffi::c_int != 0
                            && (*pCD).clientTitleWin != 0)
                    {
                        XDeleteContext(
                            wmGD.display,
                            (*pCD).clientTitleWin as XID,
                            wmGD.windowContextType,
                        );
                    }
                    if UpdateClientDecorations(pCD) == 0 {
                        UnManageWindow(pCD);
                    } else {
                        if decor_diff & WM_DECOR_TITLE as ::core::ffi::c_ulong != 0
                            && ((*(*pCD).pSD).decoupleTitleAppearance as ::core::ffi::c_int != 0
                                && (*pCD).clientTitleWin != 0)
                        {
                            XSaveContext(
                                wmGD.display,
                                (*pCD).clientTitleWin as XID,
                                wmGD.windowContextType,
                                pCD as XPointer as *const ::core::ffi::c_char,
                            );
                        }
                        if decor_diff & (WM_DECOR_TITLE | WM_DECOR_RESIZE) as ::core::ffi::c_ulong
                            != 0
                        {
                            let mut maximized = ((*pCD).clientState == MAXIMIZED_STATE)
                                as ::core::ffi::c_int
                                as Boolean;
                            if maximized != 0 {
                                SetClientState(pCD, NORMAL_STATE, GetTimestamp());
                            }
                            RecomputeMaxConfig(pCD);
                            if maximized != 0 {
                                SetClientState(pCD, MAXIMIZED_STATE, GetTimestamp());
                            }
                        }
                        if full_screen != 0 {
                            ConfigureEwmhFullScreen(pCD, True as Boolean);
                        }
                    }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn HandleCButtonPress(
    mut pCD: *mut ClientData,
    mut buttonEvent: *mut XButtonEvent,
) -> Boolean {
    let mut dispatchEvent = False as Boolean;
    let mut replayEvent = True as Boolean;
    let mut context: Context = 0;
    let mut partContext: ::core::ffi::c_int = 0;
    let mut subContext: Context = 0;
    static mut baseWinTime: Time = 0 as Time;
    static mut baseWinButton: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    wmGD.passButtonsCheck = True as Boolean;
    if !wmGD.menuActive.is_null() {
        dispatchEvent = True as Boolean;
    } else {
        IdentifyEventContext(buttonEvent, pCD, &raw mut context, &raw mut partContext);
        subContext = ((1 as ::core::ffi::c_long) << partContext) as Context;
        if (*buttonEvent).window == (*pCD).clientBaseWin {
            baseWinTime = (*buttonEvent).time;
            baseWinButton = (*buttonEvent).button;
        }
        if !((*buttonEvent).window == (*pCD).clientFrameWin
            && (*buttonEvent).button == baseWinButton
            && (*buttonEvent).time == baseWinTime)
        {
            if wmGD.clickData.time == (*buttonEvent).time {
                dispatchEvent = False as Boolean;
            } else {
                ProcessClickBPress(buttonEvent, pCD, context, subContext);
            }
            if CheckForButtonAction(buttonEvent, context, subContext, pCD) as ::core::ffi::c_int
                != 0
                && !pCD.is_null()
            {
                CheckButtonPressBuiltin(buttonEvent, context, subContext, partContext, pCD);
                (*pCD).focusAutoRaiseDisablePending = False as Boolean;
            } else {
                replayEvent = False as Boolean;
            }
        }
    }
    if (*buttonEvent).window == (*pCD).clientBaseWin {
        ProcessButtonGrabOnClient(pCD, buttonEvent, replayEvent);
    }
    return dispatchEvent;
}
#[no_mangle]
pub unsafe extern "C" fn ProcessButtonGrabOnClient(
    mut pCD: *mut ClientData,
    mut buttonEvent: *mut XButtonEvent,
    mut replayEvent: Boolean,
) {
    let mut buttonSpec = ::core::ptr::null_mut::<ButtonSpec>();
    let mut passButton: Boolean = 0;
    if (*buttonEvent).button == SELECT_BUTTON as ::core::ffi::c_uint
        && ((*buttonEvent).state == 0 as ::core::ffi::c_uint
            || (*buttonEvent).state
                & !wmGD.lockingModMask
                & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint)
    {
        passButton = wmGD.passSelectButton;
    } else {
        passButton = wmGD.passButtons;
    }
    if (*pCD).primaryModalCount != 0 || (*pCD).fullModalCount != 0 || passButton == 0 {
        replayEvent = False as Boolean;
    } else if replayEvent != 0 {
        buttonSpec = (*wmGD.pActiveSD).buttonSpecs;
        while !buttonSpec.is_null() {
            if (*buttonSpec).eventType == ButtonRelease as ::core::ffi::c_uint
                && ((*buttonEvent).state == (*buttonSpec).state
                    || (*buttonEvent).state
                        & !wmGD.lockingModMask
                        & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                        == (*buttonSpec).state)
                && (*buttonEvent).button == (*buttonSpec).button
            {
                replayEvent = False as Boolean;
                break;
            } else {
                buttonSpec = (*buttonSpec).nextButtonSpec as *mut ButtonSpec;
            }
        }
    }
    if replayEvent as ::core::ffi::c_int != 0 && wmGD.passButtonsCheck as ::core::ffi::c_int != 0 {
        XAllowEvents(wmGD.display, ReplayPointer, CurrentTime as Time);
    } else {
        if (*pCD).primaryModalCount != 0 || (*pCD).fullModalCount != 0 {
            F_Beep(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                pCD,
                NULL as *mut XEvent,
            );
        }
        XAllowEvents(wmGD.display, AsyncPointer, CurrentTime as Time);
    }
    XAllowEvents(wmGD.display, AsyncKeyboard, CurrentTime as Time);
}
#[no_mangle]
pub unsafe extern "C" fn CheckButtonPressBuiltin(
    mut buttonEvent: *mut XButtonEvent,
    mut context: Context,
    mut subContext: Context,
    mut partContext: ::core::ffi::c_int,
    mut pCD: *mut ClientData,
) {
    if (*buttonEvent).button != SELECT_BUTTON as ::core::ffi::c_uint
        && (*buttonEvent).button != DMANIP_BUTTON as ::core::ffi::c_uint
        || (*buttonEvent).state
            & !wmGD.lockingModMask
            & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as ::core::ffi::c_uint
            != 0
    {
        return;
    }
    if context & F_CONTEXT_ICON as Context != 0 {
        HandleIconButtonPress(pCD, buttonEvent);
    } else if context & F_CONTEXT_ICONBOX as Context != 0 {
        HandleIconBoxButtonPress(pCD, buttonEvent, subContext);
    } else if context & F_CONTEXT_WINDOW as Context != 0 {
        if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
            if !pCD.is_null() && (*pCD).focusAutoRaiseDisablePending as ::core::ffi::c_int != 0 {
                (*pCD).focusAutoRaiseDisabled = True as Boolean;
            }
            Do_Focus_Key(
                pCD,
                (*buttonEvent).time,
                if partContext == FRAME_CLIENT {
                    CLIENT_AREA_FOCUS
                } else {
                    0 as ::core::ffi::c_long
                },
            );
        }
        if (*buttonEvent).button == SELECT_BUTTON as ::core::ffi::c_uint
            && subContext == F_SUBCONTEXT_W_SYSTEM as Context
        {
            let mut flags = 0 as ::core::ffi::c_int;
            PushGadgetIn(pCD, partContext);
            if wmGD.clickData.doubleClickContext == F_SUBCONTEXT_W_SYSTEM as Context
                && wmGD.systemButtonClick2 as ::core::ffi::c_int != 0
                && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
            {
                wmGD.clickData.clickPending = False as Boolean;
                wmGD.clickData.doubleClickPending = False as Boolean;
                F_Kill(
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    pCD,
                    buttonEvent as *mut XEvent,
                );
                return;
            }
            if (*pCD).clientState == NORMAL_STATE {
                context = F_CONTEXT_NORMAL as Context;
            } else if (*pCD).clientState == MAXIMIZED_STATE {
                context = F_CONTEXT_MAXIMIZE as Context;
            } else {
                context = F_CONTEXT_ICON as Context;
            }
            if wmGD.systemButtonClick != 0 {
                wmGD.checkHotspot = True as Boolean;
                flags = (flags as ::core::ffi::c_long | POST_STICKY) as ::core::ffi::c_int;
            }
            (*pCD).grabContext = context;
            PostMenu(
                (*pCD).systemMenuSpec,
                pCD,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                SELECT_BUTTON as ::core::ffi::c_uint,
                context,
                flags as ::core::ffi::c_long,
                buttonEvent as *mut XEvent,
            );
        } else if subContext == F_SUBCONTEXT_W_TITLE as Context {
            PushGadgetIn(pCD, partContext);
            if (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MOVE != 0 {
                wmGD.preMove = True as Boolean;
                wmGD.preMoveX = (*buttonEvent).x_root;
                wmGD.preMoveY = (*buttonEvent).y_root;
                wmGD.configButton = (*buttonEvent).button;
                wmGD.configAction = MOVE_CLIENT;
            }
        } else if subContext & F_SUBCONTEXT_W_RBORDER as Context != 0 {
            wmGD.preMove = True as Boolean;
            wmGD.preMoveX = (*buttonEvent).x_root;
            wmGD.preMoveY = (*buttonEvent).y_root;
            wmGD.configButton = (*buttonEvent).button;
            wmGD.configAction = RESIZE_CLIENT;
            wmGD.configPart = partContext as ::core::ffi::c_uint;
            wmGD.configSet = True as Boolean;
        } else if (*buttonEvent).button == SELECT_BUTTON as ::core::ffi::c_uint
            && subContext
                & (F_SUBCONTEXT_W_MINIMIZE | F_SUBCONTEXT_W_MAXIMIZE | F_SUBCONTEXT_W_CLOSE)
                    as Context
                != 0
        {
            PushGadgetIn(pCD, partContext);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleIconButtonPress(
    mut pCD: *mut ClientData,
    mut buttonEvent: *mut XButtonEvent,
) {
    let mut newState: ::core::ffi::c_int = 0;
    if wmGD.clickData.doubleClickContext == F_SUBCONTEXT_I_ALL as Context {
        if (*pCD).maxConfig != 0 {
            newState = MAXIMIZED_STATE;
        } else {
            newState = NORMAL_STATE;
        }
        SetClientState(pCD, newState, (*buttonEvent).time);
        wmGD.clickData.clickPending = False as Boolean;
        wmGD.clickData.doubleClickPending = False as Boolean;
    } else {
        if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT {
            Do_Focus_Key(pCD, (*buttonEvent).time, ALWAYS_SET_FOCUS);
        }
        wmGD.preMove = True as Boolean;
        wmGD.preMoveX = (*buttonEvent).x_root;
        wmGD.preMoveY = (*buttonEvent).y_root;
        wmGD.configButton = (*buttonEvent).button;
        wmGD.configAction = MOVE_CLIENT;
    };
}
#[no_mangle]
pub unsafe extern "C" fn HandleIconBoxButtonPress(
    mut pCD: *mut ClientData,
    mut buttonEvent: *mut XButtonEvent,
    mut subContext: Context,
) {
    if wmGD.clickData.doubleClickContext == F_SUBCONTEXT_IB_IICON as Context
        || wmGD.clickData.doubleClickContext == F_SUBCONTEXT_IB_WICON as Context
    {
        F_Restore_And_Raise(NULL as String_0, pCD, NULL as *mut XEvent);
    } else if subContext == F_SUBCONTEXT_IB_IICON as Context
        || subContext == F_SUBCONTEXT_IB_WICON as Context
    {
        wmGD.preMove = True as Boolean;
        wmGD.preMoveX = (*buttonEvent).x_root;
        wmGD.preMoveY = (*buttonEvent).y_root;
        wmGD.configButton = (*buttonEvent).button;
        wmGD.configAction = MOVE_CLIENT;
    }
    if (*(*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox).pCD_iconBox
        == wmGD.keyboardFocus
        || (*(*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox).pCD_iconBox
            == wmGD.nextKeyboardFocus
    {
        XmProcessTraversal(
            XtWindowToWidget(
                wmGD.display,
                (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin,
            ),
            XmTRAVERSE_CURRENT,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleCButtonRelease(
    mut pCD: *mut ClientData,
    mut buttonEvent: *mut XButtonEvent,
) {
    let mut context: Context = 0;
    let mut subContext: Context = 0;
    let mut partContext: ::core::ffi::c_int = 0;
    IdentifyEventContext(buttonEvent, pCD, &raw mut context, &raw mut partContext);
    subContext = ((1 as ::core::ffi::c_long) << partContext) as Context;
    ProcessClickBRelease(buttonEvent, pCD, context, subContext);
    if CheckForButtonAction(buttonEvent, context, subContext, pCD) as ::core::ffi::c_int != 0
        && !pCD.is_null()
    {
        CheckButtonReleaseBuiltin(buttonEvent, context, subContext, pCD);
    }
    wmGD.preMove = False as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn HandleCKeyPress(
    mut pCD: *mut ClientData,
    mut keyEvent: *mut XKeyEvent,
) -> Boolean {
    let mut dispatchEvent = False as Boolean;
    let mut checkKeyEvent = True as Boolean;
    if !wmGD.menuActive.is_null() {
        dispatchEvent = True as Boolean;
        checkKeyEvent = False as Boolean;
    }
    if ((*keyEvent).window == (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin
        || (*keyEvent).window == (*(*pCD).pSD).activeIconTextWin)
        && !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
            .pIconBox
            .is_null()
    {
        dispatchEvent = True as Boolean;
        checkKeyEvent = False as Boolean;
        if (*keyEvent).window == (*(*pCD).pSD).activeIconTextWin {
            (*keyEvent).window = (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin;
        }
    } else if wmGD.passKeysActive != 0 {
        if !wmGD.passKeysKeySpec.is_null()
            && ((*wmGD.passKeysKeySpec).state == (*keyEvent).state
                || (*wmGD.passKeysKeySpec).state
                    == (*keyEvent).state
                        & !wmGD.lockingModMask
                        & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint)
            && (*wmGD.passKeysKeySpec).keycode as ::core::ffi::c_uint == (*keyEvent).keycode
        {
            F_Pass_Key(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                NULL as *mut ClientData,
                NULL as *mut XEvent,
            );
            XAllowEvents(wmGD.display, AsyncKeyboard, CurrentTime as Time);
        } else {
            XAllowEvents(wmGD.display, ReplayKeyboard, CurrentTime as Time);
        }
        checkKeyEvent = False as Boolean;
    } else {
        XAllowEvents(wmGD.display, AsyncKeyboard, CurrentTime as Time);
    }
    if checkKeyEvent as ::core::ffi::c_int != 0
        && (*keyEvent).window == (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin
    {
        checkKeyEvent = HandleKeyPress(
            keyEvent,
            (*wmGD.pActiveSD).keySpecs,
            True as Boolean,
            F_CONTEXT_ICON as Context,
            False as Boolean,
            NULL as *mut ClientData,
        );
        if checkKeyEvent as ::core::ffi::c_int != 0 && (*wmGD.pActiveSD).acceleratorMenuCount != 0 {
            let mut n: ::core::ffi::c_int = 0;
            n = 0 as ::core::ffi::c_int;
            while (*keyEvent).keycode != 0 as ::core::ffi::c_uint
                && (n as ::core::ffi::c_uint) < (*wmGD.pActiveSD).acceleratorMenuCount
            {
                if HandleKeyPress(
                    keyEvent,
                    (**(*wmGD.pActiveSD).acceleratorMenuSpecs.offset(n as isize)).accelKeySpecs,
                    True as Boolean,
                    F_CONTEXT_ICON as Context,
                    True as Boolean,
                    NULL as *mut ClientData,
                ) == 0
                {
                    checkKeyEvent = False as Boolean;
                    break;
                } else {
                    n += 1;
                }
            }
        }
    }
    if checkKeyEvent as ::core::ffi::c_int != 0
        && !(*pCD).systemMenuSpec.is_null()
        && !(*(*pCD).systemMenuSpec).accelKeySpecs.is_null()
    {
        HandleKeyPress(
            keyEvent,
            (*(*pCD).systemMenuSpec).accelKeySpecs,
            FALSE as Boolean,
            0 as Context,
            TRUE as Boolean,
            NULL as *mut ClientData,
        );
    }
    return dispatchEvent;
}
#[no_mangle]
pub unsafe extern "C" fn CheckButtonReleaseBuiltin(
    mut buttonEvent: *mut XButtonEvent,
    mut context: Context,
    mut subContext: Context,
    mut pCD: *mut ClientData,
) {
    if !((*buttonEvent).button == SELECT_BUTTON as ::core::ffi::c_uint
        && (*buttonEvent).state
            & !wmGD.lockingModMask
            & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as ::core::ffi::c_uint
            == SELECT_BUTTON_MASK as ::core::ffi::c_uint)
        && !((*buttonEvent).button == DMANIP_BUTTON as ::core::ffi::c_uint
            && (*buttonEvent).state
                & !wmGD.lockingModMask
                & !((3 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as ::core::ffi::c_uint
                == DMANIP_BUTTON_MASK as ::core::ffi::c_uint)
    {
        return;
    }
    if (*buttonEvent).button == SELECT_BUTTON as ::core::ffi::c_uint
        && context & F_CONTEXT_ICON as Context != 0
    {
        if wmGD.iconClick as ::core::ffi::c_int != 0
            && wmGD.clickData.clickContext == F_SUBCONTEXT_I_ALL as Context
        {
            wmGD.checkHotspot = True as Boolean;
            (*pCD).grabContext = F_CONTEXT_ICON as Context;
            PostMenu(
                (*pCD).systemMenuSpec,
                pCD,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NoButton as ::core::ffi::c_uint,
                F_CONTEXT_ICON as Context,
                POST_STICKY,
                buttonEvent as *mut XEvent,
            );
        }
    } else if (*buttonEvent).button == SELECT_BUTTON as ::core::ffi::c_uint
        && context & F_CONTEXT_ICONBOX as Context != 0
    {
        if wmGD.iconClick as ::core::ffi::c_int != 0
            && ((*pCD).clientState == MINIMIZED_STATE
                && wmGD.clickData.clickContext == F_SUBCONTEXT_IB_IICON as Context
                || wmGD.clickData.clickContext == F_SUBCONTEXT_IB_WICON as Context)
        {
            wmGD.checkHotspot = True as Boolean;
            if wmGD.clickData.clickContext == F_SUBCONTEXT_IB_IICON as Context
                && (*pCD).clientState == MINIMIZED_STATE
            {
                (*pCD).grabContext = F_SUBCONTEXT_IB_IICON as Context;
                PostMenu(
                    (*pCD).systemMenuSpec,
                    pCD,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    NoButton as ::core::ffi::c_uint,
                    F_SUBCONTEXT_IB_IICON as Context,
                    POST_STICKY,
                    buttonEvent as *mut XEvent,
                );
            } else {
                (*pCD).grabContext = F_SUBCONTEXT_IB_WICON as Context;
                PostMenu(
                    (*pCD).systemMenuSpec,
                    pCD,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    NoButton as ::core::ffi::c_uint,
                    F_SUBCONTEXT_IB_WICON as Context,
                    POST_STICKY,
                    buttonEvent as *mut XEvent,
                );
            }
        }
    } else if context & F_CONTEXT_WINDOW as Context != 0 {
        if (*buttonEvent).button == SELECT_BUTTON as ::core::ffi::c_uint
            && subContext == F_SUBCONTEXT_W_CLOSE as Context
        {
            if wmGD.clickData.clickContext == F_SUBCONTEXT_W_CLOSE as Context
                && ((*pCD).decor & MWM_DECOR_TITLE != 0
                    && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0)
            {
                wmGD.clickData.clickPending = False as Boolean;
                wmGD.clickData.doubleClickPending = False as Boolean;
                F_Kill(
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    pCD,
                    buttonEvent as *mut XEvent,
                );
            }
        } else if (*buttonEvent).button == SELECT_BUTTON as ::core::ffi::c_uint
            && subContext == F_SUBCONTEXT_W_MINIMIZE as Context
        {
            if wmGD.clickData.clickContext == F_SUBCONTEXT_W_MINIMIZE as Context {
                SetClientState(pCD, MINIMIZED_STATE, (*buttonEvent).time);
            }
        } else if (*buttonEvent).button == SELECT_BUTTON as ::core::ffi::c_uint
            && subContext == F_SUBCONTEXT_W_MAXIMIZE as Context
        {
            if wmGD.clickData.clickContext == F_SUBCONTEXT_W_MAXIMIZE as Context {
                if (*pCD).clientState == NORMAL_STATE {
                    SetClientState(pCD, MAXIMIZED_STATE, (*buttonEvent).time);
                } else {
                    SetClientState(pCD, NORMAL_STATE, (*buttonEvent).time);
                }
            }
        }
    }
    wmGD.preMove = False as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn HandleCMotionNotify(
    mut pCD: *mut ClientData,
    mut motionEvent: *mut XMotionEvent,
) {
    let mut diffX: ::core::ffi::c_int = 0;
    let mut diffY: ::core::ffi::c_int = 0;
    if wmGD.preMove != 0 {
        diffX = (*motionEvent).x_root - wmGD.preMoveX;
        if diffX < 0 as ::core::ffi::c_int {
            diffX = -diffX;
        }
        diffY = (*motionEvent).y_root - wmGD.preMoveY;
        if diffY < 0 as ::core::ffi::c_int {
            diffY = -diffY;
        }
        if diffX >= wmGD.moveThreshold || diffY >= wmGD.moveThreshold {
            wmGD.clickData.clickPending = False as Boolean;
            wmGD.clickData.doubleClickPending = False as Boolean;
            wmGD.preMove = False as Boolean;
            if wmGD.configAction == MOVE_CLIENT {
                HandleClientFrameMove(pCD, motionEvent as *mut XEvent);
            } else if wmGD.configAction == RESIZE_CLIENT {
                HandleClientFrameResize(pCD, motionEvent as *mut XEvent);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleCEnterNotify(
    mut pCD: *mut ClientData,
    mut enterEvent: *mut XEnterWindowEvent,
) {
    let mut report = _XEvent { type_0: 0 };
    let mut MatchFound: Boolean = 0;
    let mut enterWindow: Window = 0;
    if (*pCD).clientState == MINIMIZED_STATE {
        enterWindow = (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin;
    } else {
        enterWindow = (*pCD).clientFrameWin;
    }
    MatchFound =
        XCheckTypedWindowEvent(wmGD.display, enterWindow, LeaveNotify, &raw mut report) as Boolean;
    if (*enterEvent).detail != NotifyInferior
        && ((MatchFound == 0 || report.xcrossing.detail == NotifyInferior)
            && ((*enterEvent).mode == NotifyNormal || (*enterEvent).mode == NotifyUngrab)
            && wmGD.menuActive.is_null()
            || !wmGD.keyboardFocus.is_null()
                && (*wmGD.keyboardFocus).clientFrameWin != enterWindow
                && (*enterEvent).mode == NotifyGrab)
        && (wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER
            || wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER)
    {
        if !((*enterEvent).window == (*pCD).clientFrameWin && (*pCD).clientState == MINIMIZED_STATE
            || ((*enterEvent).window
                == (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin
                && !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
                    .pIconBox
                    .is_null()
                || (*enterEvent).window == (*(*pCD).pSD).activeIconTextWin))
        {
            if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER {
                if pCD != wmGD.keyboardFocus || pCD != wmGD.nextKeyboardFocus {
                    Do_Focus_Key(pCD, (*enterEvent).time, ALWAYS_SET_FOCUS);
                    if wmGD.replayEnterEvent != 0 {
                        memcpy(
                            &raw mut wmGD.savedEnterEvent as *mut ::core::ffi::c_void,
                            enterEvent as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<XEnterWindowEvent>() as size_t,
                        );
                    }
                }
            }
            if wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER {
                SetColormapFocus(wmGD.pActiveSD, pCD);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleCLeaveNotify(
    mut pCD: *mut ClientData,
    mut leaveEvent: *mut XLeaveWindowEvent,
) {
    let mut report = _XEvent { type_0: 0 };
    let mut leaveWindow: Window = 0;
    if (*pCD).clientState == MINIMIZED_STATE {
        leaveWindow = (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin;
    } else {
        leaveWindow = (*pCD).clientFrameWin;
    }
    if !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
        .pIconBox
        .is_null()
        && (*(*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox).pCD_iconBox
            != wmGD.keyboardFocus
        && (*(*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox).pCD_iconBox
            != wmGD.nextKeyboardFocus
    {
        XCheckTypedWindowEvent(wmGD.display, leaveWindow, EnterNotify, &raw mut report);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleCFocusIn(
    mut pCD: *mut ClientData,
    mut focusChangeEvent: *mut XFocusChangeEvent,
) -> Boolean {
    let mut setupNextFocus: Boolean = 0;
    let mut doXtDispatchEvent = False as Boolean;
    if (*focusChangeEvent).window
        == (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin
        && !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
            .pIconBox
            .is_null()
    {
        doXtDispatchEvent = True as Boolean;
    } else if ((*focusChangeEvent).mode == NotifyNormal
        || (*focusChangeEvent).mode == NotifyWhileGrabbed)
        && !((*focusChangeEvent).window == (*pCD).clientBaseWin
            && (*pCD).clientState == MINIMIZED_STATE)
        && !((*focusChangeEvent).window
            == (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin
            && (*pCD).clientState != MINIMIZED_STATE)
    {
        setupNextFocus =
            (wmGD.keyboardFocus == wmGD.nextKeyboardFocus) as ::core::ffi::c_int as Boolean;
        if wmGD.keyboardFocus != pCD {
            if (*focusChangeEvent).detail == NotifyNonlinear
                || (*focusChangeEvent).detail == NotifyNonlinearVirtual
            {
                SetKeyboardFocus(pCD, REFRESH_LAST_FOCUS);
                if setupNextFocus != 0 {
                    wmGD.nextKeyboardFocus = wmGD.keyboardFocus;
                }
            } else {
                SetKeyboardFocus(pCD, REFRESH_LAST_FOCUS);
                if setupNextFocus != 0 {
                    wmGD.nextKeyboardFocus = wmGD.keyboardFocus;
                }
            }
        } else if (*focusChangeEvent).detail == NotifyInferior
            && wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_EXPLICIT
        {
            if wmGD.autoKeyFocus != 0 {
                AutoResetKeyFocus(wmGD.keyboardFocus, GetTimestamp());
            } else {
                Do_Focus_Key(NULL as *mut ClientData, GetTimestamp(), ALWAYS_SET_FOCUS);
            }
        }
    }
    (*pCD).focusAutoRaiseDisabled = False as Boolean;
    return doXtDispatchEvent;
}
#[no_mangle]
pub unsafe extern "C" fn HandleCFocusOut(
    mut pCD: *mut ClientData,
    mut focusChangeEvent: *mut XFocusChangeEvent,
) -> Boolean {
    let mut doXtDispatchEvent = False as Boolean;
    let mut focusFlags = REFRESH_LAST_FOCUS;
    (*pCD).focusAutoRaiseDisabled = False as Boolean;
    if (*focusChangeEvent).window
        == (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin
        && !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
            .pIconBox
            .is_null()
        || (*(*pCD).pSD).screen != (*wmGD.pActiveSD).screen
    {
        doXtDispatchEvent = True as Boolean;
    } else if wmGD.keyboardFocus == pCD
        && (*focusChangeEvent).mode == NotifyNormal
        && ((*focusChangeEvent).detail == NotifyNonlinear
            || (*focusChangeEvent).detail == NotifyNonlinearVirtual)
        && !((*focusChangeEvent).window == (*pCD).clientBaseWin
            && (*pCD).clientState == MINIMIZED_STATE)
        && !((*focusChangeEvent).window
            == (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconFrameWin
            && (*pCD).clientState != MINIMIZED_STATE)
    {
        if (*(*pCD).pSD).screen != (*wmGD.pActiveSD).screen {
            focusFlags |= SCREEN_SWITCH_FOCUS;
        }
        SetKeyboardFocus(NULL as *mut ClientData, focusFlags);
        if wmGD.nextKeyboardFocus == pCD {
            wmGD.nextKeyboardFocus = ::core::ptr::null_mut::<ClientData>();
        }
    }
    return doXtDispatchEvent;
}
#[no_mangle]
pub unsafe extern "C" fn HandleCConfigureRequest(
    mut pCD: *mut ClientData,
    mut configureRequest: *mut XConfigureRequestEvent,
) {
    let mut mask = (*configureRequest).value_mask as ::core::ffi::c_uint;
    let mut stackMode = (*configureRequest).detail;
    let mut changeMask: ::core::ffi::c_uint = 0;
    let mut pcdLeader = ::core::ptr::null_mut::<ClientData>();
    let mut pcdSibling = ::core::ptr::null_mut::<ClientData>();
    let mut pStackEntry = ::core::ptr::null_mut::<ClientListEntry>();
    if (*configureRequest).window == (*pCD).client
        && mask & (CWX | CWY | CWWidth | CWHeight | CWBorderWidth) as ::core::ffi::c_uint != 0
    {
        if (*pCD).fullScreen != 0 {
            let mut evt = XConfigureEvent {
                type_0: 0,
                serial: 0,
                send_event: 0,
                display: ::core::ptr::null_mut::<Display>(),
                event: 0,
                window: 0,
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                border_width: 0,
                above: 0,
                override_redirect: 0,
            };
            evt.type_0 = ConfigureNotify;
            evt.display = wmGD.display;
            evt.border_width = (*configureRequest).border_width;
            evt.event = (*pCD).client;
            evt.window = (*pCD).client;
            evt.x = (*pCD).fullScreenX + evt.border_width;
            evt.y = (*pCD).fullScreenY + evt.border_width;
            evt.width = (*pCD).fullScreenWidth - evt.border_width * 2 as ::core::ffi::c_int;
            evt.height = (*pCD).fullScreenHeight - evt.border_width * 2 as ::core::ffi::c_int;
            XSendEvent(
                wmGD.display,
                (*pCD).client,
                False,
                StructureNotifyMask,
                &raw mut evt as *mut XEvent,
            );
        } else {
            let mut cx = if mask & CWX as ::core::ffi::c_uint != 0 {
                (*configureRequest).x
            } else if (*pCD).maxConfig as ::core::ffi::c_int != 0 {
                (*pCD).maxX
            } else {
                (*pCD).clientX
            };
            let mut cy = if mask & CWY as ::core::ffi::c_uint != 0 {
                (*configureRequest).y
            } else if (*pCD).maxConfig as ::core::ffi::c_int != 0 {
                (*pCD).maxY
            } else {
                (*pCD).clientY
            };
            let mut width = (if mask & CWWidth as ::core::ffi::c_uint != 0 {
                (*configureRequest).width
            } else {
                (*pCD).clientWidth
            }) as ::core::ffi::c_uint;
            let mut height = (if mask & CWHeight as ::core::ffi::c_uint != 0 {
                (*configureRequest).height
            } else {
                (*pCD).clientHeight
            }) as ::core::ffi::c_uint;
            if (*pCD).windowGravity != StaticGravity {
                if mask & CWX as ::core::ffi::c_uint == 0 {
                    cx -= (*pCD).clientOffset.x as ::core::ffi::c_int;
                }
                if mask & CWY as ::core::ffi::c_uint == 0 {
                    cy -= (*pCD).clientOffset.y as ::core::ffi::c_int;
                }
            }
            AdjustCoordinatesToGravity(
                pCD,
                (*pCD).windowGravity,
                &raw mut cx,
                &raw mut cy,
                &raw mut width,
                &raw mut height,
            );
            ProcessNewConfiguration(pCD, cx, cy, width, height, True as Boolean);
        }
    }
    if mask & CWStackMode as ::core::ffi::c_uint != 0 {
        changeMask = mask & (CWSibling | CWStackMode) as ::core::ffi::c_uint;
        if changeMask & CWSibling as ::core::ffi::c_uint != 0 {
            if XFindContext(
                wmGD.display,
                (*configureRequest).above as XID,
                wmGD.windowContextType,
                &raw mut pcdSibling as *mut XPointer,
            ) != 0
            {
                changeMask &= !CWSibling as ::core::ffi::c_uint;
            } else {
                pcdLeader = FindTransientTreeLeader(pCD);
                pcdSibling = FindTransientTreeLeader(pcdSibling);
                if pcdLeader == pcdSibling {
                    changeMask &= !CWSibling as ::core::ffi::c_uint;
                } else {
                    pStackEntry = &raw mut (*pcdSibling).clientEntry;
                    if stackMode == Above || stackMode == TopIf {
                        Do_Lower(pcdLeader, pStackEntry, STACK_NORMAL);
                    } else if stackMode == Below || stackMode == BottomIf {
                        Do_Raise(pcdLeader, pStackEntry, STACK_NORMAL);
                    } else if stackMode == Opposite {
                        F_Raise_Lower(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            pCD,
                            configureRequest as *mut XEvent,
                        );
                    }
                }
            }
        }
        if changeMask & CWSibling as ::core::ffi::c_uint == 0 {
            if stackMode == Above || stackMode == TopIf {
                Do_Raise(pCD, NULL as *mut ClientListEntry, STACK_NORMAL);
            } else if stackMode == Below || stackMode == BottomIf {
                Do_Lower(pCD, NULL as *mut ClientListEntry, STACK_NORMAL);
            } else if stackMode == Opposite {
                F_Raise_Lower(
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    pCD,
                    configureRequest as *mut XEvent,
                );
            }
        }
        if (*configureRequest).window == (*pCD).client
            && mask & (CWX | CWY | CWWidth | CWHeight | CWBorderWidth) as ::core::ffi::c_uint == 0
        {
            SendConfigureNotify(pCD);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn AdjustCoordinatesToGravity(
    mut pCD: *mut ClientData,
    mut gravity: ::core::ffi::c_int,
    mut px: *mut ::core::ffi::c_int,
    mut py: *mut ::core::ffi::c_int,
    mut pwidth: *mut ::core::ffi::c_uint,
    mut pheight: *mut ::core::ffi::c_uint,
) {
    let mut dpy_width: ::core::ffi::c_int = 0;
    let mut dpy_height: ::core::ffi::c_int = 0;
    let mut cw = (if (*pCD).maxConfig as ::core::ffi::c_int != 0 {
        (*pCD).maxWidth
    } else {
        (*pCD).clientWidth
    }) as ::core::ffi::c_uint;
    let mut ch = (if (*pCD).maxConfig as ::core::ffi::c_int != 0 {
        (*pCD).maxHeight
    } else {
        (*pCD).clientHeight
    }) as ::core::ffi::c_uint;
    match gravity {
        NorthWestGravity | SouthWestGravity | WestGravity => {
            *px += (*pCD).clientOffset.x as ::core::ffi::c_int;
        }
        NorthGravity | SouthGravity | CenterGravity => {
            *px = (*px as ::core::ffi::c_uint).wrapping_sub(
                (*pwidth)
                    .wrapping_sub(cw)
                    .wrapping_div(2 as ::core::ffi::c_uint)
                    .wrapping_sub((*pCD).clientOffset.x as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        NorthEastGravity | EastGravity | SouthEastGravity => {
            *px = (*px as ::core::ffi::c_uint).wrapping_sub(
                (*pwidth)
                    .wrapping_sub(cw)
                    .wrapping_sub((*pCD).clientOffset.x as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        StaticGravity | _ => {}
    }
    match gravity {
        NorthWestGravity | NorthGravity | NorthEastGravity => {
            *py += (*pCD).clientOffset.y as ::core::ffi::c_int;
        }
        EastGravity | WestGravity | CenterGravity => {
            *py = (*py as ::core::ffi::c_uint).wrapping_sub(
                (*pheight)
                    .wrapping_sub(ch)
                    .wrapping_div(2 as ::core::ffi::c_uint)
                    .wrapping_sub((*pCD).clientOffset.y as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        SouthEastGravity | SouthGravity | SouthWestGravity => {
            *py = (*py as ::core::ffi::c_uint).wrapping_sub(
                (*pheight)
                    .wrapping_sub(ch)
                    .wrapping_sub((*pCD).clientOffset.y as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        StaticGravity | _ => {}
    }
    dpy_width = XDisplayWidth(wmGD.display, (*(*pCD).pSD).screen);
    dpy_height = XDisplayHeight(wmGD.display, (*(*pCD).pSD).screen);
    if (*px - (*pCD).clientOffset.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        *px = (*pCD).clientOffset.x as ::core::ffi::c_int;
    } else if *px > dpy_width {
        *px = (dpy_width as ::core::ffi::c_uint).wrapping_sub((*pCD).frameInfo.titleBarHeight)
            as ::core::ffi::c_int;
    }
    if (*py - (*pCD).clientOffset.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        *py = (*pCD).clientOffset.y as ::core::ffi::c_int;
    } else if *py > dpy_height {
        *py = dpy_height;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleCColormapNotify(
    mut pCD: *mut ClientData,
    mut colorEvent: *mut XColormapEvent,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut cmap_window_data = ::core::ptr::null_mut::<*mut ClientData>();
    let mut newClientColormap = False as Boolean;
    if (*colorEvent).new != 0 {
        if WmGetWindowAttributes((*colorEvent).window) != 0 {
            (*colorEvent).colormap = wmGD.windowAttributes.colormap;
        } else {
            return;
        }
        if (*pCD).clientCmapCount == 0 as ::core::ffi::c_int
            && (*colorEvent).window == (*pCD).client
        {
            if (*colorEvent).colormap == None_0 as Colormap {
                (*pCD).clientColormap = (*wmGD.pActiveSD).workspaceColormap;
            } else {
                (*pCD).clientColormap = (*colorEvent).colormap;
            }
            newClientColormap = True as Boolean;
        }
        if XFindContext(
            wmGD.display,
            (*colorEvent).window as XID,
            wmGD.cmapWindowContextType,
            &raw mut cmap_window_data as *mut XPointer,
        ) == 0
        {
            let mut any_pCD = ::core::ptr::null_mut::<ClientData>();
            let mut j: ::core::ffi::c_int = 0;
            j = 0 as ::core::ffi::c_int;
            while !(*cmap_window_data.offset(j as isize)).is_null() {
                any_pCD = *cmap_window_data.offset(j as isize);
                i = 0 as ::core::ffi::c_int;
                while i < (*any_pCD).clientCmapCount {
                    if *(*any_pCD).cmapWindows.offset(i as isize) == (*colorEvent).window {
                        if (*colorEvent).colormap == None_0 as Colormap {
                            *(*any_pCD).clientCmapList.offset(i as isize) =
                                (*wmGD.pActiveSD).workspaceColormap;
                        } else {
                            *(*any_pCD).clientCmapList.offset(i as isize) = (*colorEvent).colormap;
                        }
                        if i == (*any_pCD).clientCmapIndex {
                            (*any_pCD).clientColormap =
                                *(*any_pCD).clientCmapList.offset(i as isize);
                            if any_pCD == pCD {
                                newClientColormap = True as Boolean;
                            }
                        }
                        break;
                    } else {
                        i += 1;
                    }
                }
                j += 1;
            }
        }
        if (*wmGD.pActiveSD).colormapFocus == pCD
            && newClientColormap as ::core::ffi::c_int != 0
            && ((*pCD).clientState == NORMAL_STATE || (*pCD).clientState == MAXIMIZED_STATE)
        {
            WmInstallColormap(wmGD.pActiveSD, (*pCD).clientColormap);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleClientMessage(
    mut pCD: *mut ClientData,
    mut clientEvent: *mut XClientMessageEvent,
) {
    let mut newState = 0 as ::core::ffi::c_uint;
    if (*clientEvent).message_type == wmGD.xa_WM_CHANGE_STATE {
        if (*clientEvent).data.l[0 as ::core::ffi::c_int as usize]
            == IconicState as ::core::ffi::c_long
            && (*pCD).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0
        {
            newState = MINIMIZED_STATE as ::core::ffi::c_uint;
        } else if (*clientEvent).data.l[0 as ::core::ffi::c_int as usize]
            == NormalState as ::core::ffi::c_long
        {
            newState = NORMAL_STATE as ::core::ffi::c_uint;
        }
        if ClientInWorkspace((*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData, pCD) == 0 {
            newState |= UNSEEN_STATE as ::core::ffi::c_uint;
        }
        SetClientState(pCD, newState as ::core::ffi::c_int, GetTimestamp());
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleCShapeNotify(
    mut pCD: *mut ClientData,
    mut shapeEvent: *mut XShapeEvent,
) {
    if !pCD.is_null() {
        if (*shapeEvent).kind != ShapeBounding {
            return;
        }
        (*pCD).wShaped = (*shapeEvent).shaped as ::core::ffi::c_short;
        SetFrameShape(pCD);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetParentWindow(mut window: Window) -> Window {
    let mut root: Window = 0;
    let mut parent: Window = 0;
    let mut children = ::core::ptr::null_mut::<Window>();
    let mut nchildren: ::core::ffi::c_uint = 0;
    if XQueryTree(
        wmGD.display,
        window,
        &raw mut root,
        &raw mut parent,
        &raw mut children,
        &raw mut nchildren,
    ) != 0
    {
        if nchildren != 0 {
            XFree(children as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        }
    } else {
        parent = 0 as ::core::ffi::c_long as Window;
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn DetermineActiveScreen(mut pEvent: *mut XEvent) {
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    match (*pEvent).type_0 {
        NoExpose | GraphicsExpose => {}
        _ => {
            pSD = GetScreenForWindow((*pEvent).xany.window);
            if !pSD.is_null() {
                wmGD.pActiveSD = pSD;
                wmGD.queryScreen = False as Boolean;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetScreenForWindow(mut win: Window) -> *mut WmScreenData {
    let mut attribs = XWindowAttributes {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        depth: 0,
        visual: ::core::ptr::null::<Visual>() as *mut Visual,
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
        screen: ::core::ptr::null::<Screen>() as *mut Screen,
    };
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    if XGetWindowAttributes(wmGD.display, win, &raw mut attribs) != 0 {
        if XFindContext(
            wmGD.display,
            attribs.root as XID,
            wmGD.screenContextType,
            &raw mut pSD as *mut XPointer,
        ) == 0
        {
            if !pSD.is_null() && (*pSD).screenTopLevelW.is_null() {
                pSD = ::core::ptr::null_mut::<WmScreenData>();
            }
        }
    }
    return pSD;
}
unsafe extern "C" fn HandleRRScreenChangeNotify(mut evt: *mut XEvent) {
    let mut e = ::core::ptr::null_mut::<ClientListEntry>();
    let mut pSD = GetScreenForWindow((*evt).xany.window);
    let mut iws: ::core::ffi::c_int = 0;
    if pSD.is_null() {
        return;
    }
    XRRUpdateConfiguration(evt);
    UpdateXineramaInfo();
    iws = 0 as ::core::ffi::c_int;
    while iws < (*pSD).numWorkspaces {
        InitIconPlacement((*pSD).pWS as *mut WmWorkspaceData);
        iws += 1;
    }
    e = (*pSD).clientList;
    while !e.is_null() {
        let mut cd = (*e).pCD as *mut ClientData;
        let mut xsi = XineramaScreenInfo {
            screen_number: 0,
            x_org: 0,
            y_org: 0,
            width: 0,
            height: 0,
        };
        let mut swidth: ::core::ffi::c_int = 0;
        let mut sheight: ::core::ffi::c_int = 0;
        if (*e).type_0 == MINIMIZED_STATE {
            e = (*e).nextSibling as *mut ClientListEntry;
        } else {
            if GetXineramaScreenFromLocation((*cd).clientX, (*cd).clientY, &raw mut xsi) != 0 {
                swidth = xsi.width as ::core::ffi::c_int;
                sheight = xsi.height as ::core::ffi::c_int;
            } else {
                swidth = XDisplayWidth(wmGD.display, (*(*cd).pSD).screen);
                sheight = XDisplayHeight(wmGD.display, (*(*cd).pSD).screen);
            }
            if (*cd).fullScreen != 0 {
                ConfigureEwmhFullScreen(cd, False as Boolean);
            }
            (*cd).maxWidth =
                swidth - (*cd).clientOffset.x as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
            (*cd).maxHeight = sheight
                - ((*cd).clientOffset.x as ::core::ffi::c_int
                    + (*cd).clientOffset.y as ::core::ffi::c_int);
            if (*cd).maxWidth > (*cd).maxWidthLimit {
                (*cd).maxWidth = (*cd).maxWidthLimit;
            }
            if (*cd).maxHeight > (*cd).maxHeightLimit {
                (*cd).maxHeight = (*cd).maxHeightLimit;
            }
            (*cd).maxWidth -= ((*cd).maxWidth - (*cd).baseWidth) % (*cd).widthInc;
            (*cd).maxHeight -= ((*cd).maxHeight - (*cd).baseHeight) % (*cd).heightInc;
            PlaceFrameOnScreen(
                cd,
                &raw mut (*cd).maxX,
                &raw mut (*cd).maxY,
                (*cd).maxWidth,
                (*cd).maxHeight,
            );
            ProcessNewConfiguration(
                cd,
                (*cd).clientX,
                (*cd).clientY,
                (*cd).clientWidth as ::core::ffi::c_uint,
                (*cd).clientHeight as ::core::ffi::c_uint,
                True as Boolean,
            );
            e = (*e).nextSibling as *mut ClientListEntry;
        }
    }
}
