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
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn XListProperties(_: *mut Display, _: Window, _: *mut ::core::ffi::c_int) -> *mut Atom;
    fn XFreeStringList(_: *mut *mut ::core::ffi::c_char);
    fn XChangeProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: Atom,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XChangeWindowAttributes(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> ::core::ffi::c_int;
    fn XDeleteProperty(_: *mut Display, _: Window, _: Atom) -> ::core::ffi::c_int;
    fn XFlush(_: *mut Display) -> ::core::ffi::c_int;
    fn XFree(_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn XGetWindowProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_int,
        _: Atom,
        _: *mut Atom,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_ulong,
        _: *mut ::core::ffi::c_ulong,
        _: *mut *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn XGetWindowAttributes(
        _: *mut Display,
        _: Window,
        _: *mut XWindowAttributes,
    ) -> ::core::ffi::c_int;
    fn XSync(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
    fn XGetTextProperty(
        _: *mut Display,
        _: Window,
        _: *mut XTextProperty,
        _: Atom,
    ) -> ::core::ffi::c_int;
    fn XSetTextProperty(_: *mut Display, _: Window, _: *mut XTextProperty, _: Atom);
    fn XmbTextListToTextProperty(
        display: *mut Display,
        list: *mut *mut ::core::ffi::c_char,
        count: ::core::ffi::c_int,
        style: XICCEncodingStyle,
        text_prop_return: *mut XTextProperty,
    ) -> ::core::ffi::c_int;
    fn XmbTextPropertyToTextList(
        display: *mut Display,
        text_prop: *const XTextProperty,
        list_return: *mut *mut *mut ::core::ffi::c_char,
        count_return: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtCalloc(_: Cardinal, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    static mut wmGD: WmGlobalData;
    fn XmInternAtom(display: *mut Display, name: String_0, only_if_exists: Boolean) -> Atom;
    fn ResetColormapData(pCD: *mut ClientData, pWindows: *mut Window, count: ::core::ffi::c_int);
    fn Warning(message: *mut ::core::ffi::c_char);
    fn ParseMwmMenuStr(pSD: *mut WmScreenData, menuStr: *mut ::core::ffi::c_uchar)
        -> *mut MenuItem;
    fn WmXmStringToString(xmString: XmString) -> String_0;
}
pub const XUTF8StringStyle: XICCEncodingStyle = 4;
pub const XStdICCTextStyle: XICCEncodingStyle = 3;
pub const XTextStyle: XICCEncodingStyle = 2;
pub const XCompoundTextStyle: XICCEncodingStyle = 1;
pub const XStringStyle: XICCEncodingStyle = 0;
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const AnyPropertyType: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const ColormapChangeMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 23 as ::core::ffi::c_int;
pub const Success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CWEventMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 11 as ::core::ffi::c_int;
pub const PropModeReplace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PROP_MOTIF_WM_HINTS_ELEMENTS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PROP_MWM_HINTS_ELEMENTS: ::core::ffi::c_int = PROP_MOTIF_WM_HINTS_ELEMENTS;
pub const PROP_MOTIF_WM_INFO_ELEMENTS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PROP_MWM_INFO_ELEMENTS: ::core::ffi::c_int = PROP_MOTIF_WM_INFO_ELEMENTS;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XA_ATOM: Atom = 4 as ::core::ffi::c_int as Atom;
pub const XA_WM_NORMAL_HINTS: Atom = 40 as ::core::ffi::c_int as Atom;
pub const XA_WM_SIZE_HINTS: Atom = 41 as ::core::ffi::c_int as Atom;
pub const _XA_MWM_WORKSPACE_INFO: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"_MWM_WORKSPACE_INFO\0")
};
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PROTOCOL_WM_SAVE_YOURSELF: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const PROTOCOL_WM_DELETE_WINDOW: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const PROTOCOL_WM_TAKE_FOCUS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const PROTOCOL_MWM_MESSAGES: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const PROTOCOL_MWM_OFFSET: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const MAX_CLIENT_PROTOCOL_COUNT: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const MAX_COLORMAP_WINDOWS_COUNT: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const MAX_MWM_MESSAGES_COUNT: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const BIGSIZE: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
pub const PROP_SIZE_HINTS_ELEMENTS: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const PROP_WM_STATE_ELEMENTS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICCC_R2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ICCC_CURRENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICCC_UNKNOWN: ::core::ffi::c_int = ICCC_CURRENT;
static mut sizeHints: SizeHints = _SizeHints {
    icccVersion: 0,
    flags: 0,
    x: 0,
    y: 0,
    width: 0,
    height: 0,
    min_width: 0,
    min_height: 0,
    max_width: 0,
    max_height: 0,
    width_inc: 0,
    height_inc: 0,
    min_aspect: _AspectRatio { x: 0, y: 0 },
    max_aspect: _AspectRatio { x: 0, y: 0 },
    base_width: 0,
    base_height: 0,
    win_gravity: 0,
};
#[no_mangle]
pub unsafe extern "C" fn GetNormalHints(mut pCD: *mut ClientData) -> *mut SizeHints {
    let mut property = ::core::ptr::null_mut::<PropSizeHints>();
    let mut actualType: Atom = 0;
    let mut actualFormat: ::core::ffi::c_int = 0;
    let mut leftover: ::core::ffi::c_ulong = 0;
    let mut nitems: ::core::ffi::c_ulong = 0;
    if HasProperty(pCD, XA_WM_NORMAL_HINTS) == 0
        || (Success
            != XGetWindowProperty(
                wmGD.display,
                (*pCD).client,
                XA_WM_NORMAL_HINTS,
                0 as ::core::ffi::c_long,
                PROP_SIZE_HINTS_ELEMENTS as ::core::ffi::c_long,
                False,
                XA_WM_SIZE_HINTS,
                &raw mut actualType,
                &raw mut actualFormat,
                &raw mut nitems,
                &raw mut leftover,
                &raw mut property as *mut *mut ::core::ffi::c_uchar,
            )
            || actualType != XA_WM_SIZE_HINTS
            || nitems
                < (PROP_SIZE_HINTS_ELEMENTS - 3 as ::core::ffi::c_int) as ::core::ffi::c_ulong
            || actualFormat != 32 as ::core::ffi::c_int)
    {
        sizeHints.icccVersion = ICCC_UNKNOWN;
        sizeHints.flags = 0 as ::core::ffi::c_long;
    } else {
        sizeHints.flags = (*property).flags as ::core::ffi::c_long;
        sizeHints.x = (*property).x as ::core::ffi::c_int;
        sizeHints.y = (*property).y as ::core::ffi::c_int;
        sizeHints.width = (*property).width as ::core::ffi::c_int;
        sizeHints.height = (*property).height as ::core::ffi::c_int;
        sizeHints.min_width = (if (*property).minWidth > BIGSIZE as ::core::ffi::c_long {
            BIGSIZE as ::core::ffi::c_long
        } else {
            (*property).minWidth
        }) as ::core::ffi::c_int;
        sizeHints.min_height = (if (*property).minHeight > BIGSIZE as ::core::ffi::c_long {
            BIGSIZE as ::core::ffi::c_long
        } else {
            (*property).minHeight
        }) as ::core::ffi::c_int;
        sizeHints.max_width = (if (*property).maxWidth > BIGSIZE as ::core::ffi::c_long {
            BIGSIZE as ::core::ffi::c_long
        } else {
            (*property).maxWidth
        }) as ::core::ffi::c_int;
        sizeHints.max_height = (if (*property).maxHeight > BIGSIZE as ::core::ffi::c_long {
            BIGSIZE as ::core::ffi::c_long
        } else {
            (*property).maxHeight
        }) as ::core::ffi::c_int;
        sizeHints.width_inc = (if (*property).widthInc > BIGSIZE as ::core::ffi::c_long {
            BIGSIZE as ::core::ffi::c_long
        } else {
            (*property).widthInc
        }) as ::core::ffi::c_int;
        sizeHints.height_inc = (if (*property).heightInc > BIGSIZE as ::core::ffi::c_long {
            BIGSIZE as ::core::ffi::c_long
        } else {
            (*property).heightInc
        }) as ::core::ffi::c_int;
        sizeHints.min_aspect.x = if (*property).minAspectX as ::core::ffi::c_int > BIGSIZE {
            BIGSIZE
        } else {
            (*property).minAspectX as ::core::ffi::c_int
        };
        sizeHints.min_aspect.y = if (*property).minAspectY as ::core::ffi::c_int > BIGSIZE {
            BIGSIZE
        } else {
            (*property).minAspectY as ::core::ffi::c_int
        };
        sizeHints.max_aspect.x = if (*property).maxAspectX as ::core::ffi::c_int > BIGSIZE {
            BIGSIZE
        } else {
            (*property).maxAspectX as ::core::ffi::c_int
        };
        sizeHints.max_aspect.y = if (*property).maxAspectY as ::core::ffi::c_int > BIGSIZE {
            BIGSIZE
        } else {
            (*property).maxAspectY as ::core::ffi::c_int
        };
        if nitems == (PROP_SIZE_HINTS_ELEMENTS - 3 as ::core::ffi::c_int) as ::core::ffi::c_ulong {
            sizeHints.icccVersion = ICCC_R2;
        } else {
            sizeHints.icccVersion = ICCC_CURRENT;
            sizeHints.base_width = (if (*property).baseWidth > BIGSIZE as ::core::ffi::c_long {
                BIGSIZE as ::core::ffi::c_long
            } else {
                (*property).baseWidth
            }) as ::core::ffi::c_int;
            sizeHints.base_height = (if (*property).baseHeight > BIGSIZE as ::core::ffi::c_long {
                BIGSIZE as ::core::ffi::c_long
            } else {
                (*property).baseHeight
            }) as ::core::ffi::c_int;
            sizeHints.win_gravity = (*property).winGravity as ::core::ffi::c_int;
        }
    }
    if !property.is_null() {
        XFree(property as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    }
    return &raw mut sizeHints;
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWmProtocols(mut pCD: *mut ClientData) {
    let mut rValue: ::core::ffi::c_int = 0;
    let mut property = ::core::ptr::null_mut::<Atom>();
    let mut actualType: Atom = 0;
    let mut actualFormat: ::core::ffi::c_int = 0;
    let mut leftover: ::core::ffi::c_ulong = 0;
    let mut nitems: ::core::ffi::c_ulong = 0;
    let mut i: ::core::ffi::c_int = 0;
    if !(*pCD).clientProtocols.is_null() {
        XtFree((*pCD).clientProtocols as *mut ::core::ffi::c_char);
        (*pCD).clientProtocols = ::core::ptr::null_mut::<Atom>();
    }
    (*pCD).clientProtocolCount = 0 as ::core::ffi::c_int;
    (*pCD).protocolFlags = 0 as ::core::ffi::c_long;
    if HasProperty(pCD, wmGD.xa_WM_PROTOCOLS) == 0 {
        rValue = -(1 as ::core::ffi::c_int);
    } else {
        rValue = XGetWindowProperty(
            wmGD.display,
            (*pCD).client,
            wmGD.xa_WM_PROTOCOLS,
            0 as ::core::ffi::c_long,
            MAX_CLIENT_PROTOCOL_COUNT as ::core::ffi::c_long,
            False,
            AnyPropertyType as Atom,
            &raw mut actualType,
            &raw mut actualFormat,
            &raw mut nitems,
            &raw mut leftover,
            &raw mut property as *mut *mut ::core::ffi::c_uchar,
        );
    }
    if rValue != Success || actualType == None_0 as Atom || actualFormat != 32 as ::core::ffi::c_int
    {
        (*pCD).clientProtocols = ::core::ptr::null_mut::<Atom>();
    } else {
        (*pCD).clientProtocols = XtMalloc(
            (nitems as usize).wrapping_mul(::core::mem::size_of::<Atom>() as usize) as Cardinal,
        ) as *mut Atom;
        if (*pCD).clientProtocols.is_null() {
            Warning(
                b"Insufficient memory for window management data\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            (*pCD).clientProtocolCount = nitems as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while (i as ::core::ffi::c_ulong) < nitems {
                *(*pCD).clientProtocols.offset(i as isize) = *property.offset(i as isize);
                if *property.offset(i as isize) == wmGD.xa_WM_SAVE_YOURSELF {
                    (*pCD).protocolFlags |= PROTOCOL_WM_SAVE_YOURSELF;
                } else if *property.offset(i as isize) == wmGD.xa_WM_TAKE_FOCUS {
                    (*pCD).protocolFlags |= PROTOCOL_WM_TAKE_FOCUS;
                } else if *property.offset(i as isize) == wmGD.xa_WM_DELETE_WINDOW {
                    (*pCD).protocolFlags |= PROTOCOL_WM_DELETE_WINDOW;
                } else if *property.offset(i as isize) == wmGD.xa_MWM_MESSAGES {
                    (*pCD).protocolFlags |= PROTOCOL_MWM_MESSAGES;
                }
                i += 1;
            }
        }
    }
    if !property.is_null() {
        XFree(property as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessMwmMessages(mut pCD: *mut ClientData) {
    let mut rValue: ::core::ffi::c_int = 0;
    let mut property = ::core::ptr::null_mut::<::core::ffi::c_long>();
    let mut actualType: Atom = 0;
    let mut actualFormat: ::core::ffi::c_int = 0;
    let mut leftover: ::core::ffi::c_ulong = 0;
    let mut nitems: ::core::ffi::c_ulong = 0;
    let mut i: ::core::ffi::c_int = 0;
    if !(*pCD).mwmMessages.is_null() {
        XtFree((*pCD).mwmMessages as *mut ::core::ffi::c_char);
        (*pCD).mwmMessages = ::core::ptr::null_mut::<::core::ffi::c_long>();
    }
    (*pCD).mwmMessagesCount = 0 as ::core::ffi::c_int;
    if HasProperty(pCD, wmGD.xa_MWM_MESSAGES) == 0 {
        rValue = !Success;
    } else {
        rValue = XGetWindowProperty(
            wmGD.display,
            (*pCD).client,
            wmGD.xa_MWM_MESSAGES,
            0 as ::core::ffi::c_long,
            MAX_MWM_MESSAGES_COUNT as ::core::ffi::c_long,
            False,
            AnyPropertyType as Atom,
            &raw mut actualType,
            &raw mut actualFormat,
            &raw mut nitems,
            &raw mut leftover,
            &raw mut property as *mut *mut ::core::ffi::c_uchar,
        );
    }
    if rValue != Success
        || actualType == None_0 as Atom
        || actualFormat != 32 as ::core::ffi::c_int
        || nitems == 0 as ::core::ffi::c_ulong
    {
        (*pCD).mwmMessages = ::core::ptr::null_mut::<::core::ffi::c_long>();
    } else {
        (*pCD).mwmMessages = XtMalloc(
            (nitems as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as usize)
                as Cardinal,
        ) as *mut ::core::ffi::c_long;
        if (*pCD).mwmMessages.is_null() {
            Warning(
                b"Insufficient memory for window management data\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            (*pCD).mwmMessagesCount = nitems as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while (i as ::core::ffi::c_ulong) < nitems {
                let ref mut fresh0 = *(*pCD).mwmMessages.offset(i as isize);
                *fresh0 = *property.offset(i as isize);
                if *fresh0 as Atom == wmGD.xa_MWM_OFFSET {
                    (*pCD).protocolFlags |= PROTOCOL_MWM_OFFSET;
                }
                i += 1;
            }
        }
    }
    if !property.is_null() {
        XFree(property as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetMwmInfo(
    mut propWindow: Window,
    mut flags: ::core::ffi::c_long,
    mut wmWindow: Window,
) {
    let mut property = PropMotifWmInfo {
        flags: 0,
        wmWindow: 0,
    };
    property.flags = flags;
    property.wmWindow = wmWindow;
    XChangeProperty(
        wmGD.display,
        propWindow,
        wmGD.xa_MWM_INFO,
        wmGD.xa_MWM_INFO,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut property as *mut ::core::ffi::c_uchar,
        PROP_MWM_INFO_ELEMENTS,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SetMwmSaveSessionInfo(mut wmWindow: Window) {
    let mut property: Atom = 0;
    property = wmGD.xa_WM_SAVE_YOURSELF;
    XChangeProperty(
        wmGD.display,
        wmWindow,
        wmGD.xa_WM_PROTOCOLS,
        XA_ATOM,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut property as *mut ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int,
    );
    SetWMState(wmWindow, NORMAL_STATE, 0 as Window);
}
#[no_mangle]
pub unsafe extern "C" fn GetWMState(mut window: Window) -> *mut PropWMState {
    let mut ret_val: ::core::ffi::c_int = 0;
    let mut property = ::core::ptr::null_mut::<PropWMState>();
    let mut actual_type: Atom = 0;
    let mut actual_format: ::core::ffi::c_int = 0;
    let mut nitems: ::core::ffi::c_ulong = 0;
    let mut leftover: ::core::ffi::c_ulong = 0;
    ret_val = XGetWindowProperty(
        wmGD.display,
        window,
        wmGD.xa_WM_STATE,
        0 as ::core::ffi::c_long,
        PROP_WM_STATE_ELEMENTS as ::core::ffi::c_long,
        False,
        wmGD.xa_WM_STATE,
        &raw mut actual_type,
        &raw mut actual_format,
        &raw mut nitems,
        &raw mut leftover,
        &raw mut property as *mut *mut ::core::ffi::c_uchar,
    );
    if !(ret_val == Success
        && actual_type == wmGD.xa_WM_STATE
        && nitems == PROP_WM_STATE_ELEMENTS as ::core::ffi::c_ulong)
    {
        if !property.is_null() {
            XFree(property as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
            property = ::core::ptr::null_mut::<PropWMState>();
        }
    }
    return property;
}
#[no_mangle]
pub unsafe extern "C" fn SetWMState(
    mut window: Window,
    mut state: ::core::ffi::c_int,
    mut icon: Window,
) {
    let mut property = _PropWMState { state: 0, icon: 0 };
    property.state = state as ::core::ffi::c_ulong;
    property.icon = icon as ::core::ffi::c_ulong;
    XChangeProperty(
        wmGD.display,
        window,
        wmGD.xa_WM_STATE,
        wmGD.xa_WM_STATE,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut property as *mut ::core::ffi::c_uchar,
        PROP_WM_STATE_ELEMENTS,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GetMwmHints(mut pCD: *mut ClientData) -> *mut PropMwmHints {
    let mut ret_val: ::core::ffi::c_int = 0;
    let mut property = ::core::ptr::null_mut::<PropMwmHints>();
    let mut actual_type: Atom = 0;
    let mut actual_format: ::core::ffi::c_int = 0;
    let mut nitems: ::core::ffi::c_ulong = 0;
    let mut leftover: ::core::ffi::c_ulong = 0;
    if HasProperty(pCD, wmGD.xa_MWM_HINTS) == 0 {
        ret_val = !Success;
    } else {
        ret_val = XGetWindowProperty(
            wmGD.display,
            (*pCD).client,
            wmGD.xa_MWM_HINTS,
            0 as ::core::ffi::c_long,
            PROP_MWM_HINTS_ELEMENTS as ::core::ffi::c_long,
            False,
            wmGD.xa_MWM_HINTS,
            &raw mut actual_type,
            &raw mut actual_format,
            &raw mut nitems,
            &raw mut leftover,
            &raw mut property as *mut *mut ::core::ffi::c_uchar,
        );
    }
    if ret_val == Success && actual_type == wmGD.xa_MWM_HINTS {
        return property;
    }
    if !property.is_null() {
        XFree(property as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    }
    return ::core::ptr::null_mut::<PropMwmHints>();
}
#[no_mangle]
pub unsafe extern "C" fn GetMwmInfo(mut rootWindowOfScreen: Window) -> *mut PropMwmInfo {
    let mut ret_val: ::core::ffi::c_int = 0;
    let mut property = ::core::ptr::null_mut::<PropMwmInfo>();
    let mut actual_type: Atom = 0;
    let mut actual_format: ::core::ffi::c_int = 0;
    let mut nitems: ::core::ffi::c_ulong = 0;
    let mut leftover: ::core::ffi::c_ulong = 0;
    ret_val = XGetWindowProperty(
        wmGD.display,
        rootWindowOfScreen,
        wmGD.xa_MWM_INFO,
        0 as ::core::ffi::c_long,
        PROP_MWM_INFO_ELEMENTS as ::core::ffi::c_long,
        False,
        wmGD.xa_MWM_INFO,
        &raw mut actual_type,
        &raw mut actual_format,
        &raw mut nitems,
        &raw mut leftover,
        &raw mut property as *mut *mut ::core::ffi::c_uchar,
    );
    if ret_val == Success
        && actual_type == wmGD.xa_MWM_INFO
        && nitems == PROP_MWM_INFO_ELEMENTS as ::core::ffi::c_ulong
    {
        return property;
    }
    if !property.is_null() {
        XFree(property as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    }
    return ::core::ptr::null_mut::<PropMwmInfo>();
}
#[no_mangle]
pub unsafe extern "C" fn ProcessWmColormapWindows(mut pCD: *mut ClientData) {
    let mut rValue: ::core::ffi::c_int = 0;
    let mut property = ::core::ptr::null_mut::<Window>();
    let mut actualType: Atom = 0;
    let mut actualFormat: ::core::ffi::c_int = 0;
    let mut leftover: ::core::ffi::c_ulong = 0;
    let mut nitems: ::core::ffi::c_ulong = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut pWindows = ::core::ptr::null_mut::<Window>();
    let mut pColormaps = ::core::ptr::null_mut::<Colormap>();
    let mut colormapCount: ::core::ffi::c_int = 0;
    let mut wAttributes = XWindowAttributes {
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
    let mut pcd = ::core::ptr::null_mut::<ClientData>();
    let mut sAttributes = XSetWindowAttributes {
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
    let mut pCmapFlags = ::core::ptr::null_mut::<::core::ffi::c_int>();
    rValue = XGetWindowProperty(
        wmGD.display,
        (*pCD).client,
        wmGD.xa_WM_COLORMAP_WINDOWS,
        0 as ::core::ffi::c_long,
        MAX_COLORMAP_WINDOWS_COUNT as ::core::ffi::c_long,
        False,
        AnyPropertyType as Atom,
        &raw mut actualType,
        &raw mut actualFormat,
        &raw mut nitems,
        &raw mut leftover,
        &raw mut property as *mut *mut ::core::ffi::c_uchar,
    );
    if rValue == Success
        && actualType != None_0 as Atom
        && actualFormat == 32 as ::core::ffi::c_int
        && nitems > 0 as ::core::ffi::c_ulong
    {
        pWindows = XtCalloc(
            nitems.wrapping_add(1 as ::core::ffi::c_ulong) as Cardinal,
            ::core::mem::size_of::<Window>() as Cardinal,
        ) as *mut Window;
        if pWindows.is_null() || {
            pColormaps = XtCalloc(
                nitems.wrapping_add(1 as ::core::ffi::c_ulong) as Cardinal,
                ::core::mem::size_of::<Colormap>() as Cardinal,
            ) as *mut Colormap;
            pColormaps.is_null()
        } {
            Warning(
                b"Insufficient memory for window management data\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            if !pWindows.is_null() {
                XtFree(pWindows as *mut ::core::ffi::c_char);
            }
        } else {
            pCmapFlags = XtCalloc(
                nitems.wrapping_add(1 as ::core::ffi::c_ulong) as Cardinal,
                ::core::mem::size_of::<::core::ffi::c_int>() as Cardinal,
            ) as *mut ::core::ffi::c_int;
            if pCmapFlags.is_null() {
                Warning(
                    b"Insufficient memory for window manager flags\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                XtFree(pWindows as *mut ::core::ffi::c_char);
                XtFree(pColormaps as *mut ::core::ffi::c_char);
            } else {
                i = 0 as ::core::ffi::c_int;
                while (i as ::core::ffi::c_ulong) < nitems {
                    if *property.offset(i as isize) == (*pCD).client {
                        break;
                    }
                    i += 1;
                }
                colormapCount = 0 as ::core::ffi::c_int;
                if i as ::core::ffi::c_ulong == nitems {
                    *pWindows.offset(0 as ::core::ffi::c_int as isize) = (*pCD).client;
                    *pColormaps.offset(0 as ::core::ffi::c_int as isize) =
                        FindColormap(pCD, (*pCD).client);
                    colormapCount += 1;
                }
                sAttributes.event_mask = (1 as ::core::ffi::c_long) << 23 as ::core::ffi::c_int;
                i = 0 as ::core::ffi::c_int;
                while (i as ::core::ffi::c_ulong) < nitems {
                    let ref mut fresh1 = *pColormaps.offset(colormapCount as isize);
                    *fresh1 = FindColormap(pCD, *property.offset(i as isize));
                    if *fresh1 != None_0 as Colormap {
                        *pWindows.offset(colormapCount as isize) = *property.offset(i as isize);
                        colormapCount += 1;
                    } else if XFindContext(
                        wmGD.display,
                        *property.offset(i as isize) as XID,
                        wmGD.windowContextType,
                        &raw mut pcd as *mut XPointer,
                    ) != 0
                    {
                        XChangeWindowAttributes(
                            wmGD.display,
                            *property.offset(i as isize),
                            CWEventMask as ::core::ffi::c_ulong,
                            &raw mut sAttributes,
                        );
                        if XGetWindowAttributes(
                            wmGD.display,
                            *property.offset(i as isize),
                            &raw mut wAttributes,
                        ) != 0
                        {
                            *pWindows.offset(colormapCount as isize) = *property.offset(i as isize);
                            *pColormaps.offset(colormapCount as isize) = wAttributes.colormap;
                            colormapCount += 1;
                        }
                    }
                    i += 1;
                }
                ResetColormapData(pCD, pWindows, colormapCount);
                (*pCD).clientColormap = *pColormaps.offset(0 as ::core::ffi::c_int as isize);
                if colormapCount > 1 as ::core::ffi::c_int {
                    (*pCD).clientCmapCount = colormapCount;
                    (*pCD).cmapWindows = pWindows;
                    (*pCD).clientCmapList = pColormaps;
                    (*pCD).clientCmapIndex = 0 as ::core::ffi::c_int;
                    (*pCD).clientCmapFlags = pCmapFlags;
                } else {
                    (*pCD).clientCmapCount = 0 as ::core::ffi::c_int;
                    XtFree(pWindows as *mut ::core::ffi::c_char);
                    XtFree(pColormaps as *mut ::core::ffi::c_char);
                    XtFree(pCmapFlags as *mut ::core::ffi::c_char);
                }
            }
        }
    }
    if !property.is_null() {
        XFree(property as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn FindColormap(mut pCD: *mut ClientData, mut window: Window) -> Colormap {
    let mut colormap = 0 as ::core::ffi::c_int as Colormap;
    let mut i: ::core::ffi::c_int = 0;
    if (*pCD).clientCmapCount == 0 as ::core::ffi::c_int {
        if window == (*pCD).client {
            colormap = (*pCD).clientColormap;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < (*pCD).clientCmapCount {
            if *(*pCD).cmapWindows.offset(i as isize) == window {
                colormap = *(*pCD).clientCmapList.offset(i as isize);
                break;
            } else {
                i += 1;
            }
        }
    }
    return colormap;
}
#[no_mangle]
pub unsafe extern "C" fn GetMwmMenuItems(mut pCD: *mut ClientData) -> *mut MenuItem {
    let mut rValue: ::core::ffi::c_int = 0;
    let mut textProperty = XTextProperty {
        value: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        encoding: 0,
        format: 0,
        nitems: 0,
    };
    let mut menuItems = ::core::ptr::null_mut::<MenuItem>();
    textProperty.value = NULL as *mut ::core::ffi::c_uchar;
    rValue = XGetTextProperty(
        wmGD.display,
        (*pCD).client,
        &raw mut textProperty,
        wmGD.xa_MWM_MENU,
    );
    if rValue == 0 as ::core::ffi::c_int || textProperty.value.is_null() {
        menuItems = ::core::ptr::null_mut::<MenuItem>();
    } else {
        let mut textList = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut nItems: ::core::ffi::c_int = 0;
        if XmbTextPropertyToTextList(
            wmGD.display,
            &raw mut textProperty,
            &raw mut textList,
            &raw mut nItems,
        ) != Success
        {
            menuItems = ::core::ptr::null_mut::<MenuItem>();
        } else if !textList.is_null() {
            menuItems = ParseMwmMenuStr(
                (*pCD).pSD,
                *textList.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar,
            );
            XFreeStringList(textList);
        } else {
            menuItems = ::core::ptr::null_mut::<MenuItem>();
        }
        XFree(textProperty.value as *mut ::core::ffi::c_void);
    }
    return menuItems;
}
#[no_mangle]
pub unsafe extern "C" fn SetWorkspaceListProperty(mut pSD: *mut WmScreenData) {
    let mut pws = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut pWsList = ::core::ptr::null_mut::<Atom>();
    let mut count: ::core::ffi::c_int = 0;
    pWsList = XtMalloc(
        ((*pSD).numWorkspaces as usize).wrapping_mul(::core::mem::size_of::<Atom>() as usize)
            as Cardinal,
    ) as *mut Atom;
    pws = (*pSD).pWS as *mut WmWorkspaceData;
    count = 0 as ::core::ffi::c_int;
    while count < (*pSD).numWorkspaces {
        *pWsList.offset(count as isize) = (*pws).id as Atom;
        pws = pws.offset(1);
        count += 1;
    }
    XChangeProperty(
        wmGD.display,
        (*pSD).wmWorkspaceWin,
        wmGD.xa_MWM_WORKSPACE_LIST,
        XA_ATOM,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        pWsList as *mut ::core::ffi::c_uchar,
        ((*pSD).numWorkspaces as usize)
            .wrapping_mul(::core::mem::size_of::<Atom>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_long>() as usize)
            as ::core::ffi::c_int,
    );
    XtFree(pWsList as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn SetCurrentWorkspaceProperty(mut pSD: *mut WmScreenData) {
    let mut aCurrent: Atom = 0;
    aCurrent = (*(*pSD).pActiveWS).id as Atom;
    XChangeProperty(
        wmGD.display,
        (*pSD).wmWorkspaceWin,
        wmGD.xa_MWM_WORKSPACE_CURRENT,
        XA_ATOM,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        &raw mut aCurrent as *mut ::core::ffi::c_uchar,
        (::core::mem::size_of::<Atom>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_long>() as usize)
            as ::core::ffi::c_int,
    );
    XSync(wmGD.display, False);
}
pub const WIP_NUMBER_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn SetWorkspaceInfoProperty(mut pWS: *mut WmWorkspaceData) {
    let mut pch = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aProperty: Atom = 0;
    let mut sTitle = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ppchList = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut iNumStrings: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    let mut tp = XTextProperty {
        value: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        encoding: 0,
        format: 0,
        nitems: 0,
    };
    pch = WorkspacePropertyName(pWS);
    aProperty = XmInternAtom(wmGD.display, pch as String_0, FALSE as Boolean);
    XtFree(pch);
    iNumStrings = 6 as ::core::ffi::c_int;
    count = 1 as ::core::ffi::c_int;
    iNumStrings += count;
    ppchList = XtMalloc(
        (iNumStrings as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as Cardinal,
    ) as *mut *mut ::core::ffi::c_char;
    pch = XtMalloc(
        ((iNumStrings * WIP_NUMBER_SIZE) as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as Cardinal,
    );
    i = 0 as ::core::ffi::c_int;
    sTitle = WmXmStringToString((*pWS).title);
    let fresh2 = i;
    i = i + 1;
    let ref mut fresh3 = *ppchList.offset(fresh2 as isize);
    *fresh3 = sTitle;
    ix = i * WIP_NUMBER_SIZE;
    sprintf(
        pch.offset(ix as isize) as *mut ::core::ffi::c_char,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*pWS).backdrop.colorSet,
    );
    let fresh4 = i;
    i = i + 1;
    let ref mut fresh5 = *ppchList.offset(fresh4 as isize);
    *fresh5 = pch.offset(ix as isize) as *mut ::core::ffi::c_char;
    ix = i * WIP_NUMBER_SIZE;
    sprintf(
        pch.offset(ix as isize) as *mut ::core::ffi::c_char,
        b"0x%lx\0" as *const u8 as *const ::core::ffi::c_char,
        (*pWS).backdrop.background,
    );
    let fresh6 = i;
    i = i + 1;
    let ref mut fresh7 = *ppchList.offset(fresh6 as isize);
    *fresh7 = pch.offset(ix as isize) as *mut ::core::ffi::c_char;
    ix = i * WIP_NUMBER_SIZE;
    sprintf(
        pch.offset(ix as isize) as *mut ::core::ffi::c_char,
        b"0x%lx\0" as *const u8 as *const ::core::ffi::c_char,
        (*pWS).backdrop.foreground,
    );
    let fresh8 = i;
    i = i + 1;
    let ref mut fresh9 = *ppchList.offset(fresh8 as isize);
    *fresh9 = pch.offset(ix as isize) as *mut ::core::ffi::c_char;
    ix = i * WIP_NUMBER_SIZE;
    sprintf(
        pch.offset(ix as isize) as *mut ::core::ffi::c_char,
        b"0x%lx\0" as *const u8 as *const ::core::ffi::c_char,
        (*pWS).backdrop.nameAtom,
    );
    let fresh10 = i;
    i = i + 1;
    let ref mut fresh11 = *ppchList.offset(fresh10 as isize);
    *fresh11 = pch.offset(ix as isize) as *mut ::core::ffi::c_char;
    ix = i * WIP_NUMBER_SIZE;
    if (*pWS).backdrop.window == None_0 as Window {
        strcpy(
            pch.offset(ix as isize) as *mut ::core::ffi::c_char,
            b"0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        sprintf(
            pch.offset(ix as isize) as *mut ::core::ffi::c_char,
            b"%d\0" as *const u8 as *const ::core::ffi::c_char,
            count,
        );
    }
    let fresh12 = i;
    i = i + 1;
    let ref mut fresh13 = *ppchList.offset(fresh12 as isize);
    *fresh13 = pch.offset(ix as isize) as *mut ::core::ffi::c_char;
    ix = i * WIP_NUMBER_SIZE;
    sprintf(
        pch.offset(ix as isize) as *mut ::core::ffi::c_char,
        b"0x%lx\0" as *const u8 as *const ::core::ffi::c_char,
        (*pWS).backdrop.window,
    );
    let fresh14 = i;
    i = i + 1;
    let ref mut fresh15 = *ppchList.offset(fresh14 as isize);
    *fresh15 = pch.offset(ix as isize) as *mut ::core::ffi::c_char;
    status = XmbTextListToTextProperty(
        wmGD.display,
        ppchList,
        iNumStrings,
        XStdICCTextStyle,
        &raw mut tp,
    );
    if status == Success || status > 0 as ::core::ffi::c_int {
        XSetTextProperty(
            wmGD.display,
            (*(*pWS).pSD).wmWorkspaceWin,
            &raw mut tp,
            aProperty,
        );
        XFree(tp.value as *mut ::core::ffi::c_void);
    }
    XtFree(ppchList as *mut ::core::ffi::c_char);
    XtFree(pch);
    if !sTitle.is_null() {
        XtFree(sTitle);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DeleteWorkspaceInfoProperty(mut pWS: *mut WmWorkspaceData) {
    let mut pch = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aProperty: Atom = 0;
    pch = WorkspacePropertyName(pWS);
    aProperty = XmInternAtom(wmGD.display, pch as String_0, FALSE as Boolean);
    XtFree(pch);
    XDeleteProperty(wmGD.display, (*(*pWS).pSD).wmWorkspaceWin, aProperty);
    XFlush(wmGD.display);
}
#[no_mangle]
pub unsafe extern "C" fn WorkspacePropertyName(
    mut pWS: *mut WmWorkspaceData,
) -> *mut ::core::ffi::c_char {
    let mut pch = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pchName = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    pchName = (*pWS).name as *mut ::core::ffi::c_char;
    len = strlen(pchName)
        .wrapping_add(strlen(_XA_MWM_WORKSPACE_INFO.as_ptr()))
        .wrapping_add(4 as size_t) as ::core::ffi::c_int;
    pch = XtMalloc(len as Cardinal);
    strcpy(pch, _XA_MWM_WORKSPACE_INFO.as_ptr());
    strcat(pch, b"_\0" as *const u8 as *const ::core::ffi::c_char);
    strcat(pch, pchName);
    return pch;
}
#[no_mangle]
pub unsafe extern "C" fn SetWorkspacePresence(
    mut propWindow: Window,
    mut pWsPresence: *mut Atom,
    mut cPresence: ::core::ffi::c_ulong,
) {
    XChangeProperty(
        wmGD.display,
        propWindow,
        wmGD.xa_MWM_WORKSPACE_PRESENCE,
        wmGD.xa_MWM_WORKSPACE_PRESENCE,
        32 as ::core::ffi::c_int,
        PropModeReplace,
        pWsPresence as *mut ::core::ffi::c_uchar,
        cPresence as ::core::ffi::c_int,
    );
    XFlush(wmGD.display);
}
#[no_mangle]
pub unsafe extern "C" fn GetWmRequest(
    mut pSD: *mut WmScreenData,
    mut pszReq: *mut *mut ::core::ffi::c_char,
    mut pmore: *mut Boolean,
) {
    let mut rValue: ::core::ffi::c_int = 0;
    let mut chRequest = ::core::ptr::null_mut::<::core::ffi::c_char>();
    static mut property: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    static mut iNext: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut actualType: Atom = 0;
    let mut actualFormat: ::core::ffi::c_int = 0;
    let mut leftover: ::core::ffi::c_ulong = 0;
    static mut nitems: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    if property.is_null() {
        rValue = XGetWindowProperty(
            wmGD.display,
            (*pSD).wmWorkspaceWin,
            wmGD.xa_MWM_WM_REQUEST,
            0 as ::core::ffi::c_long,
            1000000 as ::core::ffi::c_int as ::core::ffi::c_long,
            True,
            AnyPropertyType as Atom,
            &raw mut actualType,
            &raw mut actualFormat,
            &raw mut nitems,
            &raw mut leftover,
            &raw mut property as *mut *mut ::core::ffi::c_uchar,
        );
        if rValue != Success
            || actualType == None_0 as Atom
            || actualFormat != 8 as ::core::ffi::c_int
        {
            property = ::core::ptr::null_mut::<::core::ffi::c_char>();
            iNext = -(1 as ::core::ffi::c_int);
            nitems = 0 as ::core::ffi::c_ulong;
        } else {
            iNext = 0 as ::core::ffi::c_int;
        }
    }
    if !property.is_null() && iNext >= 0 as ::core::ffi::c_int {
        let mut len = 0 as ::core::ffi::c_int;
        i = iNext;
        while (i as ::core::ffi::c_ulong) < nitems {
            if *property.offset(i as isize) as ::core::ffi::c_int == '\0' as i32 {
                break;
            }
            i += 1;
        }
        if i as ::core::ffi::c_ulong >= nitems {
            i = nitems as ::core::ffi::c_int;
        }
        len = i - iNext
            + 1 as ::core::ffi::c_int
            + (if *property.offset(i as isize) as ::core::ffi::c_int == '\0' as i32 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            });
        chRequest = XtMalloc(len as Cardinal);
        if chRequest.is_null() {
            Warning(
                b"Insufficient memory for window management data\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            strncpy(
                chRequest,
                property.offset(iNext as isize) as *mut ::core::ffi::c_char,
                len as size_t,
            );
            if *property.offset(i as isize) as ::core::ffi::c_int != '\0' as i32 {
                *chRequest.offset((len - 1 as ::core::ffi::c_int) as isize) =
                    '\0' as i32 as ::core::ffi::c_char;
            }
            iNext = i + 1 as ::core::ffi::c_int;
        }
        if iNext as ::core::ffi::c_ulong >= nitems {
            XFree(property as *mut ::core::ffi::c_void);
            iNext = -(1 as ::core::ffi::c_int);
            property = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    *pmore = (property != NULL as *mut ::core::ffi::c_char) as ::core::ffi::c_int as Boolean;
    *pszReq = chRequest;
}
#[no_mangle]
pub unsafe extern "C" fn GetInitialPropertyList(mut pCD: *mut ClientData) {
    let mut paList = ::core::ptr::null_mut::<Atom>();
    let mut iProps: ::core::ffi::c_int = 0;
    paList = XListProperties(wmGD.display, (*pCD).client, &raw mut iProps);
    if !paList.is_null() {
        (*pCD).paInitialProperties = paList;
        (*pCD).numInitialProperties = iProps;
    } else {
        (*pCD).paInitialProperties = ::core::ptr::null_mut::<Atom>();
        (*pCD).numInitialProperties = 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn DiscardInitialPropertyList(mut pCD: *mut ClientData) {
    if !(*pCD).paInitialProperties.is_null() {
        XFree((*pCD).paInitialProperties as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        (*pCD).paInitialProperties = ::core::ptr::null_mut::<Atom>();
        (*pCD).numInitialProperties = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HasProperty(mut pCD: *mut ClientData, mut aProperty: Atom) -> Boolean {
    let mut bFound = False as Boolean;
    let mut paList = ::core::ptr::null_mut::<Atom>();
    let mut count: ::core::ffi::c_int = 0;
    paList = (*pCD).paInitialProperties;
    if !paList.is_null() {
        count = (*pCD).numInitialProperties;
        while bFound == 0 && count > 0 as ::core::ffi::c_int {
            bFound = (*paList == aProperty) as ::core::ffi::c_int as Boolean;
            count -= 1;
            paList = paList.offset(1);
        }
    } else {
        bFound = True as Boolean;
    }
    return bFound;
}
