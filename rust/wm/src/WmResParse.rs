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
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_2;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_3;
use crate::abi::C2RustUnnamed_WmArgb as C2RustUnnamed;
use crate::abi::*;
use ::c2rust_bitfields;
extern "C" {
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn XGetModifierMapping(_: *mut Display) -> *mut XModifierKeymap;
    fn XKeysymToString(_: KeySym) -> *mut ::core::ffi::c_char;
    fn XInternAtom(_: *mut Display, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int) -> Atom;
    fn XStringToKeysym(_: *const ::core::ffi::c_char) -> KeySym;
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
    fn XDeleteProperty(_: *mut Display, _: Window, _: Atom) -> ::core::ffi::c_int;
    fn XFreeModifiermap(_: *mut XModifierKeymap) -> ::core::ffi::c_int;
    fn XKeysymToKeycode(_: *mut Display, _: KeySym) -> KeyCode;
    fn XParseGeometry(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strncat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtCalloc(_: Cardinal, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtRealloc(_: *mut ::core::ffi::c_char, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    fn XmeGetHomeDirName() -> String_0;
    static mut wmGD: WmGlobalData;
    static mut builtinSystemMenu: [::core::ffi::c_char; 0];
    fn PrintVersionInfo();
    static mut builtinRootMenu: [::core::ffi::c_char; 0];
    fn _XmVirtKeysLoadFileBindings(
        fileName: *mut ::core::ffi::c_char,
        binding: *mut String_0,
    ) -> Boolean;
    fn _XmVirtKeysLoadFallbackBindings(
        display: *mut Display,
        binding: *mut String_0,
    ) -> ::core::ffi::c_int;
    fn XkbKeycodeToKeysym(
        _: *mut Display,
        _: KeyCode,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> KeySym;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn mblen(__s: *const ::core::ffi::c_char, __n: size_t) -> ::core::ffi::c_int;
    fn mbtowc(
        __pwc: *mut wchar_t,
        __s: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn F_CreateWorkspace(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_DeleteWorkspace(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_AddToAllWorkspaces(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Remove(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn Warning(message: *mut ::core::ffi::c_char);
    fn F_Beep(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Lower(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Circle_Down(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Circle_Up(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Focus_Color(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Exec(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Quit_Mwm(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Focus_Key(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Goto_Workspace(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Next_Key(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Prev_Cmap(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Prev_Key(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Pass_Key(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Marquee_Selection(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Maximize(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Menu(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Minimize(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Move(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Next_Cmap(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Nop(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Normalize(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Normalize_And_Raise(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Pack_Icons(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Post_SMenu(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Kill(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Refresh(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Resize(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Restart(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Restore(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Restore_And_Raise(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Send_Msg(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Set_Context(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Separator(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Raise(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Raise_Lower(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Refresh_Win(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Set_Behavior(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Title(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Tile(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Screen(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Next_Workspace(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Prev_Workspace(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn F_Workspace_Presence(args: String_0, pCD: *mut ClientData, event: *mut XEvent) -> Boolean;
    fn GetBitmapIndex(
        pSD: *mut WmScreenData,
        name: *mut ::core::ffi::c_char,
        bReportError: Boolean,
    ) -> ::core::ffi::c_int;
    fn ExitWM(exitCode: ::core::ffi::c_int) -> !;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const XmPIXMAP_AND_STRING: C2RustUnnamed_2 = 3;
pub const XmSTRING: C2RustUnnamed_2 = 2;
pub const XmPIXMAP: C2RustUnnamed_2 = 1;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
pub const XA_STRING: Atom = 31 as ::core::ffi::c_int as Atom;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NoSymbol: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const KeyPress: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ButtonPress: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ButtonRelease: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ShiftMask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const LockMask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const ControlMask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const Mod1Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const Mod2Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const Mod3Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const Mod4Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const Mod5Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
pub const Button1Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const Button2Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
pub const Button3Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
pub const Button4Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int;
pub const Button5Mask: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int;
pub const Button1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Button2: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const Button3: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const Button4: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const Button5: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PropModeReplace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XValue: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const YValue: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WidthValue: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const HeightValue: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_FUNC_RESIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const MWM_FUNC_MOVE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const MWM_FUNC_MINIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const MWM_FUNC_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const MWM_FUNC_CLOSE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CRS_BUTTON: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const CRS_KEY: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const CRS_MENU: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const CRS_ACCEL: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const CRS_ANY: ::core::ffi::c_long = CRS_BUTTON | CRS_KEY | CRS_MENU | CRS_ACCEL;
pub const WM_ERROR_EXIT_VALUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SELECT_BUTTON: ::core::ffi::c_int = Button1;
pub const SELECT_BUTTON_MASK: ::core::ffi::c_int = Button1Mask;
pub const DMANIP_BUTTON: ::core::ffi::c_int = Button2;
pub const DMANIP_BUTTON_MASK: ::core::ffi::c_int = Button2Mask;
pub const BMENU_BUTTON: ::core::ffi::c_int = Button3;
pub const BMENU_BUTTON_MASK: ::core::ffi::c_int = Button3Mask;
pub const WSM_FUNCTION_OCCUPY_WS: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int;
pub const F_CONTEXT_ROOT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const F_CONTEXT_ICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const F_CONTEXT_NORMAL: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const F_CONTEXT_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const F_CONTEXT_ICONBOX: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const F_CONTEXT_IFKEY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
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
pub const FRAME_NBORDER: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const FRAME_MATTE: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const FRAME_CLOSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const ICON_PART_ALL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ROOT_PART_ALL: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const ICONBOX_PART_IICON: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const ICONBOX_PART_WICON: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const F_SUBCONTEXT_I_ALL: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << ICON_PART_ALL;
pub const F_SUBCONTEXT_R_ALL: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << ROOT_PART_ALL;
pub const F_SUBCONTEXT_IB_IICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_IICON;
pub const F_SUBCONTEXT_IB_WICON: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << ICONBOX_PART_WICON;
pub const F_SUBCONTEXT_W_CLIENT: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << FRAME_CLIENT;
pub const F_SUBCONTEXT_W_APP: ::core::ffi::c_long = F_SUBCONTEXT_W_CLIENT;
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
pub const F_SUBCONTEXT_W_NBORDER: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << FRAME_NBORDER;
pub const F_SUBCONTEXT_W_MATTE: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << FRAME_MATTE;
pub const F_SUBCONTEXT_W_RBORDER: ::core::ffi::c_long = F_SUBCONTEXT_W_RESIZE_NW
    | F_SUBCONTEXT_W_RESIZE_N
    | F_SUBCONTEXT_W_RESIZE_NE
    | F_SUBCONTEXT_W_RESIZE_E
    | F_SUBCONTEXT_W_RESIZE_SE
    | F_SUBCONTEXT_W_RESIZE_S
    | F_SUBCONTEXT_W_RESIZE_SW
    | F_SUBCONTEXT_W_RESIZE_W;
pub const F_SUBCONTEXT_W_BORDER: ::core::ffi::c_long =
    F_SUBCONTEXT_W_RBORDER | F_SUBCONTEXT_W_NBORDER;
pub const F_SUBCONTEXT_W_TITLEBAR: ::core::ffi::c_long = F_SUBCONTEXT_W_SYSTEM
    | F_SUBCONTEXT_W_TITLE
    | F_SUBCONTEXT_W_MINIMIZE
    | F_SUBCONTEXT_W_MAXIMIZE
    | F_SUBCONTEXT_W_CLOSE;
pub const F_SUBCONTEXT_W_FRAME: ::core::ffi::c_long =
    F_SUBCONTEXT_W_BORDER | F_SUBCONTEXT_W_TITLEBAR;
pub const F_SUBCONTEXT_W_ALL: ::core::ffi::c_long =
    F_SUBCONTEXT_W_FRAME | F_SUBCONTEXT_W_MATTE | F_SUBCONTEXT_W_CLIENT;
pub const F_GROUP_WINDOW: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const F_GROUP_ICON: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const F_GROUP_DEFAULT: ::core::ffi::c_long = F_GROUP_WINDOW | F_GROUP_ICON;
pub const F_GROUP_TRANSIENT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const NORMAL_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SM_LAUNCHED: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 17 as ::core::ffi::c_int;
pub const SM_X: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 18 as ::core::ffi::c_int;
pub const SM_Y: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 19 as ::core::ffi::c_int;
pub const SM_WIDTH: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 20 as ::core::ffi::c_int;
pub const SM_HEIGHT: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 21 as ::core::ffi::c_int;
pub const SM_CLIENT_STATE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 22 as ::core::ffi::c_int;
pub const MAXWMPATH: ::core::ffi::c_int = 1023 as ::core::ffi::c_int;
pub const MOTIF_BINDINGS_FILE: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b".motifbind\0") };
pub const XK_KP_F1: ::core::ffi::c_int = 0xff91 as ::core::ffi::c_int;
pub const XK_KP_F2: ::core::ffi::c_int = 0xff92 as ::core::ffi::c_int;
pub const XK_KP_F3: ::core::ffi::c_int = 0xff93 as ::core::ffi::c_int;
pub const XK_KP_F4: ::core::ffi::c_int = 0xff94 as ::core::ffi::c_int;
pub const XK_F9: ::core::ffi::c_int = 0xffc6 as ::core::ffi::c_int;
pub const XK_F10: ::core::ffi::c_int = 0xffc7 as ::core::ffi::c_int;
pub const XK_F11: ::core::ffi::c_int = 0xffc8 as ::core::ffi::c_int;
pub const XK_F12: ::core::ffi::c_int = 0xffc9 as ::core::ffi::c_int;
pub const __LC_CTYPE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const LC_CTYPE: ::core::ffi::c_int = __LC_CTYPE;
pub const MAXLINE: ::core::ffi::c_int = MAXWMPATH + 1 as ::core::ffi::c_int;
static mut cfileName: [::core::ffi::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut pWarningStringFile: [::core::ffi::c_char; 44] = unsafe {
    ::core::mem::transmute::<[u8; 44], [::core::ffi::c_char; 44]>(
        *b"%s: %s on line %d of configuration file %s\n\0",
    )
};
#[no_mangle]
pub static mut pWarningStringLine: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<[u8; 43], [::core::ffi::c_char; 43]>(
        *b"%s: %s on line %d of specification string\n\0",
    )
};
static mut cfileP: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
static mut line: [::core::ffi::c_uchar; 1025] = [0; 1025];
static mut linec: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut parseP: *mut ::core::ffi::c_uchar =
    ::core::ptr::null::<::core::ffi::c_uchar>() as *mut ::core::ffi::c_uchar;
static mut modifierStrings: [MaskTableEntry; 13] = [
    MaskTableEntry {
        name: b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: None_0 as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: b"ctrl\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: ControlMask as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: b"shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: ShiftMask as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: b"alt\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: Mod1Mask as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: b"meta\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: Mod1Mask as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: b"super\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: Mod4Mask as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: b"lock\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: LockMask as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: b"mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: Mod1Mask as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: b"mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: Mod2Mask as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: b"mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: Mod3Mask as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: b"mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: Mod4Mask as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: b"mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        mask: Mod5Mask as ::core::ffi::c_uint,
    },
    MaskTableEntry {
        name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        mask: 0 as ::core::ffi::c_uint,
    },
];
pub const ALT_INDEX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const META_INDEX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SUPER_INDEX: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
static mut buttonEvents: [EventTableEntry; 21] = unsafe {
    [
        EventTableEntry {
            event: b"btn1down\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonPress as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: SELECT_BUTTON as ::core::ffi::c_uint,
            fClick: FALSE as Boolean,
        },
        EventTableEntry {
            event: b"btn1up\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonRelease as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: SELECT_BUTTON as ::core::ffi::c_uint,
            fClick: FALSE as Boolean,
        },
        EventTableEntry {
            event: b"btn1click\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonRelease as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: SELECT_BUTTON as ::core::ffi::c_uint,
            fClick: TRUE as Boolean,
        },
        EventTableEntry {
            event: b"btn1click2\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonPress as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: SELECT_BUTTON as ::core::ffi::c_uint,
            fClick: TRUE as Boolean,
        },
        EventTableEntry {
            event: b"btn2down\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonPress as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: DMANIP_BUTTON as ::core::ffi::c_uint,
            fClick: FALSE as Boolean,
        },
        EventTableEntry {
            event: b"btn2up\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonRelease as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: DMANIP_BUTTON as ::core::ffi::c_uint,
            fClick: FALSE as Boolean,
        },
        EventTableEntry {
            event: b"btn2click\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonRelease as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: DMANIP_BUTTON as ::core::ffi::c_uint,
            fClick: TRUE as Boolean,
        },
        EventTableEntry {
            event: b"btn2click2\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonPress as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: DMANIP_BUTTON as ::core::ffi::c_uint,
            fClick: TRUE as Boolean,
        },
        EventTableEntry {
            event: b"btn3down\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonPress as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: BMENU_BUTTON as ::core::ffi::c_uint,
            fClick: FALSE as Boolean,
        },
        EventTableEntry {
            event: b"btn3up\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonRelease as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: BMENU_BUTTON as ::core::ffi::c_uint,
            fClick: FALSE as Boolean,
        },
        EventTableEntry {
            event: b"btn3click\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonRelease as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: BMENU_BUTTON as ::core::ffi::c_uint,
            fClick: TRUE as Boolean,
        },
        EventTableEntry {
            event: b"btn3click2\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonPress as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: BMENU_BUTTON as ::core::ffi::c_uint,
            fClick: TRUE as Boolean,
        },
        EventTableEntry {
            event: b"btn4down\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonPress as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: Button4 as ::core::ffi::c_uint,
            fClick: FALSE as Boolean,
        },
        EventTableEntry {
            event: b"btn4up\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonRelease as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: Button4 as ::core::ffi::c_uint,
            fClick: FALSE as Boolean,
        },
        EventTableEntry {
            event: b"btn4click\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonRelease as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: Button4 as ::core::ffi::c_uint,
            fClick: TRUE as Boolean,
        },
        EventTableEntry {
            event: b"btn4click2\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonPress as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: Button4 as ::core::ffi::c_uint,
            fClick: TRUE as Boolean,
        },
        EventTableEntry {
            event: b"btn5down\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonPress as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: Button5 as ::core::ffi::c_uint,
            fClick: FALSE as Boolean,
        },
        EventTableEntry {
            event: b"btn5up\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonRelease as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: Button5 as ::core::ffi::c_uint,
            fClick: FALSE as Boolean,
        },
        EventTableEntry {
            event: b"btn5click\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonRelease as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: Button5 as ::core::ffi::c_uint,
            fClick: TRUE as Boolean,
        },
        EventTableEntry {
            event: b"btn5click2\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            eventType: ButtonPress as ::core::ffi::c_uint,
            parseProc: Some(
                ParseImmed
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: Button5 as ::core::ffi::c_uint,
            fClick: TRUE as Boolean,
        },
        EventTableEntry {
            event: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            eventType: 0 as ::core::ffi::c_uint,
            parseProc: None,
            closure: 0 as ::core::ffi::c_uint,
            fClick: 0 as Boolean,
        },
    ]
};
static mut keyEvents: [EventTableEntry; 2] = unsafe {
    [
        EventTableEntry {
            event: b"key\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            eventType: KeyPress as ::core::ffi::c_uint,
            parseProc: Some(
                ParseKeySym
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_uint,
                    ) -> Boolean,
            ),
            closure: 0 as ::core::ffi::c_uint,
            fClick: FALSE as Boolean,
        },
        EventTableEntry {
            event: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            eventType: 0 as ::core::ffi::c_uint,
            parseProc: None,
            closure: 0 as ::core::ffi::c_uint,
            fClick: 0 as Boolean,
        },
    ]
};
#[no_mangle]
pub static mut buttonModifierMasks: [::core::ffi::c_uint; 6] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    SELECT_BUTTON_MASK as ::core::ffi::c_uint,
    DMANIP_BUTTON_MASK as ::core::ffi::c_uint,
    BMENU_BUTTON_MASK as ::core::ffi::c_uint,
    Button4Mask as ::core::ffi::c_uint,
    Button5Mask as ::core::ffi::c_uint,
];
#[no_mangle]
pub static mut functionTable: [FunctionTableEntry; 49] = unsafe {
    [
        FunctionTableEntry {
            funcName: b"f.beep\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Beep as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.circle_down\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_SUBCONTEXT_IB_IICON | F_SUBCONTEXT_IB_WICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Circle_Down
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut GroupArg,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncGrpArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut GroupArg,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.circle_up\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_SUBCONTEXT_IB_IICON | F_SUBCONTEXT_IB_WICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Circle_Up
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut GroupArg,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncGrpArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut GroupArg,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.create_workspace\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_CreateWorkspace
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.delete_workspace\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_DeleteWorkspace
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.exec\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Exec as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncStrArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.focus_color\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_SUBCONTEXT_IB_IICON | F_SUBCONTEXT_IB_WICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Focus_Color
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.focus_key\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_SUBCONTEXT_IB_IICON | F_SUBCONTEXT_IB_WICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Focus_Key
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.goto_workspace\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Goto_Workspace
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncStrArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.kill\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: F_CONTEXT_ROOT as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: MWM_FUNC_CLOSE,
            wmFunction: Some(
                F_Kill as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.lower\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_SUBCONTEXT_IB_IICON | F_SUBCONTEXT_IB_WICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Lower as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncMaybeStrArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.marquee_selection\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_WINDOW | F_CONTEXT_ICON | F_SUBCONTEXT_IB_IICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Marquee_Selection
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.maximize\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ROOT | F_CONTEXT_MAXIMIZE | F_SUBCONTEXT_IB_WICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: MWM_FUNC_MAXIMIZE,
            wmFunction: Some(
                F_Maximize
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.menu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Menu as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncStrArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.minimize\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ICON | F_CONTEXT_ROOT | F_SUBCONTEXT_IB_IICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: MWM_FUNC_MINIMIZE,
            wmFunction: Some(
                F_Minimize
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.move\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: F_CONTEXT_ROOT as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: MWM_FUNC_MOVE,
            wmFunction: Some(
                F_Move as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.next_cmap\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Next_Cmap
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.next_key\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Next_Key
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut GroupArg,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncGrpArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut GroupArg,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.next_workspace\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Next_Workspace
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.nop\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ROOT
                | F_CONTEXT_ICON
                | F_CONTEXT_WINDOW
                | F_SUBCONTEXT_IB_WICON
                | F_SUBCONTEXT_IB_IICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Nop as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.normalize\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ROOT | F_CONTEXT_NORMAL | F_SUBCONTEXT_IB_WICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Normalize
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.normalize_and_raise\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ROOT | F_CONTEXT_NORMAL) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Normalize_And_Raise
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.occupy_all\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ICONBOX | F_CONTEXT_ROOT) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: WSM_FUNCTION_OCCUPY_WS,
            wmFunction: Some(
                F_AddToAllWorkspaces
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.pack_icons\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Pack_Icons
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.pass_keys\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Pass_Key
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.post_wmenu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: (CRS_BUTTON | CRS_KEY) as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Post_SMenu
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.prev_cmap\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Prev_Cmap
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.prev_key\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Prev_Key
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut GroupArg,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncGrpArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut GroupArg,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.prev_workspace\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Prev_Workspace
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.quit\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Quit_Mwm
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.quit_mwm\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Quit_Mwm
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.quit_wm\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Quit_Mwm
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.raise\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_SUBCONTEXT_IB_IICON | F_SUBCONTEXT_IB_WICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Raise as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncMaybeStrArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.raise_lower\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ROOT | F_SUBCONTEXT_IB_IICON | F_SUBCONTEXT_IB_WICON)
                as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Raise_Lower
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.refresh\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Refresh
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.refresh_win\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ICON | F_CONTEXT_ROOT) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Refresh_Win
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.remove\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: F_CONTEXT_ROOT as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: WSM_FUNCTION_OCCUPY_WS,
            wmFunction: Some(
                F_Remove as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.resize\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ICON
                | F_CONTEXT_ROOT
                | F_SUBCONTEXT_IB_IICON
                | F_SUBCONTEXT_IB_WICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: MWM_FUNC_RESIZE,
            wmFunction: Some(
                F_Resize as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.restart\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Restart
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.restore\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ROOT | F_CONTEXT_NORMAL | F_SUBCONTEXT_IB_WICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Restore
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.restore_and_raise\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ROOT | F_CONTEXT_NORMAL) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Restore_And_Raise
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.screen\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Screen as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncStrArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.send_msg\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: F_CONTEXT_ROOT as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Send_Msg
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut ::core::ffi::c_ulong,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNbrArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut ::core::ffi::c_ulong,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.separator\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_MENU as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Separator
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.set_behavior\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Set_Behavior
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.set_context\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Set_Context
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut ::core::ffi::c_ulong,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNbrArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut ::core::ffi::c_ulong,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.tile\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ROOT | F_CONTEXT_ICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: MWM_FUNC_MOVE | MWM_FUNC_RESIZE,
            wmFunction: Some(
                F_Tile as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncStrArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.title\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: 0 as Context,
            resource: CRS_MENU as ::core::ffi::c_uint,
            mgtMask: 0 as ::core::ffi::c_long,
            wmFunction: Some(
                F_Title as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
        FunctionTableEntry {
            funcName: b"f.workspace_presence\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            greyedContext: (F_CONTEXT_ROOT | F_CONTEXT_ICONBOX | F_SUBCONTEXT_IB_WICON) as Context,
            resource: CRS_ANY as ::core::ffi::c_uint,
            mgtMask: WSM_FUNCTION_OCCUPY_WS,
            wmFunction: Some(
                F_Workspace_Presence
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            ),
            parseProc: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
                >,
                FunctionTableParseProcT,
            >(Some(
                ParseWmFuncNoArg
                    as unsafe extern "C" fn(
                        *mut *mut ::core::ffi::c_uchar,
                        WmFunction,
                        *mut String_0,
                    ) -> Boolean,
            )),
        },
    ]
};
pub const WMFUNCTIONTABLESIZE: usize = (::core::mem::size_of::<[FunctionTableEntry; 49]>()
    as usize)
    .wrapping_div(::core::mem::size_of::<FunctionTableEntry>() as usize);
#[no_mangle]
pub static mut F_ACTION_INDEX: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut F_EXEC_INDEX: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut F_NOP_INDEX: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn GetFunctionTableValues(
    mut execIndex: *mut ::core::ffi::c_int,
    mut nopIndex: *mut ::core::ffi::c_int,
    mut actionIndex: *mut ::core::ffi::c_int,
) {
    GetExecIndex(WMFUNCTIONTABLESIZE as ::core::ffi::c_int, execIndex);
    GetActionIndex(WMFUNCTIONTABLESIZE as ::core::ffi::c_int, actionIndex);
    GetNopIndex(WMFUNCTIONTABLESIZE as ::core::ffi::c_int, nopIndex);
}
#[no_mangle]
pub unsafe extern "C" fn GetExecIndex(
    mut tableSize: ::core::ffi::c_int,
    mut execIndex: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < tableSize {
        if strcmp(
            b"f.exec\0" as *const u8 as *const ::core::ffi::c_char,
            functionTable[i as usize].funcName,
        ) == 0
        {
            *execIndex = i;
            return;
        } else {
            *execIndex = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetActionIndex(
    mut tableSize: ::core::ffi::c_int,
    mut actionIndex: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < tableSize {
        if strcmp(
            b"f.action\0" as *const u8 as *const ::core::ffi::c_char,
            functionTable[i as usize].funcName,
        ) == 0
        {
            *actionIndex = i;
            return;
        } else {
            *actionIndex = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetNopIndex(
    mut tableSize: ::core::ffi::c_int,
    mut nopIndex: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < tableSize {
        if strcmp(
            b"f.nop\0" as *const u8 as *const ::core::ffi::c_char,
            functionTable[i as usize].funcName,
        ) == 0
        {
            *nopIndex = i;
            return;
        } else {
            *nopIndex = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn FindSessionMatch(
    mut commandArgc: ::core::ffi::c_int,
    mut commandArgv: *mut *mut ::core::ffi::c_char,
    mut pCD: *mut ClientData,
    mut pSD: *mut WmScreenData,
    mut pWorkSpaceList: *mut *mut ::core::ffi::c_char,
    mut clientMachine: *mut ::core::ffi::c_char,
) -> Boolean {
    let mut count: ::core::ffi::c_int = 0;
    let mut relCount: ::core::ffi::c_int = 0;
    let mut argNum: ::core::ffi::c_int = 0;
    let mut sessionGeom = ::core::ptr::null_mut::<SessionGeom>();
    count = 0 as ::core::ffi::c_int;
    while count < (*pSD).totalSessionItems {
        if (*(*pSD).pSessionItems.offset(count as isize)).processed == 0
            && (*(*pSD).pSessionItems.offset(count as isize)).commandArgc == commandArgc
        {
            if !(!clientMachine.is_null()
                && !(*(*pSD).pSessionItems.offset(count as isize))
                    .clientMachine
                    .is_null()
                && strcmp(
                    clientMachine,
                    (*(*pSD).pSessionItems.offset(count as isize)).clientMachine
                        as *const ::core::ffi::c_char,
                ) != 0)
            {
                argNum = 0 as ::core::ffi::c_int;
                while argNum < commandArgc {
                    if strcmp(
                        *commandArgv.offset(argNum as isize),
                        *(*(*pSD).pSessionItems.offset(count as isize))
                            .commandArgv
                            .offset(argNum as isize),
                    ) != 0
                    {
                        argNum -= 1;
                        break;
                    } else {
                        argNum += 1;
                    }
                }
                if argNum == commandArgc {
                    (*(*pSD).pSessionItems.offset(count as isize)).processed = True as Boolean;
                    (*pSD).remainingSessionItems -= 1;
                    (*pCD).clientFlags |= SM_LAUNCHED;
                    relCount = 0 as ::core::ffi::c_int;
                    while relCount < commandArgc {
                        XtFree(
                            *(*(*pSD).pSessionItems.offset(count as isize))
                                .commandArgv
                                .offset(relCount as isize),
                        );
                        relCount += 1;
                    }
                    XtFree(
                        (*(*pSD).pSessionItems.offset(count as isize)).commandArgv
                            as *mut ::core::ffi::c_char,
                    );
                    if (*(*pSD).pSessionItems.offset(count as isize)).clientState != 0 {
                        (*pCD).clientState =
                            (*(*pSD).pSessionItems.offset(count as isize)).clientState;
                        (*pCD).clientFlags |= SM_CLIENT_STATE;
                    }
                    if !(*(*pSD).pSessionItems.offset(count as isize))
                        .sessionGeom
                        .is_null()
                    {
                        sessionGeom = (*(*pSD).pSessionItems.offset(count as isize)).sessionGeom
                            as *mut SessionGeom;
                        if (*sessionGeom).flags & XValue != 0 {
                            (*pCD).clientX = (*sessionGeom).clientX;
                            (*pCD).clientFlags |= SM_X;
                        }
                        if (*sessionGeom).flags & YValue != 0 {
                            (*pCD).clientY = (*sessionGeom).clientY;
                            (*pCD).clientFlags |= SM_Y;
                        }
                        if (*sessionGeom).flags & WidthValue != 0 {
                            (*pCD).clientWidth = (*sessionGeom).clientWidth;
                            (*pCD).clientFlags |= SM_WIDTH;
                        }
                        if (*sessionGeom).flags & HeightValue != 0 {
                            (*pCD).clientHeight = (*sessionGeom).clientHeight;
                            (*pCD).clientFlags |= SM_HEIGHT;
                        }
                        XtFree(
                            (*(*pSD).pSessionItems.offset(count as isize)).sessionGeom
                                as *mut ::core::ffi::c_char,
                        );
                    }
                    if !(*(*pSD).pSessionItems.offset(count as isize))
                        .clientMachine
                        .is_null()
                    {
                        XtFree((*(*pSD).pSessionItems.offset(count as isize)).clientMachine);
                        let ref mut fresh21 =
                            (*(*pSD).pSessionItems.offset(count as isize)).clientMachine;
                        *fresh21 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    if !(*(*pSD).pSessionItems.offset(count as isize))
                        .workspaces
                        .is_null()
                    {
                        *pWorkSpaceList = (*(*pSD).pSessionItems.offset(count as isize)).workspaces
                            as *mut ::core::ffi::c_char;
                    }
                    if (*pSD).remainingSessionItems == 0 as ::core::ffi::c_int {
                        XtFree((*pSD).pSessionItems as *mut ::core::ffi::c_char);
                    }
                    return 1 as Boolean;
                }
            }
        }
        count += 1;
    }
    return 0 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ParseSessionClientState(
    mut pSD: *mut WmScreenData,
    mut count: ::core::ffi::c_int,
    mut string: *mut ::core::ffi::c_uchar,
) {
    if strcmp(
        string as *mut ::core::ffi::c_char,
        b"NormalState\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*pSD).pSessionItems.offset(count as isize)).clientState = NORMAL_STATE;
    } else if strcmp(
        string as *mut ::core::ffi::c_char,
        b"IconicState\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*pSD).pSessionItems.offset(count as isize)).clientState = MINIMIZED_STATE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ParseSessionGeometry(
    mut pSD: *mut WmScreenData,
    mut count: ::core::ffi::c_int,
    mut string: *mut ::core::ffi::c_uchar,
) {
    let mut pTmpSessionGeom = ::core::ptr::null_mut::<SessionGeom>();
    let mut mask: ::core::ffi::c_int = 0;
    let mut X: ::core::ffi::c_int = 0;
    let mut Y: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    height = 0 as ::core::ffi::c_int;
    width = height;
    Y = width;
    X = Y;
    mask = XParseGeometry(
        string as *mut ::core::ffi::c_char,
        &raw mut X,
        &raw mut Y,
        &raw mut width as *mut ::core::ffi::c_uint,
        &raw mut height as *mut ::core::ffi::c_uint,
    );
    if mask != 0 {
        pTmpSessionGeom =
            XtMalloc(::core::mem::size_of::<SessionGeom>() as Cardinal) as *mut SessionGeom;
        if pTmpSessionGeom.is_null() {
            Warning(
                b"Insufficient memory for session geometry item\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return;
        }
        (*pTmpSessionGeom).flags = mask;
        (*pTmpSessionGeom).clientX = X;
        (*pTmpSessionGeom).clientY = Y;
        (*pTmpSessionGeom).clientWidth = width;
        (*pTmpSessionGeom).clientHeight = height;
        let ref mut fresh22 = (*(*pSD).pSessionItems.offset(count as isize)).sessionGeom;
        *fresh22 = pTmpSessionGeom as *mut _SessionGeom;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ParseSessionWorkspaces(
    mut pSD: *mut WmScreenData,
    mut count: ::core::ffi::c_int,
    mut string: *mut ::core::ffi::c_uchar,
) {
    let ref mut fresh23 = (*(*pSD).pSessionItems.offset(count as isize)).workspaces;
    *fresh23 =
        XtMalloc(strlen(string as *mut ::core::ffi::c_char).wrapping_add(1 as size_t) as Cardinal);
    if (*fresh23).is_null() {
        Warning(
            b"Insufficient memory for workspaces list in sesssion item\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    strcpy(
        (*(*pSD).pSessionItems.offset(count as isize)).workspaces as *mut ::core::ffi::c_char,
        string as *mut ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ParseSessionHost(
    mut pSD: *mut WmScreenData,
    mut count: ::core::ffi::c_int,
    mut string: *mut ::core::ffi::c_uchar,
) {
    let ref mut fresh24 = (*(*pSD).pSessionItems.offset(count as isize)).clientMachine;
    *fresh24 =
        XtMalloc(strlen(string as *mut ::core::ffi::c_char).wrapping_add(1 as size_t) as Cardinal);
    if (*fresh24).is_null() {
        Warning(
            b"Insufficient memory for host name in sesssion item\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    strcpy(
        (*(*pSD).pSessionItems.offset(count as isize)).clientMachine as *mut ::core::ffi::c_char,
        string as *mut ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GetSessionHintsInfo(
    mut pSD: *mut WmScreenData,
    mut numItems: ::core::ffi::c_long,
) -> Boolean {
    (*pSD).pSessionItems = XtMalloc(
        (numItems as usize).wrapping_mul(::core::mem::size_of::<WmSessionItem>() as usize)
            as Cardinal,
    ) as *mut WmSessionItem as *mut _WmSessionItem;
    if (*pSD).pSessionItems.is_null() {
        Warning(
            b"Insufficient memory for WM Session Hints\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as Boolean;
    }
    memset(
        (*pSD).pSessionItems as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (numItems as size_t).wrapping_mul(::core::mem::size_of::<WmSessionItem>() as size_t),
    );
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn PeekAhead(
    mut currentChar: *mut ::core::ffi::c_uchar,
    mut currentLev: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut done = False as Boolean;
    let mut tmpLev = 1 as ::core::ffi::c_uint;
    let mut chlen: ::core::ffi::c_uint = 0;
    loop {
        chlen = mblen(
            currentChar as *mut ::core::ffi::c_char,
            __ctype_get_mb_cur_max(),
        ) as ::core::ffi::c_uint;
        if !(chlen > 0 as ::core::ffi::c_uint
            && chlen == 1 as ::core::ffi::c_uint
            && (*currentChar as ::core::ffi::c_int == '"' as i32
                || *currentChar as ::core::ffi::c_int == '\\' as i32)
            && done as ::core::ffi::c_int == False)
        {
            break;
        }
        currentChar = currentChar.offset(1);
        chlen = mblen(
            currentChar as *mut ::core::ffi::c_char,
            __ctype_get_mb_cur_max(),
        ) as ::core::ffi::c_uint;
        if chlen > 0 as ::core::ffi::c_uint
            && chlen == 1 as ::core::ffi::c_uint
            && (*currentChar as ::core::ffi::c_int == '"' as i32
                || *currentChar as ::core::ffi::c_int == '\\' as i32)
        {
            tmpLev = tmpLev.wrapping_add(1);
            if *currentChar as ::core::ffi::c_int == '"' as i32 {
                done = True as Boolean;
            } else {
                currentChar = currentChar.offset(1);
            }
        }
    }
    if done as ::core::ffi::c_int == True {
        return tmpLev;
    } else {
        return 0 as ::core::ffi::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SyncModifierStrings() {
    let mut map = ::core::ptr::null_mut::<XModifierKeymap>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k = 0 as ::core::ffi::c_int;
    map = XGetModifierMapping(wmGD.display);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < (*map).max_keypermod {
            if *(*map).modifiermap.offset(k as isize) != 0 {
                let mut ks = XkbKeycodeToKeysym(
                    wmGD.display,
                    *(*map).modifiermap.offset(k as isize),
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                let mut nm = XKeysymToString(ks);
                if !nm.is_null()
                    && strncmp(
                        b"Alt\0" as *const u8 as *const ::core::ffi::c_char,
                        nm,
                        3 as size_t,
                    ) == 0
                {
                    modifierStrings[ALT_INDEX as usize].mask =
                        ((1 as ::core::ffi::c_int) << i) as ::core::ffi::c_uint;
                } else if !nm.is_null()
                    && strncmp(
                        b"Meta\0" as *const u8 as *const ::core::ffi::c_char,
                        nm,
                        4 as size_t,
                    ) == 0
                {
                    modifierStrings[META_INDEX as usize].mask =
                        ((1 as ::core::ffi::c_int) << i) as ::core::ffi::c_uint;
                } else if !nm.is_null()
                    && strncmp(
                        b"Super\0" as *const u8 as *const ::core::ffi::c_char,
                        nm,
                        5 as size_t,
                    ) == 0
                {
                    modifierStrings[SUPER_INDEX as usize].mask =
                        ((1 as ::core::ffi::c_int) << i) as ::core::ffi::c_uint;
                }
            }
            k += 1;
            j += 1;
        }
        i += 1;
    }
    XFreeModifiermap(map);
}
pub const MENU_SPEC: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"menu\0") };
pub const BUTTON_SPEC: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"buttons\0") };
pub const KEY_SPEC: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"keys\0") };
#[no_mangle]
pub unsafe extern "C" fn ProcessWmFile(mut pSD: *mut WmScreenData) {
    let mut lineP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut n: ::core::ffi::c_uint = 0;
    let mut menuSpec = ::core::ptr::null_mut::<MenuSpec>();
    (*pSD).buttonSpecs = ::core::ptr::null_mut::<ButtonSpec>();
    (*pSD).keySpecs = ::core::ptr::null_mut::<KeySpec>();
    (*pSD).menuSpecs = ::core::ptr::null_mut::<MenuSpec>();
    GetFunctionTableValues(
        &raw mut F_EXEC_INDEX,
        &raw mut F_NOP_INDEX,
        &raw mut F_ACTION_INDEX,
    );
    cfileP = ::core::ptr::null_mut::<FILE>();
    linec = 0 as ::core::ffi::c_int;
    parseP = &raw mut builtinSystemMenu as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
    if !parseP.is_null() && !GetNextLine().is_null() {
        lineP = &raw mut line as *mut ::core::ffi::c_uchar;
        ParseMenuSet(pSD, lineP);
    }
    linec = 0 as ::core::ffi::c_int;
    parseP = &raw mut builtinRootMenu as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
    if !parseP.is_null() && !GetNextLine().is_null() {
        lineP = &raw mut line as *mut ::core::ffi::c_uchar;
        ParseMenuSet(pSD, lineP);
    }
    cfileP = FopenConfigFile();
    if cfileP.is_null() {
        if wmGD.useStandardBehavior == 0 {
            Warning(
                b"Cannot open configuration file\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        return;
    }
    linec = 0 as ::core::ffi::c_int;
    while !GetNextLine().is_null() {
        lineP = &raw mut line as *mut ::core::ffi::c_uchar;
        if *(&raw mut line as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int == '!' as i32 || {
            string = GetString(&raw mut lineP);
            string.is_null()
        } {
            continue;
        }
        ToLower(string);
        if strcmp(string as *mut ::core::ffi::c_char, MENU_SPEC.as_ptr()) == 0 {
            ParseMenuSet(pSD, lineP);
        } else if strcmp(string as *mut ::core::ffi::c_char, BUTTON_SPEC.as_ptr()) == 0 {
            ParseButtonSet(pSD, lineP);
        } else if strcmp(string as *mut ::core::ffi::c_char, KEY_SPEC.as_ptr()) == 0 {
            ParseKeySet(pSD, lineP);
        }
    }
    fclose(cfileP);
    n = 0 as ::core::ffi::c_uint;
    menuSpec = (*pSD).menuSpecs;
    while !menuSpec.is_null() {
        n = n.wrapping_add(1);
        menuSpec = (*menuSpec).nextMenuSpec as *mut MenuSpec;
    }
    (*pSD).acceleratorMenuSpecs = ::core::ptr::null_mut::<*mut MenuSpec>();
    if n != 0 {
        (*pSD).acceleratorMenuSpecs = XtCalloc(
            n as Cardinal,
            ::core::mem::size_of::<*mut MenuSpec>() as Cardinal,
        ) as *mut *mut MenuSpec;
        if (*pSD).acceleratorMenuSpecs.is_null() {
            Warning(
                b"Insufficient memory for menu accelerators\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    (*pSD).acceleratorMenuCount = 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn ExtractLocaleName(mut lang: String_0) -> *mut ::core::ffi::c_char {
    return lang as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn FopenConfigFile() -> *mut FILE {
    let mut LANG = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut LANGp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fileP = ::core::ptr::null_mut::<FILE>();
    let mut homeDir = XmeGetHomeDirName() as *mut ::core::ffi::c_char;
    LANGp = setlocale(LC_CTYPE, ::core::ptr::null::<::core::ffi::c_char>());
    LANGp = ExtractLocaleName(LANGp as String_0);
    if LANGp.is_null() || strlen(LANGp) == 0 as size_t {
        LANG = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        LANG = XtMalloc(strlen(LANGp).wrapping_add(1 as size_t) as Cardinal);
        if LANG.is_null() {
            PWarning(
                b"Insufficient memory to get LANG environment variable.\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<FILE>();
        }
        strcpy(LANG, LANGp);
    }
    if !wmGD.configFile.is_null()
        && *wmGD.configFile.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '\0' as i32
    {
        if *wmGD.configFile.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '~' as i32
            && *wmGD.configFile.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '/' as i32
        {
            strcpy(&raw mut cfileName as *mut ::core::ffi::c_char, homeDir);
            if !LANG.is_null() {
                strncat(
                    &raw mut cfileName as *mut ::core::ffi::c_char,
                    b"/\0" as *const u8 as *const ::core::ffi::c_char,
                    (MAXWMPATH as size_t)
                        .wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
                );
                strncat(
                    &raw mut cfileName as *mut ::core::ffi::c_char,
                    LANG,
                    (MAXWMPATH as size_t)
                        .wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
                );
            }
            strncat(
                &raw mut cfileName as *mut ::core::ffi::c_char,
                wmGD.configFile.offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                (MAXWMPATH as size_t)
                    .wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
            );
            fileP = fopen(
                &raw mut cfileName as *mut ::core::ffi::c_char,
                b"r\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if !fileP.is_null() {
                if !LANG.is_null() {
                    XtFree(LANG);
                    LANG = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                return fileP;
            } else {
                strcpy(&raw mut cfileName as *mut ::core::ffi::c_char, homeDir);
                strncat(
                    &raw mut cfileName as *mut ::core::ffi::c_char,
                    wmGD.configFile.offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    (MAXWMPATH as size_t)
                        .wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
                );
                fileP = fopen(
                    &raw mut cfileName as *mut ::core::ffi::c_char,
                    b"r\0" as *const u8 as *const ::core::ffi::c_char,
                ) as *mut FILE;
                if !fileP.is_null() {
                    if !LANG.is_null() {
                        XtFree(LANG);
                        LANG = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    return fileP;
                }
            }
        } else {
            fileP = fopen(
                wmGD.configFile as *const ::core::ffi::c_char,
                b"r\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if !fileP.is_null() {
                if !LANG.is_null() {
                    XtFree(LANG);
                    LANG = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                return fileP;
            }
        }
    }
    strcpy(&raw mut cfileName as *mut ::core::ffi::c_char, homeDir);
    if !LANG.is_null() {
        strncat(
            &raw mut cfileName as *mut ::core::ffi::c_char,
            b"/\0" as *const u8 as *const ::core::ffi::c_char,
            (MAXWMPATH as size_t)
                .wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
        );
        strncat(
            &raw mut cfileName as *mut ::core::ffi::c_char,
            LANG,
            (MAXWMPATH as size_t)
                .wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
        );
    }
    strncat(
        &raw mut cfileName as *mut ::core::ffi::c_char,
        HOME_MWMRC.as_ptr(),
        (MAXWMPATH as size_t).wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
    );
    fileP = fopen(
        &raw mut cfileName as *mut ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if !fileP.is_null() {
        if !LANG.is_null() {
            XtFree(LANG);
            LANG = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return fileP;
    } else {
        strcpy(&raw mut cfileName as *mut ::core::ffi::c_char, homeDir);
        strncat(
            &raw mut cfileName as *mut ::core::ffi::c_char,
            HOME_MWMRC.as_ptr(),
            (MAXWMPATH as size_t)
                .wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
        );
        fileP = fopen(
            &raw mut cfileName as *mut ::core::ffi::c_char,
            b"r\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if !fileP.is_null() {
            if !LANG.is_null() {
                XtFree(LANG);
                LANG = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            return fileP;
        }
    }
    if !LANG.is_null() {
        strcpy(
            &raw mut cfileName as *mut ::core::ffi::c_char,
            RCDIR.as_ptr(),
        );
        strncat(
            &raw mut cfileName as *mut ::core::ffi::c_char,
            b"/\0" as *const u8 as *const ::core::ffi::c_char,
            (MAXWMPATH as size_t)
                .wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
        );
        strncat(
            &raw mut cfileName as *mut ::core::ffi::c_char,
            LANG,
            (MAXWMPATH as size_t)
                .wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
        );
        strncat(
            &raw mut cfileName as *mut ::core::ffi::c_char,
            SLASH_MWMRC.as_ptr(),
            (MAXWMPATH as size_t)
                .wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
        );
        fileP = fopen(
            &raw mut cfileName as *mut ::core::ffi::c_char,
            b"r\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if !fileP.is_null() {
            XtFree(LANG);
            LANG = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return fileP;
        }
    }
    strcpy(
        &raw mut cfileName as *mut ::core::ffi::c_char,
        RCDIR.as_ptr(),
    );
    strncat(
        &raw mut cfileName as *mut ::core::ffi::c_char,
        SLASH_MWMRC.as_ptr(),
        (MAXWMPATH as size_t).wrapping_sub(strlen(&raw mut cfileName as *mut ::core::ffi::c_char)),
    );
    if !LANG.is_null() {
        XtFree(LANG);
        LANG = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return fopen(
        &raw mut cfileName as *mut ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
}
pub const HOME_MWMRC: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"/.emwmrc\0") };
pub const SLASH_MWMRC: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"/system.emwmrc\0") };
#[no_mangle]
pub unsafe extern "C" fn SaveMenuAccelerators(
    mut pSD: *mut WmScreenData,
    mut newMenuSpec: *mut MenuSpec,
) {
    let mut pMenuSpec = ::core::ptr::null_mut::<*mut MenuSpec>();
    pMenuSpec = (*pSD).acceleratorMenuSpecs;
    if pMenuSpec.is_null() {
        return;
    }
    while !(*pMenuSpec).is_null() && *pMenuSpec != newMenuSpec {
        pMenuSpec = pMenuSpec.offset(1);
    }
    if (*pMenuSpec).is_null() {
        *pMenuSpec = newMenuSpec;
        (*pSD).acceleratorMenuCount = (*pSD).acceleratorMenuCount.wrapping_add(1);
    }
}
unsafe extern "C" fn ParseMenuSet(
    mut pSD: *mut WmScreenData,
    mut lineP: *mut ::core::ffi::c_uchar,
) {
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut menuSpec = ::core::ptr::null_mut::<MenuSpec>();
    string = GetString(&raw mut lineP);
    if string.is_null() {
        return;
    }
    menuSpec = XtMalloc(::core::mem::size_of::<MenuSpec>() as Cardinal) as *mut MenuSpec;
    if menuSpec.is_null() {
        PWarning(
            b"Insufficient memory for menu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return;
    }
    (*menuSpec).currentContext = 0 as Context;
    (*menuSpec).menuWidget = ::core::ptr::null_mut::<_WidgetRec>();
    (*menuSpec).whichButton = SELECT_BUTTON as ::core::ffi::c_uint;
    (*menuSpec).menuItems = ::core::ptr::null_mut::<MenuItem>();
    (*menuSpec).accelContext = 0 as Context;
    (*menuSpec).accelKeySpecs = ::core::ptr::null_mut::<KeySpec>();
    (*menuSpec).nextMenuSpec = ::core::ptr::null_mut::<_MenuSpec>();
    (*menuSpec).name =
        XtMalloc(strlen(string as *mut ::core::ffi::c_char).wrapping_add(1 as size_t) as Cardinal);
    if (*menuSpec).name.is_null() {
        PWarning(
            b"Insufficient memory for menu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        XtFree(menuSpec as *mut ::core::ffi::c_char);
        return;
    }
    strcpy(
        (*menuSpec).name as *mut ::core::ffi::c_char,
        string as *mut ::core::ffi::c_char,
    );
    (*menuSpec).nextMenuSpec = (*pSD).menuSpecs as *mut _MenuSpec;
    (*pSD).menuSpecs = menuSpec;
    while !GetNextLine().is_null() {
        lineP = &raw mut line as *mut ::core::ffi::c_uchar;
        ScanWhitespace(&raw mut lineP);
        if lineP.is_null()
            || *(&raw mut line as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int == '!' as i32
            || *lineP as ::core::ffi::c_int == '\0' as i32
            || *lineP as ::core::ffi::c_int == '#' as i32
        {
            continue;
        }
        if *lineP as ::core::ffi::c_int == '{' as i32 {
            break;
        }
        PWarning(
            b"Expected '{' after menu name\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return;
    }
    (*menuSpec).menuItems = ParseMenuItems(pSD);
}
#[no_mangle]
pub unsafe extern "C" fn ParseMwmMenuStr(
    mut pSD: *mut WmScreenData,
    mut menuStr: *mut ::core::ffi::c_uchar,
) -> *mut MenuItem {
    cfileP = ::core::ptr::null_mut::<FILE>();
    linec = 0 as ::core::ffi::c_int;
    parseP = menuStr;
    return ParseMenuItems(pSD);
}
unsafe extern "C" fn ParseMenuItems(mut pSD: *mut WmScreenData) -> *mut MenuItem {
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut lineP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut firstMenuItem = ::core::ptr::null_mut::<MenuItem>();
    let mut lastMenuItem = ::core::ptr::null_mut::<MenuItem>();
    let mut menuItem = ::core::ptr::null_mut::<MenuItem>();
    let mut ix = 0 as ::core::ffi::c_int;
    lastMenuItem = ::core::ptr::null_mut::<MenuItem>();
    firstMenuItem = lastMenuItem;
    while !GetNextLine().is_null() {
        lineP = &raw mut line as *mut ::core::ffi::c_uchar;
        if *(&raw mut line as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int == '!' as i32
            || *(&raw mut line as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int == '#' as i32
            || {
                string = GetString(&raw mut lineP);
                string.is_null()
            }
        {
            continue;
        }
        if *string as ::core::ffi::c_int == '}' as i32 {
            break;
        }
        menuItem = XtMalloc(::core::mem::size_of::<MenuItem>() as Cardinal) as *mut MenuItem;
        if menuItem.is_null() {
            PWarning(
                b"Insufficient memory for menu item\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else {
            (*menuItem).nextMenuItem = ::core::ptr::null_mut::<_MenuItem>();
            (*menuItem).wmFunction =
                ::core::mem::transmute::<*mut ::core::ffi::c_void, WmFunction>(NULL);
            (*menuItem).wmFuncArgs = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if ParseWmLabel(pSD, menuItem, string) == 0 {
                XtFree(menuItem as *mut ::core::ffi::c_char);
            } else {
                ParseWmMnemonic(&raw mut lineP, menuItem);
                if ParseWmAccelerator(&raw mut lineP, menuItem) == 0 {
                    FreeMenuItem(menuItem);
                } else {
                    ix = ParseWmFunction(
                        &raw mut lineP,
                        CRS_MENU as ::core::ffi::c_uint,
                        &raw mut (*menuItem).wmFunction,
                    );
                    (*menuItem).greyedContext = functionTable[ix as usize].greyedContext;
                    (*menuItem).mgtMask = functionTable[ix as usize].mgtMask;
                    if Some(
                        (*(&raw mut functionTable as *mut FunctionTableEntry).offset(ix as isize))
                            .parseProc
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        &raw mut lineP,
                        (*menuItem).wmFunction,
                        &raw mut (*menuItem).wmFuncArgs as *mut ::core::ffi::c_void,
                    ) == 0
                    {
                        FreeMenuItem(menuItem);
                    } else {
                        if !lastMenuItem.is_null() {
                            (*lastMenuItem).nextMenuItem = menuItem as *mut _MenuItem;
                        } else {
                            firstMenuItem = menuItem;
                        }
                        lastMenuItem = menuItem;
                    }
                }
            }
        }
    }
    return firstMenuItem;
}
unsafe extern "C" fn ParseWmLabel(
    mut pSD: *mut WmScreenData,
    mut menuItem: *mut MenuItem,
    mut string: *mut ::core::ffi::c_uchar,
) -> Boolean {
    (*menuItem).label =
        XtMalloc(strlen(string as *mut ::core::ffi::c_char).wrapping_add(1 as size_t) as Cardinal);
    if (*menuItem).label.is_null() {
        PWarning(
            b"Insufficient memory for menu item\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return 0 as Boolean;
    }
    strcpy(
        (*menuItem).label as *mut ::core::ffi::c_char,
        string as *mut ::core::ffi::c_char,
    );
    (*menuItem).labelType = XmSTRING as ::core::ffi::c_int;
    if *string as ::core::ffi::c_int == '@' as i32 {
        string = string.offset(1);
        (*menuItem).labelBitmapIndex =
            GetBitmapIndex(pSD, string as *mut ::core::ffi::c_char, True as Boolean);
        if (*menuItem).labelBitmapIndex >= 0 as ::core::ffi::c_int {
            (*menuItem).labelType = XmPIXMAP as ::core::ffi::c_int;
        }
    }
    return 1 as Boolean;
}
unsafe extern "C" fn ParseWmMnemonic(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut menuItem: *mut MenuItem,
) {
    let mut lineP = *linePP;
    let mut mnemonic = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    ScanWhitespace(&raw mut lineP);
    (*menuItem).mnemonic = NULL as KeySym;
    if *lineP as ::core::ffi::c_int == '_' as i32 {
        let mut ks: KeySym = 0;
        lineP = lineP.offset(1);
        mnemonic = GetString(&raw mut lineP);
        if (*menuItem).labelType == XmSTRING as ::core::ffi::c_int
            && !mnemonic.is_null()
            && {
                ks = XStringToKeysym(mnemonic as *mut ::core::ffi::c_char);
                ks != NoSymbol as KeySym
            }
            && !strchr(
                (*menuItem).label as *const ::core::ffi::c_char,
                (ks & 0xff as KeySym) as ::core::ffi::c_char as ::core::ffi::c_int,
            )
            .is_null()
        {
            (*menuItem).mnemonic = ks;
        } else {
            PWarning(
                b"Invalid mnemonic specification\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    }
    *linePP = lineP;
}
unsafe extern "C" fn ParseWmAccelerator(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut menuItem: *mut MenuItem,
) -> Boolean {
    let mut lineP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut eventType: ::core::ffi::c_uint = 0;
    let mut state: ::core::ffi::c_uint = 0;
    let mut keycode: KeyCode = 0;
    let mut status: Boolean = 0;
    (*menuItem).accelState = 0 as ::core::ffi::c_uint;
    (*menuItem).accelKeyCode = 0 as KeyCode;
    (*menuItem).accelText = ::core::ptr::null_mut::<::core::ffi::c_char>();
    status = TRUE as Boolean;
    if (*linePP).is_null() {
        return 0 as Boolean;
    }
    ScanWhitespace(linePP);
    lineP = *linePP;
    if *lineP as ::core::ffi::c_int != '\0' as i32
        && *lineP as ::core::ffi::c_int != '!' as i32
        && *lineP as ::core::ffi::c_int != 'f' as i32
        && *lineP.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '.' as i32
    {
        if ParseKeyEvent(
            &raw mut lineP,
            &raw mut eventType,
            &raw mut keycode,
            &raw mut state,
        ) != 0
        {
            string = XtMalloc(
                (lineP.offset_from(*linePP) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                    as Cardinal,
            );
            if string.is_null() {
                PWarning(
                    b"Insufficient memory for accelerator specification\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                status = FALSE as Boolean;
            } else {
                ProcessAccelText(*linePP, lineP, string as *mut ::core::ffi::c_uchar);
                (*menuItem).accelState = state;
                (*menuItem).accelKeyCode = keycode;
                (*menuItem).accelText = string;
            }
        } else {
            PWarning(
                b"Invalid accelerator specification\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            status = FALSE as Boolean;
        }
        *linePP = lineP;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn ParseWmFunction(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut res_spec: ::core::ffi::c_uint,
    mut pWmFunction: *mut WmFunction,
) -> ::core::ffi::c_int {
    let mut lineP = *linePP;
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut low: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    let mut high: ::core::ffi::c_int = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    ScanWhitespace(&raw mut lineP);
    if *lineP as ::core::ffi::c_int == '!' as i32 {
        lineP = lineP.offset(1);
        *linePP = lineP;
        *pWmFunction =
            Some(F_Exec as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean)
                as WmFunction;
        return F_EXEC_INDEX;
    }
    string = GetString(&raw mut lineP);
    *linePP = lineP;
    if !string.is_null() {
        ToLower(string);
        low = 0 as ::core::ffi::c_int;
        high = WMFUNCTIONTABLESIZE.wrapping_sub(1 as usize) as ::core::ffi::c_int;
        while low <= high {
            mid = (low + high) / 2 as ::core::ffi::c_int;
            cmp = strcmp(
                functionTable[mid as usize].funcName,
                string as *mut ::core::ffi::c_char,
            );
            if cmp == 0 {
                if res_spec & functionTable[mid as usize].resource != 0 {
                    *pWmFunction = functionTable[mid as usize].wmFunction;
                    return mid;
                }
                break;
            } else if cmp > 0 as ::core::ffi::c_int {
                high = mid - 1 as ::core::ffi::c_int;
            } else {
                low = mid + 1 as ::core::ffi::c_int;
            }
        }
    }
    *pWmFunction =
        Some(F_Nop as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean)
            as WmFunction;
    return F_NOP_INDEX;
}
unsafe extern "C" fn ParseWmFuncMaybeStrArg(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut wmFunction: WmFunction,
    mut pArgs: *mut String_0,
) -> Boolean {
    let mut string = *linePP;
    let mut len: ::core::ffi::c_uint = 0;
    ScanWhitespace(&raw mut string);
    len = strlen(string as *mut ::core::ffi::c_char) as ::core::ffi::c_uint;
    if len != 0 as ::core::ffi::c_uint {
        *pArgs = XtMalloc((len as Cardinal).wrapping_add(1 as Cardinal));
        if (*pArgs).is_null() {
            PWarning(
                b"Insufficient memory\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return 0 as Boolean;
        }
        strcpy(*pArgs, string as *mut ::core::ffi::c_char);
        return 1 as Boolean;
    } else {
        *pArgs = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as Boolean;
    };
}
unsafe extern "C" fn ParseWmFuncNoArg(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut wmFunction: WmFunction,
    mut pArgs: *mut String_0,
) -> Boolean {
    *pArgs = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return 1 as Boolean;
}
unsafe extern "C" fn ParseWmFuncStrArg(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut wmFunction: WmFunction,
    mut pArgs: *mut String_0,
) -> Boolean {
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_uint = 0;
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut last: wchar_t = 0;
    let mut delim: ::core::ffi::c_char = 0;
    let mut wdelim: wchar_t = 0;
    let mut lastlen: ::core::ffi::c_int = 0;
    string = GetString(linePP);
    if !string.is_null() {
        len = strlen(string as *mut ::core::ffi::c_char) as ::core::ffi::c_uint;
        *pArgs = XtMalloc((len as Cardinal).wrapping_add(2 as Cardinal));
        if (*pArgs).is_null() {
            PWarning(
                b"Insufficient memory\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return 0 as Boolean;
        }
        strcpy(*pArgs, string as *mut ::core::ffi::c_char);
        if wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Exec as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) {
            lastlen = 0 as ::core::ffi::c_int;
            p = *pArgs as *mut ::core::ffi::c_char;
            while *p as ::core::ffi::c_int != 0 && {
                len = mblen(p, __ctype_get_mb_cur_max()) as ::core::ffi::c_uint;
                len > 0 as ::core::ffi::c_uint
            } {
                mbtowc(&raw mut last, p, __ctype_get_mb_cur_max());
                lastlen = len as ::core::ffi::c_int;
                p = p.offset(len as isize);
            }
            delim = '&' as i32 as ::core::ffi::c_char;
            mbtowc(&raw mut wdelim, &raw mut delim, __ctype_get_mb_cur_max());
            if lastlen == 1 as ::core::ffi::c_int && last != wdelim {
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = '&' as i32 as ::core::ffi::c_char;
                *p = '\0' as i32 as ::core::ffi::c_char;
            }
        }
    } else {
        *pArgs = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn FreeMenuItem(mut menuItem: *mut MenuItem) {
    if !(*menuItem).label.is_null() {
        XtFree((*menuItem).label);
    }
    if !(*menuItem).accelText.is_null() {
        XtFree((*menuItem).accelText);
    }
    if !(*menuItem).wmFuncArgs.is_null()
        && ((*menuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Exec as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) || (*menuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Menu as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) || (*menuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Lower as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) || (*menuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Raise as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) || (*menuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Raise_Lower
                    as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }) || (*menuItem).wmFunction.is_some_and(|action| {
            std::ptr::fn_addr_eq(
                action,
                F_Screen as unsafe extern "C" fn(String_0, *mut ClientData, *mut XEvent) -> Boolean,
            )
        }))
    {
        XtFree((*menuItem).wmFuncArgs);
    }
    XtFree(menuItem as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn ParseWmFuncGrpArg(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut wmFunction: WmFunction,
    mut pGroup: *mut GroupArg,
) -> Boolean {
    let mut lineP = *linePP;
    let mut startP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut grpStr: [::core::ffi::c_uchar; 21] = [0; 21];
    let mut len: ::core::ffi::c_int = 0;
    *pGroup = 0 as GroupArg;
    loop {
        ScanWhitespace(&raw mut lineP);
        startP = lineP;
        ScanAlphanumeric(&raw mut lineP);
        if startP == lineP {
            if *pGroup != 0 {
                PWarning(
                    b"Missing group specification\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 0 as Boolean;
            } else {
                *pGroup = F_GROUP_DEFAULT as GroupArg;
                break;
            }
        } else {
            len = (if lineP.offset_from(startP) as ::core::ffi::c_long > 20 as ::core::ffi::c_long {
                20 as ::core::ffi::c_long
            } else {
                lineP.offset_from(startP) as ::core::ffi::c_long
            }) as ::core::ffi::c_int;
            strncpy(
                &raw mut grpStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
                startP as *mut ::core::ffi::c_char,
                len as size_t,
            );
            grpStr[len as usize] = '\0' as i32 as ::core::ffi::c_uchar;
            ToLower(&raw mut grpStr as *mut ::core::ffi::c_uchar);
            if strcmp(
                b"icon\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut grpStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            ) == 0
            {
                *pGroup |= F_GROUP_ICON as GroupArg;
            } else if strcmp(
                b"window\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut grpStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            ) == 0
            {
                *pGroup |= F_GROUP_WINDOW as GroupArg;
            } else if strcmp(
                b"transient\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut grpStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            ) == 0
            {
                *pGroup |= F_GROUP_TRANSIENT as GroupArg;
            } else {
                PWarning(
                    b"Invalid group specification\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 0 as Boolean;
            }
            ScanWhitespace(&raw mut lineP);
            if lineP.is_null() || *lineP as ::core::ffi::c_int == '\0' as i32 {
                break;
            }
            if *lineP as ::core::ffi::c_int == '|' as i32 {
                lineP = lineP.offset(1);
            }
        }
    }
    *linePP = lineP;
    return 1 as Boolean;
}
unsafe extern "C" fn ParseWmFuncNbrArg(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut wmFunction: WmFunction,
    mut pNumber: *mut ::core::ffi::c_ulong,
) -> Boolean {
    let mut val: ::core::ffi::c_int = 0;
    val = StrToNum(GetString(linePP)) as ::core::ffi::c_int;
    if val == -(1 as ::core::ffi::c_int) {
        PWarning(
            b"Invalid number specification\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        *pNumber = 0 as ::core::ffi::c_ulong;
        return 0 as Boolean;
    }
    *pNumber = val as ::core::ffi::c_ulong;
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ParseButtonStr(
    mut pSD: *mut WmScreenData,
    mut buttonStr: *mut ::core::ffi::c_uchar,
) {
    let mut lineP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    cfileP = ::core::ptr::null_mut::<FILE>();
    linec = 0 as ::core::ffi::c_int;
    parseP = buttonStr;
    if !parseP.is_null() && !GetNextLine().is_null() {
        lineP = &raw mut line as *mut ::core::ffi::c_uchar;
        ParseButtonSet(pSD, lineP);
    }
}
unsafe extern "C" fn ParseButtonSet(
    mut pSD: *mut WmScreenData,
    mut lineP: *mut ::core::ffi::c_uchar,
) {
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut buttonSpec = ::core::ptr::null_mut::<ButtonSpec>();
    let mut lastButtonSpec = ::core::ptr::null_mut::<ButtonSpec>();
    let mut ix: ::core::ffi::c_int = 0;
    string = GetString(&raw mut lineP);
    if string.is_null()
        || (*pSD).buttonBindings.is_null()
        || strcmp(
            string as *mut ::core::ffi::c_char,
            (*pSD).buttonBindings as *const ::core::ffi::c_char,
        ) != 0
    {
        return;
    }
    while !GetNextLine().is_null() {
        lineP = &raw mut line as *mut ::core::ffi::c_uchar;
        ScanWhitespace(&raw mut lineP);
        if lineP.is_null()
            || *(&raw mut line as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int == '!' as i32
            || *lineP as ::core::ffi::c_int == '\0' as i32
            || *lineP as ::core::ffi::c_int == '#' as i32
        {
            continue;
        }
        if *lineP as ::core::ffi::c_int == '{' as i32 {
            break;
        }
        PWarning(
            b"Expected '{' after button set name\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return;
    }
    lastButtonSpec = (*pSD).buttonSpecs;
    if !lastButtonSpec.is_null() {
        while !(*lastButtonSpec).nextButtonSpec.is_null() {
            lastButtonSpec = (*lastButtonSpec).nextButtonSpec as *mut ButtonSpec;
        }
    }
    while !GetNextLine().is_null() {
        lineP = &raw mut line as *mut ::core::ffi::c_uchar;
        ScanWhitespace(&raw mut lineP);
        if lineP.is_null()
            || *(&raw mut line as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int == '!' as i32
            || *lineP as ::core::ffi::c_int == '\0' as i32
            || *lineP as ::core::ffi::c_int == '#' as i32
        {
            continue;
        }
        if *lineP as ::core::ffi::c_int == '}' as i32 {
            break;
        }
        buttonSpec = XtMalloc(::core::mem::size_of::<ButtonSpec>() as Cardinal) as *mut ButtonSpec;
        if buttonSpec.is_null() {
            PWarning(
                b"Insufficient memory for button specification\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            (*buttonSpec).wmFunction =
                ::core::mem::transmute::<*mut ::core::ffi::c_void, WmFunction>(NULL);
            (*buttonSpec).wmFuncArgs = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*buttonSpec).nextButtonSpec = ::core::ptr::null_mut::<_ButtonSpec>();
            lineP = &raw mut line as *mut ::core::ffi::c_uchar;
            if ParseBtnEvent(
                &raw mut lineP,
                &raw mut (*buttonSpec).eventType,
                &raw mut (*buttonSpec).button,
                &raw mut (*buttonSpec).state,
                &raw mut (*buttonSpec).click,
            ) == 0
            {
                PWarning(
                    b"Invalid button specification\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                XtFree(buttonSpec as *mut ::core::ffi::c_char);
            } else if ParseContext(
                &raw mut lineP,
                &raw mut (*buttonSpec).context,
                &raw mut (*buttonSpec).subContext,
            ) == 0
            {
                PWarning(
                    b"Invalid button context\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                XtFree(buttonSpec as *mut ::core::ffi::c_char);
            } else {
                ix = ParseWmFunction(
                    &raw mut lineP,
                    CRS_BUTTON as ::core::ffi::c_uint,
                    &raw mut (*buttonSpec).wmFunction,
                );
                if functionTable[ix as usize].greyedContext & F_SUBCONTEXT_IB_IICON as Context != 0
                    && (*buttonSpec).subContext & F_SUBCONTEXT_IB_IICON as Context != 0
                {
                    (*buttonSpec).subContext &= !F_SUBCONTEXT_IB_IICON as Context;
                }
                if functionTable[ix as usize].greyedContext & F_SUBCONTEXT_IB_WICON as Context != 0
                    && (*buttonSpec).subContext & F_SUBCONTEXT_IB_WICON as Context != 0
                {
                    (*buttonSpec).subContext &= !F_SUBCONTEXT_IB_WICON as Context;
                }
                if (*buttonSpec).button == Button3 as ::core::ffi::c_uint
                    && ((*buttonSpec).wmFunction.is_some_and(|action| {
                        std::ptr::fn_addr_eq(
                            action,
                            F_Menu
                                as unsafe extern "C" fn(
                                    String_0,
                                    *mut ClientData,
                                    *mut XEvent,
                                ) -> Boolean,
                        )
                    }) || (*buttonSpec).wmFunction.is_some_and(|action| {
                        std::ptr::fn_addr_eq(
                            action,
                            F_Post_SMenu
                                as unsafe extern "C" fn(
                                    String_0,
                                    *mut ClientData,
                                    *mut XEvent,
                                ) -> Boolean,
                        )
                    }))
                {
                    (*buttonSpec).button = wmGD.bMenuButton;
                }
                if Some(
                    (*(&raw mut functionTable as *mut FunctionTableEntry).offset(ix as isize))
                        .parseProc
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    &raw mut lineP,
                    (*buttonSpec).wmFunction,
                    &raw mut (*buttonSpec).wmFuncArgs as *mut ::core::ffi::c_void,
                ) == 0
                {
                    XtFree(buttonSpec as *mut ::core::ffi::c_char);
                } else {
                    if !lastButtonSpec.is_null() {
                        (*lastButtonSpec).nextButtonSpec = buttonSpec as *mut _ButtonSpec;
                    } else {
                        (*pSD).buttonSpecs = buttonSpec;
                    }
                    lastButtonSpec = buttonSpec;
                }
            }
        }
    }
}
unsafe extern "C" fn ParseContext(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut context: *mut Context,
    mut subContext: *mut Context,
) -> Boolean {
    let mut lineP = *linePP;
    let mut startP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut ctxStr: [::core::ffi::c_uchar; 21] = [0; 21];
    let mut len: ::core::ffi::c_int = 0;
    *context = 0 as Context;
    *subContext = 0 as Context;
    loop {
        ScanWhitespace(&raw mut lineP);
        startP = lineP;
        ScanAlphanumeric(&raw mut lineP);
        if startP == lineP {
            return 0 as Boolean;
        }
        len = (if lineP.offset_from(startP) as ::core::ffi::c_long > 20 as ::core::ffi::c_long {
            20 as ::core::ffi::c_long
        } else {
            lineP.offset_from(startP) as ::core::ffi::c_long
        }) as ::core::ffi::c_int;
        strncpy(
            &raw mut ctxStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            startP as *mut ::core::ffi::c_char,
            len as size_t,
        );
        ctxStr[len as usize] = '\0' as i32 as ::core::ffi::c_uchar;
        ToLower(&raw mut ctxStr as *mut ::core::ffi::c_uchar);
        if strcmp(
            b"root\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut ctxStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        ) == 0
        {
            *context |= F_CONTEXT_ROOT as Context;
            *subContext |= F_SUBCONTEXT_R_ALL as Context;
        } else if strcmp(
            b"icon\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut ctxStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        ) == 0
        {
            *context |= (F_CONTEXT_ICON
                | F_CONTEXT_ICONBOX
                | F_SUBCONTEXT_IB_IICON
                | F_SUBCONTEXT_IB_WICON) as Context;
            *subContext |=
                (F_SUBCONTEXT_I_ALL | F_SUBCONTEXT_IB_IICON | F_SUBCONTEXT_IB_WICON) as Context;
        } else if strcmp(
            b"window\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut ctxStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        ) == 0
        {
            *context |= F_CONTEXT_WINDOW as Context;
            *subContext |= F_SUBCONTEXT_W_ALL as Context;
        } else if strcmp(
            b"frame\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut ctxStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        ) == 0
        {
            *context |= F_CONTEXT_WINDOW as Context;
            *subContext |= F_SUBCONTEXT_W_FRAME as Context;
        } else if strcmp(
            b"title\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut ctxStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        ) == 0
        {
            *context |= F_CONTEXT_WINDOW as Context;
            *subContext |= F_SUBCONTEXT_W_TITLE as Context;
        } else if strcmp(
            b"border\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut ctxStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        ) == 0
        {
            *context |= F_CONTEXT_WINDOW as Context;
            *subContext |= F_SUBCONTEXT_W_BORDER as Context;
        } else if strcmp(
            b"app\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut ctxStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        ) == 0
        {
            *context |= F_CONTEXT_WINDOW as Context;
            *subContext |= F_SUBCONTEXT_W_APP as Context;
        } else if strcmp(
            b"ifkey\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut ctxStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        ) == 0
        {
            *context |= F_CONTEXT_IFKEY as Context;
        } else {
            return 0 as Boolean;
        }
        ScanWhitespace(&raw mut lineP);
        if *lineP as ::core::ffi::c_int != '|' as i32 {
            break;
        }
        lineP = lineP.offset(1);
    }
    *linePP = lineP;
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ParseKeyStr(
    mut pSD: *mut WmScreenData,
    mut keyStr: *mut ::core::ffi::c_uchar,
) {
    let mut lineP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    cfileP = ::core::ptr::null_mut::<FILE>();
    linec = 0 as ::core::ffi::c_int;
    parseP = keyStr;
    if !parseP.is_null() && !GetNextLine().is_null() {
        lineP = &raw mut line as *mut ::core::ffi::c_uchar;
        ParseKeySet(pSD, lineP);
    }
}
unsafe extern "C" fn ParseKeySet(mut pSD: *mut WmScreenData, mut lineP: *mut ::core::ffi::c_uchar) {
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut keySpec = ::core::ptr::null_mut::<KeySpec>();
    let mut lastKeySpec = ::core::ptr::null_mut::<KeySpec>();
    let mut eventType: ::core::ffi::c_uint = 0;
    let mut ix: ::core::ffi::c_int = 0;
    let mut bBadKey: Boolean = 0;
    string = GetString(&raw mut lineP);
    if string.is_null()
        || (*pSD).keyBindings.is_null()
        || strcmp(
            string as *mut ::core::ffi::c_char,
            (*pSD).keyBindings as *const ::core::ffi::c_char,
        ) != 0
    {
        return;
    }
    while !GetNextLine().is_null() {
        lineP = &raw mut line as *mut ::core::ffi::c_uchar;
        ScanWhitespace(&raw mut lineP);
        if !(lineP.is_null()
            || *(&raw mut line as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int == '!' as i32
            || *lineP as ::core::ffi::c_int == '\0' as i32
            || *lineP as ::core::ffi::c_int == '#' as i32)
        {
            if *lineP as ::core::ffi::c_int == '{' as i32 {
                break;
            }
            PWarning(
                b"Expected '{' after key set name\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return;
        }
    }
    lastKeySpec = (*pSD).keySpecs;
    if !lastKeySpec.is_null() {
        while !(*lastKeySpec).nextKeySpec.is_null() {
            lastKeySpec = (*lastKeySpec).nextKeySpec as *mut KeySpec;
        }
    }
    while !GetNextLine().is_null() {
        lineP = &raw mut line as *mut ::core::ffi::c_uchar;
        ScanWhitespace(&raw mut lineP);
        if !(lineP.is_null()
            || *(&raw mut line as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int == '!' as i32
            || *lineP as ::core::ffi::c_int == '\0' as i32
            || *lineP as ::core::ffi::c_int == '#' as i32)
        {
            if *lineP as ::core::ffi::c_int == '}' as i32 {
                break;
            }
            keySpec = XtMalloc(::core::mem::size_of::<KeySpec>() as Cardinal) as *mut KeySpec;
            if keySpec.is_null() {
                PWarning(
                    b"Insufficient memory for key specification\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                (*keySpec).wmFunction =
                    ::core::mem::transmute::<*mut ::core::ffi::c_void, WmFunction>(NULL);
                (*keySpec).wmFuncArgs = ::core::ptr::null_mut::<::core::ffi::c_char>();
                (*keySpec).nextKeySpec = ::core::ptr::null_mut::<_KeySpec>();
                bBadKey = False as Boolean;
                if ParseKeyEvent(
                    &raw mut lineP,
                    &raw mut eventType,
                    &raw mut (*keySpec).keycode,
                    &raw mut (*keySpec).state,
                ) == 0
                {
                    bBadKey = True as Boolean;
                }
                if ParseContext(
                    &raw mut lineP,
                    &raw mut (*keySpec).context,
                    &raw mut (*keySpec).subContext,
                ) == 0
                {
                    if bBadKey != 0 {
                        PWarning(
                            b"Invalid key specification\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    }
                    PWarning(
                        b"Invalid key context\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    XtFree(keySpec as *mut ::core::ffi::c_char);
                } else if bBadKey != 0 {
                    if (*keySpec).context & F_CONTEXT_IFKEY as Context == 0 {
                        PWarning(
                            b"Invalid key specification\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    }
                    XtFree(keySpec as *mut ::core::ffi::c_char);
                } else {
                    (*keySpec).context &= !F_CONTEXT_IFKEY as Context;
                    ix = ParseWmFunction(
                        &raw mut lineP,
                        CRS_KEY as ::core::ffi::c_uint,
                        &raw mut (*keySpec).wmFunction,
                    );
                    if functionTable[ix as usize].greyedContext & F_SUBCONTEXT_IB_IICON as Context
                        != 0
                        && (*keySpec).subContext & F_SUBCONTEXT_IB_IICON as Context != 0
                    {
                        (*keySpec).subContext &= !F_SUBCONTEXT_IB_IICON as Context;
                    }
                    if functionTable[ix as usize].greyedContext & F_SUBCONTEXT_IB_WICON as Context
                        != 0
                        && (*keySpec).subContext & F_SUBCONTEXT_IB_WICON as Context != 0
                    {
                        (*keySpec).subContext &= !F_SUBCONTEXT_IB_WICON as Context;
                    }
                    if Some(
                        (*(&raw mut functionTable as *mut FunctionTableEntry).offset(ix as isize))
                            .parseProc
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        &raw mut lineP,
                        (*keySpec).wmFunction,
                        &raw mut (*keySpec).wmFuncArgs as *mut ::core::ffi::c_void,
                    ) == 0
                    {
                        XtFree(keySpec as *mut ::core::ffi::c_char);
                    } else {
                        if !lastKeySpec.is_null() {
                            (*lastKeySpec).nextKeySpec = keySpec as *mut _KeySpec;
                        } else {
                            (*pSD).keySpecs = keySpec;
                        }
                        lastKeySpec = keySpec;
                    }
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetNextLine() -> *mut ::core::ffi::c_uchar {
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    let mut chlen: ::core::ffi::c_int = 0;
    let mut last: wchar_t = 0;
    let mut wdelim: wchar_t = 0;
    let mut delim: ::core::ffi::c_char = 0;
    let mut lastlen: ::core::ffi::c_int = 0;
    if !cfileP.is_null() {
        string = fgets(
            &raw mut line as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            MAXLINE,
            cfileP,
        ) as *mut ::core::ffi::c_uchar;
        if !string.is_null() {
            lastlen = 0 as ::core::ffi::c_int;
            while *string as ::core::ffi::c_int != 0 && {
                len = mblen(string as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
                len > 0 as ::core::ffi::c_int
            } {
                mbtowc(
                    &raw mut last,
                    string as *mut ::core::ffi::c_char,
                    __ctype_get_mb_cur_max(),
                );
                lastlen = len;
                string = string.offset(len as isize);
            }
            delim = '\\' as i32 as ::core::ffi::c_char;
            mbtowc(&raw mut wdelim, &raw mut delim, __ctype_get_mb_cur_max());
            if lastlen == 1 as ::core::ffi::c_int && last == wdelim {
                while !fgets(
                    string as *mut ::core::ffi::c_char,
                    (MAXLINE as ::core::ffi::c_long
                        - string.offset_from(&raw mut line as *mut ::core::ffi::c_uchar)
                            as ::core::ffi::c_long) as ::core::ffi::c_int,
                    cfileP,
                )
                .is_null()
                {
                    lastlen = 0 as ::core::ffi::c_int;
                    while *string as ::core::ffi::c_int != 0 && {
                        len = mblen(string as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
                        len > 0 as ::core::ffi::c_int
                    } {
                        mbtowc(
                            &raw mut last,
                            string as *mut ::core::ffi::c_char,
                            __ctype_get_mb_cur_max(),
                        );
                        lastlen = len;
                        string = string.offset(len as isize);
                    }
                    linec += 1;
                    if !(lastlen == 1 as ::core::ffi::c_int && last == wdelim) {
                        break;
                    }
                }
            }
            string = &raw mut line as *mut ::core::ffi::c_uchar;
        }
    } else if !parseP.is_null() && *parseP as ::core::ffi::c_int != '\0' as i32 {
        string = &raw mut line as *mut ::core::ffi::c_uchar;
        chlen = mblen(parseP as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
        if chlen == -(1 as ::core::ffi::c_int) {
            string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        while *parseP as ::core::ffi::c_int != '\0' as i32
            && {
                chlen = mblen(parseP as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
                chlen > 0 as ::core::ffi::c_int
            }
            && *parseP as ::core::ffi::c_int != '\n' as i32
        {
            loop {
                let fresh10 = chlen;
                chlen = chlen - 1;
                if !(fresh10 != 0) {
                    break;
                }
                let fresh11 = parseP;
                parseP = parseP.offset(1);
                let fresh12 = string;
                string = string.offset(1);
                *fresh12 = *fresh11;
            }
        }
        if !string.is_null() {
            *string = '\0' as i32 as ::core::ffi::c_uchar;
        }
        if *parseP as ::core::ffi::c_int == '\n' as i32 {
            parseP = parseP.offset(1);
        }
    } else {
        string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    linec += 1;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn GetString(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
) -> *mut ::core::ffi::c_uchar {
    let mut lineP = *linePP;
    let mut endP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut curP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut lnwsP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut chlen: ::core::ffi::c_int = 0;
    ScanWhitespace(&raw mut lineP);
    if *lineP as ::core::ffi::c_int == '\0' as i32
        || {
            chlen = mblen(lineP as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
            chlen < 1 as ::core::ffi::c_int
        }
        || chlen > 0 as ::core::ffi::c_int
            && (*lineP as ::core::ffi::c_int == '!' as i32
                || *lineP as ::core::ffi::c_int == '#' as i32)
    {
        *linePP = lineP;
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if chlen == 1 as ::core::ffi::c_int && *lineP as ::core::ffi::c_int == '"' as i32 {
        let fresh1 = lineP;
        lineP = lineP.offset(1);
        lnwsP = fresh1;
        endP = lineP;
        curP = endP;
        loop {
            *endP = *curP;
            if !(*endP as ::core::ffi::c_int != 0
                && {
                    chlen = mblen(curP as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
                    chlen > 0 as ::core::ffi::c_int
                }
                && (chlen > 1 as ::core::ffi::c_int || *curP as ::core::ffi::c_int != '"' as i32))
            {
                break;
            }
            curP = curP.offset(1);
            if chlen == 1 as ::core::ffi::c_int && *endP as ::core::ffi::c_int == '\\' as i32 && {
                chlen = mblen(curP as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
                chlen > 0 as ::core::ffi::c_int
            } {
                let fresh2 = curP;
                curP = curP.offset(1);
                *endP = *fresh2;
            }
            if chlen == 1 as ::core::ffi::c_int {
                if *(*__ctype_b_loc()).offset(*endP as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                    != 0
                {
                    endP = endP.offset(1);
                } else {
                    let fresh3 = endP;
                    endP = endP.offset(1);
                    lnwsP = fresh3;
                }
            } else if chlen > 1 as ::core::ffi::c_int {
                let fresh4 = endP;
                endP = endP.offset(1);
                lnwsP = fresh4;
                loop {
                    chlen -= 1;
                    if !(chlen != 0) {
                        break;
                    }
                    let fresh5 = curP;
                    curP = curP.offset(1);
                    let fresh6 = endP;
                    endP = endP.offset(1);
                    *fresh6 = *fresh5;
                    lnwsP = lnwsP.offset(1);
                }
            }
        }
        lnwsP = lnwsP.offset(1);
        if lnwsP < endP {
            *lnwsP = '\0' as i32 as ::core::ffi::c_uchar;
        }
    } else {
        endP = lineP;
        curP = endP;
        loop {
            *endP = *curP;
            if !(*endP as ::core::ffi::c_int != 0
                && {
                    chlen = mblen(curP as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
                    chlen > 0 as ::core::ffi::c_int
                }
                && (chlen > 1 as ::core::ffi::c_int
                    || *(*__ctype_b_loc()).offset(*curP as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        == 0
                        && *curP as ::core::ffi::c_int != '#' as i32))
            {
                break;
            }
            curP = curP.offset(1);
            if chlen == 1 as ::core::ffi::c_int && *endP as ::core::ffi::c_int == '\\' as i32 && {
                chlen = mblen(curP as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
                chlen > 0 as ::core::ffi::c_int
            } {
                let fresh7 = curP;
                curP = curP.offset(1);
                *endP = *fresh7;
            }
            endP = endP.offset(1);
            if chlen > 1 as ::core::ffi::c_int {
                loop {
                    chlen -= 1;
                    if !(chlen != 0) {
                        break;
                    }
                    let fresh8 = curP;
                    curP = curP.offset(1);
                    let fresh9 = endP;
                    endP = endP.offset(1);
                    *fresh9 = *fresh8;
                }
            }
        }
    }
    if *endP as ::core::ffi::c_int == '#' as i32 {
        *endP = '\0' as i32 as ::core::ffi::c_uchar;
        *linePP = endP;
    } else if *endP as ::core::ffi::c_int != '\0' as i32 {
        *endP = '\0' as i32 as ::core::ffi::c_uchar;
        curP = curP.offset(1);
        *linePP = curP;
    } else {
        *linePP = endP;
    }
    return lineP;
}
#[no_mangle]
pub unsafe extern "C" fn ParseBtnEvent(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut eventType: *mut ::core::ffi::c_uint,
    mut button: *mut ::core::ffi::c_uint,
    mut state: *mut ::core::ffi::c_uint,
    mut fClick: *mut Boolean,
) -> Boolean {
    if ParseEvent(
        linePP,
        &raw mut buttonEvents as *mut EventTableEntry,
        eventType,
        button,
        state,
        fClick,
    ) == 0
    {
        return 0 as Boolean;
    }
    if *eventType == ButtonRelease as ::core::ffi::c_uint {
        *state |= buttonModifierMasks[*button as usize];
    }
    return 1 as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn ParseKeyEvent(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut eventType: *mut ::core::ffi::c_uint,
    mut keyCode: *mut KeyCode,
    mut state: *mut ::core::ffi::c_uint,
) -> Boolean {
    let mut fClick: Boolean = 0;
    let mut keySym = 0 as ::core::ffi::c_uint;
    if ParseEvent(
        linePP,
        &raw mut keyEvents as *mut EventTableEntry,
        eventType,
        &raw mut keySym,
        state,
        &raw mut fClick,
    ) == 0
    {
        return 0 as Boolean;
    }
    *keyCode = XKeysymToKeycode(wmGD.display, keySym as KeySym);
    if *keyCode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if keySym == XK_F9 as ::core::ffi::c_uint {
            keySym = XK_KP_F1 as ::core::ffi::c_uint;
        } else if keySym == XK_F10 as ::core::ffi::c_uint {
            keySym = XK_KP_F2 as ::core::ffi::c_uint;
        } else if keySym == XK_F11 as ::core::ffi::c_uint {
            keySym = XK_KP_F3 as ::core::ffi::c_uint;
        } else if keySym == XK_F12 as ::core::ffi::c_uint {
            keySym = XK_KP_F4 as ::core::ffi::c_uint;
        }
        *keyCode = XKeysymToKeycode(wmGD.display, keySym as KeySym);
    }
    return (*keyCode as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as Boolean;
}
unsafe extern "C" fn ParseEvent(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut table: *mut EventTableEntry,
    mut eventType: *mut ::core::ffi::c_uint,
    mut detail: *mut ::core::ffi::c_uint,
    mut state: *mut ::core::ffi::c_uint,
    mut fClick: *mut Boolean,
) -> Boolean {
    let mut lineP = *linePP;
    let mut ix: Cardinal = 0;
    let mut status: Boolean = 0;
    if ParseModifiers(&raw mut lineP, state) == 0 || *lineP as ::core::ffi::c_int != '<' as i32 {
        return 0 as Boolean;
    }
    lineP = lineP.offset(1);
    if ParseEventType(&raw mut lineP, table, eventType, &raw mut ix) == 0
        || *lineP as ::core::ffi::c_int != '>' as i32
    {
        return 0 as Boolean;
    }
    lineP = lineP.offset(1);
    status = Some(
        (*table.offset(ix as isize))
            .parseProc
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        &raw mut lineP,
        (*table.offset(ix as isize)).closure,
        detail,
    );
    *fClick = (*table.offset(ix as isize)).fClick;
    if status != 0 {
        *linePP = lineP;
    }
    return status;
}
unsafe extern "C" fn ParseModifiers(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut state: *mut ::core::ffi::c_uint,
) -> Boolean {
    let mut lineP = *linePP;
    let mut startP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut modStr: [::core::ffi::c_uchar; 21] = [0; 21];
    let mut fNot: Boolean = 0;
    let mut maskBit: ::core::ffi::c_uint = 0;
    let mut len: ::core::ffi::c_int = 0;
    *state = 0 as ::core::ffi::c_uint;
    ScanWhitespace(&raw mut lineP);
    while *lineP as ::core::ffi::c_int != '\0' as i32 && *lineP as ::core::ffi::c_int != '<' as i32
    {
        if *lineP as ::core::ffi::c_int == '~' as i32 {
            fNot = TRUE as Boolean;
            lineP = lineP.offset(1);
        } else {
            fNot = FALSE as Boolean;
        }
        startP = lineP;
        ScanAlphanumeric(&raw mut lineP);
        if startP == lineP {
            return 0 as Boolean;
        }
        len = (if lineP.offset_from(startP) as ::core::ffi::c_long > 20 as ::core::ffi::c_long {
            20 as ::core::ffi::c_long
        } else {
            lineP.offset_from(startP) as ::core::ffi::c_long
        }) as ::core::ffi::c_int;
        strncpy(
            &raw mut modStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            startP as *mut ::core::ffi::c_char,
            len as size_t,
        );
        modStr[len as usize] = '\0' as i32 as ::core::ffi::c_uchar;
        if LookupModifier(
            &raw mut modStr as *mut ::core::ffi::c_uchar,
            &raw mut maskBit,
        ) == 0
        {
            return 0 as Boolean;
        }
        if fNot != 0 {
            *state &= !maskBit;
        } else {
            *state |= maskBit;
        }
        ScanWhitespace(&raw mut lineP);
    }
    *linePP = lineP;
    return 1 as Boolean;
}
unsafe extern "C" fn LookupModifier(
    mut name: *mut ::core::ffi::c_uchar,
    mut valueP: *mut ::core::ffi::c_uint,
) -> Boolean {
    let mut i: ::core::ffi::c_int = 0;
    if !name.is_null() {
        ToLower(name);
        i = 0 as ::core::ffi::c_int;
        while !modifierStrings[i as usize].name.is_null() {
            if strcmp(
                modifierStrings[i as usize].name,
                name as *mut ::core::ffi::c_char,
            ) == 0
            {
                *valueP = modifierStrings[i as usize].mask;
                return 1 as Boolean;
            }
            i += 1;
        }
    }
    return 0 as Boolean;
}
unsafe extern "C" fn ParseEventType(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut table: *mut EventTableEntry,
    mut eventType: *mut ::core::ffi::c_uint,
    mut ix: *mut Cardinal,
) -> Boolean {
    let mut lineP = *linePP;
    let mut startP = *linePP;
    let mut eventTypeStr: [::core::ffi::c_uchar; 21] = [0; 21];
    let mut len: ::core::ffi::c_int = 0;
    ScanAlphanumeric(&raw mut lineP);
    if startP != lineP {
        len = (if lineP.offset_from(startP) as ::core::ffi::c_long > 20 as ::core::ffi::c_long {
            20 as ::core::ffi::c_long
        } else {
            lineP.offset_from(startP) as ::core::ffi::c_long
        }) as ::core::ffi::c_int;
        strncpy(
            &raw mut eventTypeStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            startP as *mut ::core::ffi::c_char,
            len as size_t,
        );
        eventTypeStr[len as usize] = '\0' as i32 as ::core::ffi::c_uchar;
        ToLower(&raw mut eventTypeStr as *mut ::core::ffi::c_uchar);
        len = 0 as ::core::ffi::c_int;
        while !(*table.offset(len as isize)).event.is_null() {
            if strcmp(
                (*table.offset(len as isize)).event,
                &raw mut eventTypeStr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            ) == 0
            {
                *ix = len as Cardinal;
                *eventType = (*table.offset(*ix as isize)).eventType;
                *linePP = lineP;
                return 1 as Boolean;
            }
            len += 1;
        }
    }
    return 0 as Boolean;
}
unsafe extern "C" fn ParseImmed(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut closure: ::core::ffi::c_uint,
    mut detail: *mut ::core::ffi::c_uint,
) -> Boolean {
    *detail = closure;
    return 1 as Boolean;
}
unsafe extern "C" fn ParseKeySym(
    mut linePP: *mut *mut ::core::ffi::c_uchar,
    mut closure: ::core::ffi::c_uint,
    mut detail: *mut ::core::ffi::c_uint,
) -> Boolean {
    let mut lineP = *linePP;
    let mut startP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut keySymName: [::core::ffi::c_char; 101] = [0; 101];
    let mut len: ::core::ffi::c_int = 0;
    let mut chlen: ::core::ffi::c_int = 0;
    ScanWhitespace(&raw mut lineP);
    startP = lineP;
    while *lineP as ::core::ffi::c_int != 0
        && {
            chlen = mblen(lineP as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
            chlen > 0 as ::core::ffi::c_int
        }
        && (chlen > 1 as ::core::ffi::c_int
            || *(*__ctype_b_loc()).offset(*lineP as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                == 0
                && *lineP as ::core::ffi::c_int != ',' as i32
                && *lineP as ::core::ffi::c_int != ':' as i32)
    {
        lineP = lineP.offset(chlen as isize);
    }
    len = (if lineP.offset_from(startP) as ::core::ffi::c_long > 100 as ::core::ffi::c_long {
        100 as ::core::ffi::c_long
    } else {
        lineP.offset_from(startP) as ::core::ffi::c_long
    }) as ::core::ffi::c_int;
    strncpy(
        &raw mut keySymName as *mut ::core::ffi::c_char,
        startP as *mut ::core::ffi::c_char,
        len as size_t,
    );
    keySymName[len as usize] = '\0' as i32 as ::core::ffi::c_char;
    *detail =
        XStringToKeysym(&raw mut keySymName as *mut ::core::ffi::c_char) as ::core::ffi::c_uint;
    if *detail as ::core::ffi::c_long == NoSymbol
        && mblen(
            &raw mut keySymName as *mut ::core::ffi::c_char,
            __ctype_get_mb_cur_max(),
        ) == 1 as ::core::ffi::c_int
    {
        if *(*__ctype_b_loc())
            .offset(keySymName[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            == 0
            || {
                *detail = StrToNum(
                    (&raw mut keySymName as *mut ::core::ffi::c_char)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                );
                *detail == -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint
            }
        {
            *detail = NoSymbol as ::core::ffi::c_uint;
            return 0 as Boolean;
        }
    }
    *linePP = lineP;
    return 1 as Boolean;
}
unsafe extern "C" fn StrToNum(mut str: *mut ::core::ffi::c_uchar) -> ::core::ffi::c_uint {
    let mut c: ::core::ffi::c_uchar = 0;
    let mut val = 0 as ::core::ffi::c_uint;
    if *str as ::core::ffi::c_int == '0' as i32 {
        str = str.offset(1);
        if *str as ::core::ffi::c_int == 'x' as i32 || *str as ::core::ffi::c_int == 'X' as i32 {
            str = str.offset(1);
            return StrToHex(str);
        }
        return StrToOct(str);
    }
    loop {
        c = *str;
        if !(c as ::core::ffi::c_int != '\0' as i32) {
            break;
        }
        if '0' as i32 <= c as ::core::ffi::c_int && c as ::core::ffi::c_int <= '9' as i32 {
            val = val
                .wrapping_mul(10 as ::core::ffi::c_uint)
                .wrapping_add(c as ::core::ffi::c_uint)
                .wrapping_sub('0' as i32 as ::core::ffi::c_uint);
        } else {
            return -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        }
        str = str.offset(1);
    }
    return val;
}
unsafe extern "C" fn StrToHex(mut str: *mut ::core::ffi::c_uchar) -> ::core::ffi::c_uint {
    let mut c: ::core::ffi::c_uchar = 0;
    let mut val = 0 as ::core::ffi::c_uint;
    loop {
        c = *str;
        if !(c as ::core::ffi::c_int != '\0' as i32) {
            break;
        }
        if '0' as i32 <= c as ::core::ffi::c_int && c as ::core::ffi::c_int <= '9' as i32 {
            val = val
                .wrapping_mul(16 as ::core::ffi::c_uint)
                .wrapping_add(c as ::core::ffi::c_uint)
                .wrapping_sub('0' as i32 as ::core::ffi::c_uint);
        } else if 'a' as i32 <= c as ::core::ffi::c_int && c as ::core::ffi::c_int <= 'f' as i32 {
            val = val
                .wrapping_mul(16 as ::core::ffi::c_uint)
                .wrapping_add(c as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                .wrapping_add(10 as ::core::ffi::c_uint);
        } else if 'A' as i32 <= c as ::core::ffi::c_int && c as ::core::ffi::c_int <= 'F' as i32 {
            val = val
                .wrapping_mul(16 as ::core::ffi::c_uint)
                .wrapping_add(c as ::core::ffi::c_uint)
                .wrapping_sub('A' as i32 as ::core::ffi::c_uint)
                .wrapping_add(10 as ::core::ffi::c_uint);
        } else {
            return -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        }
        str = str.offset(1);
    }
    return val;
}
unsafe extern "C" fn StrToOct(mut str: *mut ::core::ffi::c_uchar) -> ::core::ffi::c_uint {
    let mut c: ::core::ffi::c_uchar = 0;
    let mut val = 0 as ::core::ffi::c_uint;
    loop {
        c = *str;
        if !(c as ::core::ffi::c_int != '\0' as i32) {
            break;
        }
        if '0' as i32 <= c as ::core::ffi::c_int && c as ::core::ffi::c_int <= '7' as i32 {
            val = val
                .wrapping_mul(8 as ::core::ffi::c_uint)
                .wrapping_add(c as ::core::ffi::c_uint)
                .wrapping_sub('0' as i32 as ::core::ffi::c_uint);
        } else {
            return -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        }
        str = str.offset(1);
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn ScanAlphanumeric(mut linePP: *mut *mut ::core::ffi::c_uchar) {
    let mut chlen: ::core::ffi::c_int = 0;
    while !(*linePP).is_null()
        && {
            chlen = mblen(
                *linePP as *mut ::core::ffi::c_char,
                __ctype_get_mb_cur_max(),
            );
            chlen > 0 as ::core::ffi::c_int
        }
        && (chlen > 1 as ::core::ffi::c_int
            || *(*__ctype_b_loc()).offset(**linePP as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0)
    {
        *linePP = (*linePP).offset(chlen as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScanWhitespace(mut linePP: *mut *mut ::core::ffi::c_uchar) {
    while !(*linePP).is_null()
        && mblen(
            *linePP as *mut ::core::ffi::c_char,
            __ctype_get_mb_cur_max(),
        ) == 1 as ::core::ffi::c_int
        && *(*__ctype_b_loc()).offset(**linePP as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
    {
        *linePP = (*linePP).offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ToLower(mut string: *mut ::core::ffi::c_uchar) {
    let mut pch = string;
    let mut chlen: ::core::ffi::c_int = 0;
    while *pch as ::core::ffi::c_int != 0 && {
        chlen = mblen(pch as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
        chlen > 0 as ::core::ffi::c_int
    } {
        if chlen == 1 as ::core::ffi::c_int
            && *(*__ctype_b_loc()).offset(*pch as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
        {
            *pch = ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_uchar>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c = *pch as ::core::ffi::c_int;
                        __res = (if __c < -(128 as ::core::ffi::c_int)
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = tolower(*pch as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(*pch as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            }) as ::core::ffi::c_uchar;
        }
        pch = pch.offset(chlen as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PWarning(mut message: *mut ::core::ffi::c_char) {
    if !cfileP.is_null() {
        fprintf(
            stderr,
            b"%s: %s on line %d of configuration file %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            wmGD.mwmName,
            message,
            linec,
            if !wmGD.configFile.is_null() {
                wmGD.configFile as *mut ::core::ffi::c_char
            } else {
                &raw mut cfileName as *mut ::core::ffi::c_char
            },
        );
    } else {
        fprintf(
            stderr,
            b"%s: %s on line %d of specification string\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            wmGD.mwmName,
            message,
            linec,
        );
    }
    fflush(stderr);
}
unsafe extern "C" fn InitKeySubs(
    mut ppKeySubs: *mut *mut KeySub,
    mut pNumKeySubs: *mut ::core::ffi::c_int,
) {
    let mut numKS: ::core::ffi::c_int = 0;
    let mut pKS = ::core::ptr::null_mut::<KeySub>();
    let mut pKSret = ::core::ptr::null_mut::<KeySub>();
    let mut pch0 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut pch1 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_int = 0;
    let mut chlen: ::core::ffi::c_int = 0;
    pch0 = b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
    if pch0.is_null() || *pch0 as ::core::ffi::c_int == '\0' as i32 {
        *ppKeySubs = ::core::ptr::null_mut::<KeySub>();
        *pNumKeySubs = 0 as ::core::ffi::c_int;
        return;
    }
    pKSret = ::core::ptr::null_mut::<KeySub>();
    numKS = 0 as ::core::ffi::c_int;
    while *pch0 as ::core::ffi::c_int != '\0' as i32 {
        ScanWhitespace(&raw mut pch0);
        if *pch0 as ::core::ffi::c_int == '\0' as i32 {
            break;
        }
        if pKSret.is_null() {
            pKSret = XtMalloc(
                (1 as usize).wrapping_mul(::core::mem::size_of::<KeySub>() as usize) as Cardinal,
            ) as *mut KeySub;
        } else {
            pKSret = XtRealloc(
                pKSret as *mut ::core::ffi::c_char,
                ((numKS + 1 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<KeySub>() as usize)
                    as Cardinal,
            ) as *mut KeySub;
        }
        pKS = pKSret.offset(numKS as isize) as *mut KeySub;
        pch1 = pch0;
        while *pch1 as ::core::ffi::c_int != 0 && {
            chlen = mblen(pch1 as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
            chlen > 0 as ::core::ffi::c_int
        } {
            if chlen == 1 as ::core::ffi::c_int && *pch1 as ::core::ffi::c_int == ' ' as i32 {
                break;
            }
            pch1 = pch1.offset(chlen as isize);
        }
        (*pKS).lenFrom = pch1.offset_from(pch0) as ::core::ffi::c_long as ::core::ffi::c_int;
        if (*pKS).lenFrom < 1 as ::core::ffi::c_int {
            break;
        }
        (*pKS).pchFrom = XtMalloc((1 as ::core::ffi::c_int + (*pKS).lenFrom) as Cardinal)
            as *mut ::core::ffi::c_uchar;
        memcpy(
            (*pKS).pchFrom as *mut ::core::ffi::c_void,
            pch0 as *const ::core::ffi::c_void,
            (*pKS).lenFrom as size_t,
        );
        *(*pKS).pchFrom.offset((*pKS).lenFrom as isize) = '\0' as i32 as ::core::ffi::c_uchar;
        ScanWhitespace(&raw mut pch1);
        pch0 = pch1;
        while *pch1 as ::core::ffi::c_int != 0 && {
            chlen = mblen(pch1 as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
            chlen > 0 as ::core::ffi::c_int
        } {
            if chlen == 1 as ::core::ffi::c_int && *pch1 as ::core::ffi::c_int == ' ' as i32 {
                break;
            }
            pch1 = pch1.offset(chlen as isize);
        }
        len = pch1.offset_from(pch0) as ::core::ffi::c_long as ::core::ffi::c_int;
        if len < 1 as ::core::ffi::c_int {
            break;
        }
        (*pKS).pchTo =
            XtMalloc((1 as ::core::ffi::c_int + len) as Cardinal) as *mut ::core::ffi::c_uchar;
        memcpy(
            (*pKS).pchTo as *mut ::core::ffi::c_void,
            pch0 as *const ::core::ffi::c_void,
            len as size_t,
        );
        *(*pKS).pchTo.offset(len as isize) = '\0' as i32 as ::core::ffi::c_uchar;
        pch0 = pch1;
        numKS += 1;
    }
    *ppKeySubs = pKSret;
    *pNumKeySubs = numKS;
}
unsafe extern "C" fn ProcessAccelText(
    mut startP: *mut ::core::ffi::c_uchar,
    mut endP: *mut ::core::ffi::c_uchar,
    mut destP: *mut ::core::ffi::c_uchar,
) {
    let mut chlen: ::core::ffi::c_int = 0;
    static mut bAccelInit: Boolean = False as Boolean;
    static mut pKeySub: *mut KeySub = ::core::ptr::null::<KeySub>() as *mut KeySub;
    static mut numKeySubs: ::core::ffi::c_int = 0;
    let mut pchFirst = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut pchSub = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut lenSub: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if bAccelInit == 0 {
        InitKeySubs(&raw mut pKeySub, &raw mut numKeySubs);
        bAccelInit = True as Boolean;
    }
    ScanWhitespace(&raw mut startP);
    while *startP as ::core::ffi::c_int != '<' as i32 {
        if *startP as ::core::ffi::c_int == '~' as i32 {
            let fresh13 = startP;
            startP = startP.offset(1);
            let fresh14 = destP;
            destP = destP.offset(1);
            *fresh14 = *fresh13;
        }
        pchFirst = startP;
        while *startP as ::core::ffi::c_int != 0 && {
            chlen = mblen(startP as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max());
            chlen > 1 as ::core::ffi::c_int
                || *(*__ctype_b_loc()).offset(*startP as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                    != 0
        } {
            loop {
                let fresh15 = chlen;
                chlen = chlen - 1;
                if !(fresh15 != 0) {
                    break;
                }
                startP = startP.offset(1);
            }
        }
        pchSub = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        lenSub = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < numKeySubs {
            if (*pKeySub.offset(i as isize)).lenFrom as ::core::ffi::c_long
                == startP.offset_from(pchFirst) as ::core::ffi::c_long
                && strncasecmp(
                    (*pKeySub.offset(i as isize)).pchFrom as *mut ::core::ffi::c_char,
                    pchFirst as *mut ::core::ffi::c_char,
                    (*pKeySub.offset(i as isize)).lenFrom as size_t,
                ) == 0
            {
                pchSub = (*pKeySub.offset(i as isize)).pchTo;
                lenSub = strlen(pchSub as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
                break;
            } else {
                i += 1;
            }
        }
        if !pchSub.is_null() && lenSub > 0 as ::core::ffi::c_int {
            memcpy(
                destP as *mut ::core::ffi::c_void,
                pchSub as *const ::core::ffi::c_void,
                lenSub as size_t,
            );
            destP = destP.offset(lenSub as isize);
        } else {
            memcpy(
                destP as *mut ::core::ffi::c_void,
                pchFirst as *const ::core::ffi::c_void,
                startP.offset_from(pchFirst) as ::core::ffi::c_long as size_t,
            );
            destP = destP.offset(startP.offset_from(pchFirst) as ::core::ffi::c_long as isize);
        }
        let fresh16 = destP;
        destP = destP.offset(1);
        *fresh16 = '+' as i32 as ::core::ffi::c_uchar;
        ScanWhitespace(&raw mut startP);
    }
    startP = startP.offset(1);
    while *startP as ::core::ffi::c_int != '>' as i32 {
        startP = startP
            .offset(mblen(startP as *mut ::core::ffi::c_char, __ctype_get_mb_cur_max()) as isize);
    }
    startP = startP.offset(1);
    ScanWhitespace(&raw mut startP);
    while startP != endP {
        let fresh17 = startP;
        startP = startP.offset(1);
        let fresh18 = destP;
        destP = destP.offset(1);
        *fresh18 = *fresh17;
    }
    *destP = '\0' as i32 as ::core::ffi::c_uchar;
}
pub const SCREENS_OPT: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"-screens\0") };
pub const MULTI_SCREEN_OPT: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"-multiscreen\0") };
pub const VERSION_OPT: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"-version\0") };
#[no_mangle]
pub unsafe extern "C" fn ProcessCommandLine(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut argnum: ::core::ffi::c_int = 0;
    let mut lineP = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    argnum = 1 as ::core::ffi::c_int;
    while argnum < argc {
        lineP = *argv.offset(argnum as isize) as *mut ::core::ffi::c_uchar;
        string = GetString(&raw mut lineP);
        if !string.is_null() {
            if strcmp(
                string as *mut ::core::ffi::c_char,
                MULTI_SCREEN_OPT.as_ptr(),
            ) == 0
            {
                wmGD.multiScreen = True as Boolean;
                wmGD.numScreens = (*(wmGD.display as _XPrivDisplay)).nscreens;
            } else if strcmp(string as *mut ::core::ffi::c_char, SCREENS_OPT.as_ptr()) == 0 {
                argnum += 1;
                ParseScreensArgument(argc, argv, &raw mut argnum, lineP);
            } else if strcmp(string as *mut ::core::ffi::c_char, VERSION_OPT.as_ptr()) == 0 {
                PrintVersionInfo();
                exit(0 as ::core::ffi::c_int);
            }
        }
        argnum += 1;
    }
}
unsafe extern "C" fn ParseScreensArgument(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut pArgnum: *mut ::core::ffi::c_int,
    mut lineP: *mut ::core::ffi::c_uchar,
) {
    let mut string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut sNum = 0 as ::core::ffi::c_int;
    let mut lastLen: ::core::ffi::c_int = 0;
    let mut nameCount = 0 as ::core::ffi::c_int;
    let mut current_block_8: u64;
    while *pArgnum < argc && sNum < (*(wmGD.display as _XPrivDisplay)).nscreens {
        lineP = *argv.offset(*pArgnum as isize) as *mut ::core::ffi::c_uchar;
        if **argv.offset(*pArgnum as isize) as ::core::ffi::c_int == '"' as i32 {
            string = GetString(&raw mut lineP);
            if string.is_null() {
                current_block_8 = 8258075665625361029;
            } else {
                current_block_8 = 13513818773234778473;
            }
        } else {
            string = *argv.offset(*pArgnum as isize) as *mut ::core::ffi::c_uchar;
            if *string as ::core::ffi::c_int == '-' as i32 {
                break;
            }
            current_block_8 = 13513818773234778473;
        }
        match current_block_8 {
            13513818773234778473 => {
                let ref mut fresh19 = *wmGD.screenNames.offset(sNum as isize);
                *fresh19 = XtRealloc(
                    *wmGD.screenNames.offset(sNum as isize) as *mut ::core::ffi::c_char,
                    (1 as size_t).wrapping_add(strlen(string as *mut ::core::ffi::c_char))
                        as Cardinal,
                ) as *mut ::core::ffi::c_uchar;
                if (*fresh19).is_null() {
                    Warning(
                        b"Insufficient memory for screen names\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    ExitWM(WM_ERROR_EXIT_VALUE);
                } else {
                    strcpy(
                        *wmGD.screenNames.offset(sNum as isize) as *mut ::core::ffi::c_char,
                        string as *mut ::core::ffi::c_char,
                    );
                    nameCount += 1;
                }
            }
            _ => {}
        }
        *pArgnum += 1;
        sNum += 1;
    }
    *pArgnum -= 1;
    if nameCount > 0 as ::core::ffi::c_int {
        lastLen = (1 as size_t).wrapping_add(strlen(
            *wmGD.screenNames.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        )) as ::core::ffi::c_int;
        while sNum < (*(wmGD.display as _XPrivDisplay)).nscreens {
            let ref mut fresh20 = *wmGD.screenNames.offset(sNum as isize);
            *fresh20 = XtRealloc(
                *wmGD.screenNames.offset(sNum as isize) as *mut ::core::ffi::c_char,
                lastLen as Cardinal,
            ) as *mut ::core::ffi::c_uchar;
            if (*fresh20).is_null() {
                Warning(
                    b"Insufficient memory for screen names\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                ExitWM(WM_ERROR_EXIT_VALUE);
            } else {
                strcpy(
                    *wmGD.screenNames.offset(sNum as isize) as *mut ::core::ffi::c_char,
                    *wmGD.screenNames.offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                );
            }
            sNum += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessMotifBindings() {
    let mut fileName: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut bindings = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut homeDir = XmeGetHomeDirName() as *mut ::core::ffi::c_char;
    strcpy(&raw mut fileName as *mut ::core::ffi::c_char, homeDir);
    strncat(
        &raw mut fileName as *mut ::core::ffi::c_char,
        b"/\0" as *const u8 as *const ::core::ffi::c_char,
        (MAXWMPATH as size_t).wrapping_sub(strlen(&raw mut fileName as *mut ::core::ffi::c_char)),
    );
    strncat(
        &raw mut fileName as *mut ::core::ffi::c_char,
        MOTIF_BINDINGS_FILE.as_ptr(),
        (MAXWMPATH as size_t).wrapping_sub(strlen(&raw mut fileName as *mut ::core::ffi::c_char)),
    );
    XDeleteProperty(
        wmGD.display,
        (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset(0 as ::core::ffi::c_int as isize))
        .root,
        XInternAtom(
            wmGD.display,
            b"_MOTIF_BINDINGS\0" as *const u8 as *const ::core::ffi::c_char,
            False,
        ),
    );
    XDeleteProperty(
        wmGD.display,
        (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset(0 as ::core::ffi::c_int as isize))
        .root,
        XInternAtom(
            wmGD.display,
            b"_MOTIF_DEFAULT_BINDINGS\0" as *const u8 as *const ::core::ffi::c_char,
            False,
        ),
    );
    if _XmVirtKeysLoadFileBindings(
        &raw mut fileName as *mut ::core::ffi::c_char,
        &raw mut bindings,
    ) as ::core::ffi::c_int
        == True
    {
        XChangeProperty(
            wmGD.display,
            (*(*(wmGD.display as _XPrivDisplay))
                .screens
                .offset(0 as ::core::ffi::c_int as isize))
            .root,
            XInternAtom(
                wmGD.display,
                b"_MOTIF_BINDINGS\0" as *const u8 as *const ::core::ffi::c_char,
                False,
            ),
            XA_STRING,
            8 as ::core::ffi::c_int,
            PropModeReplace,
            bindings as *mut ::core::ffi::c_uchar,
            strlen(bindings) as ::core::ffi::c_int,
        );
    } else {
        _XmVirtKeysLoadFallbackBindings(wmGD.display, &raw mut bindings);
    }
    XtFree(bindings);
}
const RCDIR: &std::ffi::CStr =
    match std::ffi::CStr::from_bytes_with_nul(concat!(env!("EMWM_RCDIR"), "\0").as_bytes()) {
        Ok(dir) => dir,
        Err(_) => panic!("compiled resource directory must be a C string"),
    };
