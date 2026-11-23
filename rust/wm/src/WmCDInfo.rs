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
extern "C" {
    static mut wmGD: WmGlobalData;
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MWM_FUNC_CLOSE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const MWM_DECOR_BORDER: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int;
pub const MWM_DECOR_RESIZEH: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 2 as ::core::ffi::c_int;
pub const MWM_DECOR_TITLE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 3 as ::core::ffi::c_int;
pub const MWM_DECOR_MENU: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 4 as ::core::ffi::c_int;
pub const MWM_DECOR_MINIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 5 as ::core::ffi::c_int;
pub const MWM_DECOR_MAXIMIZE: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 6 as ::core::ffi::c_int;
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STRETCH_COUNT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const WINDOW_PART_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FRAME_NONE: ::core::ffi::c_int = WINDOW_PART_NONE;
pub const FRAME_CLIENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FRAME_SYSTEM: ::core::ffi::c_int = 2;
pub const FRAME_TITLE: ::core::ffi::c_int = 3;
pub const FRAME_MINIMIZE: ::core::ffi::c_int = 4;
pub const FRAME_MAXIMIZE: ::core::ffi::c_int = 5;
pub const FRAME_RESIZE_NW: ::core::ffi::c_int = 6;
pub const FRAME_RESIZE_N: ::core::ffi::c_int = 7;
pub const FRAME_RESIZE_NE: ::core::ffi::c_int = 8;
pub const FRAME_RESIZE_E: ::core::ffi::c_int = 9;
pub const FRAME_RESIZE_SE: ::core::ffi::c_int = 10;
pub const FRAME_RESIZE_S: ::core::ffi::c_int = 11;
pub const FRAME_RESIZE_SW: ::core::ffi::c_int = 12;
pub const FRAME_RESIZE_W: ::core::ffi::c_int = 13;
pub const FRAME_NBORDER: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const FRAME_MATTE: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const FRAME_TITLEBAR: ::core::ffi::c_int = 17;
pub const FRAME_CLOSE: ::core::ffi::c_int = 21;
#[no_mangle]
pub unsafe extern "C" fn TitleTextHeight(mut pcd: *mut ClientData) -> ::core::ffi::c_uint {
    let mut ttextheight: ::core::ffi::c_uint = 0;
    if (*(*pcd).pSD).decoupleTitleAppearance != 0 {
        ttextheight = (*(*pcd).pSD).clientTitleAppearance.titleHeight;
    } else {
        ttextheight = (*(*pcd).pSD).clientAppearance.titleHeight;
    }
    return ttextheight;
}
#[no_mangle]
pub unsafe extern "C" fn InitTitleBarHeight(mut pcd: *mut ClientData) -> ::core::ffi::c_uint {
    let mut tbarheight: ::core::ffi::c_uint = 0;
    if (*pcd).decor & MWM_DECOR_TITLE != 0 {
        tbarheight = TitleTextHeight(pcd);
    } else {
        tbarheight = 0 as ::core::ffi::c_uint;
    }
    return tbarheight;
}
#[no_mangle]
pub unsafe extern "C" fn UpperBorderWidth(mut pcd: *mut ClientData) -> ::core::ffi::c_uint {
    let mut uswidth: ::core::ffi::c_uint = 0;
    let mut decoration = (*pcd).decor as ::core::ffi::c_uint;
    if decoration as ::core::ffi::c_long & MWM_DECOR_RESIZEH != 0 {
        uswidth = (*(*pcd).pSD).resizeBorderWidth as ::core::ffi::c_uint;
    } else if decoration as ::core::ffi::c_long & MWM_DECOR_BORDER != 0 {
        uswidth = (*(*pcd).pSD).frameBorderWidth as ::core::ffi::c_uint;
    } else if decoration as ::core::ffi::c_long & MWM_DECOR_TITLE != 0
        || (*pcd).matteWidth > 0 as ::core::ffi::c_int
    {
        uswidth = 0 as ::core::ffi::c_uint;
    } else {
        uswidth = (*pcd).xBorderWidth as ::core::ffi::c_uint;
    }
    return uswidth;
}
#[no_mangle]
pub unsafe extern "C" fn LowerBorderWidth(mut pcd: *mut ClientData) -> ::core::ffi::c_uint {
    let mut lswidth: ::core::ffi::c_uint = 0;
    let mut decoration = (*pcd).decor as ::core::ffi::c_uint;
    if decoration as ::core::ffi::c_long & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) != 0 {
        if (*pcd).matteWidth == 0 as ::core::ffi::c_int
            && decoration as ::core::ffi::c_long & MWM_DECOR_TITLE != 0
            && wmGD.frameStyle as ::core::ffi::c_uint
                == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            lswidth = UpperBorderWidth(pcd).wrapping_add(
                ((*pcd).internalBevel - (*(*pcd).pSD).joinBevel) as ::core::ffi::c_uint,
            );
        } else {
            lswidth = UpperBorderWidth(pcd);
        }
    } else if (*pcd).matteWidth > 0 as ::core::ffi::c_int {
        lswidth = 0 as ::core::ffi::c_uint;
    } else {
        lswidth = (*pcd).xBorderWidth as ::core::ffi::c_uint;
    }
    return lswidth;
}
#[no_mangle]
pub unsafe extern "C" fn FrameWidth(mut pcd: *mut ClientData) -> ::core::ffi::c_uint {
    let mut frameWidth: ::core::ffi::c_uint = 0;
    frameWidth = (((if (*pcd).maxConfig as ::core::ffi::c_int != 0 {
        (*pcd).maxWidth
    } else {
        (*pcd).clientWidth
    }) + 2 as ::core::ffi::c_int * (*pcd).matteWidth) as ::core::ffi::c_uint)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(LowerBorderWidth(pcd)));
    return frameWidth;
}
#[no_mangle]
pub unsafe extern "C" fn CornerWidth(mut pcd: *mut ClientData) -> ::core::ffi::c_uint {
    let mut cwidth: ::core::ffi::c_uint = 0;
    let mut frameWidth = FrameWidth(pcd);
    if (*pcd).decor & MWM_DECOR_RESIZEH != 0 {
        cwidth = TitleTextHeight(pcd).wrapping_add(UpperBorderWidth(pcd));
        if (3 as ::core::ffi::c_uint).wrapping_mul(cwidth) > frameWidth {
            cwidth = frameWidth.wrapping_div(3 as ::core::ffi::c_uint);
        }
    } else {
        cwidth = 0 as ::core::ffi::c_uint;
    }
    return cwidth;
}
#[no_mangle]
pub unsafe extern "C" fn FrameHeight(mut pcd: *mut ClientData) -> ::core::ffi::c_uint {
    let mut frameHeight: ::core::ffi::c_uint = 0;
    if (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) != 0 {
        frameHeight = (((if (*pcd).maxConfig as ::core::ffi::c_int != 0 {
            (*pcd).maxHeight
        } else {
            (*pcd).clientHeight
        }) + 2 as ::core::ffi::c_int * (*pcd).matteWidth)
            as ::core::ffi::c_uint)
            .wrapping_add(LowerBorderWidth(pcd))
            .wrapping_add(UpperBorderWidth(pcd))
            .wrapping_add((*pcd).frameInfo.titleBarHeight);
    } else {
        frameHeight = (((if (*pcd).maxConfig as ::core::ffi::c_int != 0 {
            (*pcd).maxHeight
        } else {
            (*pcd).clientHeight
        }) + 2 as ::core::ffi::c_int * (*pcd).matteWidth)
            as ::core::ffi::c_uint)
            .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(LowerBorderWidth(pcd)))
            .wrapping_add((*pcd).frameInfo.titleBarHeight);
    }
    return frameHeight;
}
#[no_mangle]
pub unsafe extern "C" fn CornerHeight(mut pcd: *mut ClientData) -> ::core::ffi::c_uint {
    let mut cheight: ::core::ffi::c_uint = 0;
    let mut frameHeight = FrameHeight(pcd);
    if (*pcd).decor & MWM_DECOR_RESIZEH != 0 {
        cheight = TitleTextHeight(pcd).wrapping_add(UpperBorderWidth(pcd));
        if (3 as ::core::ffi::c_uint).wrapping_mul(cheight) > frameHeight {
            cheight = frameHeight.wrapping_div(3 as ::core::ffi::c_uint);
        }
    } else {
        cheight = 0 as ::core::ffi::c_uint;
    }
    return cheight;
}
#[no_mangle]
pub unsafe extern "C" fn BaseWindowX(mut pcd: *mut ClientData) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0;
    if (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) != 0 {
        rval = LowerBorderWidth(pcd) as ::core::ffi::c_int;
    } else {
        rval = 0 as ::core::ffi::c_int;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn BaseWindowY(mut pcd: *mut ClientData) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0;
    if (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) != 0 {
        rval = UpperBorderWidth(pcd).wrapping_add((*pcd).frameInfo.titleBarHeight)
            as ::core::ffi::c_int;
    } else {
        rval = (*pcd).frameInfo.titleBarHeight as ::core::ffi::c_int;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn FrameX(mut pcd: *mut ClientData) -> ::core::ffi::c_int {
    let mut frameX: ::core::ffi::c_int = 0;
    frameX = ((if (*pcd).maxConfig as ::core::ffi::c_int != 0 {
        (*pcd).maxX
    } else {
        (*pcd).clientX
    }) as ::core::ffi::c_uint)
        .wrapping_sub(LowerBorderWidth(pcd))
        .wrapping_sub((*pcd).matteWidth as ::core::ffi::c_uint) as ::core::ffi::c_int;
    return frameX;
}
#[no_mangle]
pub unsafe extern "C" fn FrameY(mut pcd: *mut ClientData) -> ::core::ffi::c_int {
    let mut frameY: ::core::ffi::c_int = 0;
    frameY = ((if (*pcd).maxConfig as ::core::ffi::c_int != 0 {
        (*pcd).maxY
    } else {
        (*pcd).clientY
    }) as ::core::ffi::c_uint)
        .wrapping_sub(UpperBorderWidth(pcd))
        .wrapping_sub((*pcd).frameInfo.titleBarHeight)
        .wrapping_sub((*pcd).matteWidth as ::core::ffi::c_uint) as ::core::ffi::c_int;
    if (*pcd).decor & MWM_DECOR_TITLE != 0
        && (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) == 0
        && (*pcd).matteWidth == 0
    {
        frameY -= (*pcd).xBorderWidth;
    }
    return frameY;
}
#[no_mangle]
pub unsafe extern "C" fn BaseWindowWidth(mut pcd: *mut ClientData) -> ::core::ffi::c_uint {
    let mut rval: ::core::ffi::c_uint = 0;
    if (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) != 0 {
        rval = ((if (*pcd).maxConfig as ::core::ffi::c_int != 0 {
            (*pcd).maxWidth
        } else {
            (*pcd).clientWidth
        }) + 2 as ::core::ffi::c_int * (*pcd).matteWidth) as ::core::ffi::c_uint;
    } else {
        rval = (((if (*pcd).maxConfig as ::core::ffi::c_int != 0 {
            (*pcd).maxWidth
        } else {
            (*pcd).clientWidth
        }) + 2 as ::core::ffi::c_int * (*pcd).matteWidth) as ::core::ffi::c_uint)
            .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(LowerBorderWidth(pcd)));
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn BaseWindowHeight(mut pcd: *mut ClientData) -> ::core::ffi::c_uint {
    let mut rval: ::core::ffi::c_uint = 0;
    if (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) != 0 {
        rval = ((if (*pcd).maxConfig as ::core::ffi::c_int != 0 {
            (*pcd).maxHeight
        } else {
            (*pcd).clientHeight
        }) + 2 as ::core::ffi::c_int * (*pcd).matteWidth) as ::core::ffi::c_uint;
    } else {
        rval = (((if (*pcd).maxConfig as ::core::ffi::c_int != 0 {
            (*pcd).maxHeight
        } else {
            (*pcd).clientHeight
        }) + 2 as ::core::ffi::c_int * (*pcd).matteWidth) as ::core::ffi::c_uint)
            .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(LowerBorderWidth(pcd)));
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn GetFramePartInfo(
    mut pcd: *mut ClientData,
    mut part: ::core::ffi::c_int,
    mut pX: *mut ::core::ffi::c_int,
    mut pY: *mut ::core::ffi::c_int,
    mut pWidth: *mut ::core::ffi::c_uint,
    mut pHeight: *mut ::core::ffi::c_uint,
) -> Boolean {
    let mut boxdim = (*pcd).frameInfo.titleBarHeight;
    let mut decor = (*pcd).decor as ::core::ffi::c_ulong;
    let mut vert: ::core::ffi::c_int = 0;
    let mut horiz: ::core::ffi::c_int = 0;
    let mut rval = FALSE as Boolean;
    match part {
        FRAME_SYSTEM => {
            if decor & MWM_DECOR_MENU as ::core::ffi::c_ulong != 0 {
                *pX = (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int;
                *pY = (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int;
                *pWidth = boxdim;
                *pHeight = boxdim;
                rval = TRUE as Boolean;
            }
        }
        FRAME_TITLE => {
            if decor & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0 {
                *pX = (*pcd).frameInfo.upperBorderWidth.wrapping_add(
                    (if decor & MWM_DECOR_MENU as ::core::ffi::c_ulong != 0 {
                        boxdim
                    } else {
                        0 as ::core::ffi::c_uint
                    }),
                ) as ::core::ffi::c_int;
                *pY = (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int;
                *pWidth = (*pcd)
                    .frameInfo
                    .width
                    .wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.upperBorderWidth),
                    )
                    .wrapping_sub(
                        (if decor & MWM_DECOR_MENU as ::core::ffi::c_ulong != 0 {
                            boxdim
                        } else {
                            0 as ::core::ffi::c_uint
                        }),
                    )
                    .wrapping_sub(
                        (if decor & MWM_DECOR_MINIMIZE as ::core::ffi::c_ulong != 0 {
                            boxdim
                        } else {
                            0 as ::core::ffi::c_uint
                        }),
                    )
                    .wrapping_sub(
                        (if decor & MWM_DECOR_MAXIMIZE as ::core::ffi::c_ulong != 0 {
                            boxdim
                        } else {
                            0 as ::core::ffi::c_uint
                        }),
                    )
                    .wrapping_sub(
                        (if (*pcd).decor & MWM_DECOR_TITLE != 0
                            && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
                        {
                            boxdim
                        } else {
                            0 as ::core::ffi::c_uint
                        }),
                    );
                *pHeight = boxdim;
                rval = TRUE as Boolean;
            }
        }
        FRAME_TITLEBAR => {
            if decor & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0 {
                *pX = (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int;
                *pY = (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int;
                *pWidth = (*pcd).frameInfo.width.wrapping_sub(
                    (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.upperBorderWidth),
                );
                *pHeight = boxdim;
                rval = TRUE as Boolean;
            }
        }
        FRAME_MINIMIZE => {
            if decor & MWM_DECOR_MINIMIZE as ::core::ffi::c_ulong != 0 {
                *pX = (*pcd)
                    .frameInfo
                    .width
                    .wrapping_sub((*pcd).frameInfo.upperBorderWidth)
                    .wrapping_sub(boxdim)
                    .wrapping_sub(
                        (if decor & MWM_DECOR_MAXIMIZE as ::core::ffi::c_ulong != 0 {
                            boxdim
                        } else {
                            0 as ::core::ffi::c_uint
                        }),
                    )
                    .wrapping_sub(
                        (if (*pcd).decor & MWM_DECOR_TITLE != 0
                            && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
                        {
                            boxdim
                        } else {
                            0 as ::core::ffi::c_uint
                        }),
                    ) as ::core::ffi::c_int;
                *pY = (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int;
                *pWidth = boxdim;
                *pHeight = boxdim;
                rval = TRUE as Boolean;
            }
        }
        FRAME_MAXIMIZE => {
            if decor & MWM_DECOR_MAXIMIZE as ::core::ffi::c_ulong != 0 {
                *pX = (*pcd)
                    .frameInfo
                    .width
                    .wrapping_sub((*pcd).frameInfo.upperBorderWidth)
                    .wrapping_sub(boxdim)
                    .wrapping_sub(
                        (if (*pcd).decor & MWM_DECOR_TITLE != 0
                            && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
                        {
                            boxdim
                        } else {
                            0 as ::core::ffi::c_uint
                        }),
                    ) as ::core::ffi::c_int;
                *pY = (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int;
                *pWidth = boxdim;
                *pHeight = boxdim;
                rval = TRUE as Boolean;
            }
        }
        FRAME_CLOSE => {
            if (*pcd).decor & MWM_DECOR_TITLE != 0
                && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
            {
                *pX = (*pcd)
                    .frameInfo
                    .width
                    .wrapping_sub((*pcd).frameInfo.upperBorderWidth)
                    .wrapping_sub(boxdim) as ::core::ffi::c_int;
                *pY = (*pcd).frameInfo.upperBorderWidth as ::core::ffi::c_int;
                *pHeight = boxdim;
                *pWidth = *pHeight;
                rval = TRUE as Boolean;
            }
        }
        FRAME_RESIZE_NW => {
            if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
                *pX = 0 as ::core::ffi::c_int;
                *pY = 0 as ::core::ffi::c_int;
                if (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pWidth = (*pcd).frameInfo.cornerWidth;
                } else {
                    horiz = (*pcd).frameInfo.width.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerWidth),
                    ) as ::core::ffi::c_int;
                    *pWidth = (*pcd)
                        .frameInfo
                        .cornerWidth
                        .wrapping_add((horiz / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                        .wrapping_add((horiz % 2 as ::core::ffi::c_int) as ::core::ffi::c_uint);
                }
                if (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pHeight = (*pcd).frameInfo.cornerHeight;
                } else {
                    vert = (*pcd).frameInfo.height.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerHeight),
                    ) as ::core::ffi::c_int;
                    *pHeight = (*pcd)
                        .frameInfo
                        .cornerHeight
                        .wrapping_add((vert / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                        .wrapping_add((vert % 2 as ::core::ffi::c_int) as ::core::ffi::c_uint);
                }
                rval = TRUE as Boolean;
            }
        }
        FRAME_RESIZE_N => {
            if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
                *pX = (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int;
                *pY = 0 as ::core::ffi::c_int;
                *pHeight = (*pcd).frameInfo.upperBorderWidth;
                if (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pWidth = (*pcd).frameInfo.width.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerWidth),
                    );
                } else {
                    *pWidth = 0 as ::core::ffi::c_uint;
                }
                rval = TRUE as Boolean;
            }
        }
        FRAME_RESIZE_NE => {
            if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
                if (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pWidth = (*pcd).frameInfo.cornerWidth;
                } else {
                    horiz = (*pcd).frameInfo.width.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerWidth),
                    ) as ::core::ffi::c_int;
                    *pWidth = (*pcd)
                        .frameInfo
                        .cornerWidth
                        .wrapping_add((horiz / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint);
                }
                *pX = (*pcd).frameInfo.width.wrapping_sub(*pWidth) as ::core::ffi::c_int;
                if (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pHeight = (*pcd).frameInfo.cornerHeight;
                } else {
                    vert = (*pcd).frameInfo.height.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerHeight),
                    ) as ::core::ffi::c_int;
                    *pHeight = (*pcd)
                        .frameInfo
                        .cornerHeight
                        .wrapping_add((vert / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                        .wrapping_add((vert % 2 as ::core::ffi::c_int) as ::core::ffi::c_uint);
                }
                *pY = 0 as ::core::ffi::c_int;
                rval = TRUE as Boolean;
            }
        }
        FRAME_RESIZE_E => {
            if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
                *pX = (*pcd)
                    .frameInfo
                    .width
                    .wrapping_sub((*pcd).frameInfo.lowerBorderWidth)
                    as ::core::ffi::c_int;
                *pY = (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int;
                *pWidth = (*pcd).frameInfo.lowerBorderWidth;
                if (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pHeight = (*pcd).frameInfo.height.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerHeight),
                    );
                } else {
                    *pHeight = 0 as ::core::ffi::c_uint;
                }
                rval = TRUE as Boolean;
            }
        }
        FRAME_RESIZE_SE => {
            if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
                if (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pWidth = (*pcd).frameInfo.cornerWidth;
                } else {
                    horiz = (*pcd).frameInfo.width.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerWidth),
                    ) as ::core::ffi::c_int;
                    *pWidth = (*pcd)
                        .frameInfo
                        .cornerWidth
                        .wrapping_add((horiz / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint);
                }
                *pX = (*pcd).frameInfo.width.wrapping_sub(*pWidth) as ::core::ffi::c_int;
                if (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pHeight = (*pcd).frameInfo.cornerHeight;
                } else {
                    vert = (*pcd).frameInfo.height.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerHeight),
                    ) as ::core::ffi::c_int;
                    *pHeight = (*pcd)
                        .frameInfo
                        .cornerHeight
                        .wrapping_add((vert / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint);
                }
                *pY = (*pcd).frameInfo.height.wrapping_sub(*pHeight) as ::core::ffi::c_int;
                rval = TRUE as Boolean;
            }
        }
        FRAME_RESIZE_S => {
            if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
                *pX = (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int;
                *pY = (*pcd)
                    .frameInfo
                    .height
                    .wrapping_sub((*pcd).frameInfo.lowerBorderWidth)
                    as ::core::ffi::c_int;
                if (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pWidth = (*pcd).frameInfo.width.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerWidth),
                    );
                } else {
                    *pWidth = 0 as ::core::ffi::c_uint;
                }
                *pHeight = (*pcd).frameInfo.lowerBorderWidth;
                rval = TRUE as Boolean;
            }
        }
        FRAME_RESIZE_SW => {
            if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
                if (*pcd).frameInfo.cornerWidth as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pWidth = (*pcd).frameInfo.cornerWidth;
                } else {
                    horiz = (*pcd).frameInfo.width.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerWidth),
                    ) as ::core::ffi::c_int;
                    *pWidth = (*pcd)
                        .frameInfo
                        .cornerWidth
                        .wrapping_add((horiz / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                        .wrapping_add((horiz % 2 as ::core::ffi::c_int) as ::core::ffi::c_uint);
                }
                *pX = 0 as ::core::ffi::c_int;
                if (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pHeight = (*pcd).frameInfo.cornerHeight;
                } else {
                    vert = (*pcd).frameInfo.height.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerHeight),
                    ) as ::core::ffi::c_int;
                    *pHeight = (*pcd)
                        .frameInfo
                        .cornerHeight
                        .wrapping_add((vert / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint);
                }
                *pY = (*pcd).frameInfo.height.wrapping_sub(*pHeight) as ::core::ffi::c_int;
                rval = TRUE as Boolean;
            }
        }
        FRAME_RESIZE_W => {
            if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
                *pX = 0 as ::core::ffi::c_int;
                *pY = (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int;
                *pWidth = (*pcd).frameInfo.lowerBorderWidth;
                if (*pcd).frameInfo.cornerHeight as ::core::ffi::c_int
                    > (*pcd).frameInfo.lowerBorderWidth as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                {
                    *pHeight = (*pcd).frameInfo.height.wrapping_sub(
                        (2 as ::core::ffi::c_uint).wrapping_mul((*pcd).frameInfo.cornerHeight),
                    );
                } else {
                    *pHeight = 0 as ::core::ffi::c_uint;
                }
                rval = TRUE as Boolean;
            }
        }
        _ => {}
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn IdentifyFramePart(
    mut pCD: *mut ClientData,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut decor = (*pCD).decor as ::core::ffi::c_ulong;
    let mut clientWidth = if (*pCD).maxConfig as ::core::ffi::c_int != 0 {
        (*pCD).maxWidth
    } else {
        (*pCD).clientWidth
    };
    let mut clientHeight = if (*pCD).maxConfig as ::core::ffi::c_int != 0 {
        (*pCD).maxHeight
    } else {
        (*pCD).clientHeight
    };
    let mut rval: ::core::ffi::c_int = 0;
    if x >= (*pCD).clientOffset.x as ::core::ffi::c_int
        && x < (*pCD).clientOffset.x as ::core::ffi::c_int + clientWidth
        && y >= (*pCD).clientOffset.y as ::core::ffi::c_int
        && y < (*pCD).clientOffset.y as ::core::ffi::c_int + clientHeight
    {
        return 1 as ::core::ffi::c_int;
    }
    if (*pCD).matteWidth != 0 {
        if x >= (*pCD).matteRectangle.x as ::core::ffi::c_int
            && x < (*pCD).matteRectangle.x as ::core::ffi::c_int
                + (*pCD).matteRectangle.width as ::core::ffi::c_int
            && y >= (*pCD).matteRectangle.y as ::core::ffi::c_int
            && y < (*pCD).matteRectangle.y as ::core::ffi::c_int
                + (*pCD).matteRectangle.height as ::core::ffi::c_int
        {
            return 15 as ::core::ffi::c_int;
        }
    }
    if decor & MWM_DECOR_TITLE as ::core::ffi::c_ulong != 0 {
        if x >= (*pCD).titleRectangle.x as ::core::ffi::c_int
            && x < (*pCD).titleRectangle.x as ::core::ffi::c_int
                + (*pCD).titleRectangle.width as ::core::ffi::c_int
            && y >= (*pCD).titleRectangle.y as ::core::ffi::c_int
            && y < (*pCD).titleRectangle.y as ::core::ffi::c_int
                + (*pCD).titleRectangle.height as ::core::ffi::c_int
        {
            return GadgetID(
                x,
                y,
                (*pCD).pTitleGadgets,
                (*pCD).cTitleGadgets as ::core::ffi::c_uint,
            );
        }
    }
    if decor & MWM_DECOR_RESIZEH as ::core::ffi::c_ulong != 0 {
        rval = GadgetID(
            x,
            y,
            (*pCD).pResizeGadgets,
            STRETCH_COUNT as ::core::ffi::c_uint,
        );
    } else {
        rval = FRAME_NONE;
    }
    if rval == FRAME_NONE {
        if x >= 0 as ::core::ffi::c_int
            && y >= 0 as ::core::ffi::c_int
            && (x as ::core::ffi::c_uint) < FrameWidth(pCD)
            && (y as ::core::ffi::c_uint) < FrameHeight(pCD)
        {
            rval = FRAME_NBORDER;
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn GadgetID(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut pgadget: *mut GadgetRectangle,
    mut count: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut ix: ::core::ffi::c_int = 0;
    ix = 0 as ::core::ffi::c_int;
    while (ix as ::core::ffi::c_uint) < count {
        if x >= (*pgadget).rect.x as ::core::ffi::c_int
            && x < (*pgadget).rect.x as ::core::ffi::c_int
                + (*pgadget).rect.width as ::core::ffi::c_int
            && y >= (*pgadget).rect.y as ::core::ffi::c_int
            && y < (*pgadget).rect.y as ::core::ffi::c_int
                + (*pgadget).rect.height as ::core::ffi::c_int
        {
            return (*pgadget).id as ::core::ffi::c_ulong as ::core::ffi::c_int;
        }
        ix += 1;
        pgadget = pgadget.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FrameToClient(
    mut pcd: *mut ClientData,
    mut pX: *mut ::core::ffi::c_int,
    mut pY: *mut ::core::ffi::c_int,
    mut pWidth: *mut ::core::ffi::c_uint,
    mut pHeight: *mut ::core::ffi::c_uint,
) {
    *pWidth = (*pWidth).wrapping_sub(
        (2 as ::core::ffi::c_int * (*pcd).clientOffset.x as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
    *pHeight = (*pHeight)
        .wrapping_sub((*pcd).clientOffset.x as ::core::ffi::c_uint)
        .wrapping_sub((*pcd).clientOffset.y as ::core::ffi::c_uint);
    *pX = *pX + (*pcd).clientOffset.x as ::core::ffi::c_int;
    *pY = *pY + (*pcd).clientOffset.y as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ClientToFrame(
    mut pcd: *mut ClientData,
    mut pX: *mut ::core::ffi::c_int,
    mut pY: *mut ::core::ffi::c_int,
    mut pWidth: *mut ::core::ffi::c_uint,
    mut pHeight: *mut ::core::ffi::c_uint,
) {
    *pWidth = (*pWidth).wrapping_add(
        (2 as ::core::ffi::c_int * (*pcd).clientOffset.x as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
    *pHeight = (*pHeight)
        .wrapping_add((*pcd).clientOffset.x as ::core::ffi::c_uint)
        .wrapping_add((*pcd).clientOffset.y as ::core::ffi::c_uint);
    *pX = *pX - (*pcd).clientOffset.x as ::core::ffi::c_int;
    *pY = *pY - (*pcd).clientOffset.y as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GetDepressInfo(
    mut pcd: *mut ClientData,
    mut part: ::core::ffi::c_int,
    mut pX: *mut ::core::ffi::c_int,
    mut pY: *mut ::core::ffi::c_int,
    mut pWidth: *mut ::core::ffi::c_uint,
    mut pHeight: *mut ::core::ffi::c_uint,
    mut pInvertWidth: *mut ::core::ffi::c_uint,
) -> Boolean {
    let mut rval: Boolean = 0;
    let mut exBevel: ::core::ffi::c_uint = 0;
    let mut eBevel: ::core::ffi::c_uint = 0;
    let mut wBevel: ::core::ffi::c_uint = 0;
    let mut sBevel: ::core::ffi::c_uint = 0;
    let mut insideBevel: ::core::ffi::c_uint = 0;
    let mut meBevel: ::core::ffi::c_uint = 0;
    let mut decoration = (*pcd).decor as ::core::ffi::c_uint;
    let mut tmp: ::core::ffi::c_uint = 0;
    match part {
        FRAME_SYSTEM | FRAME_TITLE | FRAME_MINIMIZE | FRAME_MAXIMIZE | FRAME_CLOSE => {
            rval = GetFramePartInfo(pcd, part, pX, pY, pWidth, pHeight);
            if rval != 0 {
                *pInvertWidth = (1 as ::core::ffi::c_int
                    + (*(*pcd).pSD).joinBevel / 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint;
                insideBevel = (if (*pcd).matteWidth > 0 as ::core::ffi::c_int {
                    (*(*pcd).pSD).joinBevel
                } else {
                    (*pcd).internalBevel
                }) as ::core::ffi::c_uint;
                if decoration as ::core::ffi::c_long & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) != 0 {
                    exBevel = (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint;
                    sBevel = insideBevel;
                    eBevel = (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint;
                    wBevel = (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint;
                    meBevel = (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint;
                } else {
                    exBevel = (*(*pcd).pSD).externalBevel as ::core::ffi::c_uint;
                    sBevel = if (*pcd).matteWidth > 0 as ::core::ffi::c_int {
                        insideBevel
                    } else {
                        (*(*pcd).pSD).externalBevel as ::core::ffi::c_uint
                    };
                    eBevel = (if decoration as ::core::ffi::c_long
                        & (MWM_DECOR_MINIMIZE | MWM_DECOR_MAXIMIZE)
                        != 0
                        || (*pcd).decor & MWM_DECOR_TITLE != 0
                            && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
                    {
                        (*(*pcd).pSD).joinBevel
                    } else {
                        (*(*pcd).pSD).externalBevel
                    }) as ::core::ffi::c_uint;
                    wBevel = (if decoration as ::core::ffi::c_long & MWM_DECOR_MENU != 0 {
                        (*(*pcd).pSD).joinBevel
                    } else {
                        (*(*pcd).pSD).externalBevel
                    }) as ::core::ffi::c_uint;
                    meBevel = (if decoration as ::core::ffi::c_long & MWM_DECOR_MAXIMIZE != 0
                        || (*pcd).decor & MWM_DECOR_TITLE != 0
                            && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE != 0
                    {
                        (*(*pcd).pSD).joinBevel
                    } else {
                        (*(*pcd).pSD).externalBevel
                    }) as ::core::ffi::c_uint;
                }
                if sBevel > *pInvertWidth {
                    *pHeight = (*pHeight).wrapping_sub(sBevel.wrapping_sub(*pInvertWidth));
                }
                match part {
                    FRAME_SYSTEM => {
                        if exBevel > *pInvertWidth {
                            tmp = exBevel.wrapping_sub(*pInvertWidth);
                            *pX = (*pX as ::core::ffi::c_uint).wrapping_add(tmp)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                            *pWidth = (*pWidth).wrapping_sub(tmp);
                            *pY = (*pY as ::core::ffi::c_uint).wrapping_add(tmp)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                            *pHeight = (*pHeight).wrapping_sub(tmp);
                        }
                        if wBevel > *pInvertWidth {
                            *pWidth = (*pWidth).wrapping_sub(wBevel.wrapping_sub(*pInvertWidth));
                        }
                    }
                    FRAME_TITLE => {
                        if wBevel > *pInvertWidth {
                            tmp = wBevel.wrapping_sub(*pInvertWidth);
                            *pX = (*pX as ::core::ffi::c_uint).wrapping_add(tmp)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                            *pWidth = (*pWidth).wrapping_sub(tmp);
                        }
                        if exBevel > *pInvertWidth {
                            tmp = exBevel.wrapping_sub(*pInvertWidth);
                            *pY = (*pY as ::core::ffi::c_uint).wrapping_add(tmp)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                            *pHeight = (*pHeight).wrapping_sub(tmp);
                        }
                        if eBevel > *pInvertWidth {
                            *pWidth = (*pWidth).wrapping_sub(eBevel.wrapping_sub(*pInvertWidth));
                        }
                    }
                    FRAME_MINIMIZE => {
                        if exBevel > *pInvertWidth {
                            tmp = exBevel.wrapping_sub(*pInvertWidth);
                            *pY = (*pY as ::core::ffi::c_uint).wrapping_add(tmp)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                            *pHeight = (*pHeight).wrapping_sub(tmp);
                        }
                        if eBevel > *pInvertWidth {
                            tmp = eBevel.wrapping_sub(*pInvertWidth);
                            *pX = (*pX as ::core::ffi::c_uint).wrapping_add(tmp)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                            *pWidth = (*pWidth).wrapping_sub(tmp);
                        }
                        if meBevel > *pInvertWidth {
                            *pWidth = (*pWidth).wrapping_sub(meBevel.wrapping_sub(*pInvertWidth));
                        }
                    }
                    FRAME_MAXIMIZE | FRAME_CLOSE => {
                        if exBevel > *pInvertWidth {
                            tmp = exBevel.wrapping_sub(*pInvertWidth);
                            *pY = (*pY as ::core::ffi::c_uint).wrapping_add(tmp)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                            *pHeight = (*pHeight).wrapping_sub(tmp);
                        }
                        tmp = if part == FRAME_MAXIMIZE
                            && ((*pcd).decor & MWM_DECOR_TITLE != 0
                                && (*pcd).clientFunctions as ::core::ffi::c_long & MWM_FUNC_CLOSE
                                    != 0)
                        {
                            (*(*pcd).pSD).joinBevel as ::core::ffi::c_uint
                        } else {
                            exBevel
                        };
                        if tmp > *pInvertWidth {
                            *pWidth = (*pWidth).wrapping_sub(tmp.wrapping_sub(*pInvertWidth));
                        }
                        if eBevel > *pInvertWidth {
                            tmp = eBevel.wrapping_sub(*pInvertWidth);
                            *pX = (*pX as ::core::ffi::c_uint).wrapping_add(tmp)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                            *pWidth = (*pWidth).wrapping_sub(tmp);
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {
            rval = False as Boolean;
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn SetFrameInfo(mut pcd: *mut ClientData) {
    (*pcd).frameInfo.titleBarHeight = InitTitleBarHeight(pcd);
    (*pcd).frameInfo.x = FrameX(pcd);
    (*pcd).frameInfo.y = FrameY(pcd);
    (*pcd).frameInfo.width = FrameWidth(pcd);
    (*pcd).frameInfo.height = FrameHeight(pcd);
    (*pcd).frameInfo.upperBorderWidth = UpperBorderWidth(pcd);
    (*pcd).frameInfo.lowerBorderWidth = LowerBorderWidth(pcd);
    (*pcd).frameInfo.cornerWidth = CornerWidth(pcd);
    (*pcd).frameInfo.cornerHeight = CornerHeight(pcd);
    if (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) != 0
        || (*pcd).matteWidth > 0 as ::core::ffi::c_int
    {
        (*pcd).clientOffset.x = (*pcd)
            .frameInfo
            .lowerBorderWidth
            .wrapping_add((*pcd).matteWidth as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
        (*pcd).clientOffset.y = (*pcd)
            .frameInfo
            .upperBorderWidth
            .wrapping_add((*pcd).frameInfo.titleBarHeight)
            .wrapping_add((*pcd).matteWidth as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
    } else {
        (*pcd).clientOffset.x = ((*pcd).xBorderWidth + (*pcd).matteWidth) as ::core::ffi::c_short;
        (*pcd).clientOffset.y = ((*pcd).xBorderWidth as ::core::ffi::c_uint)
            .wrapping_add((*pcd).frameInfo.titleBarHeight)
            .wrapping_add((*pcd).matteWidth as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SetClientOffset(mut pcd: *mut ClientData) {
    (*pcd).frameInfo.titleBarHeight = InitTitleBarHeight(pcd);
    (*pcd).frameInfo.upperBorderWidth = UpperBorderWidth(pcd);
    (*pcd).frameInfo.lowerBorderWidth = LowerBorderWidth(pcd);
    if (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) != 0
        || (*pcd).matteWidth > 0 as ::core::ffi::c_int
    {
        (*pcd).clientOffset.x = (*pcd)
            .frameInfo
            .lowerBorderWidth
            .wrapping_add((*pcd).matteWidth as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
        (*pcd).clientOffset.y = (*pcd)
            .frameInfo
            .upperBorderWidth
            .wrapping_add((*pcd).frameInfo.titleBarHeight)
            .wrapping_add((*pcd).matteWidth as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
    } else {
        (*pcd).clientOffset.x = ((*pcd).xBorderWidth + (*pcd).matteWidth) as ::core::ffi::c_short;
        (*pcd).clientOffset.y = ((*pcd).xBorderWidth as ::core::ffi::c_uint)
            .wrapping_add((*pcd).frameInfo.titleBarHeight)
            .wrapping_add((*pcd).matteWidth as ::core::ffi::c_uint)
            as ::core::ffi::c_short;
    };
}
#[no_mangle]
pub unsafe extern "C" fn XBorderIsShowing(mut pcd: *mut ClientData) -> Boolean {
    let mut rval: Boolean = 0;
    if (*pcd).decor & (MWM_DECOR_RESIZEH | MWM_DECOR_BORDER) != 0
        || (*pcd).matteWidth > 0 as ::core::ffi::c_int
    {
        rval = False as Boolean;
    } else {
        rval = True as Boolean;
    }
    return rval;
}
