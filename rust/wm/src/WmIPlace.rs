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
use crate::abi::WmWorkspaceData_WmBackdrop as WmWorkspaceData;
use crate::abi::*;
extern "C" {
    fn XDisplayHeight(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XDisplayWidth(_: *mut Display, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn XMoveWindow(
        _: *mut Display,
        _: Window,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XFindContext(_: *mut Display, _: XID, _: XContext, _: *mut XPointer) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn XtCalloc(_: Cardinal, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtRealloc(_: *mut ::core::ffi::c_char, _: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    static mut wmGD: WmGlobalData;
    fn Warning(message: *mut ::core::ffi::c_char);
    fn HideActiveIconText(pSD: *mut WmScreenData);
    fn ShowActiveIconText(pcd: *mut ClientData);
    fn ExtendIconList(pIBD: *mut IconBoxData, incr: ::core::ffi::c_int) -> Boolean;
    fn GetClipDimensions(pCD: *mut ClientData, fromRoot: Boolean);
    fn GetXineramaScreenFromLocation(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
    fn GetPrimaryXineramaScreen(xsi: *mut XineramaScreenInfo) -> ::core::ffi::c_int;
    fn GetXineramaScreenInfo(
        index: ::core::ffi::c_int,
        xsi: *mut XineramaScreenInfo,
    ) -> ::core::ffi::c_int;
    fn GetXineramaScreenCount(count: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn GetWsClientData(pWS: *mut WmWorkspaceData, pCD: *mut ClientData) -> *mut WsClientData;
    static mut clipWidth: Dimension;
    static mut clipHeight: Dimension;
    static mut clipX: Position;
    static mut clipY: Position;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MINIMIZED_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICON_PLACE_LEFT_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 0 as ::core::ffi::c_int;
pub const ICON_PLACE_RIGHT_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const ICON_PLACE_TOP_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const ICON_PLACE_BOTTOM_PRIMARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const ICON_PLACE_LEFT_SECONDARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const ICON_PLACE_RIGHT_SECONDARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const ICON_PLACE_TOP_SECONDARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const ICON_PLACE_BOTTOM_SECONDARY: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 7 as ::core::ffi::c_int;
pub const ICON_PLACE_TIGHT: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 9 as ::core::ffi::c_int;
pub const NO_ICON_PLACE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const MINIMUM_ICON_SPACING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn InitIconPlacement(mut pWS: *mut WmWorkspaceData) {
    let mut i: ::core::ffi::c_int = 0;
    let mut nxs = 0 as ::core::ffi::c_int;
    let mut xsi = XineramaScreenInfo {
        screen_number: 0,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    };
    let mut xinerama = False as Boolean;
    let mut scr_width = 0 as ::core::ffi::c_int;
    let mut scr_height = 0 as ::core::ffi::c_int;
    let mut ipd = ::core::ptr::null_mut::<IconPlacementData>();
    let mut realloc = False as Boolean;
    xinerama = GetXineramaScreenCount(&raw mut nxs) as Boolean;
    if xinerama == 0 {
        nxs = 1 as ::core::ffi::c_int;
        scr_width = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*pWS).pSD).screen as isize))
        .width;
        scr_height = (*(*(wmGD.display as _XPrivDisplay))
            .screens
            .offset((*(*pWS).pSD).screen as isize))
        .height;
    }
    ipd = XtCalloc(
        nxs as Cardinal,
        ::core::mem::size_of::<IconPlacementData>() as Cardinal,
    ) as *mut IconPlacementData;
    if ipd.is_null() {
        Warning(
            b"Insufficient memory for icon placement\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        wmGD.iconAutoPlace = False as Boolean;
        return;
    }
    if !(*pWS).IPData.is_null() {
        XtFree((*pWS).IPData as *mut ::core::ffi::c_char);
        realloc = True as Boolean;
    }
    (*pWS).IPData = ipd;
    i = 0 as ::core::ffi::c_int;
    while i < nxs {
        if xinerama as ::core::ffi::c_int != 0 && GetXineramaScreenInfo(i, &raw mut xsi) != 0 {
            scr_width = xsi.width as ::core::ffi::c_int;
            scr_height = xsi.height as ::core::ffi::c_int;
            (*(*pWS).IPData.offset(i as isize)).xiOrgX = xsi.x_org as ::core::ffi::c_int;
            (*(*pWS).IPData.offset(i as isize)).xiOrgY = xsi.y_org as ::core::ffi::c_int;
            (*(*pWS).IPData.offset(i as isize)).xiScreen = i;
        }
        if InitIconPlacementData(
            (*pWS).pSD,
            (*pWS).IPData.offset(i as isize),
            scr_width,
            scr_height,
        ) == 0
        {
            Warning(
                b"Insufficient memory for icon placement\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            wmGD.iconAutoPlace = False as Boolean;
            return;
        }
        i += 1;
    }
    if realloc != 0 {
        ResetClientIconPlacementData(pWS);
    }
}
unsafe extern "C" fn InitIconPlacementData(
    mut pSD: *mut WmScreenData,
    mut IPData: *mut IconPlacementData,
    mut sW: ::core::ffi::c_int,
    mut sH: ::core::ffi::c_int,
) -> Boolean {
    let mut tmp = *IPData;
    let mut useMargin: Boolean = 0;
    let mut iSpaceX: ::core::ffi::c_int = 0;
    let mut iSpaceY: ::core::ffi::c_int = 0;
    let mut placementW: ::core::ffi::c_int = 0;
    let mut placementH: ::core::ffi::c_int = 0;
    let mut extraXSpace: ::core::ffi::c_int = 0;
    let mut extraYSpace: ::core::ffi::c_int = 0;
    let mut xMargin: ::core::ffi::c_int = 0;
    let mut yMargin: ::core::ffi::c_int = 0;
    let mut extraPX: ::core::ffi::c_int = 0;
    let mut extraPY: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    extraPY = 0 as ::core::ffi::c_int;
    extraPX = extraPY;
    yMargin = extraPX;
    xMargin = yMargin;
    useMargin =
        ((*pSD).iconPlacementMargin >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as Boolean;
    tmp.iconPlacement = (*pSD).iconPlacement as ::core::ffi::c_uint;
    if useMargin != 0 {
        tmp.placementCols =
            (sW - 2 as ::core::ffi::c_int * (*pSD).iconPlacementMargin) / (*pSD).iconWidth;
        tmp.placementRows =
            (sH - 2 as ::core::ffi::c_int * (*pSD).iconPlacementMargin) / (*pSD).iconHeight;
    } else {
        tmp.placementCols = sW / (*pSD).iconWidth;
        tmp.placementRows = sH / (*pSD).iconHeight;
    }
    if tmp.iconPlacement as ::core::ffi::c_long & ICON_PLACE_TIGHT != 0 {
        iSpaceX = 0 as ::core::ffi::c_int;
        iSpaceY = 0 as ::core::ffi::c_int;
        xMargin = 2 as ::core::ffi::c_int;
        yMargin = 2 as ::core::ffi::c_int;
    } else {
        loop {
            if useMargin != 0 {
                iSpaceX = (sW
                    - 2 as ::core::ffi::c_int * (*pSD).iconPlacementMargin
                    - tmp.placementCols * (*pSD).iconWidth)
                    / (tmp.placementCols - 1 as ::core::ffi::c_int);
            } else {
                iSpaceX = (sW - tmp.placementCols * (*pSD).iconWidth) / tmp.placementCols;
            }
            if iSpaceX < MINIMUM_ICON_SPACING {
                tmp.placementCols -= 1;
            }
            if !(iSpaceX < MINIMUM_ICON_SPACING) {
                break;
            }
        }
        loop {
            if useMargin != 0 {
                iSpaceY = (sH
                    - 2 as ::core::ffi::c_int * (*pSD).iconPlacementMargin
                    - tmp.placementRows * (*pSD).iconHeight)
                    / (tmp.placementRows - 1 as ::core::ffi::c_int);
            } else {
                iSpaceY = (sH - tmp.placementRows * (*pSD).iconHeight) / tmp.placementRows;
            }
            if iSpaceY < MINIMUM_ICON_SPACING {
                tmp.placementRows -= 1;
            }
            if !(iSpaceY < MINIMUM_ICON_SPACING) {
                break;
            }
        }
    }
    tmp.iPlaceW = (*pSD).iconWidth + iSpaceX;
    tmp.iPlaceH = (*pSD).iconHeight + iSpaceY;
    placementW = tmp.placementCols * tmp.iPlaceW;
    placementH = tmp.placementRows * tmp.iPlaceH;
    tmp.placeIconX =
        (tmp.iPlaceW - (*pSD).iconWidth + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    tmp.placeIconY =
        (tmp.iPlaceH - (*pSD).iconHeight + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    if tmp.iconPlacement as ::core::ffi::c_long & ICON_PLACE_TIGHT != 0 {
        if useMargin != 0 {
            xMargin = (*pSD).iconPlacementMargin;
            yMargin = (*pSD).iconPlacementMargin;
        }
        extraXSpace = 0 as ::core::ffi::c_int;
        extraYSpace = 0 as ::core::ffi::c_int;
        if tmp.iconPlacement as ::core::ffi::c_long & ICON_PLACE_RIGHT_PRIMARY != 0
            || tmp.iconPlacement as ::core::ffi::c_long & ICON_PLACE_RIGHT_SECONDARY != 0
        {
            xMargin = sW - placementW - xMargin;
        }
        if tmp.iconPlacement as ::core::ffi::c_long & ICON_PLACE_BOTTOM_PRIMARY != 0
            || tmp.iconPlacement as ::core::ffi::c_long & ICON_PLACE_BOTTOM_SECONDARY != 0
        {
            yMargin = sH - placementH - yMargin;
        }
    } else if useMargin != 0 {
        xMargin = (*pSD).iconPlacementMargin - tmp.placeIconX;
        extraXSpace =
            sW - 2 as ::core::ffi::c_int * (*pSD).iconPlacementMargin - (placementW - iSpaceX);
        extraPX = if tmp.iconPlacement as ::core::ffi::c_long & ICON_PLACE_RIGHT_PRIMARY != 0 {
            1 as ::core::ffi::c_int
        } else {
            tmp.placementCols - extraXSpace
        };
        yMargin = (*pSD).iconPlacementMargin - tmp.placeIconY;
        extraYSpace =
            sH - 2 as ::core::ffi::c_int * (*pSD).iconPlacementMargin - (placementH - iSpaceY);
        extraPY = if tmp.iconPlacement as ::core::ffi::c_long & ICON_PLACE_BOTTOM_PRIMARY != 0 {
            1 as ::core::ffi::c_int
        } else {
            tmp.placementRows - extraYSpace
        };
    } else {
        xMargin = (sW - placementW + (tmp.iPlaceW - (*pSD).iconWidth & 1 as ::core::ffi::c_int))
            / 2 as ::core::ffi::c_int;
        extraXSpace = 0 as ::core::ffi::c_int;
        yMargin = (sH - placementH + (tmp.iPlaceH - (*pSD).iconHeight & 1 as ::core::ffi::c_int))
            / 2 as ::core::ffi::c_int;
        extraYSpace = 0 as ::core::ffi::c_int;
        if tmp.iconPlacement as ::core::ffi::c_long & ICON_PLACE_RIGHT_PRIMARY != 0 {
            xMargin = sW - placementW - xMargin;
            tmp.placeIconX = tmp.iPlaceW - (*pSD).iconWidth - tmp.placeIconX;
        }
        if tmp.iconPlacement as ::core::ffi::c_long & ICON_PLACE_BOTTOM_PRIMARY != 0 {
            yMargin = sH - placementH - yMargin;
            tmp.placeIconY = tmp.iPlaceH - (*pSD).iconHeight - tmp.placeIconY;
        }
    }
    if (*IPData).placementRows < tmp.placementRows + 2 as ::core::ffi::c_int {
        tmp.placementRowY = XtRealloc(
            (*IPData).placementRowY as *mut ::core::ffi::c_char,
            ((tmp.placementRows + 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                as Cardinal,
        ) as *mut ::core::ffi::c_int;
        if tmp.placementRowY.is_null() {
            return False as Boolean;
        }
    } else {
        tmp.placementRowY = (*IPData).placementRowY;
    }
    if (*IPData).placementCols < tmp.placementCols + 2 as ::core::ffi::c_int {
        tmp.placementColX = XtRealloc(
            (*IPData).placementColX as *mut ::core::ffi::c_char,
            ((tmp.placementCols + 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                as Cardinal,
        ) as *mut ::core::ffi::c_int;
        if tmp.placementColX.is_null() {
            return False as Boolean;
        }
    } else {
        tmp.placementColX = (*IPData).placementColX;
    }
    *tmp.placementRowY.offset(0 as ::core::ffi::c_int as isize) = yMargin;
    i = 1 as ::core::ffi::c_int;
    while i <= tmp.placementRows {
        *tmp.placementRowY.offset(i as isize) = *tmp
            .placementRowY
            .offset((i - 1 as ::core::ffi::c_int) as isize)
            + tmp.iPlaceH;
        if extraYSpace > 0 as ::core::ffi::c_int && i >= extraPY {
            let ref mut fresh1 = *tmp.placementRowY.offset(i as isize);
            *fresh1 += 1;
            extraYSpace -= 1;
        }
        i += 1;
    }
    *tmp.placementColX.offset(0 as ::core::ffi::c_int as isize) = xMargin;
    i = 1 as ::core::ffi::c_int;
    while i <= tmp.placementCols {
        *tmp.placementColX.offset(i as isize) = *tmp
            .placementColX
            .offset((i - 1 as ::core::ffi::c_int) as isize)
            + tmp.iPlaceW;
        if extraXSpace > 0 as ::core::ffi::c_int && i >= extraPX {
            let ref mut fresh2 = *tmp.placementColX.offset(i as isize);
            *fresh2 += 1;
            extraXSpace -= 1;
        }
        i += 1;
    }
    tmp.totalPlaces = tmp.placementRows * tmp.placementCols;
    if (*IPData).totalPlaces < tmp.totalPlaces {
        tmp.placeList = XtRealloc(
            (*IPData).placeList as *mut ::core::ffi::c_char,
            (tmp.totalPlaces as usize).wrapping_mul(::core::mem::size_of::<IconInfo>() as usize)
                as Cardinal,
        ) as *mut IconInfo;
        if tmp.placeList.is_null() {
            return False as Boolean;
        }
    } else {
        tmp.placeList = (*IPData).placeList;
    }
    memset(
        tmp.placeList as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (tmp.totalPlaces as size_t).wrapping_mul(::core::mem::size_of::<IconInfo>() as size_t),
    );
    tmp.onRootWindow = True as Boolean;
    *IPData = tmp;
    return True as Boolean;
}
#[no_mangle]
pub unsafe extern "C" fn GetNextIconPlace(mut pIPD: *mut IconPlacementData) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pIPD).totalPlaces {
        if (*(*pIPD).placeList.offset(i as isize)).pCD.is_null() {
            return i;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CvtIconPlaceToPosition(
    mut pIPD: *mut IconPlacementData,
    mut place: ::core::ffi::c_int,
    mut pX: *mut ::core::ffi::c_int,
    mut pY: *mut ::core::ffi::c_int,
) {
    let mut row: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    if (*pIPD).iconPlacement as ::core::ffi::c_long
        & (ICON_PLACE_LEFT_PRIMARY | ICON_PLACE_RIGHT_PRIMARY)
        != 0
    {
        col = place % (*pIPD).placementCols;
        row = place / (*pIPD).placementCols;
    } else {
        col = place / (*pIPD).placementRows;
        row = place % (*pIPD).placementRows;
    }
    if (*pIPD).iconPlacement as ::core::ffi::c_long
        & (ICON_PLACE_RIGHT_PRIMARY | ICON_PLACE_RIGHT_SECONDARY)
        != 0
    {
        col = (*pIPD).placementCols - col - 1 as ::core::ffi::c_int;
    }
    if (*pIPD).iconPlacement as ::core::ffi::c_long
        & (ICON_PLACE_BOTTOM_PRIMARY | ICON_PLACE_BOTTOM_SECONDARY)
        != 0
    {
        row = (*pIPD).placementRows - row - 1 as ::core::ffi::c_int;
    }
    if (*pIPD).onRootWindow != 0 {
        *pX = *(*pIPD).placementColX.offset(col as isize) + (*pIPD).placeIconX + (*pIPD).xiOrgX;
        *pY = *(*pIPD).placementRowY.offset(row as isize) + (*pIPD).placeIconY + (*pIPD).xiOrgY;
    } else {
        *pX = col * (*pIPD).iPlaceW;
        *pY = row * (*pIPD).iPlaceH;
    };
}
#[no_mangle]
pub unsafe extern "C" fn FindIconPlace(
    mut pCD: *mut ClientData,
    mut pIPD: *mut IconPlacementData,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut place: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut diffX: ::core::ffi::c_int = 0;
    let mut diffY: ::core::ffi::c_int = 0;
    let mut altX: ::core::ffi::c_int = 0;
    let mut altY: ::core::ffi::c_int = 0;
    let mut amt: ::core::ffi::c_int = 0;
    place = CvtIconPositionToPlace(pIPD, x, y);
    if place < (*pIPD).totalPlaces {
        if (*(*pIPD).placeList.offset(place as isize)).pCD.is_null() {
            return place;
        }
    } else {
        if (*pIPD).iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_PRIMARY != 0 {
            amt = (*pIPD).placementCols;
        } else {
            amt = (*pIPD).placementRows;
        }
        if ExtendIconList(
            (*(*pCD).pWsList.offset((*pCD).currentWsc as isize)).pIconBox,
            amt,
        ) == 0
        {
            Warning(
                b"Insufficient memory to create icon box data\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    x -= (*pIPD).xiOrgX;
    y -= (*pIPD).xiOrgY;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        match i {
            0 => {
                diffX = (*(*pCD).pSD).iconWidth / 2 as ::core::ffi::c_int;
                diffY = (*(*pCD).pSD).iconHeight / 2 as ::core::ffi::c_int;
            }
            1 => {
                diffX = (*pIPD).iPlaceW / 2 as ::core::ffi::c_int;
                diffY = (*pIPD).iPlaceH / 2 as ::core::ffi::c_int;
            }
            _ => {}
        }
        j = 0 as ::core::ffi::c_int;
        while j < 4 as ::core::ffi::c_int {
            match j {
                0 => {
                    if (*pIPD).iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_PRIMARY != 0 {
                        altX = x - diffX;
                        altY = y;
                    } else if (*pIPD).iconPlacement as ::core::ffi::c_long
                        & ICON_PLACE_RIGHT_PRIMARY
                        != 0
                    {
                        altX = x + diffX;
                        altY = y;
                    } else if (*pIPD).iconPlacement as ::core::ffi::c_long & ICON_PLACE_TOP_PRIMARY
                        != 0
                    {
                        altX = x;
                        altY = y - diffY;
                    } else {
                        altX = x;
                        altY = y + diffY;
                    }
                }
                1 => {
                    if (*pIPD).iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_PRIMARY != 0 {
                        altX = x + diffX;
                        altY = y;
                    } else if (*pIPD).iconPlacement as ::core::ffi::c_long
                        & ICON_PLACE_RIGHT_PRIMARY
                        != 0
                    {
                        altX = x - diffX;
                        altY = y;
                    } else if (*pIPD).iconPlacement as ::core::ffi::c_long & ICON_PLACE_TOP_PRIMARY
                        != 0
                    {
                        altX = x;
                        altY = y + diffY;
                    } else {
                        altX = x;
                        altY = y - diffY;
                    }
                }
                2 => {
                    if (*pIPD).iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_SECONDARY != 0
                    {
                        altX = x - diffX;
                        altY = y;
                    } else if (*pIPD).iconPlacement as ::core::ffi::c_long
                        & ICON_PLACE_RIGHT_SECONDARY
                        != 0
                    {
                        altX = x + diffX;
                        altY = y;
                    } else if (*pIPD).iconPlacement as ::core::ffi::c_long
                        & ICON_PLACE_TOP_SECONDARY
                        != 0
                    {
                        altX = x;
                        altY = y + diffY;
                    } else {
                        altX = x;
                        altY = y - diffY;
                    }
                }
                3 => {
                    if (*pIPD).iconPlacement as ::core::ffi::c_long & ICON_PLACE_LEFT_SECONDARY != 0
                    {
                        altX = x + diffX;
                        altY = y;
                    } else if (*pIPD).iconPlacement as ::core::ffi::c_long
                        & ICON_PLACE_RIGHT_SECONDARY
                        != 0
                    {
                        altX = x - diffX;
                        altY = y;
                    } else if (*pIPD).iconPlacement as ::core::ffi::c_long
                        & ICON_PLACE_TOP_SECONDARY
                        != 0
                    {
                        altX = x;
                        altY = y - diffY;
                    } else {
                        altX = x;
                        altY = y + diffY;
                    }
                }
                _ => {}
            }
            if !(*(*pCD).pWsList.offset((*pCD).currentWsc as isize))
                .pIconBox
                .is_null()
            {
                GetClipDimensions(pCD, False as Boolean);
                if altX < clipX as ::core::ffi::c_int {
                    return -(1 as ::core::ffi::c_int);
                }
                if altY < clipY as ::core::ffi::c_int {
                    return -(1 as ::core::ffi::c_int);
                }
                if altX
                    > clipX as ::core::ffi::c_int + clipWidth as ::core::ffi::c_int
                        - (*(*pCD).pSD).iconWidth
                {
                    return -(1 as ::core::ffi::c_int);
                }
                if altY
                    > clipY as ::core::ffi::c_int + clipHeight as ::core::ffi::c_int
                        - (*(*pCD).pSD).iconHeight
                {
                    return -(1 as ::core::ffi::c_int);
                }
            }
            place = CvtIconPositionToPlace(pIPD, altX + (*pIPD).xiOrgX, altY + (*pIPD).xiOrgY);
            if (*(*pIPD).placeList.offset(place as isize)).pCD.is_null() {
                return place;
            }
            j += 1;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CvtIconPositionToPlace(
    mut pIPD: *mut IconPlacementData,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut row: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    if (*pIPD).onRootWindow != 0 {
        x -= (*pIPD).xiOrgX;
        y -= (*pIPD).xiOrgY;
        row = 1 as ::core::ffi::c_int;
        while row < (*pIPD).placementRows {
            if y < *(*pIPD).placementRowY.offset(row as isize) {
                break;
            }
            row += 1;
        }
        row -= 1;
        col = 1 as ::core::ffi::c_int;
        while col < (*pIPD).placementCols {
            if x < *(*pIPD).placementColX.offset(col as isize) {
                break;
            }
            col += 1;
        }
        col -= 1;
        if (*pIPD).iconPlacement as ::core::ffi::c_long
            & (ICON_PLACE_RIGHT_PRIMARY | ICON_PLACE_RIGHT_SECONDARY)
            != 0
        {
            col = (*pIPD).placementCols - col - 1 as ::core::ffi::c_int;
        }
        if (*pIPD).iconPlacement as ::core::ffi::c_long
            & (ICON_PLACE_BOTTOM_PRIMARY | ICON_PLACE_BOTTOM_SECONDARY)
            != 0
        {
            row = (*pIPD).placementRows - row - 1 as ::core::ffi::c_int;
        }
    } else {
        col = x / (*pIPD).iPlaceW;
        row = y / (*pIPD).iPlaceH;
    }
    if (*pIPD).iconPlacement as ::core::ffi::c_long
        & (ICON_PLACE_LEFT_PRIMARY | ICON_PLACE_RIGHT_PRIMARY)
        != 0
    {
        return row * (*pIPD).placementCols + col;
    } else {
        return col * (*pIPD).placementRows + row;
    };
}
#[no_mangle]
pub unsafe extern "C" fn PositionToPlacementData(
    mut pWS: *mut WmWorkspaceData,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> *mut IconPlacementData {
    let mut xsi = XineramaScreenInfo {
        screen_number: 0,
        x_org: 0,
        y_org: 0,
        width: 0,
        height: 0,
    };
    if wmGD.xineramaIconifyToPrimary as ::core::ffi::c_int != 0
        && GetPrimaryXineramaScreen(&raw mut xsi) != 0
    {
        return (*pWS).IPData.offset(xsi.screen_number as isize);
    } else if GetXineramaScreenFromLocation(x, y, &raw mut xsi) != 0 {
        return (*pWS).IPData.offset(xsi.screen_number as isize);
    }
    return (*pWS).IPData;
}
#[no_mangle]
pub unsafe extern "C" fn PackRootIcons() {
    let mut iOld: ::core::ffi::c_int = 0;
    let mut iNew: ::core::ffi::c_int = 0;
    let mut pCD = ::core::ptr::null_mut::<ClientData>();
    let mut pCD_active = ::core::ptr::null_mut::<ClientData>();
    let mut hasActiveText = 1 as ::core::ffi::c_int;
    let mut nxsi: ::core::ffi::c_int = 0;
    let mut xsi: ::core::ffi::c_int = 0;
    let mut pWsc = ::core::ptr::null_mut::<WsClientData>();
    if XFindContext(
        wmGD.display,
        (*wmGD.pActiveSD).activeIconTextWin as XID,
        wmGD.windowContextType,
        &raw mut pCD_active as *mut XPointer,
    ) != 0
    {
        hasActiveText = 0 as ::core::ffi::c_int;
    }
    if wmGD.iconAutoPlace != 0 {
        if GetXineramaScreenCount(&raw mut nxsi) == 0 {
            nxsi = 1 as ::core::ffi::c_int;
        }
        xsi = 0 as ::core::ffi::c_int;
        while xsi < nxsi {
            let mut IPData = (*(*wmGD.pActiveSD).pActiveWS).IPData.offset(xsi as isize);
            iNew = 0 as ::core::ffi::c_int;
            iOld = iNew;
            while iOld < (*IPData).totalPlaces {
                if (*(*IPData).placeList.offset(iOld as isize)).pCD.is_null() {
                    loop {
                        iOld += 1;
                        if !(iOld < (*IPData).totalPlaces
                            && (*(*IPData).placeList.offset(iOld as isize)).pCD.is_null())
                        {
                            break;
                        }
                    }
                }
                if iOld < (*IPData).totalPlaces && iOld != iNew {
                    MoveIconInfo(IPData, iOld, IPData, iNew);
                    pCD = (*(*IPData).placeList.offset(iNew as isize)).pCD as *mut ClientData;
                    pWsc =
                        GetWsClientData((*wmGD.pActiveSD).pActiveWS as *mut WmWorkspaceData, pCD);
                    (*pWsc).iconPlace = iNew;
                    CvtIconPlaceToPosition(
                        IPData,
                        (*pWsc).iconPlace,
                        &raw mut (*pWsc).iconX,
                        &raw mut (*pWsc).iconY,
                    );
                    if hasActiveText != 0 && pCD == pCD_active {
                        HideActiveIconText(NULL as *mut WmScreenData);
                        XMoveWindow(
                            wmGD.display,
                            (*pWsc).iconFrameWin,
                            (*pWsc).iconX,
                            (*pWsc).iconY,
                        );
                        ShowActiveIconText(pCD);
                    } else {
                        XMoveWindow(
                            wmGD.display,
                            (*pWsc).iconFrameWin,
                            (*pWsc).iconX,
                            (*pWsc).iconY,
                        );
                    }
                }
                iOld += 1;
                iNew += 1;
            }
            xsi += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn MoveIconInfo(
    mut pIPD1: *mut IconPlacementData,
    mut p1: ::core::ffi::c_int,
    mut pIPD2: *mut IconPlacementData,
    mut p2: ::core::ffi::c_int,
) {
    let mut pWsc = ::core::ptr::null_mut::<WsClientData>();
    if (*(*pIPD2).placeList.offset(p2 as isize)).pCD.is_null() {
        let ref mut fresh3 = (*(*pIPD2).placeList.offset(p2 as isize)).pCD;
        *fresh3 = (*(*pIPD1).placeList.offset(p1 as isize)).pCD;
        let ref mut fresh4 = (*(*pIPD2).placeList.offset(p2 as isize)).theWidget;
        *fresh4 = (*(*pIPD1).placeList.offset(p1 as isize)).theWidget;
        pWsc = GetWsClientData(
            (*(*(*(*pIPD2).placeList.offset(p2 as isize)).pCD).pSD).pActiveWS
                as *mut WmWorkspaceData,
            (*(*pIPD2).placeList.offset(p2 as isize)).pCD as *mut ClientData,
        );
        (*pWsc).iconPlace = p2;
        let ref mut fresh5 = (*(*pIPD1).placeList.offset(p1 as isize)).pCD;
        *fresh5 = ::core::ptr::null_mut::<_ClientData>();
        let ref mut fresh6 = (*(*pIPD1).placeList.offset(p1 as isize)).theWidget;
        *fresh6 = ::core::ptr::null_mut::<_WidgetRec>();
    }
}
unsafe extern "C" fn ResetClientIconPlacementData(mut pWS: *mut WmWorkspaceData) {
    let mut e = (*(*pWS).pSD).clientList;
    while !e.is_null() {
        let mut xsi = XineramaScreenInfo {
            screen_number: 0,
            x_org: 0,
            y_org: 0,
            width: 0,
            height: 0,
        };
        let mut s_width: ::core::ffi::c_int = 0;
        let mut s_height: ::core::ffi::c_int = 0;
        let mut s_xorg = 0 as ::core::ffi::c_int;
        let mut s_yorg = 0 as ::core::ffi::c_int;
        let mut cd = (*e).pCD as *mut ClientData;
        let mut wscd = ::core::ptr::null_mut::<WsClientData>();
        let mut iws: ::core::ffi::c_int = 0;
        if (*e).type_0 == MINIMIZED_STATE {
            e = (*e).nextSibling as *mut ClientListEntry;
        } else {
            iws = 0 as ::core::ffi::c_int;
            while iws < (*cd).sizeWsList {
                wscd =
                    (*cd).pWsList.offset(iws as isize) as *mut _WsClientData as *mut WsClientData;
                if (*wscd).iconPlace != NO_ICON_PLACE
                    && (*(*cd).pWsList.offset((*cd).currentWsc as isize))
                        .pIconBox
                        .is_null()
                {
                    if GetXineramaScreenFromLocation((*wscd).iconX, (*wscd).iconY, &raw mut xsi)
                        != 0
                        || GetXineramaScreenFromLocation((*cd).clientX, (*cd).clientY, &raw mut xsi)
                            != 0
                        || GetPrimaryXineramaScreen(&raw mut xsi) != 0
                    {
                        (*wscd).IPData = (*pWS).IPData.offset(xsi.screen_number as isize);
                        s_width = xsi.width as ::core::ffi::c_int;
                        s_height = xsi.height as ::core::ffi::c_int;
                        s_xorg = xsi.x_org as ::core::ffi::c_int;
                        s_yorg = xsi.y_org as ::core::ffi::c_int;
                    } else {
                        s_width = XDisplayWidth(wmGD.display, (*(*cd).pSD).screen);
                        s_height = XDisplayHeight(wmGD.display, (*(*cd).pSD).screen);
                        (*wscd).IPData = (*pWS).IPData;
                    }
                    if (*wscd).iconX - s_xorg >= s_width || (*wscd).iconY - s_yorg >= s_height {
                        (*wscd).iconX = 0 as ::core::ffi::c_int;
                        (*wscd).iconY = 0 as ::core::ffi::c_int;
                        (*wscd).iconPlace = NO_ICON_PLACE;
                    } else {
                        (*wscd).iconPlace =
                            FindIconPlace(cd, (*wscd).IPData, (*wscd).iconX, (*wscd).iconY);
                    }
                    if (*wscd).iconPlace != NO_ICON_PLACE || {
                        (*wscd).iconPlace = GetNextIconPlace((*wscd).IPData);
                        (*wscd).iconPlace != NO_ICON_PLACE
                    } {
                        CvtIconPlaceToPosition(
                            (*wscd).IPData,
                            (*wscd).iconPlace,
                            &raw mut (*wscd).iconX,
                            &raw mut (*wscd).iconY,
                        );
                        let ref mut fresh0 = (*(*(*wscd).IPData)
                            .placeList
                            .offset((*wscd).iconPlace as isize))
                        .pCD;
                        *fresh0 = cd as *mut _ClientData;
                        XMoveWindow(
                            wmGD.display,
                            (*(*cd).pWsList.offset((*cd).currentWsc as isize)).iconFrameWin,
                            (*(*cd).pWsList.offset((*cd).currentWsc as isize)).iconX,
                            (*(*cd).pWsList.offset((*cd).currentWsc as isize)).iconY,
                        );
                    } else {
                        (*wscd).IPData = ::core::ptr::null_mut::<IconPlacementData>();
                    }
                } else {
                    (*wscd).IPData = ::core::ptr::null_mut::<IconPlacementData>();
                }
                iws += 1;
            }
            e = (*e).nextSibling as *mut ClientListEntry;
        }
    }
}
