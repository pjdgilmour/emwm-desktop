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
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_1;
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
use ::c2rust_bitfields;
extern "C" {
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn XGetTextProperty(
        _: *mut Display,
        _: Window,
        _: *mut XTextProperty,
        _: Atom,
    ) -> ::core::ffi::c_int;
    fn XGetWMClientMachine(_: *mut Display, _: Window, _: *mut XTextProperty)
        -> ::core::ffi::c_int;
    fn XrmStringToQuark(_: *const ::core::ffi::c_char) -> XrmQuark;
    fn XrmDestroyDatabase(_: XrmDatabase);
    fn XrmPutLineResource(_: *mut XrmDatabase, _: *const ::core::ffi::c_char);
    fn XrmGetResource(
        _: XrmDatabase,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: *mut XrmValue,
    ) -> ::core::ffi::c_int;
    fn XrmGetFileDatabase(_: *const ::core::ffi::c_char) -> XrmDatabase;
    fn XrmEnumerateDatabase(
        _: XrmDatabase,
        _: XrmNameList,
        _: XrmClassList,
        _: ::core::ffi::c_int,
        _: Option<
            unsafe extern "C" fn(
                *mut XrmDatabase,
                XrmBindingList,
                XrmQuarkList,
                *mut XrmRepresentation,
                *mut XrmValue,
                XPointer,
            ) -> ::core::ffi::c_int,
        >,
        _: XPointer,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XtAddCallback(_: Widget, _: *const ::core::ffi::c_char, _: XtCallbackProc, _: XtPointer);
    fn XtSetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtVaSetValues(_: Widget, ...);
    fn XtGetValues(_: Widget, _: ArgList, _: Cardinal);
    fn XtVaGetValues(_: Widget, ...);
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtRealloc(_: *mut ::core::ffi::c_char, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    static XtShellStrings: [::core::ffi::c_char; 0];
    fn XmeGetHomeDirName() -> String_0;
    static mut wmGD: WmGlobalData;
    fn GetWorkspaceData(pSD: *mut WmScreenData, wsID: WorkspaceID) -> *mut WmWorkspaceData;
}
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
pub const XrmBindLoosely: C2RustUnnamed_1 = 1;
pub const XrmBindTightly: C2RustUnnamed_1 = 0;
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const NULLQUARK: XrmQuark = 0 as ::core::ffi::c_int;
pub const XrmEnumOneLevel: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SmSaveLocal: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SmSaveBoth: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XA_STRING: Atom = 31 as ::core::ffi::c_int as Atom;
pub const XA_WM_COMMAND: Atom = 34 as ::core::ffi::c_int as Atom;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const UNSEEN_STATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SM_X: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 18 as ::core::ffi::c_int;
pub const SM_Y: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 19 as ::core::ffi::c_int;
pub const SM_WIDTH: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 20 as ::core::ffi::c_int;
pub const SM_HEIGHT: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 21 as ::core::ffi::c_int;
pub const SM_CLIENT_STATE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 22 as ::core::ffi::c_int;
pub const SM_ICON_X: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 23 as ::core::ffi::c_int;
pub const SM_ICON_Y: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 24 as ::core::ffi::c_int;
pub const WMSAVE_X: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const WMSAVE_Y: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const WMSAVE_WIDTH: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const WMSAVE_HEIGHT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const WMSAVE_STATE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const WMSAVE_WORKSPACES: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const WMSAVE_ICON_X: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const WMSAVE_ICON_Y: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
static mut wmFileName: *mut ::core::ffi::c_char =
    b".mwmclientdb\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
pub const EXTRA_FN_CHARS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
static mut xPositionStr: *mut ::core::ffi::c_char =
    b"%s.position.x\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut yPositionStr: *mut ::core::ffi::c_char =
    b"%s.position.y\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut widthSizeStr: *mut ::core::ffi::c_char =
    b"%s.size.width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut heightSizeStr: *mut ::core::ffi::c_char =
    b"%s.size.height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut initialStateStr: *mut ::core::ffi::c_char =
    b"%s.initialState\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut wmCommandStr: *mut ::core::ffi::c_char =
    b"%s.wmCommand\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut wmClientMachineStr: *mut ::core::ffi::c_char =
    b"%s.wmClientMachine\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut screenStr: *mut ::core::ffi::c_char =
    b"%s.screen\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut workspacesStr: *mut ::core::ffi::c_char =
    b"%s.workspaces\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut iconXPosStr: *mut ::core::ffi::c_char =
    b"%s.iconPos.x.%s\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut iconYPosStr: *mut ::core::ffi::c_char =
    b"%s.iconPos.y.%s\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut dbHeader: *mut ::core::ffi::c_char = b"! %s\n!\n.version: %s\n.mwmID: %s\n\0"
    as *const u8 as *const ::core::ffi::c_char
    as *mut ::core::ffi::c_char;
static mut dbClientFormat: *mut ::core::ffi::c_char =
    b"!\n%s.%s: %s\n!\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut intArg: *mut ::core::ffi::c_char =
    b": %d\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut strArg: *mut ::core::ffi::c_char =
    b": %s\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut normalStateStr: *mut ::core::ffi::c_char =
    b"NormalState\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut iconicStateStr: *mut ::core::ffi::c_char =
    b"IconicState\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut XSMPClientStr: *mut ::core::ffi::c_char =
    b"Client\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut proxyClientStr: *mut ::core::ffi::c_char =
    b"ProxyClient\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut dbFileArgStr: *mut ::core::ffi::c_char =
    b"-session\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut smClientDBCheckpointed: Boolean = False as Boolean;
unsafe extern "C" fn smSaveYourselfCallback(
    mut w: Widget,
    mut clientData: XtPointer,
    mut callData: XtPointer,
) {
    let mut cpToken = callData as XtCheckpointToken;
    let mut newClientDB = ::core::ptr::null_mut::<_XrmHashBucketRec>();
    static mut firstTime: Boolean = True as Boolean;
    if firstTime != 0 {
        firstTime = False as Boolean;
        return;
    }
    if (*cpToken).save_type != SmSaveLocal && (*cpToken).save_type != SmSaveBoth {
        return;
    }
    if (*cpToken).shutdown as ::core::ffi::c_int != 0
        && ((*cpToken).cancel_shutdown as ::core::ffi::c_int != 0
            || (*cpToken).request_cancel as ::core::ffi::c_int != 0
            || (*cpToken).save_success == 0)
    {
        return;
    }
    if (*cpToken).phase == 1 as ::core::ffi::c_int {
        (*cpToken).request_next_phase = True as Boolean;
        return;
    }
    newClientDB = SaveClientResourceDB();
    if !newClientDB.is_null() {
        if !wmGD.clientResourceDB.is_null() {
            XrmDestroyDatabase(wmGD.clientResourceDB);
        }
        wmGD.clientResourceDB = newClientDB;
        smClientDBCheckpointed = True as Boolean;
        if !wmGD.dbFileName.is_null() {
            let mut newRestartCmd = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
            let mut newDiscardCmd: [*mut ::core::ffi::c_char; 4] =
                [::core::ptr::null_mut::<::core::ffi::c_char>(); 4];
            let mut args: [Arg; 10] = [Arg {
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                value: 0,
            }; 10];
            let mut nargs: ::core::ffi::c_int = 0;
            newDiscardCmd[0 as ::core::ffi::c_int as usize] =
                b"rm\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            newDiscardCmd[1 as ::core::ffi::c_int as usize] =
                b"-f\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            newDiscardCmd[2 as ::core::ffi::c_int as usize] = wmGD.dbFileName;
            newDiscardCmd[3 as ::core::ffi::c_int as usize] = NULL_0 as *mut ::core::ffi::c_char;
            newRestartCmd = getNewRestartCmd();
            nargs = 0 as ::core::ffi::c_int;
            args[nargs as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
                .offset(1133 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            args[nargs as usize].value = newRestartCmd as XtArgVal;
            nargs += 1;
            args[nargs as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
                .offset(986 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0;
            args[nargs as usize].value =
                &raw mut newDiscardCmd as *mut *mut ::core::ffi::c_char as XtArgVal;
            nargs += 1;
            XtSetValues(wmGD.topLevelW, &raw mut args as ArgList, nargs as Cardinal);
            freeNewRestartCmd(newRestartCmd);
        }
    }
}
unsafe extern "C" fn smDieCallback(
    mut w: Widget,
    mut clientData: XtPointer,
    mut callData: XtPointer,
) {
    ExitWM(0 as ::core::ffi::c_int);
}
unsafe extern "C" fn buildDBFileName(
    mut fileNameBuf: *mut ::core::ffi::c_char,
    mut doingSave: Boolean,
) {
    strcpy(
        fileNameBuf as *mut ::core::ffi::c_char,
        if wmGD.dbFileName.is_null() {
            wmFileName
        } else {
            wmGD.dbFileName
        },
    );
}
unsafe extern "C" fn getClientDBName() {
    let mut argP = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    wmGD.dbFileName = NULL_0 as *mut ::core::ffi::c_char;
    if !wmGD.argv.is_null() {
        argP = wmGD.argv;
        while !(*argP).is_null() {
            if strcmp(*argP, dbFileArgStr) == 0 as ::core::ffi::c_int {
                argP = argP.offset(1);
                if !(*argP).is_null() {
                    wmGD.dbFileName = XtMalloc(
                        strlen(*argP)
                            .wrapping_add(1 as size_t)
                            .wrapping_add(EXTRA_FN_CHARS as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t)
                            as Cardinal,
                    );
                    if !wmGD.dbFileName.is_null() {
                        strcpy(wmGD.dbFileName, *argP);
                    }
                }
                break;
            } else {
                argP = argP.offset(1);
            }
        }
    }
    if wmGD.dbFileName.is_null() {
        if !wmGD.sessionClientDB.is_null() {
            wmGD.dbFileName = XtMalloc(
                strlen(wmGD.sessionClientDB as *const ::core::ffi::c_char)
                    .wrapping_add(1 as size_t)
                    .wrapping_add(EXTRA_FN_CHARS as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t)
                    as Cardinal,
            );
            if !wmGD.dbFileName.is_null() {
                strcpy(
                    wmGD.dbFileName,
                    wmGD.sessionClientDB as *const ::core::ffi::c_char,
                );
            }
        }
    }
    if wmGD.dbFileName.is_null() {
        let mut homeDir = XmeGetHomeDirName() as *mut ::core::ffi::c_char;
        wmGD.dbFileName = XtMalloc(
            strlen(homeDir)
                .wrapping_add(strlen(wmFileName))
                .wrapping_add(2 as size_t)
                .wrapping_add(EXTRA_FN_CHARS as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t)
                as Cardinal,
        );
        if !wmGD.dbFileName.is_null() {
            sprintf(
                wmGD.dbFileName,
                b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
                homeDir,
                wmFileName,
            );
        }
    }
}
unsafe extern "C" fn setClientDBName() {
    let mut ptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if wmGD.dbFileName.is_null() {
        return;
    }
    ptr = strrchr(wmGD.dbFileName, '.' as i32);
    if !ptr.is_null() {
        let mut p1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ptr = ptr.offset(1);
        p1 = ptr;
        while *p1 as ::core::ffi::c_int != '\0' as i32 {
            if *(*__ctype_b_loc()).offset(*p1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                == 0
            {
                break;
            }
            p1 = p1.offset(1);
        }
        if *p1 as ::core::ffi::c_int == '\0' as i32 {
            let mut numSuffix: ::core::ffi::c_int = 0;
            numSuffix = atoi(ptr) + 1 as ::core::ffi::c_int;
            sprintf(
                ptr,
                b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                numSuffix,
            );
            return;
        }
    }
    strcat(
        wmGD.dbFileName,
        b".0\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn getNewRestartCmd() -> *mut *mut ::core::ffi::c_char {
    let mut argP = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut argc: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut fileArgIndex = -(1 as ::core::ffi::c_int);
    let mut args: [Arg; 10] = [Arg {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: 0,
    }; 10];
    let mut nargs: ::core::ffi::c_int = 0;
    let mut restartCmd = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut newRestartCmd = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    nargs = 0 as ::core::ffi::c_int;
    args[nargs as usize].name = (&raw const XtShellStrings as *const ::core::ffi::c_char)
        .offset(1133 as ::core::ffi::c_int as isize)
        as *const ::core::ffi::c_char as String_0;
    args[nargs as usize].value = &raw mut restartCmd as XtArgVal;
    nargs += 1;
    XtGetValues(wmGD.topLevelW, &raw mut args as ArgList, nargs as Cardinal);
    if restartCmd.is_null() {
        return NULL_0 as *mut *mut ::core::ffi::c_char;
    }
    argc = 0 as ::core::ffi::c_int;
    argP = restartCmd;
    while !(*argP).is_null() {
        if strcmp(*argP, dbFileArgStr) == 0 as ::core::ffi::c_int {
            argP = argP.offset(1);
            if (*argP).is_null() {
                break;
            }
            let fresh1 = argc;
            argc = argc + 1;
            fileArgIndex = fresh1;
        }
        argP = argP.offset(1);
        argc += 1;
    }
    if fileArgIndex < 0 as ::core::ffi::c_int {
        fileArgIndex = argc;
        argc += 2 as ::core::ffi::c_int;
    }
    newRestartCmd = XtMalloc(
        ((argc + 1 as ::core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as Cardinal,
    ) as *mut *mut ::core::ffi::c_char;
    if newRestartCmd.is_null() {
        return NULL_0 as *mut *mut ::core::ffi::c_char;
    }
    i = 0 as ::core::ffi::c_int;
    while i < argc {
        if i != fileArgIndex {
            let ref mut fresh2 = *newRestartCmd.offset(i as isize);
            *fresh2 = if !(*restartCmd.offset(i as isize)).is_null() {
                strcpy(
                    XtMalloc(
                        (strlen(*restartCmd.offset(i as isize)) as Cardinal)
                            .wrapping_add(1 as Cardinal),
                    ),
                    *restartCmd.offset(i as isize),
                )
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
        } else {
            let fresh3 = i;
            i = i + 1;
            let ref mut fresh4 = *newRestartCmd.offset(fresh3 as isize);
            *fresh4 = if !dbFileArgStr.is_null() {
                strcpy(
                    XtMalloc((strlen(dbFileArgStr) as Cardinal).wrapping_add(1 as Cardinal)),
                    dbFileArgStr,
                )
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
            let ref mut fresh5 = *newRestartCmd.offset(i as isize);
            *fresh5 = if !wmGD.dbFileName.is_null() {
                strcpy(
                    XtMalloc((strlen(wmGD.dbFileName) as Cardinal).wrapping_add(1 as Cardinal)),
                    wmGD.dbFileName,
                )
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
        }
        i += 1;
    }
    let ref mut fresh6 = *newRestartCmd.offset(i as isize);
    *fresh6 = NULL_0 as *mut ::core::ffi::c_char;
    return newRestartCmd;
}
unsafe extern "C" fn freeNewRestartCmd(mut restartCmd: *mut *mut ::core::ffi::c_char) {
    if !restartCmd.is_null() {
        let mut tmp = restartCmd;
        while !(*restartCmd).is_null() {
            let fresh0 = restartCmd;
            restartCmd = restartCmd.offset(1);
            XtFree(*fresh0);
        }
        XtFree(tmp as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn getClientWorkspaces(mut pCD: *mut ClientData) -> *mut ::core::ffi::c_char {
    let mut pSD = (*pCD).pSD;
    let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut cwsP = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tmpP = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wsNameP = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pLen = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pCD).numInhabited {
        pWS = GetWorkspaceData(pSD, (*(*pCD).pWsList.offset(i as isize)).wsID);
        if !pWS.is_null() {
            wsNameP = (*pWS).name as *mut ::core::ffi::c_char;
            if pLen == 0 as ::core::ffi::c_int {
                pLen = strlen(wsNameP).wrapping_add(1 as size_t) as ::core::ffi::c_int;
                cwsP = XtMalloc(
                    (pLen as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                        as Cardinal,
                );
                if cwsP.is_null() {
                    return NULL_0 as *mut ::core::ffi::c_char;
                }
                strcpy(cwsP, wsNameP);
            } else {
                pLen = (pLen as size_t).wrapping_add(strlen(wsNameP).wrapping_add(1 as size_t))
                    as ::core::ffi::c_int as ::core::ffi::c_int;
                tmpP = XtRealloc(
                    cwsP,
                    (pLen as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                        as Cardinal,
                );
                if tmpP.is_null() {
                    XtFree(cwsP);
                    return NULL_0 as *mut ::core::ffi::c_char;
                }
                cwsP = tmpP;
                strcat(cwsP, b" \0" as *const u8 as *const ::core::ffi::c_char);
                strcat(cwsP, wsNameP);
            }
        }
        i += 1;
    }
    return cwsP;
}
unsafe extern "C" fn addClientToList(
    mut cdList: *mut *mut *mut ClientData,
    mut nClients: *mut ::core::ffi::c_int,
    mut pCD: *mut ClientData,
) -> Boolean {
    let mut newPtr = XtRealloc(
        *cdList as *mut ::core::ffi::c_char,
        ((*nClients + 1 as ::core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ClientData>() as usize) as Cardinal,
    ) as *mut *mut ClientData;
    if newPtr.is_null() {
        if !(*cdList).is_null() {
            XtFree(*cdList as *mut ::core::ffi::c_char);
        }
        return False as Boolean;
    }
    *cdList = newPtr;
    let ref mut fresh7 = *newPtr.offset(*nClients as isize);
    *fresh7 = pCD;
    *nClients += 1;
    return True as Boolean;
}
unsafe extern "C" fn clientWorkspaceCompare(
    mut ppCD1: *const ::core::ffi::c_void,
    mut ppCD2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pCD1 = *(ppCD1 as *mut *mut ClientData);
    let mut pCD2 = *(ppCD2 as *mut *mut ClientData);
    let mut screenDiff: ::core::ffi::c_int = 0;
    screenDiff = (*(*pCD1).pSD).screen - (*(*pCD2).pSD).screen;
    if screenDiff != 0 as ::core::ffi::c_int {
        return screenDiff;
    }
    return (*(*pCD1).pWsList.offset((*pCD1).currentWsc as isize))
        .wsID
        .wrapping_sub((*(*pCD2).pWsList.offset((*pCD2).currentWsc as isize)).wsID)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn getClientResource(
    mut clientID: *mut ::core::ffi::c_char,
    mut fmtStr: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut resourceBuf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut resourceType = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut resourceValue = XrmValue {
        size: 0,
        addr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    sprintf(
        &raw mut resourceBuf as *mut ::core::ffi::c_char,
        fmtStr,
        clientID,
    );
    if XrmGetResource(
        wmGD.clientResourceDB,
        &raw mut resourceBuf as *mut ::core::ffi::c_char,
        &raw mut resourceBuf as *mut ::core::ffi::c_char,
        &raw mut resourceType,
        &raw mut resourceValue,
    ) != 0
    {
        return resourceValue.addr;
    }
    return NULL_0 as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn getXSMPResource(
    mut pCD: *mut ClientData,
    mut resourceFlag: ::core::ffi::c_int,
    mut fmtStr: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if (*pCD).ignoreWMSaveHints as ::core::ffi::c_int != 0
        || (*pCD).wmSaveHintFlags & resourceFlag == 0
    {
        return getClientResource((*pCD).smClientID as *mut ::core::ffi::c_char, fmtStr);
    }
    return NULL_0 as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn findXSMPClientDBMatch(
    mut pCD: *mut ClientData,
    mut workSpaceNamesP: *mut *mut ::core::ffi::c_char,
) -> Boolean {
    if !(*pCD).smClientID.is_null() {
        if !wmGD.clientResourceDB.is_null() {
            let mut resourcePtr = ::core::ptr::null_mut::<::core::ffi::c_char>();
            resourcePtr = getXSMPResource(pCD, WMSAVE_X as ::core::ffi::c_int, xPositionStr);
            if !resourcePtr.is_null() {
                (*pCD).clientX = atoi(resourcePtr);
                (*pCD).clientFlags |= SM_X;
            }
            resourcePtr = getXSMPResource(pCD, WMSAVE_Y as ::core::ffi::c_int, yPositionStr);
            if !resourcePtr.is_null() {
                (*pCD).clientY = atoi(resourcePtr);
                (*pCD).clientFlags |= SM_Y;
            }
            resourcePtr = getXSMPResource(pCD, WMSAVE_ICON_X as ::core::ffi::c_int, iconXPosStr);
            if !resourcePtr.is_null() {
                (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconX = atoi(resourcePtr);
                (*pCD).clientFlags |= SM_ICON_X;
            }
            resourcePtr = getXSMPResource(pCD, WMSAVE_ICON_Y as ::core::ffi::c_int, iconYPosStr);
            if !resourcePtr.is_null() {
                (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).iconY = atoi(resourcePtr);
                (*pCD).clientFlags |= SM_ICON_Y;
            }
            resourcePtr = getXSMPResource(pCD, WMSAVE_WIDTH as ::core::ffi::c_int, widthSizeStr);
            if !resourcePtr.is_null() {
                (*pCD).clientWidth = atoi(resourcePtr);
                (*pCD).clientFlags |= SM_WIDTH;
            }
            resourcePtr = getXSMPResource(pCD, WMSAVE_HEIGHT as ::core::ffi::c_int, heightSizeStr);
            if !resourcePtr.is_null() {
                (*pCD).clientHeight = atoi(resourcePtr);
                (*pCD).clientFlags |= SM_HEIGHT;
            }
            resourcePtr = getXSMPResource(pCD, WMSAVE_STATE as ::core::ffi::c_int, initialStateStr);
            if !resourcePtr.is_null() {
                (*pCD).clientState =
                    if strcmp(resourcePtr, normalStateStr) == 0 as ::core::ffi::c_int {
                        NORMAL_STATE
                    } else {
                        MINIMIZED_STATE
                    };
                (*pCD).clientFlags |= SM_CLIENT_STATE;
            }
            if !workSpaceNamesP.is_null() && {
                resourcePtr =
                    getXSMPResource(pCD, WMSAVE_WORKSPACES as ::core::ffi::c_int, workspacesStr);
                !resourcePtr.is_null()
            } {
                *workSpaceNamesP = if !resourcePtr.is_null() {
                    strcpy(
                        XtMalloc((strlen(resourcePtr) as Cardinal).wrapping_add(1 as Cardinal)),
                        resourcePtr,
                    )
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                };
            }
        }
        return True as Boolean;
    }
    return False as Boolean;
}
unsafe extern "C" fn getProxyClientInfo(
    mut pCD: *mut ClientData,
    mut proxyClientInfo: *mut ProxyClientInfo,
) -> Boolean {
    let mut textProperty = XTextProperty {
        value: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        encoding: 0,
        format: 0,
        nitems: 0,
    };
    let mut i: ::core::ffi::c_ulong = 0;
    if XGetTextProperty(
        wmGD.display,
        (*pCD).client,
        &raw mut textProperty,
        XA_WM_COMMAND,
    ) == 0
    {
        return False as Boolean;
    }
    if textProperty.encoding != XA_STRING
        || textProperty.format != 8 as ::core::ffi::c_int
        || *textProperty.value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32
    {
        if !textProperty.value.is_null() {
            free(textProperty.value as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        }
        return False as Boolean;
    }
    i = 0 as ::core::ffi::c_ulong;
    while i < textProperty.nitems.wrapping_sub(1 as ::core::ffi::c_ulong) {
        if *textProperty.value.offset(i as isize) as ::core::ffi::c_int == '\0' as i32 {
            *textProperty.value.offset(i as isize) = ' ' as i32 as ::core::ffi::c_uchar;
        }
        i = i.wrapping_add(1);
    }
    (*proxyClientInfo).screen = (*(*pCD).pSD).screen;
    (*proxyClientInfo).wmCommand = textProperty.value as *mut ::core::ffi::c_char;
    if XGetWMClientMachine(wmGD.display, (*pCD).client, &raw mut textProperty) != 0 {
        (*proxyClientInfo).wmClientMachine = textProperty.value as *mut ::core::ffi::c_char;
    } else {
        (*proxyClientInfo).wmClientMachine = NULL_0 as *mut ::core::ffi::c_char;
    }
    (*proxyClientInfo).clientID = NULL_0 as *mut ::core::ffi::c_char;
    return True as Boolean;
}
unsafe extern "C" fn cmpProxyClientProc(
    mut clientDB: *mut XrmDatabase,
    mut bindingList: XrmBindingList,
    mut quarkList: XrmQuarkList,
    mut reps: *mut XrmRepresentation,
    mut value: *mut XrmValue,
    mut uData: XPointer,
) -> ::core::ffi::c_int {
    let mut clientScreen = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wmCommand = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wmClientMachine = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut clientID = (*value).addr;
    let mut proxyClientInfo = uData as *mut ProxyClientInfo;
    wmCommand = getClientResource(clientID, wmCommandStr);
    if wmCommand.is_null()
        || strcmp(wmCommand, (*proxyClientInfo).wmCommand) != 0 as ::core::ffi::c_int
        || {
            clientScreen = getClientResource(clientID, screenStr);
            clientScreen.is_null()
        }
        || atoi(clientScreen) != (*proxyClientInfo).screen
    {
        return FALSE;
    }
    if (*proxyClientInfo).wmClientMachine.is_null()
        || {
            wmClientMachine = getClientResource(clientID, wmClientMachineStr);
            wmClientMachine.is_null()
        }
        || strcmp((*proxyClientInfo).wmClientMachine, wmClientMachine) == 0 as ::core::ffi::c_int
    {
        (*proxyClientInfo).clientID = clientID;
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn findProxyClientID(mut pCD: *mut ClientData) -> *mut ::core::ffi::c_char {
    let mut proxyClientInfo = _ProxyClientInfo {
        screen: 0,
        wmCommand: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        wmClientMachine: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        clientID: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut clientID = NULL_0 as *mut ::core::ffi::c_char;
    static mut proxyName: [XrmName; 2] = [NULLQUARK, NULLQUARK];
    static mut proxyClass: [XrmClass; 2] = [NULLQUARK, NULLQUARK];
    if proxyName[0 as ::core::ffi::c_int as usize] == NULLQUARK {
        proxyName[0 as ::core::ffi::c_int as usize] = XrmStringToQuark(proxyClientStr) as XrmName;
        proxyClass[0 as ::core::ffi::c_int as usize] = XrmStringToQuark(proxyClientStr) as XrmClass;
    }
    if getProxyClientInfo(pCD, &raw mut proxyClientInfo) == 0 {
        return clientID;
    }
    if XrmEnumerateDatabase(
        wmGD.clientResourceDB,
        &raw mut proxyName as XrmNameList,
        &raw mut proxyClass as XrmClassList,
        XrmEnumOneLevel,
        Some(
            cmpProxyClientProc
                as unsafe extern "C" fn(
                    *mut XrmDatabase,
                    XrmBindingList,
                    XrmQuarkList,
                    *mut XrmRepresentation,
                    *mut XrmValue,
                    XPointer,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut proxyClientInfo as XPointer,
    ) != 0
    {
        clientID = proxyClientInfo.clientID;
    }
    if !proxyClientInfo.wmCommand.is_null() {
        free(proxyClientInfo.wmCommand as *mut ::core::ffi::c_void);
    }
    if !proxyClientInfo.wmClientMachine.is_null() {
        free(proxyClientInfo.wmClientMachine as *mut ::core::ffi::c_void);
    }
    return clientID;
}
unsafe extern "C" fn findProxyClientDBMatch(
    mut pCD: *mut ClientData,
    mut workSpaceNamesP: *mut *mut ::core::ffi::c_char,
) -> Boolean {
    if (*pCD).smClientID.is_null()
        && !wmGD.clientResourceDB.is_null()
        && smClientDBCheckpointed == 0
    {
        let mut proxyClientID = ::core::ptr::null_mut::<::core::ffi::c_char>();
        proxyClientID = findProxyClientID(pCD);
        if !proxyClientID.is_null() {
            let mut resourcePtr = ::core::ptr::null_mut::<::core::ffi::c_char>();
            resourcePtr = getClientResource(proxyClientID, xPositionStr);
            if !resourcePtr.is_null() {
                (*pCD).clientX = atoi(resourcePtr);
                (*pCD).clientFlags |= SM_X;
            }
            resourcePtr = getClientResource(proxyClientID, yPositionStr);
            if !resourcePtr.is_null() {
                (*pCD).clientY = atoi(resourcePtr);
                (*pCD).clientFlags |= SM_Y;
            }
            resourcePtr = getClientResource(proxyClientID, widthSizeStr);
            if !resourcePtr.is_null() {
                (*pCD).clientWidth = atoi(resourcePtr);
                (*pCD).clientFlags |= SM_WIDTH;
            }
            resourcePtr = getClientResource(proxyClientID, heightSizeStr);
            if !resourcePtr.is_null() {
                (*pCD).clientHeight = atoi(resourcePtr);
                (*pCD).clientFlags |= SM_HEIGHT;
            }
            resourcePtr = getClientResource(proxyClientID, initialStateStr);
            if !resourcePtr.is_null() {
                (*pCD).clientState =
                    if strcmp(resourcePtr, normalStateStr) == 0 as ::core::ffi::c_int {
                        NORMAL_STATE
                    } else {
                        MINIMIZED_STATE
                    };
                (*pCD).clientFlags |= SM_CLIENT_STATE;
            }
            if !workSpaceNamesP.is_null() && {
                resourcePtr = getClientResource(proxyClientID, workspacesStr);
                !resourcePtr.is_null()
            } {
                *workSpaceNamesP = if !resourcePtr.is_null() {
                    strcpy(
                        XtMalloc((strlen(resourcePtr) as Cardinal).wrapping_add(1 as Cardinal)),
                        resourcePtr,
                    )
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                };
            }
            return True as Boolean;
        }
    }
    return False as Boolean;
}
unsafe extern "C" fn getClientGeometry(
    mut pCD: *mut ClientData,
    mut clientX: *mut ::core::ffi::c_int,
    mut clientY: *mut ::core::ffi::c_int,
    mut clientWd: *mut ::core::ffi::c_uint,
    mut clientHt: *mut ::core::ffi::c_uint,
) {
    *clientX = (*pCD).clientX;
    *clientY = (*pCD).clientY;
    *clientWd = (if (*pCD).widthInc != 0 as ::core::ffi::c_int {
        ((*pCD).clientWidth - (*pCD).baseWidth) / (*pCD).widthInc
    } else {
        (*pCD).clientWidth
    }) as ::core::ffi::c_uint;
    *clientHt = (if (*pCD).heightInc != 0 as ::core::ffi::c_int {
        ((*pCD).clientHeight - (*pCD).baseHeight) / (*pCD).heightInc
    } else {
        (*pCD).clientHeight
    }) as ::core::ffi::c_uint;
}
unsafe extern "C" fn saveXSMPClient(mut fp: *mut FILE, mut pCD: *mut ClientData) -> Boolean {
    let mut clientX: ::core::ffi::c_int = 0;
    let mut clientY: ::core::ffi::c_int = 0;
    let mut clientWd: ::core::ffi::c_uint = 0;
    let mut clientHt: ::core::ffi::c_uint = 0;
    let mut clientID = (*pCD).smClientID as *mut ::core::ffi::c_char;
    fprintf(fp, dbClientFormat, XSMPClientStr, clientID, clientID);
    getClientGeometry(
        pCD,
        &raw mut clientX,
        &raw mut clientY,
        &raw mut clientWd,
        &raw mut clientHt,
    );
    if (*pCD).ignoreWMSaveHints as ::core::ffi::c_int != 0
        || (*pCD).wmSaveHintFlags as ::core::ffi::c_long
            & (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int
            == 0
    {
        fprintf(fp, xPositionStr, clientID);
        fprintf(fp, intArg, clientX);
    }
    if (*pCD).ignoreWMSaveHints as ::core::ffi::c_int != 0
        || (*pCD).wmSaveHintFlags as ::core::ffi::c_long
            & (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int
            == 0
    {
        fprintf(fp, yPositionStr, clientID);
        fprintf(fp, intArg, clientY);
    }
    if (*(*pCD).pSD).useIconBox == 0 {
        let mut pSD = (*pCD).pSD;
        let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pCD).numInhabited {
            pWS = GetWorkspaceData(pSD, (*(*pCD).pWsList.offset(i as isize)).wsID);
            if !pWS.is_null() {
                if (*pCD).ignoreWMSaveHints as ::core::ffi::c_int != 0
                    || (*pCD).wmSaveHintFlags as ::core::ffi::c_long
                        & (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int
                        == 0
                {
                    fprintf(fp, iconXPosStr, clientID, (*pWS).name);
                    fprintf(fp, intArg, (*(*pCD).pWsList.offset(i as isize)).iconX);
                }
                if (*pCD).ignoreWMSaveHints as ::core::ffi::c_int != 0
                    || (*pCD).wmSaveHintFlags as ::core::ffi::c_long
                        & (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int
                        == 0
                {
                    fprintf(fp, iconYPosStr, clientID, (*pWS).name);
                    fprintf(fp, intArg, (*(*pCD).pWsList.offset(i as isize)).iconY);
                }
            }
            i += 1;
        }
    }
    if (*pCD).ignoreWMSaveHints as ::core::ffi::c_int != 0
        || (*pCD).wmSaveHintFlags as ::core::ffi::c_long
            & (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int
            == 0
    {
        fprintf(fp, widthSizeStr, clientID);
        fprintf(fp, intArg, clientWd);
    }
    if (*pCD).ignoreWMSaveHints as ::core::ffi::c_int != 0
        || (*pCD).wmSaveHintFlags as ::core::ffi::c_long
            & (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int
            == 0
    {
        fprintf(fp, heightSizeStr, clientID);
        fprintf(fp, intArg, clientHt);
    }
    if (*pCD).ignoreWMSaveHints as ::core::ffi::c_int != 0
        || (*pCD).wmSaveHintFlags as ::core::ffi::c_long
            & (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int
            == 0
    {
        let mut clientState: ::core::ffi::c_int = 0;
        clientState = (*pCD).clientState & !UNSEEN_STATE;
        fprintf(fp, initialStateStr, clientID);
        fprintf(
            fp,
            strArg,
            if clientState == NORMAL_STATE {
                normalStateStr
            } else {
                iconicStateStr
            },
        );
    }
    if (*pCD).ignoreWMSaveHints as ::core::ffi::c_int != 0
        || (*pCD).wmSaveHintFlags as ::core::ffi::c_long
            & (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int
            == 0
    {
        let mut clientWorkspaces = getClientWorkspaces(pCD);
        if !clientWorkspaces.is_null() {
            fprintf(fp, workspacesStr, clientID);
            fprintf(fp, strArg, clientWorkspaces);
            XtFree(clientWorkspaces);
        }
    }
    return True as Boolean;
}
unsafe extern "C" fn saveProxyClient(
    mut fp: *mut FILE,
    mut pCD: *mut ClientData,
    mut clientIDNum: ::core::ffi::c_int,
) -> Boolean {
    let mut clientID: [::core::ffi::c_char; 50] = [0; 50];
    let mut clientState: ::core::ffi::c_int = 0;
    let mut proxyClientInfo = _ProxyClientInfo {
        screen: 0,
        wmCommand: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        wmClientMachine: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        clientID: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut clientX: ::core::ffi::c_int = 0;
    let mut clientY: ::core::ffi::c_int = 0;
    let mut clientWd: ::core::ffi::c_uint = 0;
    let mut clientHt: ::core::ffi::c_uint = 0;
    let mut clientWorkspaces = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if getProxyClientInfo(pCD, &raw mut proxyClientInfo) == 0 {
        return False as Boolean;
    }
    sprintf(
        &raw mut clientID as *mut ::core::ffi::c_char,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        clientIDNum,
    );
    fprintf(
        fp,
        dbClientFormat,
        proxyClientStr,
        &raw mut clientID as *mut ::core::ffi::c_char,
        &raw mut clientID as *mut ::core::ffi::c_char,
    );
    fprintf(fp, screenStr, &raw mut clientID as *mut ::core::ffi::c_char);
    fprintf(fp, intArg, proxyClientInfo.screen);
    fprintf(
        fp,
        wmCommandStr,
        &raw mut clientID as *mut ::core::ffi::c_char,
    );
    fprintf(fp, strArg, proxyClientInfo.wmCommand);
    free(proxyClientInfo.wmCommand as *mut ::core::ffi::c_void);
    if !proxyClientInfo.wmClientMachine.is_null() {
        fprintf(
            fp,
            wmClientMachineStr,
            &raw mut clientID as *mut ::core::ffi::c_char,
        );
        fprintf(fp, strArg, proxyClientInfo.wmClientMachine);
        free(proxyClientInfo.wmClientMachine as *mut ::core::ffi::c_void);
    }
    getClientGeometry(
        pCD,
        &raw mut clientX,
        &raw mut clientY,
        &raw mut clientWd,
        &raw mut clientHt,
    );
    fprintf(
        fp,
        xPositionStr,
        &raw mut clientID as *mut ::core::ffi::c_char,
    );
    fprintf(fp, intArg, clientX);
    fprintf(
        fp,
        yPositionStr,
        &raw mut clientID as *mut ::core::ffi::c_char,
    );
    fprintf(fp, intArg, clientY);
    if (*(*pCD).pSD).useIconBox == 0 {
        let mut pSD = (*pCD).pSD;
        let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pCD).numInhabited {
            pWS = GetWorkspaceData(pSD, (*(*pCD).pWsList.offset(i as isize)).wsID);
            if !pWS.is_null() {
                fprintf(
                    fp,
                    iconXPosStr,
                    &raw mut clientID as *mut ::core::ffi::c_char,
                    (*pWS).name,
                );
                fprintf(fp, intArg, (*(*pCD).pWsList.offset(i as isize)).iconX);
                fprintf(
                    fp,
                    iconYPosStr,
                    &raw mut clientID as *mut ::core::ffi::c_char,
                    (*pWS).name,
                );
                fprintf(fp, intArg, (*(*pCD).pWsList.offset(i as isize)).iconY);
            }
            i += 1;
        }
    }
    fprintf(
        fp,
        widthSizeStr,
        &raw mut clientID as *mut ::core::ffi::c_char,
    );
    fprintf(fp, intArg, clientWd);
    fprintf(
        fp,
        heightSizeStr,
        &raw mut clientID as *mut ::core::ffi::c_char,
    );
    fprintf(fp, intArg, clientHt);
    clientState = (*pCD).clientState & !UNSEEN_STATE;
    fprintf(
        fp,
        initialStateStr,
        &raw mut clientID as *mut ::core::ffi::c_char,
    );
    fprintf(
        fp,
        strArg,
        if clientState == NORMAL_STATE {
            normalStateStr
        } else {
            iconicStateStr
        },
    );
    clientWorkspaces = getClientWorkspaces(pCD);
    if !clientWorkspaces.is_null() {
        fprintf(
            fp,
            workspacesStr,
            &raw mut clientID as *mut ::core::ffi::c_char,
        );
        fprintf(fp, strArg, clientWorkspaces);
        XtFree(clientWorkspaces);
    }
    return True as Boolean;
}
unsafe extern "C" fn dbRemoveProxyClientEntry(mut proxyClientID: *mut ::core::ffi::c_char) {
    let mut resourceBuf: [::core::ffi::c_char; 1024] = [0; 1024];
    sprintf(
        &raw mut resourceBuf as *mut ::core::ffi::c_char,
        wmCommandStr,
        proxyClientID,
    );
    strcat(
        &raw mut resourceBuf as *mut ::core::ffi::c_char,
        b":\0" as *const u8 as *const ::core::ffi::c_char,
    );
    XrmPutLineResource(
        &raw mut wmGD.clientResourceDB,
        &raw mut resourceBuf as *mut ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn AddSMCallbacks() {
    XtAddCallback(
        wmGD.topLevelW,
        (&raw const XtShellStrings as *const ::core::ffi::c_char)
            .offset(1189 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        Some(smSaveYourselfCallback as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()),
        NULL_0,
    );
    XtAddCallback(
        wmGD.topLevelW,
        (&raw const XtShellStrings as *const ::core::ffi::c_char)
            .offset(974 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0 as *const ::core::ffi::c_char,
        Some(smDieCallback as unsafe extern "C" fn(Widget, XtPointer, XtPointer) -> ()),
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ResignFromSM() {
    if !wmGD.topLevelW.is_null() {
        XtVaSetValues(
            wmGD.topLevelW,
            (&raw const XtShellStrings as *const ::core::ffi::c_char)
                .offset(1057 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char as String_0,
            False,
            NULL_0,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExitWM(mut exitCode: ::core::ffi::c_int) -> ! {
    ResignFromSM();
    exit(exitCode);
}
#[no_mangle]
pub unsafe extern "C" fn LoadClientResourceDB() -> XrmDatabase {
    let mut dbFileName: [::core::ffi::c_char; 4096] = [0; 4096];
    getClientDBName();
    buildDBFileName(
        &raw mut dbFileName as *mut ::core::ffi::c_char,
        False as Boolean,
    );
    return XrmGetFileDatabase(&raw mut dbFileName as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn SaveClientResourceDB() -> XrmDatabase {
    let mut mySessionID = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dbFileName: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut fp = ::core::ptr::null_mut::<FILE>();
    let mut scr: ::core::ffi::c_int = 0;
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut clientIDNum = 0 as ::core::ffi::c_int;
    let mut pCL = ::core::ptr::null_mut::<ClientListEntry>();
    setClientDBName();
    buildDBFileName(
        &raw mut dbFileName as *mut ::core::ffi::c_char,
        True as Boolean,
    );
    fp = fopen(
        &raw mut dbFileName as *mut ::core::ffi::c_char,
        b"w\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if fp.is_null() {
        return NULL_0 as XrmDatabase;
    }
    XtVaGetValues(
        wmGD.topLevelW,
        (&raw const XtShellStrings as *const ::core::ffi::c_char)
            .offset(1223 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as String_0,
        &raw mut mySessionID,
        NULL_0,
    );
    fprintf(
        fp,
        dbHeader,
        wmFileName,
        b"mwm Version XSMP1.0\0" as *const u8 as *const ::core::ffi::c_char,
        if !mySessionID.is_null() {
            mySessionID as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    scr = 0 as ::core::ffi::c_int;
    while scr < wmGD.numScreens {
        pSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
        pCL = (*pSD).clientList;
        while !pCL.is_null() {
            if !((*pCL).type_0 != NORMAL_STATE) {
                pCD = (*pCL).pCD as *mut ClientData;
                if !(*pCD).smClientID.is_null() {
                    saveXSMPClient(fp, pCD);
                } else if saveProxyClient(fp, pCD, clientIDNum) != 0 {
                    clientIDNum += 1;
                }
            }
            pCL = (*pCL).nextSibling as *mut ClientListEntry;
        }
        scr += 1;
    }
    fclose(fp);
    return XrmGetFileDatabase(&raw mut dbFileName as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn FindClientDBMatch(
    mut pCD: *mut ClientData,
    mut workSpaceNamesP: *mut *mut ::core::ffi::c_char,
) -> Boolean {
    return (findXSMPClientDBMatch(pCD, workSpaceNamesP) as ::core::ffi::c_int != 0
        || findProxyClientDBMatch(pCD, workSpaceNamesP) as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn GetSmClientIdClientList(
    mut clients: *mut *mut *mut ClientData,
    mut nClients: *mut ::core::ffi::c_int,
) -> Boolean {
    let mut scr: ::core::ffi::c_int = 0;
    let mut pSD = ::core::ptr::null_mut::<WmScreenData>();
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut pCL = ::core::ptr::null_mut::<ClientListEntry>();
    *nClients = 0 as ::core::ffi::c_int;
    *clients = NULL_0 as *mut *mut ClientData;
    scr = 0 as ::core::ffi::c_int;
    while scr < wmGD.numScreens {
        pSD = wmGD.Screens.offset(scr as isize) as *mut WmScreenData;
        pCL = (*pSD).clientList;
        while !pCL.is_null() {
            if !((*pCL).type_0 != NORMAL_STATE) {
                pCD = (*pCL).pCD as *mut ClientData;
                if !(*pCD).smClientID.is_null() {
                    if addClientToList(clients, nClients, pCD) == 0 {
                        return False as Boolean;
                    }
                }
            }
            pCL = (*pCL).nextSibling as *mut ClientListEntry;
        }
        scr += 1;
    }
    return True as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn SortClientListByWorkspace(
    mut clients: *mut *mut ClientData,
    mut nClients: ::core::ffi::c_int,
) {
    if nClients > 0 as ::core::ffi::c_int {
        qsort(
            clients as *mut ::core::ffi::c_void,
            nClients as size_t,
            ::core::mem::size_of::<*mut ClientData>() as size_t,
            Some(
                clientWorkspaceCompare
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn LoadClientIconPositions(mut pCD: *mut ClientData) {
    let mut resourceBuf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut pSD = (*pCD).pSD;
    let mut pWS = ::core::ptr::null_mut::<WmWorkspaceData>();
    let mut i: ::core::ffi::c_int = 0;
    let mut resourcePtr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if wmGD.clientResourceDB.is_null() {
        return;
    }
    if !(*pCD).smClientID.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*pCD).numInhabited {
            pWS = GetWorkspaceData(pSD, (*(*pCD).pWsList.offset(i as isize)).wsID);
            if !pWS.is_null() {
                sprintf(
                    &raw mut resourceBuf as *mut ::core::ffi::c_char,
                    iconXPosStr,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*pWS).name,
                );
                resourcePtr = getXSMPResource(
                    pCD,
                    WMSAVE_ICON_X as ::core::ffi::c_int,
                    &raw mut resourceBuf as *mut ::core::ffi::c_char,
                );
                if !resourcePtr.is_null() {
                    (*(*pCD).pWsList.offset(i as isize)).iconX = atoi(resourcePtr);
                    (*pCD).clientFlags |= SM_ICON_X;
                }
                sprintf(
                    &raw mut resourceBuf as *mut ::core::ffi::c_char,
                    iconYPosStr,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*pWS).name,
                );
                resourcePtr = getXSMPResource(
                    pCD,
                    WMSAVE_ICON_Y as ::core::ffi::c_int,
                    &raw mut resourceBuf as *mut ::core::ffi::c_char,
                );
                if !resourcePtr.is_null() {
                    (*(*pCD).pWsList.offset(i as isize)).iconY = atoi(resourcePtr);
                    (*pCD).clientFlags |= SM_ICON_Y;
                }
            }
            i += 1;
        }
        return;
    }
    if smClientDBCheckpointed == 0 {
        let mut proxyClientID = ::core::ptr::null_mut::<::core::ffi::c_char>();
        proxyClientID = findProxyClientID(pCD);
        if !proxyClientID.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*pCD).numInhabited {
                pWS = GetWorkspaceData(pSD, (*(*pCD).pWsList.offset(i as isize)).wsID);
                if !pWS.is_null() {
                    sprintf(
                        &raw mut resourceBuf as *mut ::core::ffi::c_char,
                        iconXPosStr,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        (*pWS).name,
                    );
                    resourcePtr = getClientResource(
                        proxyClientID,
                        &raw mut resourceBuf as *mut ::core::ffi::c_char,
                    );
                    if !resourcePtr.is_null() {
                        (*(*pCD).pWsList.offset(i as isize)).iconX = atoi(resourcePtr);
                        (*pCD).clientFlags |= SM_ICON_X;
                    }
                    sprintf(
                        &raw mut resourceBuf as *mut ::core::ffi::c_char,
                        iconYPosStr,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        (*pWS).name,
                    );
                    resourcePtr = getClientResource(
                        proxyClientID,
                        &raw mut resourceBuf as *mut ::core::ffi::c_char,
                    );
                    if !resourcePtr.is_null() {
                        (*(*pCD).pWsList.offset(i as isize)).iconY = atoi(resourcePtr);
                        (*pCD).clientFlags |= SM_ICON_Y;
                    }
                }
                i += 1;
            }
            dbRemoveProxyClientEntry(proxyClientID);
        }
    }
}
