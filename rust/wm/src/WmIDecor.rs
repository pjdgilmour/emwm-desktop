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
use crate::abi::_ButtonSpec_WmFeedback as _ButtonSpec;
use crate::abi::_ClickData_WmFeedback as _ClickData;
use crate::abi::_ClientData_WmFeedback as _ClientData;
use crate::abi::_ClientListEntry_WmFeedback as _ClientListEntry;
use crate::abi::_CoreClassPart_WmFeedback as _CoreClassPart;
use crate::abi::_CorePart_WmFeedback as _CorePart;
use crate::abi::_IconBoxData_WmFeedback as _IconBoxData;
use crate::abi::_IconInfo_WmFeedback as _IconInfo;
use crate::abi::_IconPlacementData_WmFeedback as _IconPlacementData;
use crate::abi::_KeySpec_WmFeedback as _KeySpec;
use crate::abi::_MenuButton_WmFeedback as _MenuButton;
use crate::abi::_MenuItem_WmFeedback as _MenuItem;
use crate::abi::_MenuSpec_WmFeedback as _MenuSpec;
use crate::abi::_WidgetClassRec_WmFeedback as _WidgetClassRec;
use crate::abi::_WidgetRec_WmFeedback as _WidgetRec;
use crate::abi::_WmGlobalData_WmFeedback as _WmGlobalData;
use crate::abi::_WmScreenData_WmFeedback as _WmScreenData;
use crate::abi::_WmTimer_WmFeedback as _WmTimer;
use crate::abi::_WmWorkspaceData_WmFeedback as _WmWorkspaceData;
use crate::abi::_WsClientData_WmFeedback as _WsClientData;
use crate::abi::_WsPresenceData_WmFeedback as _WsPresenceData;
use crate::abi::_XtActionsRec_WmFeedback as _XtActionsRec;
use crate::abi::_XtCallbackRec_WmFeedback as _XtCallbackRec;
use crate::abi::_XtTMRec_WmFeedback as _XtTMRec;
use crate::abi::ArgList_WmFeedback as ArgList;
use crate::abi::ButtonSpec_WmFeedback as ButtonSpec;
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed_0;
use crate::abi::C2RustUnnamed_WmArgb as C2RustUnnamed;
use crate::abi::ClickData_WmFeedback as ClickData;
use crate::abi::ClientData_WmFeedback as ClientData;
use crate::abi::ClientListEntry_WmFeedback as ClientListEntry;
use crate::abi::CoreClassPart_WmFeedback as CoreClassPart;
use crate::abi::CorePart_WmFeedback as CorePart;
use crate::abi::IconBoxData_WmFeedback as IconBoxData;
use crate::abi::IconInfo_WmFeedback as IconInfo;
use crate::abi::IconPlacementData_WmFeedback as IconPlacementData;
use crate::abi::KeySpec_WmFeedback as KeySpec;
use crate::abi::MenuButton_WmFeedback as MenuButton;
use crate::abi::MenuItem_WmFeedback as MenuItem;
use crate::abi::MenuSpec_WmFeedback as MenuSpec;
use crate::abi::WidgetClass_WmFeedback as WidgetClass;
use crate::abi::WidgetList_WmFeedback as WidgetList;
use crate::abi::Widget_WmFeedback as Widget;
use crate::abi::WmFunction_WmFeedback as WmFunction;
use crate::abi::WmGlobalData_WmFeedback as WmGlobalData;
use crate::abi::WmScreenData_WmFeedback as WmScreenData;
use crate::abi::WmTimer_WmFeedback as WmTimer;
use crate::abi::WsClientData_WmIDecor as WsClientData;
use crate::abi::WsPresenceData_WmFeedback as WsPresenceData;
use crate::abi::XtAcceptFocusProc_WmFeedback as XtAcceptFocusProc;
use crate::abi::XtActionList_WmFeedback as XtActionList;
use crate::abi::XtActionProc_WmFeedback as XtActionProc;
use crate::abi::XtAlmostProc_WmFeedback as XtAlmostProc;
use crate::abi::XtArgsFunc_WmFeedback as XtArgsFunc;
use crate::abi::XtArgsProc_WmFeedback as XtArgsProc;
use crate::abi::XtBoundActions_WmFeedback as XtBoundActions;
use crate::abi::XtCallbackList_WmFeedback as XtCallbackList;
use crate::abi::XtCallbackProc_WmFeedback as XtCallbackProc;
use crate::abi::XtExposeProc_WmFeedback as XtExposeProc;
use crate::abi::XtGeometryHandler_WmFeedback as XtGeometryHandler;
use crate::abi::XtInitProc_WmFeedback as XtInitProc;
use crate::abi::XtRealizeProc_WmFeedback as XtRealizeProc;
use crate::abi::XtSetValuesFunc_WmFeedback as XtSetValuesFunc;
use crate::abi::XtStringProc_WmFeedback as XtStringProc;
use crate::abi::XtTMRec_WmFeedback as XtTMRec;
use crate::abi::XtWidgetClassProc_WmFeedback as XtWidgetClassProc;
use crate::abi::XtWidgetGeometry_WmFeedback as XtWidgetGeometry;
use crate::abi::XtWidgetProc_WmFeedback as XtWidgetProc;
use crate::abi::*;
extern "C" {
    fn XCreateWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: *mut Visual,
        _: ::core::ffi::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> Window;
    fn XChangeWindowAttributes(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> ::core::ffi::c_int;
    fn XClearArea(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XConfigureWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_uint,
        _: *mut XWindowChanges,
    ) -> ::core::ffi::c_int;
    fn XCopyArea(
        _: *mut Display,
        _: Drawable,
        _: Drawable,
        _: GC,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XFillRectangle(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XFillRectangles(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XRectangle,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XGetGeometry(
        _: *mut Display,
        _: Drawable,
        _: *mut Window,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XGetWindowAttributes(
        _: *mut Display,
        _: Window,
        _: *mut XWindowAttributes,
    ) -> ::core::ffi::c_int;
    fn XMapRaised(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XMoveWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XReparentWindow(
        _: *mut Display,
        _: Window,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XUnmapWindow(_: *mut Display, _: Window) -> ::core::ffi::c_int;
    fn XDeleteContext(_: *mut Display, _: XID, _: XContext) -> ::core::ffi::c_int;
    fn XSaveContext(
        _: *mut Display,
        _: XID,
        _: XContext,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn XtWindowOfObject(_: Widget) -> Window;
    fn XtGetValues(_: Widget, _: ArgList, _: Cardinal);
    static XtStrings: [::core::ffi::c_char; 0];
    fn XmStringExtent(
        fontlist: XmFontList,
        string: XmString,
        width: *mut Dimension,
        height: *mut Dimension,
    );
    static mut wmGD: WmGlobalData;
    fn XmeClearBorder(
        display: *mut Display,
        w: Window,
        x: Position,
        y: Position,
        width: Dimension,
        height: Dimension,
        shadow_thick: Dimension,
    );
    fn Warning(message: *mut ::core::ffi::c_char);
    fn AllocateRList(amt: ::core::ffi::c_uint) -> *mut RList;
    fn BevelRectangle(
        prTop: *mut RList,
        prBot: *mut RList,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        top_wid: ::core::ffi::c_uint,
        right_wid: ::core::ffi::c_uint,
        bot_wid: ::core::ffi::c_uint,
        left_wid: ::core::ffi::c_uint,
    );
    fn WmDrawXmString(
        dpy: *mut Display,
        w: Window,
        xmfontlist: XmFontList,
        xmstring: XmString,
        gc: GC,
        x: Position,
        y: Position,
        width: Dimension,
        pbox: *mut XRectangle,
        bCenter: Boolean,
    );
    fn InsertIconIntoBox(pIBD: *mut IconBoxData, pCD: *mut ClientData) -> Boolean;
    fn GetWsClientData(pWS: *mut WmWorkspaceData, pCD: *mut ClientData) -> *mut WsClientData;
    fn GetXineramaScreenFromLocation(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const CopyFromParent: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ButtonPressMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ButtonReleaseMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const EnterWindowMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const LeaveWindowMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const Button1MotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int;
pub const Button2MotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const ExposureMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 15 as ::core::ffi::c_int;
pub const SubstructureRedirectMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 20 as ::core::ffi::c_int;
pub const FocusChangeMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 21 as ::core::ffi::c_int;
pub const InputOutput: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CWBackPixmap: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const CWBackPixel: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const CWEventMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CWCursor: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 14 as ::core::ffi::c_int;
pub const CWX: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const CWY: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const CWWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const CWHeight: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const CWBorderWidth: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_FUNC_MINIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SELECT_BUTTON_MOTION_MASK: ::core::ffi::c_long = Button1MotionMask;
pub const DMANIP_BUTTON_MOTION_MASK: ::core::ffi::c_long = Button2MotionMask;
pub const ICON_INTERNAL_SHADOW_WIDTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICON_IMAGE_TOP_PAD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICON_IMAGE_BOTTOM_PAD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICON_IMAGE_LEFT_PAD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICON_IMAGE_RIGHT_PAD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WM_TOP_TITLE_PADDING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WM_BOTTOM_TITLE_PADDING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IB_MARGIN_HEIGHT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const IB_MARGIN_WIDTH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICON_REPARENTED: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int;
pub const ICON_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const CONFIRM_BOX: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 12 as ::core::ffi::c_int;
pub const CLIENT_WM_CLIENTS: ::core::ffi::c_long = ICON_BOX | CONFIRM_BOX;
pub const CMAP_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KEYBOARD_FOCUS_POINTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICON_LABEL_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const ICON_IMAGE_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const ICON_ACTIVE_LABEL_PART: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
static mut activeIconTextWidth: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
static mut activeIconTextHeight: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
static mut pActiveIconTopRects: *mut RList = ::core::ptr::null::<RList>() as *mut RList;
static mut pActiveIconBotRects: *mut RList = ::core::ptr::null::<RList>() as *mut RList;
static mut iconShrinkX: ::core::ffi::c_int = 0;
static mut iconShrinkY: ::core::ffi::c_int = 0;
static mut iconShrinkWidth: ::core::ffi::c_uint = 0;
static mut iconShrinkHeight: ::core::ffi::c_uint = 0;
#[no_mangle]
pub unsafe extern "C" fn MakeIcon(
    mut pWS: *mut WmWorkspaceData,
    mut pcd: *mut ClientData,
) -> Boolean {
    let mut window_attribs = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    let mut attr_mask: ::core::ffi::c_ulong = 0;
    let mut xOffset: ::core::ffi::c_int = 0;
    let mut yOffset: ::core::ffi::c_int = 0;
    let mut pWsc = GetWsClientData(pWS, pcd);
    attr_mask = (CWEventMask | CWCursor) as ::core::ffi::c_ulong;
    window_attribs.event_mask = ButtonPressMask
        | ButtonReleaseMask
        | SELECT_BUTTON_MOTION_MASK
        | DMANIP_BUTTON_MOTION_MASK
        | ExposureMask
        | SubstructureRedirectMask
        | FocusChangeMask;
    window_attribs.cursor = wmGD.workspaceCursor;
    if wmGD.keyboardFocusPolicy == KEYBOARD_FOCUS_POINTER
        || wmGD.colormapFocusPolicy == CMAP_FOCUS_POINTER
    {
        window_attribs.event_mask |= EnterWindowMask | LeaveWindowMask;
    }
    if (*(*pcd).pSD).iconAppearance.backgroundPixmap != 0 {
        attr_mask |= CWBackPixmap as ::core::ffi::c_ulong;
        window_attribs.background_pixmap = (*(*pcd).pSD).iconAppearance.backgroundPixmap;
    } else {
        attr_mask |= CWBackPixel as ::core::ffi::c_ulong;
        window_attribs.background_pixel =
            (*(*pcd).pSD).iconAppearance.background as ::core::ffi::c_ulong;
    }
    if (*(*pcd).pSD).useIconBox == 0 || (*pcd).clientFlags & (ICON_BOX | CONFIRM_BOX) != 0 {
        (*pWsc).iconFrameWin = XCreateWindow(
            wmGD.display,
            (*(*pcd).pSD).rootWindow,
            (*pWsc).iconX,
            (*pWsc).iconY,
            (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint,
            (*(*pcd).pSD).iconHeight as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            CopyFromParent as ::core::ffi::c_int,
            InputOutput as ::core::ffi::c_uint,
            ::core::ptr::null_mut::<Visual>(),
            attr_mask,
            &raw mut window_attribs,
        );
    } else if (*(*pcd).pSD).useIconBox as ::core::ffi::c_int != 0
        && (*pcd).clientFlags & CLIENT_WM_CLIENTS == 0
        && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_MINIMIZE != 0
    {
        if InsertIconIntoBox((*pWS).pIconBox, pcd) == 0 {
            Warning(
                b"Could not make icon to go in icon box\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    (*pcd).piconTopShadows = AllocateRList(
        (3 as ::core::ffi::c_int * wmGD.iconExternalShadowWidth as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH) as ::core::ffi::c_uint,
    );
    if (*pcd).piconTopShadows.is_null() {
        Warning(
            b"Insufficient memory for icon creation\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as Boolean;
    }
    (*pcd).piconBottomShadows = AllocateRList(
        (3 as ::core::ffi::c_int * wmGD.iconExternalShadowWidth as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH) as ::core::ffi::c_uint,
    );
    if (*pcd).piconBottomShadows.is_null() {
        Warning(
            b"Insufficient memory for icon creation\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as Boolean;
    }
    if !(*pWS).pIconBox.is_null() && (*(*pWS).pIconBox).pCD_iconBox != pcd {
        xOffset = IB_MARGIN_WIDTH;
        yOffset = IB_MARGIN_HEIGHT;
    } else {
        xOffset = 0 as ::core::ffi::c_int;
        yOffset = 0 as ::core::ffi::c_int;
    }
    if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_IMAGE_PART != 0
        && (*pcd).iconWindow != 0
    {
        ReparentIconWindow(pcd, xOffset, yOffset);
    }
    if (*(*pcd).piconTopShadows).used == 0 as ::core::ffi::c_int {
        MakeIconShadows(pcd, xOffset, yOffset);
    }
    return TRUE as Boolean;
}
unsafe extern "C" fn MakeIconShadows(
    mut pcd: *mut ClientData,
    mut xOffset: ::core::ffi::c_int,
    mut yOffset: ::core::ffi::c_int,
) {
    match (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & (ICON_LABEL_PART | ICON_IMAGE_PART)
    {
        ICON_LABEL_PART => {
            BevelRectangle(
                (*pcd).piconTopShadows,
                (*pcd).piconBottomShadows,
                0 as ::core::ffi::c_int + xOffset,
                (*(*pcd).pSD).iconImageHeight + yOffset,
                (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint,
                (*(*pcd).pSD).iconLabelHeight as ::core::ffi::c_uint,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
            );
        }
        ICON_IMAGE_PART => {
            BevelRectangle(
                (*pcd).piconTopShadows,
                (*pcd).piconBottomShadows,
                0 as ::core::ffi::c_int + xOffset,
                0 as ::core::ffi::c_int + yOffset,
                (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint,
                (*(*pcd).pSD).iconImageHeight as ::core::ffi::c_uint,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
            );
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                BevelRectangle(
                    (*pcd).piconBottomShadows,
                    (*pcd).piconTopShadows,
                    ICON_IMAGE_LEFT_PAD
                        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                        + xOffset,
                    ICON_IMAGE_TOP_PAD
                        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                        + yOffset,
                    ((*(*pcd).pSD).iconImageMaximum.width
                        + 4 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH)
                        as ::core::ffi::c_uint,
                    ((*(*pcd).pSD).iconImageMaximum.height
                        + 4 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH)
                        as ::core::ffi::c_uint,
                    ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
                    ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
                    ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
                    ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
                );
            }
        }
        3 => {
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                BevelRectangle(
                    (*pcd).piconTopShadows,
                    (*pcd).piconBottomShadows,
                    0 as ::core::ffi::c_int + xOffset,
                    0 as ::core::ffi::c_int + yOffset,
                    (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint,
                    ((*(*pcd).pSD).iconImageHeight + (*(*pcd).pSD).iconLabelHeight)
                        as ::core::ffi::c_uint,
                    wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                    wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                    wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                    wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                );
            } else {
                BevelRectangle(
                    (*pcd).piconTopShadows,
                    (*pcd).piconBottomShadows,
                    0 as ::core::ffi::c_int + xOffset,
                    0 as ::core::ffi::c_int + yOffset,
                    (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint,
                    (*(*pcd).pSD).iconImageHeight as ::core::ffi::c_uint,
                    wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                    wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                    ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
                    wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                );
            }
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                BevelRectangle(
                    (*pcd).piconBottomShadows,
                    (*pcd).piconTopShadows,
                    ICON_IMAGE_LEFT_PAD
                        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                        + xOffset,
                    ICON_IMAGE_TOP_PAD
                        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                        + yOffset,
                    ((*(*pcd).pSD).iconImageMaximum.width
                        + 4 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH)
                        as ::core::ffi::c_uint,
                    ((*(*pcd).pSD).iconImageMaximum.height
                        + 4 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH)
                        as ::core::ffi::c_uint,
                    ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
                    ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
                    ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
                    ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
                );
            }
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                BevelRectangle(
                    (*pcd).piconTopShadows,
                    (*pcd).piconBottomShadows,
                    0 as ::core::ffi::c_int + xOffset,
                    (*(*pcd).pSD).iconImageHeight + yOffset,
                    (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint,
                    (*(*pcd).pSD).iconLabelHeight as ::core::ffi::c_uint,
                    ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint,
                    wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                    wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                    wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn IconExposureProc(mut pcd: *mut ClientData, mut expose: Boolean) {
    let mut image: Pixmap = 0;
    let mut dest_x: ::core::ffi::c_int = 0;
    let mut dest_y: ::core::ffi::c_int = 0;
    let mut xOffset: ::core::ffi::c_int = 0;
    let mut yOffset: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut iconGC = ::core::ptr::null_mut::<_XGC>();
    let mut topGC = ::core::ptr::null_mut::<_XGC>();
    let mut botGC = ::core::ptr::null_mut::<_XGC>();
    static mut shrinkRects: [XRectangle; 4] = [XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }; 4];
    if !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
        .pIconBox
        .is_null()
    {
        xOffset = IB_MARGIN_WIDTH;
        yOffset = IB_MARGIN_HEIGHT;
    } else {
        xOffset = 0 as ::core::ffi::c_int;
        yOffset = 0 as ::core::ffi::c_int;
    }
    if wmGD.keyboardFocus == pcd && (*pcd).clientState == MINIMIZED_STATE {
        iconGC = (*(*pcd).pSD).iconAppearance.activeGC;
        topGC = (*(*pcd).pSD).iconAppearance.activeTopShadowGC;
        botGC = (*(*pcd).pSD).iconAppearance.activeBottomShadowGC;
    } else {
        iconGC = (*(*pcd).pSD).iconAppearance.inactiveGC;
        topGC = (*(*pcd).pSD).iconAppearance.inactiveTopShadowGC;
        botGC = (*(*pcd).pSD).iconAppearance.inactiveBottomShadowGC;
    }
    if (*wmGD.pActiveSD).useIconBox as ::core::ffi::c_int != 0
        && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
            .pIconBox
            .is_null()
    {
        if expose != 0 {
            XClearArea(
                wmGD.display,
                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                IB_MARGIN_WIDTH,
                IB_MARGIN_HEIGHT,
                (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint,
                (*(*pcd).pSD).iconHeight as ::core::ffi::c_uint,
                False,
            );
        }
        if (*pcd).clientState == MINIMIZED_STATE {
            if !(*pcd).piconTopShadows.is_null() {
                XFillRectangles(
                    wmGD.display,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin as Drawable,
                    topGC,
                    (*(*pcd).piconTopShadows).prect,
                    (*(*pcd).piconTopShadows).used,
                );
            }
            if !(*pcd).piconBottomShadows.is_null() {
                XFillRectangles(
                    wmGD.display,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin as Drawable,
                    botGC,
                    (*(*pcd).piconBottomShadows).prect,
                    (*(*pcd).piconBottomShadows).used,
                );
            }
        } else {
            shrinkRects[0 as ::core::ffi::c_int as usize].x =
                IB_MARGIN_WIDTH as ::core::ffi::c_short;
            shrinkRects[0 as ::core::ffi::c_int as usize].y =
                IB_MARGIN_HEIGHT as ::core::ffi::c_short;
            shrinkRects[0 as ::core::ffi::c_int as usize].width =
                (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint as ::core::ffi::c_ushort;
            shrinkRects[0 as ::core::ffi::c_int as usize].height =
                (iconShrinkY - IB_MARGIN_HEIGHT) as ::core::ffi::c_ushort;
            shrinkRects[1 as ::core::ffi::c_int as usize].x =
                IB_MARGIN_WIDTH as ::core::ffi::c_short;
            shrinkRects[1 as ::core::ffi::c_int as usize].y = iconShrinkY as ::core::ffi::c_short;
            shrinkRects[1 as ::core::ffi::c_int as usize].width =
                (iconShrinkX - IB_MARGIN_WIDTH) as ::core::ffi::c_ushort;
            shrinkRects[1 as ::core::ffi::c_int as usize].height =
                iconShrinkHeight as ::core::ffi::c_ushort;
            shrinkRects[2 as ::core::ffi::c_int as usize].x = (iconShrinkX as ::core::ffi::c_uint)
                .wrapping_add(iconShrinkWidth)
                as ::core::ffi::c_short;
            shrinkRects[2 as ::core::ffi::c_int as usize].y = iconShrinkY as ::core::ffi::c_short;
            shrinkRects[2 as ::core::ffi::c_int as usize].width =
                (iconShrinkX - IB_MARGIN_WIDTH) as ::core::ffi::c_ushort;
            shrinkRects[2 as ::core::ffi::c_int as usize].height =
                iconShrinkHeight as ::core::ffi::c_ushort;
            shrinkRects[3 as ::core::ffi::c_int as usize].x =
                IB_MARGIN_WIDTH as ::core::ffi::c_short;
            shrinkRects[3 as ::core::ffi::c_int as usize].y = (iconShrinkY as ::core::ffi::c_uint)
                .wrapping_add(iconShrinkHeight)
                as ::core::ffi::c_short;
            shrinkRects[3 as ::core::ffi::c_int as usize].width =
                (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint as ::core::ffi::c_ushort;
            shrinkRects[3 as ::core::ffi::c_int as usize].height =
                (iconShrinkY - IB_MARGIN_HEIGHT) as ::core::ffi::c_ushort;
            XFillRectangles(
                wmGD.display,
                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin as Drawable,
                (*(*pcd).pSD).shrinkWrapGC,
                (&raw mut shrinkRects as *mut XRectangle).offset(0 as ::core::ffi::c_int as isize)
                    as *mut XRectangle,
                4 as ::core::ffi::c_int,
            );
        }
    } else if (*pcd).clientState == MINIMIZED_STATE {
        if !(*(*pcd).piconTopShadows).prect.is_null() {
            XFillRectangles(
                wmGD.display,
                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin as Drawable,
                topGC,
                (*(*pcd).piconTopShadows).prect,
                (*(*pcd).piconTopShadows).used,
            );
        }
        if !(*(*pcd).piconBottomShadows).prect.is_null() {
            XFillRectangles(
                wmGD.display,
                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin as Drawable,
                botGC,
                (*(*pcd).piconBottomShadows).prect,
                (*(*pcd).piconBottomShadows).used,
            );
        }
    }
    if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_LABEL_PART != 0 {
        DrawIconTitle(pcd);
    }
    if expose as ::core::ffi::c_int != 0
        && ((*pcd).iconWindow == 0
            && (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_IMAGE_PART != 0
            || (*wmGD.pActiveSD).useIconBox as ::core::ffi::c_int != 0
                && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                    .pIconBox
                    .is_null()
                && (*pcd).iconWindow != 0
                && (*pcd).clientState != MINIMIZED_STATE
                && (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_IMAGE_PART != 0)
    {
        if (*pcd).iconWindow != 0 {
            image = (*(*pcd).pSD).defaultPixmap;
        } else {
            image = if (*pcd).ewmhIconPixmap != 0 {
                (*pcd).ewmhIconPixmap
            } else {
                (*pcd).iconPixmap
            };
        }
        if image != 0 {
            if (*wmGD.pActiveSD).useIconBox as ::core::ffi::c_int != 0
                && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                    .pIconBox
                    .is_null()
            {
                if (*pcd).clientState != MINIMIZED_STATE {
                    dest_x = ICON_IMAGE_LEFT_PAD
                        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                        + ICON_INTERNAL_SHADOW_WIDTH
                        + ICON_INTERNAL_SHADOW_WIDTH
                        + xOffset;
                    dest_y = ICON_IMAGE_TOP_PAD
                        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                        + ICON_INTERNAL_SHADOW_WIDTH
                        + ICON_INTERNAL_SHADOW_WIDTH
                        + yOffset;
                    if wmGD.frameStyle as ::core::ffi::c_uint
                        == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        dest_x -= ICON_INTERNAL_SHADOW_WIDTH;
                        dest_y -= ICON_INTERNAL_SHADOW_WIDTH;
                    }
                    width = (*(*pcd).pSD).iconImageMaximum.width as ::core::ffi::c_uint;
                    height = (*(*pcd).pSD).iconImageMaximum.height as ::core::ffi::c_uint;
                    if wmGD.frameStyle as ::core::ffi::c_uint
                        == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        width = width.wrapping_add(2 as ::core::ffi::c_uint);
                        height = height.wrapping_add(2 as ::core::ffi::c_uint);
                    }
                    XCopyArea(
                        wmGD.display,
                        image as Drawable,
                        (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin
                            as Drawable,
                        iconGC,
                        ICON_INTERNAL_SHADOW_WIDTH,
                        ICON_INTERNAL_SHADOW_WIDTH,
                        width,
                        height,
                        dest_x,
                        dest_y,
                    );
                    if (*(*pcd).pSD).fadeNormalIcon != 0 {
                        iconGC = (*(*pcd).pSD).fadeIconGC;
                        XFillRectangle(
                            wmGD.display,
                            (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin
                                as Drawable,
                            iconGC,
                            dest_x,
                            dest_y,
                            width,
                            height,
                        );
                    }
                } else {
                    dest_x = ICON_IMAGE_LEFT_PAD
                        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                        + ICON_INTERNAL_SHADOW_WIDTH
                        + xOffset;
                    dest_y = ICON_IMAGE_TOP_PAD
                        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                        + ICON_INTERNAL_SHADOW_WIDTH
                        + yOffset;
                    if wmGD.frameStyle as ::core::ffi::c_uint
                        == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        dest_x -= ICON_INTERNAL_SHADOW_WIDTH;
                        dest_y -= ICON_INTERNAL_SHADOW_WIDTH;
                    }
                    width = ((*(*pcd).pSD).iconImageMaximum.width
                        + 2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH)
                        as ::core::ffi::c_uint;
                    height = ((*(*pcd).pSD).iconImageMaximum.height
                        + 2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH)
                        as ::core::ffi::c_uint;
                    if wmGD.frameStyle as ::core::ffi::c_uint
                        == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        width = width.wrapping_add(2 as ::core::ffi::c_uint);
                        height = height.wrapping_add(2 as ::core::ffi::c_uint);
                    }
                    XCopyArea(
                        wmGD.display,
                        image as Drawable,
                        (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin
                            as Drawable,
                        iconGC,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        width,
                        height,
                        dest_x,
                        dest_y,
                    );
                }
            } else {
                width = ((*(*pcd).pSD).iconImageMaximum.width
                    + 2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH)
                    as ::core::ffi::c_uint;
                height = ((*(*pcd).pSD).iconImageMaximum.height
                    + 2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH)
                    as ::core::ffi::c_uint;
                if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    dest_x =
                        ICON_IMAGE_LEFT_PAD + wmGD.iconExternalShadowWidth as ::core::ffi::c_int;
                    dest_y =
                        ICON_IMAGE_TOP_PAD + wmGD.iconExternalShadowWidth as ::core::ffi::c_int;
                    width = width.wrapping_add(2 as ::core::ffi::c_uint);
                    height = height.wrapping_add(2 as ::core::ffi::c_uint);
                } else {
                    dest_x = ICON_IMAGE_LEFT_PAD
                        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                        + ICON_INTERNAL_SHADOW_WIDTH;
                    dest_y = ICON_IMAGE_TOP_PAD
                        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                        + ICON_INTERNAL_SHADOW_WIDTH;
                }
                XCopyArea(
                    wmGD.display,
                    image as Drawable,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin as Drawable,
                    iconGC,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    width,
                    height,
                    dest_x,
                    dest_y,
                );
            }
        }
    }
}
unsafe extern "C" fn GetIconTitleBox(mut pcd: *mut ClientData, mut pBox: *mut XRectangle) {
    let mut xOffset: ::core::ffi::c_int = 0;
    let mut yOffset: ::core::ffi::c_int = 0;
    if !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
        .pIconBox
        .is_null()
    {
        xOffset = IB_MARGIN_WIDTH;
        yOffset = IB_MARGIN_HEIGHT;
    } else {
        xOffset = 0 as ::core::ffi::c_int;
        yOffset = 0 as ::core::ffi::c_int;
    }
    if !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
        .pIconBox
        .is_null()
        && (*pcd).clientState != MINIMIZED_STATE
    {
        (*pBox).x = (wmGD.iconExternalShadowWidth as ::core::ffi::c_int
            + ICON_IMAGE_LEFT_PAD
            + 2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH
            + ICON_IMAGE_LEFT_PAD
            + xOffset) as ::core::ffi::c_short;
        (*pBox).y = ((*(*pcd).pSD).iconImageHeight
            + yOffset
            + (if (*(*pcd).pSD).iconImageHeight > 0 as ::core::ffi::c_int {
                -ICON_IMAGE_BOTTOM_PAD
            } else {
                wmGD.iconExternalShadowWidth as ::core::ffi::c_int
            })
            + (if (*(*pcd).pSD).iconImageHeight > 0 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else {
                WM_TOP_TITLE_PADDING
            })) as ::core::ffi::c_short;
        if wmGD.frameStyle as ::core::ffi::c_uint
            == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*pBox).x = ((*pBox).x as ::core::ffi::c_int - ICON_INTERNAL_SHADOW_WIDTH)
                as ::core::ffi::c_short;
            (*pBox).y = ((*pBox).y as ::core::ffi::c_int - ICON_INTERNAL_SHADOW_WIDTH)
                as ::core::ffi::c_short;
        }
        (*pBox).width = ((*(*pcd).pSD).iconImageMaximum.width
            + (if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })
            - ICON_IMAGE_LEFT_PAD
            - wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        (*pBox).height = (*(*pcd).pSD).iconAppearance.fontHeight as ::core::ffi::c_ushort;
    } else if !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
        .pIconBox
        .is_null()
        && (*pcd).clientState == MINIMIZED_STATE
    {
        (*pBox).x = (wmGD.iconExternalShadowWidth as ::core::ffi::c_int
            + ICON_IMAGE_LEFT_PAD
            + 2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH
            + ICON_IMAGE_LEFT_PAD
            + xOffset) as ::core::ffi::c_short;
        (*pBox).y = ((*(*pcd).pSD).iconImageHeight
            + yOffset
            + (if (*(*pcd).pSD).iconImageHeight > 0 as ::core::ffi::c_int {
                ICON_INTERNAL_SHADOW_WIDTH
            } else {
                wmGD.iconExternalShadowWidth as ::core::ffi::c_int
            })
            + WM_TOP_TITLE_PADDING) as ::core::ffi::c_short;
        (*pBox).width = ((*(*pcd).pSD).iconImageMaximum.width
            + (if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })
            - ICON_IMAGE_LEFT_PAD) as ::core::ffi::c_ushort;
        (*pBox).height = (*(*pcd).pSD).iconAppearance.fontHeight as ::core::ffi::c_ushort;
        if wmGD.frameStyle as ::core::ffi::c_uint
            == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*pBox).x = ((*pBox).x as ::core::ffi::c_int
                - (ICON_INTERNAL_SHADOW_WIDTH + 2 as ::core::ffi::c_int * ICON_IMAGE_LEFT_PAD))
                as ::core::ffi::c_short;
            (*pBox).y = ((*pBox).y as ::core::ffi::c_int
                - 3 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH)
                as ::core::ffi::c_short;
            (*pBox).width = ((*pBox).width as ::core::ffi::c_int
                + (ICON_IMAGE_LEFT_PAD + 2 as ::core::ffi::c_int))
                as ::core::ffi::c_ushort;
        }
    } else {
        (*pBox).x = (wmGD.iconExternalShadowWidth as ::core::ffi::c_int
            + WM_TOP_TITLE_PADDING
            + xOffset) as ::core::ffi::c_short;
        if wmGD.frameStyle as ::core::ffi::c_uint
            == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            yOffset -= ICON_INTERNAL_SHADOW_WIDTH;
        }
        (*pBox).y = ((*(*pcd).pSD).iconImageHeight
            + WM_TOP_TITLE_PADDING
            + yOffset
            + (if (*(*pcd).pSD).iconImageHeight > 0 as ::core::ffi::c_int {
                ICON_INTERNAL_SHADOW_WIDTH
            } else {
                wmGD.iconExternalShadowWidth as ::core::ffi::c_int
            })) as ::core::ffi::c_short;
        (*pBox).width = ((*(*pcd).pSD).iconWidth
            - 2 as ::core::ffi::c_int * wmGD.iconExternalShadowWidth as ::core::ffi::c_int
            - WM_TOP_TITLE_PADDING
            - WM_BOTTOM_TITLE_PADDING) as ::core::ffi::c_ushort;
        (*pBox).height = (*(*pcd).pSD).iconAppearance.fontHeight as ::core::ffi::c_ushort;
    };
}
unsafe extern "C" fn DrawIconTitle(mut pcd: *mut ClientData) {
    let mut textBox = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut iconGC = ::core::ptr::null_mut::<_XGC>();
    GetIconTitleBox(pcd, &raw mut textBox);
    if (*wmGD.pActiveSD).useIconBox as ::core::ffi::c_int != 0
        && (*pcd).clientFlags & CLIENT_WM_CLIENTS == 0
        || !(wmGD.keyboardFocus == pcd)
    {
        iconGC = (*(*pcd).pSD).iconAppearance.inactiveGC;
    } else {
        iconGC = (*(*pcd).pSD).iconAppearance.activeGC;
    }
    if (*wmGD.pActiveSD).useIconBox as ::core::ffi::c_int != 0
        && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
            .pIconBox
            .is_null()
        && (*(*pcd).pSD).fadeNormalIcon as ::core::ffi::c_int != 0
        && !((*pcd).clientState == MINIMIZED_STATE)
    {
        iconGC = (*(*pcd).pSD).fadeIconTextGC;
    }
    WmDrawXmString(
        wmGD.display,
        (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
        (*(*pcd).pSD).iconAppearance.renderTable as XmFontList,
        if !(*pcd).ewmhIconTitle.is_null() {
            (*pcd).ewmhIconTitle
        } else {
            (*pcd).iconTitle
        },
        iconGC,
        textBox.x as Position,
        textBox.y as Position,
        textBox.width as Dimension,
        &raw mut textBox,
        True as Boolean,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RedisplayIconTitle(mut pcd: *mut ClientData) {
    let mut textBox = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut iconGC = ::core::ptr::null_mut::<_XGC>();
    if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_LABEL_PART != 0
        && (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin != 0
    {
        if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_LABEL_PART == 0 {
            return;
        }
        GetIconTitleBox(pcd, &raw mut textBox);
        if (*wmGD.pActiveSD).useIconBox as ::core::ffi::c_int != 0
            && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                .pIconBox
                .is_null()
            && (*pcd).clientFlags & CLIENT_WM_CLIENTS == 0
            || !(wmGD.keyboardFocus == pcd)
        {
            iconGC = (*(*pcd).pSD).iconAppearance.inactiveGC;
        } else {
            iconGC = (*(*pcd).pSD).iconAppearance.activeGC;
        }
        if (*wmGD.pActiveSD).useIconBox as ::core::ffi::c_int != 0
            && !(*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                .pIconBox
                .is_null()
            && (*(*pcd).pSD).fadeNormalIcon as ::core::ffi::c_int != 0
            && !((*pcd).clientState == MINIMIZED_STATE)
        {
            iconGC = (*(*pcd).pSD).fadeIconTextGC;
        }
        XClearArea(
            wmGD.display,
            (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
            textBox.x as ::core::ffi::c_int,
            textBox.y as ::core::ffi::c_int,
            textBox.width as ::core::ffi::c_uint,
            textBox.height as ::core::ffi::c_uint,
            FALSE,
        );
        WmDrawXmString(
            wmGD.display,
            (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
            (*(*pcd).pSD).iconAppearance.renderTable as XmFontList,
            if !(*pcd).ewmhIconTitle.is_null() {
                (*pcd).ewmhIconTitle
            } else {
                (*pcd).iconTitle
            },
            iconGC,
            textBox.x as Position,
            textBox.y as Position,
            textBox.width as Dimension,
            &raw mut textBox,
            True as Boolean,
        );
        if wmGD.keyboardFocus == pcd
            && (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART != 0
        {
            PaintActiveIconText(pcd, True as Boolean);
        }
    }
}
unsafe extern "C" fn GetIconDimensions(
    mut pSD: *mut WmScreenData,
    mut pWidth: *mut ::core::ffi::c_uint,
    mut pLabelHeight: *mut ::core::ffi::c_uint,
    mut pImageHeight: *mut ::core::ffi::c_uint,
) {
    *pWidth = ((*pSD).iconImageMaximum.width
        + (if wmGD.frameStyle as ::core::ffi::c_uint
            == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })
        + ICON_IMAGE_LEFT_PAD
        + ICON_IMAGE_RIGHT_PAD
        + 2 as ::core::ffi::c_int * wmGD.iconExternalShadowWidth as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH)
        as ::core::ffi::c_uint;
    if wmGD.frameStyle as ::core::ffi::c_uint == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *pWidth = (*pWidth).wrapping_sub(
            (2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH) as ::core::ffi::c_uint,
        );
    }
    match (*pSD).iconDecoration as ::core::ffi::c_long & (ICON_IMAGE_PART | ICON_LABEL_PART) {
        ICON_LABEL_PART => {
            *pImageHeight = 0 as ::core::ffi::c_uint;
            *pLabelHeight = ((wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                + WM_TOP_TITLE_PADDING) as ::core::ffi::c_uint)
                .wrapping_add((*pSD).iconAppearance.fontHeight)
                .wrapping_add(WM_BOTTOM_TITLE_PADDING as ::core::ffi::c_uint)
                .wrapping_add(wmGD.iconExternalShadowWidth as ::core::ffi::c_uint);
        }
        ICON_IMAGE_PART => {
            *pImageHeight = (wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                + ICON_IMAGE_TOP_PAD
                + ICON_INTERNAL_SHADOW_WIDTH
                + ICON_INTERNAL_SHADOW_WIDTH
                + (*pSD).iconImageMaximum.height
                + (if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })
                + ICON_INTERNAL_SHADOW_WIDTH
                + ICON_INTERNAL_SHADOW_WIDTH
                + ICON_IMAGE_BOTTOM_PAD
                + wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                *pImageHeight = (*pImageHeight).wrapping_sub(
                    (2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH) as ::core::ffi::c_uint,
                );
            }
            *pLabelHeight = 0 as ::core::ffi::c_uint;
        }
        3 => {
            *pImageHeight = (wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                + ICON_IMAGE_TOP_PAD
                + ICON_INTERNAL_SHADOW_WIDTH
                + ICON_INTERNAL_SHADOW_WIDTH
                + (*pSD).iconImageMaximum.height
                + (if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })
                + ICON_INTERNAL_SHADOW_WIDTH
                + ICON_INTERNAL_SHADOW_WIDTH
                + ICON_IMAGE_BOTTOM_PAD
                + ICON_INTERNAL_SHADOW_WIDTH) as ::core::ffi::c_uint;
            *pLabelHeight = ((ICON_INTERNAL_SHADOW_WIDTH + WM_TOP_TITLE_PADDING)
                as ::core::ffi::c_uint)
                .wrapping_add((*pSD).iconAppearance.fontHeight)
                .wrapping_add(WM_BOTTOM_TITLE_PADDING as ::core::ffi::c_uint)
                .wrapping_add(wmGD.iconExternalShadowWidth as ::core::ffi::c_uint);
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                *pImageHeight = (*pImageHeight).wrapping_sub(
                    (3 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH) as ::core::ffi::c_uint,
                );
                *pLabelHeight =
                    (*pLabelHeight).wrapping_sub(ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint);
            }
        }
        _ => {
            *pImageHeight = 0 as ::core::ffi::c_uint;
            *pLabelHeight = *pImageHeight;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn InitIconSize(mut pSD: *mut WmScreenData) {
    let mut label: Cardinal = 0;
    let mut image: Cardinal = 0;
    GetIconDimensions(
        pSD,
        &raw mut (*pSD).iconWidth as *mut ::core::ffi::c_uint,
        &raw mut label,
        &raw mut image,
    );
    (*pSD).iconHeight = label.wrapping_add(image) as ::core::ffi::c_int;
    (*pSD).iconImageHeight = image as ::core::ffi::c_int;
    (*pSD).iconLabelHeight = label as ::core::ffi::c_int;
    iconShrinkX = IB_MARGIN_WIDTH
        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
        + ICON_IMAGE_LEFT_PAD
        + 2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH;
    iconShrinkY = IB_MARGIN_HEIGHT
        + wmGD.iconExternalShadowWidth as ::core::ffi::c_int
        + (if (*pSD).iconDecoration as ::core::ffi::c_long & ICON_IMAGE_PART != 0 {
            ICON_IMAGE_TOP_PAD + 2 as ::core::ffi::c_int * ICON_INTERNAL_SHADOW_WIDTH
        } else {
            1 as ::core::ffi::c_int
        });
    if wmGD.frameStyle as ::core::ffi::c_uint == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        iconShrinkX -= ICON_INTERNAL_SHADOW_WIDTH;
        iconShrinkY -= ICON_INTERNAL_SHADOW_WIDTH;
    }
    iconShrinkWidth = (*pSD).iconImageMaximum.width as ::core::ffi::c_uint;
    if wmGD.frameStyle as ::core::ffi::c_uint == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        iconShrinkWidth = iconShrinkWidth.wrapping_add(2 as ::core::ffi::c_uint);
    }
    match (*pSD).iconDecoration as ::core::ffi::c_long & (ICON_IMAGE_PART | ICON_LABEL_PART) {
        ICON_LABEL_PART => {
            iconShrinkHeight = (*pSD).iconAppearance.fontHeight;
        }
        ICON_IMAGE_PART => {
            iconShrinkHeight = (*pSD).iconImageMaximum.height as ::core::ffi::c_uint;
        }
        3 => {
            iconShrinkHeight = ((*pSD).iconHeight
                - wmGD.iconExternalShadowWidth as ::core::ffi::c_int
                - ICON_IMAGE_TOP_PAD
                - ICON_INTERNAL_SHADOW_WIDTH
                - ICON_INTERNAL_SHADOW_WIDTH
                - ICON_IMAGE_BOTTOM_PAD
                - WM_BOTTOM_TITLE_PADDING
                - wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                iconShrinkHeight = iconShrinkHeight
                    .wrapping_add(ICON_INTERNAL_SHADOW_WIDTH as ::core::ffi::c_uint);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn ShowActiveIcon(mut pcd: *mut ClientData) {
    let mut attr_mask: ::core::ffi::c_ulong = 0;
    let mut window_attribs = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    if (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin != 0 {
        if (*(*pcd).pSD).iconAppearance.activeBackgroundPixmap != 0 {
            attr_mask = CWBackPixmap as ::core::ffi::c_ulong;
            window_attribs.background_pixmap = (*(*pcd).pSD).iconAppearance.activeBackgroundPixmap;
        } else {
            attr_mask = CWBackPixel as ::core::ffi::c_ulong;
            window_attribs.background_pixel =
                (*(*pcd).pSD).iconAppearance.activeBackground as ::core::ffi::c_ulong;
        }
        XChangeWindowAttributes(
            wmGD.display,
            (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
            attr_mask,
            &raw mut window_attribs,
        );
        if (*wmGD.pActiveSD).useIconBox == 0
            || (*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                .pIconBox
                .is_null()
        {
            if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_IMAGE_PART != 0 {
                let mut dheight: Dimension = 0;
                let mut dwidth: Dimension = 0;
                dwidth = ((*(*pcd).pSD).iconWidth
                    - 2 as ::core::ffi::c_int * wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
                    as Dimension;
                if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_LABEL_PART != 0 {
                    dheight = ((*(*pcd).pSD).iconImageHeight
                        - wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
                        as Dimension;
                } else {
                    dheight = ((*(*pcd).pSD).iconImageHeight
                        - 2 as ::core::ffi::c_int
                            * wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
                        as Dimension;
                }
                if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    dheight =
                        (dheight as ::core::ffi::c_int - ICON_INTERNAL_SHADOW_WIDTH) as Dimension;
                }
                XmeClearBorder(
                    wmGD.display,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                    wmGD.iconExternalShadowWidth as Position,
                    wmGD.iconExternalShadowWidth as Position,
                    dwidth,
                    dheight,
                    ICON_IMAGE_TOP_PAD as Dimension,
                );
            }
            if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_LABEL_PART != 0 {
                XClearArea(
                    wmGD.display,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                    0 as ::core::ffi::c_int,
                    (*(*pcd).pSD).iconImageHeight,
                    (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint,
                    (*(*pcd).pSD).iconHeight as ::core::ffi::c_uint,
                    False,
                );
            }
        } else {
            XClearArea(
                wmGD.display,
                (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                IB_MARGIN_WIDTH,
                IB_MARGIN_HEIGHT,
                (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint,
                (*(*pcd).pSD).iconHeight as ::core::ffi::c_uint,
                False,
            );
        }
        if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART != 0 {
            if wmGD.activeIconTextDisplayed != 0 {
                PaintActiveIconText(pcd, True as Boolean);
            } else {
                ShowActiveIconText(pcd);
            }
        }
        IconExposureProc(pcd, False as Boolean);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ShowInactiveIcon(mut pcd: *mut ClientData, mut refresh: Boolean) {
    let mut attr_mask = 0 as ::core::ffi::c_ulong;
    let mut window_attribs = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART != 0 {
        HideActiveIconText((*pcd).pSD);
    }
    if (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin != 0 {
        if (*(*pcd).pSD).iconAppearance.backgroundPixmap != 0 {
            attr_mask |= CWBackPixmap as ::core::ffi::c_ulong;
            window_attribs.background_pixmap = (*(*pcd).pSD).iconAppearance.backgroundPixmap;
        } else {
            attr_mask |= CWBackPixel as ::core::ffi::c_ulong;
            window_attribs.background_pixel =
                (*(*pcd).pSD).iconAppearance.background as ::core::ffi::c_ulong;
        }
        XChangeWindowAttributes(
            wmGD.display,
            (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
            attr_mask,
            &raw mut window_attribs,
        );
        if refresh != 0 {
            if (*wmGD.pActiveSD).useIconBox == 0
                || (*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
                    .pIconBox
                    .is_null()
            {
                XmeClearBorder(
                    wmGD.display,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                    0 as Position,
                    0 as Position,
                    (*(*pcd).pSD).iconWidth as Dimension,
                    (*(*pcd).pSD).iconImageHeight as Dimension,
                    4 as Dimension,
                );
                XClearArea(
                    wmGD.display,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                    0 as ::core::ffi::c_int,
                    (*(*pcd).pSD).iconImageHeight,
                    (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint,
                    (*(*pcd).pSD).iconHeight as ::core::ffi::c_uint,
                    False,
                );
            } else {
                XClearArea(
                    wmGD.display,
                    (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
                    IB_MARGIN_WIDTH,
                    IB_MARGIN_HEIGHT,
                    (*(*pcd).pSD).iconWidth as ::core::ffi::c_uint,
                    (*(*pcd).pSD).iconHeight as ::core::ffi::c_uint,
                    False,
                );
            }
            IconExposureProc(pcd, False as Boolean);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReparentIconWindow(
    mut pcd: *mut ClientData,
    mut xOffset: ::core::ffi::c_int,
    mut yOffset: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut rpX: ::core::ffi::c_int = 0;
    let mut rpY: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut bw: ::core::ffi::c_uint = 0;
    let mut depth: ::core::ffi::c_uint = 0;
    let mut root: Window = 0;
    let mut windowChanges = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    let mut mask: ::core::ffi::c_uint = 0;
    XGetGeometry(
        wmGD.display,
        (*pcd).iconWindow as Drawable,
        &raw mut root,
        &raw mut x,
        &raw mut y,
        &raw mut width,
        &raw mut height,
        &raw mut bw,
        &raw mut depth,
    );
    if width != 0 as ::core::ffi::c_uint {
        mask = CWBorderWidth as ::core::ffi::c_uint;
        windowChanges.border_width = 0 as ::core::ffi::c_int;
    } else {
        mask = 0 as ::core::ffi::c_uint;
    }
    if width
        > ((*(*pcd).pSD).iconImageMaximum.width
            + (if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })) as ::core::ffi::c_uint
    {
        windowChanges.width = (*(*pcd).pSD).iconImageMaximum.width
            + (if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            });
        width = windowChanges.width as ::core::ffi::c_uint;
        mask |= CWWidth as ::core::ffi::c_uint;
    } else if width < (*(*pcd).pSD).iconImageMinimum.width as ::core::ffi::c_uint {
        windowChanges.width = (*(*pcd).pSD).iconImageMinimum.width;
        width = windowChanges.width as ::core::ffi::c_uint;
        mask |= CWWidth as ::core::ffi::c_uint;
    }
    if height
        > ((*(*pcd).pSD).iconImageMaximum.height
            + (if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })) as ::core::ffi::c_uint
    {
        windowChanges.height = (*(*pcd).pSD).iconImageMaximum.height
            + (if wmGD.frameStyle as ::core::ffi::c_uint
                == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            });
        height = windowChanges.height as ::core::ffi::c_uint;
        mask |= CWHeight as ::core::ffi::c_uint;
    } else if height < (*(*pcd).pSD).iconImageMinimum.height as ::core::ffi::c_uint {
        windowChanges.height = (*(*pcd).pSD).iconImageMinimum.height;
        height = windowChanges.height as ::core::ffi::c_uint;
        mask |= CWHeight as ::core::ffi::c_uint;
    }
    if mask != 0 {
        XConfigureWindow(
            wmGD.display,
            (*pcd).iconWindow,
            mask,
            &raw mut windowChanges,
        );
    }
    if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_LABEL_PART != 0 {
        yOffset += ICON_INTERNAL_SHADOW_WIDTH;
    }
    rpX = ((*(*pcd).pSD).iconWidth as ::core::ffi::c_uint)
        .wrapping_sub(width)
        .wrapping_div(2 as ::core::ffi::c_uint)
        .wrapping_add(xOffset as ::core::ffi::c_uint) as ::core::ffi::c_int;
    rpY = ((*(*pcd).pSD).iconImageHeight as ::core::ffi::c_uint)
        .wrapping_sub(height)
        .wrapping_div(2 as ::core::ffi::c_uint)
        .wrapping_add(yOffset as ::core::ffi::c_uint) as ::core::ffi::c_int;
    XReparentWindow(
        wmGD.display,
        (*pcd).iconWindow,
        (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
        rpX,
        rpY,
    );
    (*pcd).clientFlags |= ICON_REPARENTED;
}
unsafe extern "C" fn PutIconLabelOnScreen(
    mut screen: ::core::ffi::c_int,
    mut icon_x: ::core::ffi::c_int,
    mut icon_y: ::core::ffi::c_int,
    mut px: *mut ::core::ffi::c_int,
    mut py: *mut ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
) {
    let mut xsi = XineramaScreenInfo {
        screen_number: 0 as ::core::ffi::c_int,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    };
    if GetXineramaScreenFromLocation(icon_x, icon_y, &raw mut xsi) == 0 {
        xsi.width = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset(screen as isize))
        .width as ::core::ffi::c_short;
        xsi.height = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset(screen as isize))
        .height as ::core::ffi::c_short;
    }
    if (*px as ::core::ffi::c_uint)
        .wrapping_add(width)
        .wrapping_add(1 as ::core::ffi::c_uint)
        > (xsi.x_org as ::core::ffi::c_int + xsi.width as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        *px = (*px as ::core::ffi::c_uint).wrapping_sub(
            (*px as ::core::ffi::c_uint)
                .wrapping_add(width)
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_sub(
                    (xsi.x_org as ::core::ffi::c_int + xsi.width as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                ),
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    if (*py as ::core::ffi::c_uint)
        .wrapping_add(height)
        .wrapping_add(1 as ::core::ffi::c_uint)
        > (xsi.y_org as ::core::ffi::c_int + xsi.height as ::core::ffi::c_int)
            as ::core::ffi::c_uint
    {
        *py = (*py as ::core::ffi::c_uint).wrapping_sub(
            (*py as ::core::ffi::c_uint)
                .wrapping_add(height)
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_sub(
                    (xsi.y_org as ::core::ffi::c_int + xsi.height as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                ),
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    if *px < xsi.x_org as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        *px = xsi.x_org as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    }
    if *py < xsi.y_org as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        *py = xsi.y_org as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn PutBoxInIconBox(
    mut pCD: *mut ClientData,
    mut px: *mut ::core::ffi::c_int,
    mut py: *mut ::core::ffi::c_int,
    mut width: *mut ::core::ffi::c_uint,
    mut height: *mut ::core::ffi::c_uint,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut getArgs: [Arg; 3] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 3];
    let mut bBoardWidth: Dimension = 0;
    let mut bBoardHeight: Dimension = 0;
    let mut clipWidth: ::core::ffi::c_int = 0;
    let mut clipHeight: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(872 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut bBoardWidth as XtArgVal;
    i += 1;
    getArgs[i as usize].name = (&raw const XtStrings as *const ::core::ffi::c_char)
        .offset(234 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    getArgs[i as usize].value = &raw mut bBoardHeight as XtArgVal;
    i += 1;
    XtGetValues(
        (*(*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox).bBoardWidget,
        &raw mut getArgs as ArgList,
        i as Cardinal,
    );
    clipWidth = bBoardWidth as ::core::ffi::c_int;
    clipHeight = bBoardHeight as ::core::ffi::c_int;
    if (*px as ::core::ffi::c_uint)
        .wrapping_add(*width)
        .wrapping_sub(1 as ::core::ffi::c_uint)
        > clipWidth as ::core::ffi::c_uint
    {
        *px = (*px as ::core::ffi::c_uint).wrapping_sub(
            (*px as ::core::ffi::c_uint)
                .wrapping_add(*width)
                .wrapping_sub(1 as ::core::ffi::c_uint)
                .wrapping_sub(clipWidth as ::core::ffi::c_uint),
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    if (*py as ::core::ffi::c_uint)
        .wrapping_add(*height)
        .wrapping_sub(1 as ::core::ffi::c_uint)
        > clipHeight as ::core::ffi::c_uint
    {
        *py = (*py as ::core::ffi::c_uint).wrapping_sub(
            (*py as ::core::ffi::c_uint)
                .wrapping_add(*height)
                .wrapping_sub(1 as ::core::ffi::c_uint)
                .wrapping_sub(clipHeight as ::core::ffi::c_uint),
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    if *px < 0 as ::core::ffi::c_int {
        *px = 0 as ::core::ffi::c_int;
    }
    if *py < 0 as ::core::ffi::c_int {
        *py = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CreateActiveIconTextWindow(mut pSD: *mut WmScreenData) {
    let mut window_attribs = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    let mut attr_mask: ::core::ffi::c_ulong = 0;
    attr_mask = (CWEventMask | CWCursor) as ::core::ffi::c_ulong;
    window_attribs.event_mask = ExposureMask;
    window_attribs.cursor = wmGD.workspaceCursor;
    if (*pSD).iconAppearance.activeBackgroundPixmap != 0 {
        attr_mask |= CWBackPixmap as ::core::ffi::c_ulong;
        window_attribs.background_pixmap = (*pSD).iconAppearance.activeBackgroundPixmap;
    } else {
        attr_mask |= CWBackPixel as ::core::ffi::c_ulong;
        window_attribs.background_pixel =
            (*pSD).iconAppearance.activeBackground as ::core::ffi::c_ulong;
    }
    (*pSD).activeIconTextWin = XCreateWindow(
        wmGD.display,
        (*pSD).rootWindow,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_uint,
        1 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        CopyFromParent as ::core::ffi::c_int,
        InputOutput as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<Visual>(),
        attr_mask,
        &raw mut window_attribs,
    );
    (*pSD).activeLabelParent = (*pSD).rootWindow;
}
#[no_mangle]
pub unsafe extern "C" fn PaintActiveIconText(mut pcd: *mut ClientData, mut erase: Boolean) {
    let mut textBox = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut iconGC = ::core::ptr::null_mut::<_XGC>();
    let mut topGC = ::core::ptr::null_mut::<_XGC>();
    let mut botGC = ::core::ptr::null_mut::<_XGC>();
    if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long & ICON_ACTIVE_LABEL_PART == 0 {
        return;
    }
    iconGC = (*(*pcd).pSD).iconAppearance.activeGC;
    topGC = (*(*pcd).pSD).iconAppearance.activeTopShadowGC;
    botGC = (*(*pcd).pSD).iconAppearance.activeBottomShadowGC;
    if !pActiveIconTopRects.is_null() {
        XFillRectangles(
            wmGD.display,
            (*(*pcd).pSD).activeIconTextWin as Drawable,
            topGC,
            (*pActiveIconTopRects).prect,
            (*pActiveIconTopRects).used,
        );
    }
    if !pActiveIconBotRects.is_null() {
        XFillRectangles(
            wmGD.display,
            (*(*pcd).pSD).activeIconTextWin as Drawable,
            botGC,
            (*pActiveIconBotRects).prect,
            (*pActiveIconBotRects).used,
        );
    }
    textBox.x = wmGD.iconExternalShadowWidth as ::core::ffi::c_short;
    textBox.y = wmGD.iconExternalShadowWidth as ::core::ffi::c_short;
    textBox.width = activeIconTextWidth.wrapping_sub(
        (2 as ::core::ffi::c_int * wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    ) as ::core::ffi::c_ushort;
    textBox.height = activeIconTextHeight.wrapping_sub(
        (2 as ::core::ffi::c_int * wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    ) as ::core::ffi::c_ushort;
    if erase != 0 {
        XClearArea(
            wmGD.display,
            (*(*pcd).pSD).activeIconTextWin,
            textBox.x as ::core::ffi::c_int,
            textBox.y as ::core::ffi::c_int,
            textBox.width as ::core::ffi::c_uint,
            textBox.height as ::core::ffi::c_uint,
            FALSE,
        );
    }
    WmDrawXmString(
        wmGD.display,
        (*(*pcd).pSD).activeIconTextWin,
        (*(*pcd).pSD).iconAppearance.renderTable as XmFontList,
        if !(*pcd).ewmhIconTitle.is_null() {
            (*pcd).ewmhIconTitle
        } else {
            (*pcd).iconTitle
        },
        iconGC,
        textBox.x as Position,
        textBox.y as Position,
        textBox.width as Dimension,
        &raw mut textBox,
        True as Boolean,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ShowActiveIconText(mut pcd: *mut ClientData) {
    let mut iconFrameAttribs = XWindowAttributes {
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
    let mut window_attribs = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    let mut windowChanges = XWindowChanges {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        sibling: 0,
        stack_mode: 0,
    };
    let mut mask: ::core::ffi::c_uint = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut icon_x: ::core::ffi::c_int = 0;
    let mut icon_y: ::core::ffi::c_int = 0;
    let mut junk: ::core::ffi::c_uint = 0;
    let mut root: Window = 0;
    let mut dWidth: Dimension = 0;
    let mut dHeight: Dimension = 0;
    if (*(*pcd).pSD).activeIconTextWin != 0 {
        XGetWindowAttributes(
            wmGD.display,
            (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
            &raw mut iconFrameAttribs,
        );
        window_attribs.event_mask = iconFrameAttribs.your_event_mask;
        XChangeWindowAttributes(
            wmGD.display,
            (*(*pcd).pSD).activeIconTextWin,
            CWEventMask as ::core::ffi::c_ulong,
            &raw mut window_attribs,
        );
        XmStringExtent(
            (*(*pcd).pSD).iconAppearance.renderTable as XmFontList,
            if !(*pcd).ewmhIconTitle.is_null() {
                (*pcd).ewmhIconTitle
            } else {
                (*pcd).iconTitle
            },
            &raw mut dWidth,
            &raw mut dHeight,
        );
        activeIconTextHeight = (dHeight as ::core::ffi::c_uint)
            .wrapping_add(WM_BOTTOM_TITLE_PADDING as ::core::ffi::c_uint)
            .wrapping_add(
                (2 as ::core::ffi::c_int * wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            );
        activeIconTextWidth = dWidth as ::core::ffi::c_uint;
        if (activeIconTextWidth as ::core::ffi::c_double)
            < 1.2f64 * (*(*pcd).pSD).iconWidth as ::core::ffi::c_double
        {
            activeIconTextWidth =
                (1.2f64 * (*(*pcd).pSD).iconWidth as ::core::ffi::c_double) as ::core::ffi::c_uint;
        }
        activeIconTextWidth = activeIconTextWidth.wrapping_add(
            (2 as ::core::ffi::c_int * wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
        XGetGeometry(
            wmGD.display,
            (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
            &raw mut root,
            &raw mut icon_x,
            &raw mut icon_y,
            &raw mut junk,
            &raw mut junk,
            &raw mut junk,
            &raw mut junk,
        );
        y = icon_y + (*(*pcd).pSD).iconImageHeight;
        x = (icon_x as ::core::ffi::c_uint).wrapping_sub(
            activeIconTextWidth
                .wrapping_sub((*(*pcd).pSD).iconWidth as ::core::ffi::c_uint)
                .wrapping_div(2 as ::core::ffi::c_uint),
        ) as ::core::ffi::c_int;
        if (*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
            .pIconBox
            .is_null()
        {
            PutIconLabelOnScreen(
                (*(*pcd).pSD).screen,
                icon_x,
                icon_y,
                &raw mut x,
                &raw mut y,
                activeIconTextWidth,
                activeIconTextHeight,
            );
            if (*(*pcd).pSD).activeLabelParent != root {
                XReparentWindow(wmGD.display, (*(*pcd).pSD).activeIconTextWin, root, x, y);
                (*(*pcd).pSD).activeLabelParent = root;
            }
        } else {
            x = x + IB_MARGIN_WIDTH;
            y = y + IB_MARGIN_HEIGHT;
            if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long
                & (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int
                == 0
            {
                y = (y as ::core::ffi::c_uint).wrapping_sub(activeIconTextHeight)
                    as ::core::ffi::c_int as ::core::ffi::c_int;
            }
            PutBoxInIconBox(
                pcd,
                &raw mut x,
                &raw mut y,
                &raw mut activeIconTextWidth,
                &raw mut activeIconTextHeight,
            );
            if (*(*pcd).pSD).activeLabelParent != (*pcd).client {
                XReparentWindow(
                    wmGD.display,
                    (*(*pcd).pSD).activeIconTextWin,
                    XtWindowOfObject(
                        (*(*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).pIconBox)
                            .bBoardWidget,
                    ),
                    x,
                    y,
                );
                (*(*pcd).pSD).activeLabelParent = (*pcd).client;
            }
        }
        mask = (CWX | CWY | CWWidth | CWHeight) as ::core::ffi::c_uint;
        windowChanges.x = x;
        windowChanges.y = y;
        windowChanges.width = activeIconTextWidth as ::core::ffi::c_int;
        windowChanges.height = activeIconTextHeight as ::core::ffi::c_int;
        XConfigureWindow(
            wmGD.display,
            (*(*pcd).pSD).activeIconTextWin,
            mask,
            &raw mut windowChanges,
        );
        if !pActiveIconTopRects.is_null() && !pActiveIconBotRects.is_null() || {
            pActiveIconTopRects = AllocateRList(
                (4 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(wmGD.iconExternalShadowWidth as ::core::ffi::c_uint),
            );
            !pActiveIconTopRects.is_null() && {
                pActiveIconBotRects = AllocateRList(
                    (4 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        .wrapping_mul(wmGD.iconExternalShadowWidth as ::core::ffi::c_uint),
                );
                !pActiveIconBotRects.is_null()
            }
        } {
            (*pActiveIconTopRects).used = 0 as ::core::ffi::c_int;
            (*pActiveIconBotRects).used = 0 as ::core::ffi::c_int;
            BevelRectangle(
                pActiveIconTopRects,
                pActiveIconBotRects,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                activeIconTextWidth,
                activeIconTextHeight,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
                wmGD.iconExternalShadowWidth as ::core::ffi::c_uint,
            );
        }
        XMapRaised(wmGD.display, (*(*pcd).pSD).activeIconTextWin);
        wmGD.activeIconTextDisplayed = True as Boolean;
        XSaveContext(
            wmGD.display,
            (*(*pcd).pSD).activeIconTextWin as XID,
            wmGD.windowContextType,
            pcd as XPointer as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn HideActiveIconText(mut pSD: *mut WmScreenData) {
    if !pSD.is_null() && (*pSD).activeIconTextWin != 0 || (*wmGD.pActiveSD).activeIconTextWin != 0 {
        XDeleteContext(
            wmGD.display,
            if !pSD.is_null() {
                (*pSD).activeIconTextWin as XID
            } else {
                (*wmGD.pActiveSD).activeIconTextWin as XID
            },
            wmGD.windowContextType,
        );
        XUnmapWindow(
            wmGD.display,
            if !pSD.is_null() {
                (*pSD).activeIconTextWin
            } else {
                (*wmGD.pActiveSD).activeIconTextWin
            },
        );
        wmGD.activeIconTextDisplayed = False as Boolean;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MoveActiveIconText(mut pcd: *mut ClientData) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut icon_x: ::core::ffi::c_int = 0;
    let mut icon_y: ::core::ffi::c_int = 0;
    let mut junk: ::core::ffi::c_uint = 0;
    let mut root: Window = 0;
    let mut dWidth: Dimension = 0;
    let mut dHeight: Dimension = 0;
    if (*(*pcd).pSD).activeIconTextWin != 0
        && wmGD.activeIconTextDisplayed as ::core::ffi::c_int != 0
    {
        XmStringExtent(
            (*(*pcd).pSD).iconAppearance.renderTable as XmFontList,
            if !(*pcd).ewmhIconTitle.is_null() {
                (*pcd).ewmhIconTitle
            } else {
                (*pcd).iconTitle
            },
            &raw mut dWidth,
            &raw mut dHeight,
        );
        activeIconTextHeight = (dHeight as ::core::ffi::c_uint)
            .wrapping_add(WM_BOTTOM_TITLE_PADDING as ::core::ffi::c_uint)
            .wrapping_add(
                (2 as ::core::ffi::c_int * wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            );
        activeIconTextWidth = dWidth as ::core::ffi::c_uint;
        if (activeIconTextWidth as ::core::ffi::c_double)
            < 1.2f64 * (*(*pcd).pSD).iconWidth as ::core::ffi::c_double
        {
            activeIconTextWidth =
                (1.2f64 * (*(*pcd).pSD).iconWidth as ::core::ffi::c_double) as ::core::ffi::c_uint;
        }
        activeIconTextWidth = activeIconTextWidth.wrapping_add(
            (2 as ::core::ffi::c_int * wmGD.iconExternalShadowWidth as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
        XGetGeometry(
            wmGD.display,
            (*(*pcd).pWsList.offset((*pcd).currentWsc as isize)).iconFrameWin,
            &raw mut root,
            &raw mut icon_x,
            &raw mut icon_y,
            &raw mut junk,
            &raw mut junk,
            &raw mut junk,
            &raw mut junk,
        );
        y = icon_y + (*(*pcd).pSD).iconImageHeight;
        x = (icon_x as ::core::ffi::c_uint).wrapping_sub(
            activeIconTextWidth
                .wrapping_sub((*(*pcd).pSD).iconWidth as ::core::ffi::c_uint)
                .wrapping_div(2 as ::core::ffi::c_uint),
        ) as ::core::ffi::c_int;
        if (*(*pcd).pWsList.offset((*pcd).currentWsc as isize))
            .pIconBox
            .is_null()
        {
            PutIconLabelOnScreen(
                (*(*pcd).pSD).screen,
                icon_x,
                icon_y,
                &raw mut x,
                &raw mut y,
                activeIconTextWidth,
                activeIconTextHeight,
            );
        } else {
            x = x + IB_MARGIN_WIDTH;
            y = y + IB_MARGIN_HEIGHT;
            if (*(*pcd).pSD).iconDecoration as ::core::ffi::c_long
                & (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int
                == 0
            {
                y = (y as ::core::ffi::c_uint).wrapping_sub(activeIconTextHeight)
                    as ::core::ffi::c_int as ::core::ffi::c_int;
            }
            PutBoxInIconBox(
                pcd,
                &raw mut x,
                &raw mut y,
                &raw mut activeIconTextWidth,
                &raw mut activeIconTextHeight,
            );
        }
        XMoveWindow(wmGD.display, (*(*pcd).pSD).activeIconTextWin, x, y);
    }
}
