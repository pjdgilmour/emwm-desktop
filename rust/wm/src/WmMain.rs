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
use crate::abi::*;
use ::c2rust_bitfields;
extern "C" {
    static mut stdout: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn XtDispatchEvent(_: *mut XEvent) -> Boolean;
    fn XtAppNextEvent(_: XtAppContext, _: *mut XEvent);
    fn XtSetLanguageProc(_: XtAppContext, _: XtLanguageProc, _: XtPointer) -> XtLanguageProc;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn WmDispatchClientEvent(event: *mut XEvent) -> Boolean;
    fn WmDispatchMenuEvent(event: *mut XButtonEvent) -> Boolean;
    fn WmDispatchWsEvent(event: *mut XEvent) -> Boolean;
    fn ReplayedButtonEvent(pevB1: *mut XButtonEvent, pevB2: *mut XButtonEvent) -> Boolean;
    fn InitWmGlobal(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
        environ: *mut *mut ::core::ffi::c_char,
    );
    fn IsBackdropWindow(pSD_0: *mut WmScreenData, win: Window) -> Boolean;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const ButtonPress: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ButtonRelease: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XmVERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XmREVISION: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const XmUPDATE_LEVEL: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MWM_NAME: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"EMWM\0") };
pub const MWM_VERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MWM_REVISION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
#[no_mangle]
pub static mut pSD: *mut WmScreenData = ::core::ptr::null::<WmScreenData>() as *mut WmScreenData;
#[no_mangle]
pub static mut wmGD: WmGlobalData = _WmGlobalData {
    dataType: 0,
    argv: ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char,
    environ: ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char,
    mwmName: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    topLevelW: ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec,
    topLevelW1: ::core::ptr::null::<_WidgetRec>() as *mut _WidgetRec,
    confirmDialogMapped: 0,
    mwmAppContext: ::core::ptr::null::<_XtAppStruct>() as *mut _XtAppStruct,
    windowContextType: 0,
    screenContextType: 0,
    cmapWindowContextType: 0,
    mwmWindowContextType: 0,
    commandWindow: 0,
    displayString: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    display: ::core::ptr::null::<Display>() as *mut Display,
    display1: ::core::ptr::null::<Display>() as *mut Display,
    statusColorServer: 0,
    numScreens: 0,
    screenNames: ::core::ptr::null::<*mut ::core::ffi::c_uchar>() as *mut *mut ::core::ffi::c_uchar,
    Screens: ::core::ptr::null::<WmScreenData>() as *mut WmScreenData,
    workspaceCursor: 0,
    stretchCursors: [0; 8],
    configCursor: 0,
    movePlacementCursor: 0,
    sizePlacementCursor: 0,
    pActiveSD: ::core::ptr::null::<WmScreenData>() as *mut WmScreenData,
    useStandardBehavior: 0,
    wmRestarted: 0,
    errorFlag: 0,
    errorResource: 0,
    errorRequestCode: 0,
    menuActive: ::core::ptr::null::<MenuSpec>() as *mut MenuSpec,
    menuUnpostKeySpec: ::core::ptr::null::<KeySpec>() as *mut KeySpec,
    menuClient: ::core::ptr::null::<ClientData>() as *mut ClientData,
    F_NextKeySpec: ::core::ptr::null::<KeySpec>() as *mut KeySpec,
    F_PrevKeySpec: ::core::ptr::null::<KeySpec>() as *mut KeySpec,
    grabContext: 0,
    clickData: _ClickData {
        clickPending: 0,
        doubleClickPending: 0,
        bReplayed: 0,
        button: 0,
        state: 0,
        releaseState: 0,
        pCD: ::core::ptr::null::<_ClientData>() as *mut _ClientData,
        context: 0,
        subContext: 0,
        clickContext: 0,
        doubleClickContext: 0,
        time: 0,
    },
    configAction: 0,
    configButton: 0,
    configPart: 0,
    configSet: 0,
    movingIcon: 0,
    preMove: 0,
    preMoveX: 0,
    preMoveY: 0,
    gadgetClient: ::core::ptr::null::<ClientData>() as *mut ClientData,
    gadgetDepressed: 0,
    checkHotspot: 0,
    hotspotRectangle: XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    },
    wmTimers: ::core::ptr::null::<WmTimer>() as *mut WmTimer,
    passKeysActive: 0,
    passKeysKeySpec: ::core::ptr::null::<KeySpec>() as *mut KeySpec,
    activeIconTextDisplayed: 0,
    queryScreen: 0,
    keyboardFocus: ::core::ptr::null::<ClientData>() as *mut ClientData,
    nextKeyboardFocus: ::core::ptr::null::<ClientData>() as *mut ClientData,
    systemModalActive: 0,
    systemModalClient: ::core::ptr::null::<ClientData>() as *mut ClientData,
    systemModalWindow: 0,
    clientResourceDB: ::core::ptr::null::<_XrmHashBucketRec>() as *mut _XrmHashBucketRec,
    dbFileName: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    sessionClientDB: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    xa_WM_STATE: 0,
    xa_WM_PROTOCOLS: 0,
    xa_WM_CHANGE_STATE: 0,
    xa_WM_SAVE_YOURSELF: 0,
    xa_WM_DELETE_WINDOW: 0,
    xa_WM_TAKE_FOCUS: 0,
    xa_WM_COLORMAP_WINDOWS: 0,
    xa_MWM_HINTS: 0,
    xa_MWM_MESSAGES: 0,
    xa_MWM_MENU: 0,
    xa_MWM_INFO: 0,
    xa_MWM_OFFSET: 0,
    xa_MWM_CLIENT_LIST: 0,
    xa_MOTIF_BINDINGS: 0,
    xa_COMPOUND_TEXT: 0,
    xa_SM_CLIENT_ID: 0,
    xa_WMSAVE_HINT: 0,
    xa_MWM_WM_REQUEST: 0,
    xa_MWM_WORKSPACE_HINTS: 0,
    xa_MWM_WORKSPACE_PRESENCE: 0,
    xa_MWM_WORKSPACE_INFO: 0,
    xa_MWM_WORKSPACE_LIST: 0,
    xa_MWM_WORKSPACE_CURRENT: 0,
    xa_ALL_WORKSPACES: 0,
    autoKeyFocus: 0,
    autoRaiseDelay: 0,
    bitmapDirectory: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    backdropDirs: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    clientAutoPlace: 0,
    colormapFocusPolicy: 0,
    configFile: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    cppCommand: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    deiconifyKeyFocus: 0,
    doubleClickTime: 0,
    enableWarp: 0,
    enforceKeyFocus: 0,
    useWindowOutline: 0,
    outlineWidth: 0,
    iconAutoPlace: 0,
    iconClick: 0,
    interactivePlacement: 0,
    keyboardFocusPolicy: 0,
    lowerOnIconify: 0,
    moveThreshold: 0,
    passButtonsCheck: 0,
    passButtons: 0,
    passSelectButton: 0,
    positionOnScreen: 0,
    quitTimeout: 0,
    raiseKeyFocus: 0,
    multiScreen: 0,
    screenList: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    showFeedback: 0,
    rootButtonClick: 0,
    refreshByClearing: 0,
    startupKeyFocus: 0,
    systemButtonClick: 0,
    systemButtonClick2: 0,
    useLargeCursors: 0,
    requestContextWin: 0,
    waitForClicks: 0,
    frameStyle: WmRECESSED,
    evLastButton: XButtonEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: ::core::ptr::null::<Display>() as *mut Display,
        window: 0,
        root: 0,
        subwindow: 0,
        time: 0,
        x: 0,
        y: 0,
        x_root: 0,
        y_root: 0,
        state: 0,
        button: 0,
        same_screen: 0,
    },
    bReplayedButton: 0,
    iconExternalShadowWidth: 0,
    frameExternalShadowWidth: 0,
    marqueeSelectGranularity: 0,
    bSuspendSecondaryRestack: 0,
    clientDefaultTitle: ::core::ptr::null::<__XmStringRec>() as *mut __XmStringRec,
    iconDefaultTitle: ::core::ptr::null::<__XmStringRec>() as *mut __XmStringRec,
    attributesWindow: 0,
    windowAttributes: XWindowAttributes {
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
    },
    hasShape: 0,
    shapeEventBase: 0,
    shapeErrorBase: 0,
    replayEnterEvent: 0,
    savedEnterEvent: XCrossingEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: ::core::ptr::null::<Display>() as *mut Display,
        window: 0,
        root: 0,
        subwindow: 0,
        time: 0,
        x: 0,
        y: 0,
        x_root: 0,
        y_root: 0,
        mode: 0,
        detail: 0,
        same_screen: 0,
        focus: 0,
        state: 0,
    },
    lockingModMask: 0,
    pLockMaskSequence: ::core::ptr::null::<::core::ffi::c_uint>() as *mut ::core::ffi::c_uint,
    tmpBuffer: [0; 1024],
    numMouseButtons: 0,
    bMenuButton: 0,
    primaryXineramaScreen: 0,
    xineramaScreenFocus: 0,
    xineramaIconifyToPrimary: 0,
    xrandr_present: 0,
    xrandr_base_evt: 0,
    xrandr_base_err: 0,
};
// libc passes the original Unix argv and environ to Xt, including non-UTF-8 values.
#[cfg_attr(not(test), export_name = "main")]
pub unsafe extern "C" fn wm_main(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    environ: *mut *mut libc::c_char,
) -> libc::c_int {
    // SAFETY: argv and environ are supplied by libc at process startup. Xt
    // owns the display, event loop and window records for the rest of main.
    setlocale(libc::LC_ALL, c"".as_ptr());
    XtSetLanguageProc(std::ptr::null_mut(), None, std::ptr::null_mut());
    InitWmGlobal(argc, argv, environ);
    loop {
        let mut event = _XEvent { type_0: 0 };
        XtAppNextEvent(wmGD.mwmAppContext, &raw mut event);
        wmGD.attributesWindow = 0;
        if matches!(event.type_0, ButtonPress | ButtonRelease) {
            // XNextEvent's discriminator selects the XButtonEvent union member.
            let replay = wmGD.evLastButton.button != 0
                && ReplayedButtonEvent(&raw mut wmGD.evLastButton, &raw mut event.xbutton) != 0;
            if !replay {
                wmGD.evLastButton = event.xbutton;
            }
            wmGD.bReplayedButton = replay.into();
        }

        if !wmGD.menuActive.is_null()
            && (wmGD.checkHotspot != 0
                || !wmGD.menuUnpostKeySpec.is_null()
                || !(*wmGD.menuActive).accelKeySpecs.is_null())
            && WmDispatchMenuEvent(&raw mut event.xbutton) == 0
        {
            continue;
        }

        let window = event.xany.window;
        let managed_root = if XFindContext(
            wmGD.display,
            window,
            wmGD.screenContextType,
            (&raw mut pSD).cast(),
        ) == 0
        {
            wmGD.pActiveSD = pSD;
            wmGD.queryScreen = 0;
            true
        } else {
            IsBackdropWindow(wmGD.pActiveSD, window) != 0
        };
        let dispatch_to_xt = if managed_root {
            WmDispatchWsEvent(&raw mut event)
        } else {
            WmDispatchClientEvent(&raw mut event)
        };
        if dispatch_to_xt != 0 {
            XtDispatchEvent(&raw mut event);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn PrintVersionInfo() {
    fprintf(
        stdout,
        b"%s - Version %d.%d (Motif %d.%d.%d)\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        MWM_NAME.as_ptr(),
        MWM_VERSION,
        MWM_REVISION,
        XmVERSION,
        XmREVISION,
        XmUPDATE_LEVEL,
    );
}
