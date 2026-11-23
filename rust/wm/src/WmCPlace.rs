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
use crate::abi::C2RustUnnamed_0_WmArgb as C2RustUnnamed_0;
use crate::abi::C2RustUnnamed_1_WmArgb as C2RustUnnamed_1;
use crate::abi::C2RustUnnamed_WmArgb as C2RustUnnamed;
use crate::abi::*;
extern "C" {
    fn XChangeActivePointerGrab(
        _: *mut Display,
        _: ::core::ffi::c_uint,
        _: Cursor,
        _: Time,
    ) -> ::core::ffi::c_int;
    fn XCheckIfEvent(
        _: *mut Display,
        _: *mut XEvent,
        _: Option<unsafe extern "C" fn(*mut Display, *mut XEvent, XPointer) -> ::core::ffi::c_int>,
        _: XPointer,
    ) -> ::core::ffi::c_int;
    fn XQueryPointer(
        _: *mut Display,
        _: Window,
        _: *mut Window,
        _: *mut Window,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn XWarpPointer(
        _: *mut Display,
        _: Window,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut wmGD: WmGlobalData;
    fn XkbKeycodeToKeysym(
        _: *mut Display,
        _: KeyCode,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> KeySym;
    fn ClientToFrame(
        pcd: *mut ClientData,
        pX: *mut ::core::ffi::c_int,
        pY: *mut ::core::ffi::c_int,
        pWidth: *mut ::core::ffi::c_uint,
        pHeight: *mut ::core::ffi::c_uint,
    );
    fn PopGadgetOut(pcd: *mut ClientData, gadget: ::core::ffi::c_int);
    fn HideFeedbackWindow(pSD: *mut WmScreenData);
    fn DoFeedback(
        pcd: *mut ClientData,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        newStyle: ::core::ffi::c_ulong,
        resizing: Boolean,
    );
    fn DoGrabs(
        grab_win: Window,
        cursor: Cursor,
        pmask: ::core::ffi::c_uint,
        grabTime: Time,
        pCD: *mut ClientData,
        alwaysGrab: Boolean,
    ) -> Boolean;
    fn FixFrameValues(
        pcd: *mut ClientData,
        pfX: *mut ::core::ffi::c_int,
        pfY: *mut ::core::ffi::c_int,
        pfWidth: *mut ::core::ffi::c_uint,
        pfHeight: *mut ::core::ffi::c_uint,
        resizing: Boolean,
    );
    fn GetConfigEvent(
        display: *mut Display,
        window: Window,
        mask: ::core::ffi::c_ulong,
        curX: ::core::ffi::c_int,
        curY: ::core::ffi::c_int,
        oX: ::core::ffi::c_int,
        oY: ::core::ffi::c_int,
        oWidth: ::core::ffi::c_uint,
        oHeight: ::core::ffi::c_uint,
        pev: *mut XEvent,
    );
    fn MoveOutline(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
    );
    fn SetPointerPosition(
        newX: ::core::ffi::c_int,
        newY: ::core::ffi::c_int,
        actualX: *mut ::core::ffi::c_int,
        actualY: *mut ::core::ffi::c_int,
    );
    fn UndoGrabs();
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const CurrentTime: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const KeyPressMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const ButtonPressMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ButtonReleaseMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const PointerMotionMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const PointerMotionHintMask: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
pub const KeyPress: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ButtonPress: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ButtonRelease: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MotionNotify: ::core::ffi::c_int = 6;
pub const ControlMask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const XK_Return: KeySym = 65293 as KeySym;
pub const XK_Left: KeySym = 65361 as KeySym;
pub const XK_Up: KeySym = 65362 as KeySym;
pub const XK_Right: KeySym = 65363 as KeySym;
pub const XK_Down: KeySym = 65364 as KeySym;
pub const XK_space: KeySym = 32 as KeySym;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_FUNC_RESIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FB_SIZE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const FB_POSITION: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const WM_SHOW_FB_PLACEMENT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const GRAB_MASK: ::core::ffi::c_long =
    KeyPressMask | ButtonPressMask | ButtonReleaseMask | PointerMotionMask;
pub const PGRAB_MASK: ::core::ffi::c_long =
    ButtonPressMask | ButtonReleaseMask | PointerMotionMask | PointerMotionHintMask;
pub const NOFRZ_PGRAB_MASK: ::core::ffi::c_long = ButtonPressMask | ButtonReleaseMask;
static mut placeX: ::core::ffi::c_int = 0;
static mut placeY: ::core::ffi::c_int = 0;
static mut placeWidth: ::core::ffi::c_uint = 0;
static mut placeHeight: ::core::ffi::c_uint = 0;
static mut placeResize: Boolean = 0;
static mut placementDone: Boolean = 0;
static mut placePointerX: ::core::ffi::c_int = 0;
static mut placePointerY: ::core::ffi::c_int = 0;
static mut placeKeyMultiplier: ::core::ffi::c_int = 0;
static mut placeOffsetX: ::core::ffi::c_int = 0;
static mut placeOffsetY: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn SetupPlacement(mut pcd: *mut ClientData) {
    let mut cX: ::core::ffi::c_int = 0;
    let mut cY: ::core::ffi::c_int = 0;
    let mut junk: ::core::ffi::c_int = 0;
    let mut junk_win: Window = 0;
    if !wmGD.gadgetClient.is_null() && wmGD.gadgetDepressed != 0 {
        PopGadgetOut(wmGD.gadgetClient, wmGD.gadgetDepressed);
    }
    placeOffsetX = (*pcd).clientOffset.x as ::core::ffi::c_int;
    placeOffsetY = (*pcd).clientOffset.y as ::core::ffi::c_int;
    XQueryPointer(
        wmGD.display,
        (*wmGD.pActiveSD).rootWindow,
        &raw mut junk_win,
        &raw mut junk_win,
        &raw mut cX,
        &raw mut cY,
        &raw mut junk,
        &raw mut junk,
        &raw mut junk as *mut ::core::ffi::c_uint,
    );
    placeX = cX;
    placePointerX = placeX;
    placeY = cY;
    placePointerY = placeY;
    placeWidth = (*pcd).clientWidth as ::core::ffi::c_uint;
    placeHeight = (*pcd).clientHeight as ::core::ffi::c_uint;
    ClientToFrame(
        pcd,
        &raw mut cX,
        &raw mut cY,
        &raw mut placeWidth,
        &raw mut placeHeight,
    );
    placeResize = FALSE as Boolean;
    wmGD.preMove = FALSE as Boolean;
    wmGD.movingIcon = FALSE as Boolean;
    if wmGD.showFeedback as ::core::ffi::c_long & WM_SHOW_FB_PLACEMENT != 0 {
        DoFeedback(
            pcd,
            placeX,
            placeY,
            placeWidth,
            placeHeight,
            (FB_SIZE | FB_POSITION) as ::core::ffi::c_ulong,
            placeResize,
        );
    }
    MoveOutline(placeX, placeY, placeWidth, placeHeight);
}
#[no_mangle]
pub unsafe extern "C" fn IsRepeatedKeyEvent(
    mut dpy: *mut Display,
    mut pEvent: *mut XEvent,
    mut pOldEvent: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pOld = pOldEvent as *mut XEvent;
    if (*pEvent).type_0 == KeyPress
        && (*pEvent).type_0 == (*pOld).type_0
        && (*pEvent).xkey.keycode == (*pOld).xkey.keycode
        && (*pEvent).xkey.state == (*pOld).xkey.state
        && (*pEvent).xkey.window == (*pOld).xkey.window
        && (*pEvent).xkey.root == (*pOld).xkey.root
    {
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn StartInteractiveSizing(mut pcd: *mut ClientData, mut time: Time) {
    let mut gmask: ::core::ffi::c_uint = 0;
    gmask = (if wmGD.useWindowOutline as ::core::ffi::c_int != 0 {
        NOFRZ_PGRAB_MASK
    } else {
        PGRAB_MASK
    }) as ::core::ffi::c_uint;
    XChangeActivePointerGrab(wmGD.display, gmask, wmGD.sizePlacementCursor, time);
    if wmGD.enableWarp != 0 {
        XWarpPointer(
            wmGD.display,
            None_0 as Window,
            (*wmGD.pActiveSD).rootWindow,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            (placeX as ::core::ffi::c_uint)
                .wrapping_add(placeWidth)
                .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            (placeY as ::core::ffi::c_uint)
                .wrapping_add(placeHeight)
                .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        );
    }
    if (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_RESIZE != 0 {
        placeResize = TRUE as Boolean;
        wmGD.preMove = FALSE as Boolean;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandlePlacementKeyEvent(
    mut pcd: *mut ClientData,
    mut pev: *mut XKeyEvent,
) {
    let mut KeyEvent = _XEvent { type_0: 0 };
    let mut keysym: KeySym = 0;
    let mut control: Boolean = 0;
    let mut valid: Boolean = 0;
    let mut big_inc: ::core::ffi::c_int = 0;
    let mut tmpX = 0 as ::core::ffi::c_int;
    let mut tmpY = 0 as ::core::ffi::c_int;
    let mut warpX: ::core::ffi::c_int = 0;
    let mut warpY: ::core::ffi::c_int = 0;
    let mut newX: ::core::ffi::c_int = 0;
    let mut newY: ::core::ffi::c_int = 0;
    let mut keyPlaceX = placeX;
    let mut keyPlaceY = placeY;
    let mut keyPlaceWidth = placeWidth;
    let mut keyPlaceHeight = placeHeight;
    placeKeyMultiplier = 1 as ::core::ffi::c_int;
    if (*pev).type_0 == KeyPress {
        while placeKeyMultiplier <= 10 as ::core::ffi::c_int
            && XCheckIfEvent(
                wmGD.display,
                &raw mut KeyEvent,
                Some(
                    IsRepeatedKeyEvent
                        as unsafe extern "C" fn(
                            *mut Display,
                            *mut XEvent,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                pev as XPointer,
            ) != 0
        {
            placeKeyMultiplier += 1;
        }
    }
    keysym = XkbKeycodeToKeysym(
        wmGD.display,
        (*pev).keycode as KeyCode,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    control = ((*pev).state & ControlMask as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint)
        as ::core::ffi::c_int as Boolean;
    big_inc = (*(*(wmGD.display as _XPrivDisplay))
        .screens
        .offset((*wmGD.pActiveSD).screen as isize))
    .width
        / 20 as ::core::ffi::c_int;
    valid = FALSE as Boolean;
    match keysym {
        65361 => {
            tmpX = if control as ::core::ffi::c_int != 0 {
                -big_inc
            } else {
                -(1 as ::core::ffi::c_int)
            };
            valid = TRUE as Boolean;
        }
        65362 => {
            tmpY = if control as ::core::ffi::c_int != 0 {
                -big_inc
            } else {
                -(1 as ::core::ffi::c_int)
            };
            valid = TRUE as Boolean;
        }
        65363 => {
            tmpX = if control as ::core::ffi::c_int != 0 {
                big_inc
            } else {
                1 as ::core::ffi::c_int
            };
            valid = TRUE as Boolean;
        }
        65364 => {
            tmpY = if control as ::core::ffi::c_int != 0 {
                big_inc
            } else {
                1 as ::core::ffi::c_int
            };
            valid = TRUE as Boolean;
        }
        32 => {
            StartInteractiveSizing(pcd, (*pev).time);
        }
        65293 => {
            placementDone = TRUE as Boolean;
        }
        _ => {}
    }
    if valid != 0 {
        tmpX *= placeKeyMultiplier;
        tmpY *= placeKeyMultiplier;
        if placeResize != 0 {
            keyPlaceWidth = keyPlaceWidth.wrapping_add(tmpX as ::core::ffi::c_uint);
            keyPlaceHeight = keyPlaceHeight.wrapping_add(tmpY as ::core::ffi::c_uint);
            FixFrameValues(
                pcd,
                &raw mut keyPlaceX,
                &raw mut keyPlaceY,
                &raw mut keyPlaceWidth,
                &raw mut keyPlaceHeight,
                placeResize,
            );
            warpX = (keyPlaceX as ::core::ffi::c_uint)
                .wrapping_add(keyPlaceWidth)
                .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            warpY = (keyPlaceY as ::core::ffi::c_uint)
                .wrapping_add(keyPlaceHeight)
                .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            SetPointerPosition(warpX, warpY, &raw mut newX, &raw mut newY);
            if warpX == newX && warpY == newY {
                placeWidth = keyPlaceWidth;
                placeHeight = keyPlaceHeight;
            } else {
                placeWidth = (newX - keyPlaceX + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
                placeHeight = (newY - keyPlaceY + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
            }
        } else {
            keyPlaceX += tmpX;
            keyPlaceY += tmpY;
            FixFrameValues(
                pcd,
                &raw mut keyPlaceX,
                &raw mut keyPlaceY,
                &raw mut keyPlaceWidth,
                &raw mut keyPlaceHeight,
                placeResize,
            );
            warpX = keyPlaceX;
            warpY = keyPlaceY;
            SetPointerPosition(warpX, warpY, &raw mut newX, &raw mut newY);
            placeX = newX;
            placeY = newY;
        }
        placePointerX = newX;
        placePointerY = newY;
    }
    FixFrameValues(
        pcd,
        &raw mut placeX,
        &raw mut placeY,
        &raw mut placeWidth,
        &raw mut placeHeight,
        placeResize,
    );
    MoveOutline(placeX, placeY, placeWidth, placeHeight);
    if wmGD.showFeedback as ::core::ffi::c_long & WM_SHOW_FB_PLACEMENT != 0 {
        DoFeedback(
            pcd,
            placeX,
            placeY,
            placeWidth,
            placeHeight,
            0 as ::core::ffi::c_ulong,
            placeResize,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandlePlacementButtonEvent(mut pev: *mut XButtonEvent) {
    if (*pev).button == 1 as ::core::ffi::c_uint {
        if (*pev).type_0 == ButtonRelease {
            placementDone = TRUE as Boolean;
        } else if (*pev).type_0 == ButtonPress {
            wmGD.preMoveX = (*pev).x_root;
            wmGD.preMoveY = (*pev).y_root;
            wmGD.preMove = TRUE as Boolean;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandlePlacementMotionEvent(
    mut pcd: *mut ClientData,
    mut pev: *mut XMotionEvent,
) {
    let mut diffx: ::core::ffi::c_int = 0;
    let mut diffy: ::core::ffi::c_int = 0;
    if wmGD.preMove != 0 {
        diffx = (*pev).x_root - wmGD.preMoveX;
        diffy = (*pev).y_root - wmGD.preMoveY;
        if (if diffx > 0 as ::core::ffi::c_int {
            diffx
        } else {
            -diffx
        }) > wmGD.moveThreshold
            || (if diffy > 0 as ::core::ffi::c_int {
                diffy
            } else {
                -diffy
            }) > wmGD.moveThreshold
        {
            StartInteractiveSizing(pcd, (*pev).time);
        }
        return;
    }
    if placeResize != 0 {
        if (*pev).x_root > placeX {
            placeWidth = ((*pev).x_root - placeX + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        }
        if (*pev).y_root > placeY {
            placeHeight = ((*pev).y_root - placeY + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        }
    } else {
        placeX = (*pev).x_root;
        placeY = (*pev).y_root;
    }
    placePointerX = (*pev).x_root;
    placePointerY = (*pev).y_root;
    FixFrameValues(
        pcd,
        &raw mut placeX,
        &raw mut placeY,
        &raw mut placeWidth,
        &raw mut placeHeight,
        placeResize,
    );
    MoveOutline(placeX, placeY, placeWidth, placeHeight);
    if wmGD.showFeedback as ::core::ffi::c_long & WM_SHOW_FB_PLACEMENT != 0 {
        DoFeedback(
            pcd,
            placeX,
            placeY,
            placeWidth,
            placeHeight,
            0 as ::core::ffi::c_ulong,
            placeResize,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn DoPlacement(mut pcd: *mut ClientData) {
    let mut event = _XEvent { type_0: 0 };
    SetupPlacement(pcd);
    placementDone = FALSE as Boolean;
    while placementDone == 0 {
        GetConfigEvent(
            wmGD.display,
            (*wmGD.pActiveSD).rootWindow,
            GRAB_MASK as ::core::ffi::c_ulong,
            placePointerX,
            placePointerY,
            placeX,
            placeY,
            placeWidth,
            placeHeight,
            &raw mut event,
        );
        match event.type_0 {
            KeyPress => {
                HandlePlacementKeyEvent(pcd, &raw mut event as *mut XKeyEvent);
            }
            ButtonPress => {
                HandlePlacementButtonEvent(&raw mut event as *mut XButtonEvent);
            }
            ButtonRelease => {
                HandlePlacementButtonEvent(&raw mut event as *mut XButtonEvent);
            }
            MotionNotify => {
                HandlePlacementMotionEvent(pcd, &raw mut event as *mut XMotionEvent);
            }
            _ => {}
        }
    }
    (*pcd).clientX = placeX + placeOffsetX;
    (*pcd).clientY = placeY + placeOffsetY;
    (*pcd).clientWidth = placeWidth
        .wrapping_sub((2 as ::core::ffi::c_int * placeOffsetX) as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    (*pcd).clientHeight = placeHeight
        .wrapping_sub(placeOffsetX as ::core::ffi::c_uint)
        .wrapping_sub(placeOffsetY as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    MoveOutline(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
    );
    HideFeedbackWindow((*pcd).pSD);
}
#[no_mangle]
pub unsafe extern "C" fn PlaceWindowInteractively(mut pcd: *mut ClientData) {
    let mut gmask: ::core::ffi::c_uint = 0;
    gmask = (if wmGD.useWindowOutline as ::core::ffi::c_int != 0 {
        NOFRZ_PGRAB_MASK
    } else {
        PGRAB_MASK
    }) as ::core::ffi::c_uint;
    if DoGrabs(
        (*wmGD.pActiveSD).rootWindow,
        wmGD.movePlacementCursor,
        gmask,
        CurrentTime as Time,
        pcd,
        True as Boolean,
    ) == 0
    {
        return;
    }
    DoPlacement(pcd);
    UndoGrabs();
    wmGD.preMove = FALSE as Boolean;
}
