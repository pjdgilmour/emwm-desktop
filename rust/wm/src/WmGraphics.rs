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
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_1;
use crate::abi::C2RustUnnamed_3_WmEwmh as C2RustUnnamed_2;
use crate::abi::*;
extern "C" {
    fn XGetImage(
        _: *mut Display,
        _: Drawable,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_ulong,
        _: ::core::ffi::c_int,
    ) -> *mut XImage;
    fn XChangeGC(
        _: *mut Display,
        _: GC,
        _: ::core::ffi::c_ulong,
        _: *mut XGCValues,
    ) -> ::core::ffi::c_int;
    fn XDrawImageString(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XDrawString(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XSetClipMask(_: *mut Display, _: GC, _: Pixmap) -> ::core::ffi::c_int;
    fn XSetClipRectangles(
        _: *mut Display,
        _: GC,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut XRectangle,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XTextWidth(
        _: *mut XFontStruct,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XtMalloc(_: Cardinal) -> *mut ::core::ffi::c_char;
    fn XtFree(_: *mut ::core::ffi::c_char);
    fn XmInstallImage(image: *mut XImage, image_name: *mut ::core::ffi::c_char) -> Boolean;
    fn XmStringWidth(fontlist: XmFontList, string: XmString) -> Dimension;
    fn XmStringDraw(
        d: *mut Display,
        w: Window,
        fontlist: XmFontList,
        string: XmString,
        gc: GC,
        x: Position,
        y: Position,
        width: Dimension,
        align: ::core::ffi::c_uchar,
        lay_dir: ::core::ffi::c_uchar,
        clip: *mut XRectangle,
    );
    fn XmStringDrawImage(
        d: *mut Display,
        w: Window,
        fontlist: XmFontList,
        string: XmString,
        gc: GC,
        x: Position,
        y: Position,
        width: Dimension,
        align: ::core::ffi::c_uchar,
        lay_dir: ::core::ffi::c_uchar,
        clip: *mut XRectangle,
    );
    static mut wmGD: WmGlobalData;
    fn Warning(message: *mut ::core::ffi::c_char);
}
pub const XtGeometryDone: XtGeometryResult = 3;
pub const XtGeometryAlmost: XtGeometryResult = 2;
pub const XtGeometryNo: XtGeometryResult = 1;
pub const XtGeometryYes: XtGeometryResult = 0;
pub const XmSTRING_DIRECTION_DEFAULT: C2RustUnnamed_1 = 255;
pub const XmSTRING_DIRECTION_UNSET: C2RustUnnamed_1 = 3;
pub const XmSTRING_DIRECTION_R_TO_L: C2RustUnnamed_1 = 1;
pub const XmSTRING_DIRECTION_L_TO_R: C2RustUnnamed_1 = 0;
pub const XmALIGNMENT_UNSPECIFIED: C2RustUnnamed_2 = 3;
pub const XmALIGNMENT_END: C2RustUnnamed_2 = 2;
pub const XmALIGNMENT_CENTER: C2RustUnnamed_2 = 1;
pub const XmALIGNMENT_BEGINNING: C2RustUnnamed_2 = 0;
pub const WmSLAB: _FrameStyle = 1;
pub const WmRECESSED: _FrameStyle = 0;
pub const None_0: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const Unsorted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GCClipXOrigin: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 17 as ::core::ffi::c_int;
pub const GCClipYOrigin: ::core::ffi::c_long =
    (1 as ::core::ffi::c_long) << 18 as ::core::ffi::c_int;
pub const GCClipMask: ::core::ffi::c_long = (1 as ::core::ffi::c_long) << 19 as ::core::ffi::c_int;
pub const XYBitmap: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LSBFirst: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MSBFirst: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const True: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const False: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STRETCH_NORTH_WEST: ::core::ffi::c_int = 0;
pub const STRETCH_NORTH_EAST: ::core::ffi::c_int = 2;
pub const STRETCH_SOUTH_EAST: ::core::ffi::c_int = 4;
pub const STRETCH_SOUTH_WEST: ::core::ffi::c_int = 6;
pub const RLIST_EXTENSION_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn BevelRectangle(
    mut prTop: *mut RList,
    mut prBot: *mut RList,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
    mut top_wid: ::core::ffi::c_uint,
    mut right_wid: ::core::ffi::c_uint,
    mut bot_wid: ::core::ffi::c_uint,
    mut left_wid: ::core::ffi::c_uint,
) {
    let mut prect = ::core::ptr::null_mut::<XRectangle>();
    let mut count: ::core::ffi::c_int = 0;
    let mut join1: ::core::ffi::c_int = 0;
    let mut join2: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut piTop = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut piBot = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if ((*prTop).used as ::core::ffi::c_uint).wrapping_add(top_wid.wrapping_add(left_wid))
        > (*prTop).allocated as ::core::ffi::c_uint
        && ExtendRList(
            prTop,
            (if top_wid.wrapping_add(left_wid) > 10 as ::core::ffi::c_uint {
                top_wid.wrapping_add(left_wid)
            } else {
                10 as ::core::ffi::c_uint
            }),
        ) == 0
    {
        return;
    }
    piTop = &raw mut (*prTop).used;
    prect = (*prTop).prect.offset(*piTop as isize) as *mut XRectangle;
    join1 = left_wid as ::core::ffi::c_int;
    join2 = right_wid as ::core::ffi::c_int;
    x1 = x;
    y1 = y;
    len = width as ::core::ffi::c_int;
    count = top_wid as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = len as ::core::ffi::c_ushort;
        (*prect).height = 1 as ::core::ffi::c_ushort;
        join1 -= 1;
        x1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        y1 += 1 as ::core::ffi::c_int;
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piTop += 1;
    }
    join1 = top_wid as ::core::ffi::c_int;
    join2 = bot_wid as ::core::ffi::c_int;
    x1 = x;
    y1 = y
        + (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
    len = height.wrapping_sub(
        (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    ) as ::core::ffi::c_int;
    count = left_wid as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = 1 as ::core::ffi::c_ushort;
        (*prect).height = len as ::core::ffi::c_ushort;
        x1 += 1 as ::core::ffi::c_int;
        join1 -= 1;
        y1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piTop += 1;
    }
    if ((*prBot).used as ::core::ffi::c_uint).wrapping_add(bot_wid.wrapping_add(right_wid))
        > (*prBot).allocated as ::core::ffi::c_uint
        && ExtendRList(
            prBot,
            (if bot_wid.wrapping_add(right_wid) > 10 as ::core::ffi::c_uint {
                bot_wid.wrapping_add(right_wid)
            } else {
                10 as ::core::ffi::c_uint
            }),
        ) == 0
    {
        return;
    }
    piBot = &raw mut (*prBot).used;
    prect = (*prBot).prect.offset(*piBot as isize) as *mut XRectangle;
    join1 = left_wid as ::core::ffi::c_int;
    join2 = right_wid as ::core::ffi::c_int;
    x1 = x
        + (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
    y1 = (y as ::core::ffi::c_uint)
        .wrapping_add(height)
        .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    len = width.wrapping_sub(
        (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    ) as ::core::ffi::c_int;
    if (if join1 > 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) != 0
        && bot_wid > left_wid
    {
        len += 1;
        x1 -= 1;
        join1 += 1;
    }
    count = bot_wid as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = len as ::core::ffi::c_ushort;
        (*prect).height = 1 as ::core::ffi::c_ushort;
        join1 -= 1;
        x1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        y1 -= 1 as ::core::ffi::c_int;
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piBot += 1;
    }
    join1 = top_wid as ::core::ffi::c_int;
    join2 = bot_wid as ::core::ffi::c_int;
    x1 = (x as ::core::ffi::c_uint)
        .wrapping_add(width)
        .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    y1 = y
        + (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
    len = height
        .wrapping_sub(
            (if join1 > 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        )
        .wrapping_sub(
            (if join2 > 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int;
    if (if join1 > 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) != 0
        && right_wid > top_wid
    {
        len += 1;
        y1 -= 1;
        join1 += 1;
    }
    count = right_wid as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = 1 as ::core::ffi::c_ushort;
        (*prect).height = len as ::core::ffi::c_ushort;
        x1 -= 1 as ::core::ffi::c_int;
        join1 -= 1;
        y1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piBot += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BevelDepressedRectangle(
    mut prTop: *mut RList,
    mut prBot: *mut RList,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
    mut top_wid: ::core::ffi::c_uint,
    mut right_wid: ::core::ffi::c_uint,
    mut bot_wid: ::core::ffi::c_uint,
    mut left_wid: ::core::ffi::c_uint,
    mut in_wid: ::core::ffi::c_uint,
) {
    let mut prect = ::core::ptr::null_mut::<XRectangle>();
    let mut count: ::core::ffi::c_int = 0;
    let mut join1: ::core::ffi::c_int = 0;
    let mut join2: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut piTop = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut piBot = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if ((*prTop).used as ::core::ffi::c_uint).wrapping_add(top_wid.wrapping_add(left_wid))
        > (*prTop).allocated as ::core::ffi::c_uint
        && ExtendRList(
            prTop,
            (if top_wid.wrapping_add(left_wid) > 10 as ::core::ffi::c_uint {
                top_wid.wrapping_add(left_wid)
            } else {
                10 as ::core::ffi::c_uint
            }),
        ) == 0
    {
        return;
    }
    if ((*prBot).used as ::core::ffi::c_uint).wrapping_add(bot_wid.wrapping_add(right_wid))
        > (*prBot).allocated as ::core::ffi::c_uint
        && ExtendRList(
            prBot,
            (if bot_wid.wrapping_add(right_wid) > 10 as ::core::ffi::c_uint {
                bot_wid.wrapping_add(right_wid)
            } else {
                10 as ::core::ffi::c_uint
            }),
        ) == 0
    {
        return;
    }
    piTop = &raw mut (*prTop).used;
    prect = (*prTop).prect.offset(*piTop as isize) as *mut XRectangle;
    join1 = left_wid as ::core::ffi::c_int;
    join2 = right_wid as ::core::ffi::c_int;
    x1 = x;
    y1 = y;
    len = width as ::core::ffi::c_int;
    count = top_wid.wrapping_sub(in_wid) as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = len as ::core::ffi::c_ushort;
        (*prect).height = 1 as ::core::ffi::c_ushort;
        join1 -= 1;
        x1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        y1 += 1 as ::core::ffi::c_int;
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piTop += 1;
    }
    piBot = &raw mut (*prBot).used;
    prect = (*prBot).prect.offset(*piBot as isize) as *mut XRectangle;
    count = in_wid as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = len as ::core::ffi::c_ushort;
        (*prect).height = 1 as ::core::ffi::c_ushort;
        join1 -= 1;
        x1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        y1 += 1 as ::core::ffi::c_int;
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piBot += 1;
    }
    piTop = &raw mut (*prTop).used;
    prect = (*prTop).prect.offset(*piTop as isize) as *mut XRectangle;
    join1 = top_wid as ::core::ffi::c_int;
    join2 = bot_wid as ::core::ffi::c_int;
    x1 = x;
    y1 = y
        + (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
    len = height.wrapping_sub(
        (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    ) as ::core::ffi::c_int;
    count = left_wid.wrapping_sub(in_wid) as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = 1 as ::core::ffi::c_ushort;
        (*prect).height = len as ::core::ffi::c_ushort;
        x1 += 1 as ::core::ffi::c_int;
        join1 -= 1;
        y1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piTop += 1;
    }
    piBot = &raw mut (*prBot).used;
    prect = (*prBot).prect.offset(*piBot as isize) as *mut XRectangle;
    count = in_wid as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = 1 as ::core::ffi::c_ushort;
        (*prect).height = len as ::core::ffi::c_ushort;
        x1 += 1 as ::core::ffi::c_int;
        join1 -= 1;
        y1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piBot += 1;
    }
    piBot = &raw mut (*prBot).used;
    prect = (*prBot).prect.offset(*piBot as isize) as *mut XRectangle;
    join1 = left_wid as ::core::ffi::c_int;
    join2 = right_wid as ::core::ffi::c_int;
    x1 = x
        + (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
    y1 = (y as ::core::ffi::c_uint)
        .wrapping_add(height)
        .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    len = width.wrapping_sub(
        (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    ) as ::core::ffi::c_int;
    if (if join1 > 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) != 0
        && bot_wid > left_wid
    {
        len += 1;
        x1 -= 1;
        join1 += 1;
    }
    count = bot_wid.wrapping_sub(in_wid) as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = len as ::core::ffi::c_ushort;
        (*prect).height = 1 as ::core::ffi::c_ushort;
        join1 -= 1;
        x1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        y1 -= 1 as ::core::ffi::c_int;
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piBot += 1;
    }
    piTop = &raw mut (*prTop).used;
    prect = (*prTop).prect.offset(*piTop as isize) as *mut XRectangle;
    count = in_wid as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = len as ::core::ffi::c_ushort;
        (*prect).height = 1 as ::core::ffi::c_ushort;
        join1 -= 1;
        x1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        y1 -= 1 as ::core::ffi::c_int;
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piTop += 1;
    }
    piBot = &raw mut (*prBot).used;
    prect = (*prBot).prect.offset(*piBot as isize) as *mut XRectangle;
    join1 = top_wid as ::core::ffi::c_int;
    join2 = bot_wid as ::core::ffi::c_int;
    x1 = (x as ::core::ffi::c_uint)
        .wrapping_add(width)
        .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    y1 = y
        + (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
    len = height
        .wrapping_sub(
            (if join1 > 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        )
        .wrapping_sub(
            (if join2 > 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int;
    if (if join1 > 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) != 0
        && right_wid > top_wid
    {
        len += 1;
        y1 -= 1;
        join1 += 1;
    }
    count = right_wid.wrapping_sub(in_wid) as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = 1 as ::core::ffi::c_ushort;
        (*prect).height = len as ::core::ffi::c_ushort;
        x1 -= 1 as ::core::ffi::c_int;
        join1 -= 1;
        y1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piBot += 1;
    }
    piTop = &raw mut (*prTop).used;
    prect = (*prTop).prect.offset(*piTop as isize) as *mut XRectangle;
    count = in_wid as ::core::ffi::c_int;
    while count > 0 as ::core::ffi::c_int {
        (*prect).x = x1 as ::core::ffi::c_short;
        (*prect).y = y1 as ::core::ffi::c_short;
        (*prect).width = 1 as ::core::ffi::c_ushort;
        (*prect).height = len as ::core::ffi::c_ushort;
        x1 -= 1 as ::core::ffi::c_int;
        join1 -= 1;
        y1 += if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        join2 -= 1;
        len -= (if join1 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) + (if join2 > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        count -= 1;
        prect = prect.offset(1);
        *piTop += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn StretcherCorner(
    mut prTop: *mut RList,
    mut prBot: *mut RList,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut cnum: ::core::ffi::c_int,
    mut swidth: ::core::ffi::c_uint,
    mut cwidth: ::core::ffi::c_uint,
    mut cheight: ::core::ffi::c_uint,
) {
    let mut prect = ::core::ptr::null_mut::<XRectangle>();
    let mut piTop = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut piBot = ::core::ptr::null_mut::<::core::ffi::c_int>();
    match cnum {
        STRETCH_NORTH_WEST => {
            if (*prTop).used + 4 as ::core::ffi::c_int > (*prTop).allocated
                && ExtendRList(prTop, RLIST_EXTENSION_SIZE as ::core::ffi::c_uint) == 0
            {
                return;
            }
            piTop = &raw mut (*prTop).used;
            prect = (*prTop).prect.offset(*piTop as isize) as *mut XRectangle;
            (*prect).x = x as ::core::ffi::c_short;
            (*prect).y = y as ::core::ffi::c_short;
            (*prect).width = cwidth as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piTop += 1;
            (*prect).x = (x + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).y = (y + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).width = cwidth.wrapping_sub(2 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piTop += 1;
            (*prect).x = x as ::core::ffi::c_short;
            (*prect).y = (y + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height =
                cheight.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piTop += 1;
            (*prect).x = (x + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).y = (y + 2 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height =
                cheight.wrapping_sub(3 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            *piTop += 1;
            if (*prBot).used + 4 as ::core::ffi::c_int > (*prBot).allocated
                && ExtendRList(prBot, RLIST_EXTENSION_SIZE as ::core::ffi::c_uint) == 0
            {
                return;
            }
            piBot = &raw mut (*prBot).used;
            prect = (*prBot).prect.offset(*piBot as isize) as *mut XRectangle;
            (*prect).x = (x + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).y = (y as ::core::ffi::c_uint)
                .wrapping_add(cheight)
                .wrapping_sub(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).width = swidth.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piBot += 1;
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*prect).x = (x as ::core::ffi::c_uint)
                    .wrapping_add(swidth)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    as ::core::ffi::c_short;
                (*prect).y = (y as ::core::ffi::c_uint)
                    .wrapping_add(swidth)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    as ::core::ffi::c_short;
                (*prect).width = 1 as ::core::ffi::c_ushort;
                (*prect).height = cheight.wrapping_sub(swidth) as ::core::ffi::c_ushort;
                prect = prect.offset(1);
                *piBot += 1;
                (*prect).x =
                    (x as ::core::ffi::c_uint).wrapping_add(swidth) as ::core::ffi::c_short;
                (*prect).y = (y as ::core::ffi::c_uint)
                    .wrapping_add(swidth)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    as ::core::ffi::c_short;
                (*prect).width = cwidth.wrapping_sub(swidth) as ::core::ffi::c_ushort;
                (*prect).height = 1 as ::core::ffi::c_ushort;
                prect = prect.offset(1);
                *piBot += 1;
            }
            (*prect).x = (x as ::core::ffi::c_uint)
                .wrapping_add(cwidth)
                .wrapping_sub(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).y = (y + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height = swidth.wrapping_sub(1 as ::core::ffi::c_uint).wrapping_sub(
                (if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
            *piBot += 1;
        }
        STRETCH_NORTH_EAST => {
            if (*prTop).used + 4 as ::core::ffi::c_int > (*prTop).allocated
                && ExtendRList(prTop, RLIST_EXTENSION_SIZE as ::core::ffi::c_uint) == 0
            {
                return;
            }
            piTop = &raw mut (*prTop).used;
            prect = (*prTop).prect.offset(*piTop as isize) as *mut XRectangle;
            (*prect).x = x as ::core::ffi::c_short;
            (*prect).y = y as ::core::ffi::c_short;
            (*prect).width = cwidth as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piTop += 1;
            (*prect).x = (x + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).y = (y + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).width = cwidth.wrapping_sub(2 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piTop += 1;
            (*prect).x = x as ::core::ffi::c_short;
            (*prect).y = (y + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height =
                swidth.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piTop += 1;
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*prect).x = (x as ::core::ffi::c_uint)
                    .wrapping_add(cwidth)
                    .wrapping_sub(swidth) as ::core::ffi::c_short;
                (*prect).y =
                    (y as ::core::ffi::c_uint).wrapping_add(swidth) as ::core::ffi::c_short;
                (*prect).width = 1 as ::core::ffi::c_ushort;
                (*prect).height = cheight.wrapping_sub(swidth) as ::core::ffi::c_ushort;
                *piTop += 1;
            }
            if (*prBot).used + 4 as ::core::ffi::c_int > (*prBot).allocated
                && ExtendRList(prBot, RLIST_EXTENSION_SIZE as ::core::ffi::c_uint) == 0
            {
                return;
            }
            piBot = &raw mut (*prBot).used;
            prect = (*prBot).prect.offset(*piBot as isize) as *mut XRectangle;
            (*prect).x = (x as ::core::ffi::c_uint)
                .wrapping_add(cwidth)
                .wrapping_sub(swidth)
                .wrapping_add(
                    (if wmGD.frameStyle as ::core::ffi::c_uint
                        == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ) as ::core::ffi::c_short;
            (*prect).y = (y as ::core::ffi::c_uint)
                .wrapping_add(cheight)
                .wrapping_sub(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).width = swidth.wrapping_sub(
                (if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piBot += 1;
            (*prect).x = (x as ::core::ffi::c_uint)
                .wrapping_add(cwidth)
                .wrapping_sub(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).y = (y + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height =
                cheight.wrapping_sub(2 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piBot += 1;
            (*prect).x = (x as ::core::ffi::c_uint)
                .wrapping_add(cwidth)
                .wrapping_sub(2 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).y = (y + 2 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height =
                cheight.wrapping_sub(3 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piBot += 1;
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*prect).x = (x + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
                (*prect).y = (y as ::core::ffi::c_uint)
                    .wrapping_add(swidth)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    as ::core::ffi::c_short;
                (*prect).width = cwidth.wrapping_sub(swidth) as ::core::ffi::c_ushort;
                (*prect).height = 1 as ::core::ffi::c_ushort;
                *piBot += 1;
            }
        }
        STRETCH_SOUTH_EAST => {
            if (*prTop).used + 4 as ::core::ffi::c_int > (*prTop).allocated
                && ExtendRList(prTop, RLIST_EXTENSION_SIZE as ::core::ffi::c_uint) == 0
            {
                return;
            }
            piTop = &raw mut (*prTop).used;
            prect = (*prTop).prect.offset(*piTop as isize) as *mut XRectangle;
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*prect).x = x as ::core::ffi::c_short;
                (*prect).y = (y as ::core::ffi::c_uint)
                    .wrapping_add(cheight)
                    .wrapping_sub(swidth) as ::core::ffi::c_short;
                (*prect).width = cwidth
                    .wrapping_sub(swidth)
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    as ::core::ffi::c_ushort;
                (*prect).height = 1 as ::core::ffi::c_ushort;
                prect = prect.offset(1);
                *piTop += 1;
                (*prect).x = (x as ::core::ffi::c_uint)
                    .wrapping_add(cwidth)
                    .wrapping_sub(swidth) as ::core::ffi::c_short;
                (*prect).y = y as ::core::ffi::c_short;
                (*prect).width = 1 as ::core::ffi::c_ushort;
                (*prect).height = cheight.wrapping_sub(swidth) as ::core::ffi::c_ushort;
                prect = prect.offset(1);
                *piTop += 1;
            }
            (*prect).x = (x as ::core::ffi::c_uint)
                .wrapping_add(cwidth)
                .wrapping_sub(swidth)
                .wrapping_add(
                    (if wmGD.frameStyle as ::core::ffi::c_uint
                        == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ) as ::core::ffi::c_short;
            (*prect).y = y as ::core::ffi::c_short;
            (*prect).width = swidth.wrapping_sub(2 as ::core::ffi::c_uint).wrapping_add(
                (if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piTop += 1;
            (*prect).x = x as ::core::ffi::c_short;
            (*prect).y = (y as ::core::ffi::c_uint)
                .wrapping_add(cheight)
                .wrapping_sub(swidth)
                .wrapping_add(
                    (if wmGD.frameStyle as ::core::ffi::c_uint
                        == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ) as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height = swidth.wrapping_sub(2 as ::core::ffi::c_uint).wrapping_add(
                (if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
            *piTop += 1;
            if (*prBot).used + 4 as ::core::ffi::c_int > (*prBot).allocated
                && ExtendRList(prBot, RLIST_EXTENSION_SIZE as ::core::ffi::c_uint) == 0
            {
                return;
            }
            piBot = &raw mut (*prBot).used;
            prect = (*prBot).prect.offset(*piBot as isize) as *mut XRectangle;
            (*prect).x = (x + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).y = (y as ::core::ffi::c_uint)
                .wrapping_add(cheight)
                .wrapping_sub(2 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).width = cwidth.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piBot += 1;
            (*prect).x = x as ::core::ffi::c_short;
            (*prect).y = (y as ::core::ffi::c_uint)
                .wrapping_add(cheight)
                .wrapping_sub(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).width = cwidth as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piBot += 1;
            (*prect).x = (x as ::core::ffi::c_uint)
                .wrapping_add(cwidth)
                .wrapping_sub(2 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).y = (y + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height =
                cheight.wrapping_sub(3 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piBot += 1;
            (*prect).x = (x as ::core::ffi::c_uint)
                .wrapping_add(cwidth)
                .wrapping_sub(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).y = y as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height =
                cheight.wrapping_sub(2 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            *piBot += 1;
        }
        STRETCH_SOUTH_WEST => {
            if (*prTop).used + 4 as ::core::ffi::c_int > (*prTop).allocated
                && ExtendRList(prTop, RLIST_EXTENSION_SIZE as ::core::ffi::c_uint) == 0
            {
                return;
            }
            piTop = &raw mut (*prTop).used;
            prect = (*prTop).prect.offset(*piTop as isize) as *mut XRectangle;
            (*prect).x = x as ::core::ffi::c_short;
            (*prect).y = y as ::core::ffi::c_short;
            (*prect).width = swidth as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piTop += 1;
            (*prect).x = x as ::core::ffi::c_short;
            (*prect).y = (y + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height =
                cheight.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piTop += 1;
            (*prect).x = (x + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).y = (y + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height =
                cheight.wrapping_sub(2 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piTop += 1;
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*prect).x =
                    (x as ::core::ffi::c_uint).wrapping_add(swidth) as ::core::ffi::c_short;
                (*prect).y = (y as ::core::ffi::c_uint)
                    .wrapping_add(cheight)
                    .wrapping_sub(swidth) as ::core::ffi::c_short;
                (*prect).width = cwidth.wrapping_sub(swidth) as ::core::ffi::c_ushort;
                (*prect).height = 1 as ::core::ffi::c_ushort;
                *piTop += 1;
            }
            if (*prBot).used + 4 as ::core::ffi::c_int > (*prBot).allocated
                && ExtendRList(prBot, RLIST_EXTENSION_SIZE as ::core::ffi::c_uint) == 0
            {
                return;
            }
            piBot = &raw mut (*prBot).used;
            prect = (*prBot).prect.offset(*piBot as isize) as *mut XRectangle;
            if wmGD.frameStyle as ::core::ffi::c_uint
                == WmRECESSED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*prect).x = (x as ::core::ffi::c_uint)
                    .wrapping_add(swidth)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    as ::core::ffi::c_short;
                (*prect).y = (y + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
                (*prect).width = 1 as ::core::ffi::c_ushort;
                (*prect).height = cheight.wrapping_sub(swidth) as ::core::ffi::c_ushort;
                prect = prect.offset(1);
                *piBot += 1;
            }
            (*prect).x = (x as ::core::ffi::c_uint)
                .wrapping_add(cwidth)
                .wrapping_sub(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).y = (y as ::core::ffi::c_uint)
                .wrapping_add(cheight)
                .wrapping_sub(swidth)
                .wrapping_add(
                    (if wmGD.frameStyle as ::core::ffi::c_uint
                        == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ) as ::core::ffi::c_short;
            (*prect).width = 1 as ::core::ffi::c_ushort;
            (*prect).height = swidth.wrapping_sub(
                (if wmGD.frameStyle as ::core::ffi::c_uint
                    == WmSLAB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piBot += 1;
            (*prect).x = (x + 2 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).y = (y as ::core::ffi::c_uint)
                .wrapping_add(cheight)
                .wrapping_sub(2 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).width = cwidth.wrapping_sub(3 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            prect = prect.offset(1);
            *piBot += 1;
            (*prect).x = (x + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
            (*prect).y = (y as ::core::ffi::c_uint)
                .wrapping_add(cheight)
                .wrapping_sub(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_short;
            (*prect).width = cwidth.wrapping_sub(2 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
            (*prect).height = 1 as ::core::ffi::c_ushort;
            *piBot += 1;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawStringInBox(
    mut dpy: *mut Display,
    mut win: Window,
    mut gc: GC,
    mut pfs: *mut XFontStruct,
    mut pbox: *mut XRectangle,
    mut str: String_0,
) {
    let mut gcv = XGCValues {
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
    let mut textWidth: ::core::ffi::c_int = 0;
    let mut xCenter: ::core::ffi::c_int = 0;
    let mut clipBox = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    textWidth = XTextWidth(
        pfs,
        str as *const ::core::ffi::c_char,
        strlen(str as *const ::core::ffi::c_char) as ::core::ffi::c_int,
    );
    if textWidth < (*pbox).width as ::core::ffi::c_int {
        xCenter = (*pbox).x as ::core::ffi::c_int
            + ((*pbox).width as ::core::ffi::c_int - textWidth) / 2 as ::core::ffi::c_int;
        WmDrawString(
            dpy,
            win as Drawable,
            gc,
            xCenter,
            (*pbox).y as ::core::ffi::c_int + (*pfs).ascent,
            str as *mut ::core::ffi::c_char,
            strlen(str as *const ::core::ffi::c_char) as ::core::ffi::c_uint,
        );
    } else {
        clipBox.x = 0 as ::core::ffi::c_short;
        clipBox.y = 0 as ::core::ffi::c_short;
        clipBox.width = (*pbox).width;
        clipBox.height = (*pbox).height;
        XSetClipRectangles(
            dpy,
            gc,
            (*pbox).x as ::core::ffi::c_int,
            (*pbox).y as ::core::ffi::c_int,
            &raw mut clipBox,
            1 as ::core::ffi::c_int,
            Unsorted,
        );
        WmDrawString(
            dpy,
            win as Drawable,
            gc,
            (*pbox).x as ::core::ffi::c_int,
            (*pbox).y as ::core::ffi::c_int + (*pfs).ascent,
            str as *mut ::core::ffi::c_char,
            strlen(str as *const ::core::ffi::c_char) as ::core::ffi::c_uint,
        );
        gcv.clip_x_origin = 0 as ::core::ffi::c_int;
        gcv.clip_y_origin = 0 as ::core::ffi::c_int;
        gcv.clip_mask = None_0 as Pixmap;
        XChangeGC(
            dpy,
            gc,
            (GCClipXOrigin | GCClipYOrigin | GCClipMask) as ::core::ffi::c_ulong,
            &raw mut gcv,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ExtendRList(mut prl: *mut RList, mut amt: ::core::ffi::c_uint) -> Boolean {
    let mut total: ::core::ffi::c_uint = 0;
    let mut count: ::core::ffi::c_uint = 0;
    let mut pNewRect = ::core::ptr::null_mut::<XRectangle>();
    let mut rval: Boolean = 0;
    total = ((*prl).allocated as ::core::ffi::c_uint).wrapping_add(amt);
    pNewRect = XtMalloc(
        (total as usize).wrapping_mul(::core::mem::size_of::<XRectangle>() as usize) as Cardinal,
    ) as *mut XRectangle;
    if pNewRect.is_null() {
        Warning(
            b"Insufficient memory for graphics data\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        rval = False as Boolean;
    } else {
        (*prl).allocated = total as ::core::ffi::c_int;
        rval = True as Boolean;
        if (*prl).used != 0 as ::core::ffi::c_int {
            count = ((*prl).used as usize)
                .wrapping_mul(::core::mem::size_of::<XRectangle>() as usize)
                as ::core::ffi::c_uint;
            memcpy(
                pNewRect as *mut ::core::ffi::c_void,
                (*prl).prect as *mut ::core::ffi::c_void,
                count as size_t,
            );
            if !(*prl).prect.is_null() {
                XtFree((*prl).prect as *mut ::core::ffi::c_char);
            }
            (*prl).prect = pNewRect;
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn AllocateRList(mut amt: ::core::ffi::c_uint) -> *mut RList {
    let mut prl = ::core::ptr::null_mut::<RList>();
    prl = XtMalloc(::core::mem::size_of::<RList>() as Cardinal) as *mut RList;
    if !prl.is_null() {
        (*prl).prect = XtMalloc(
            (amt as usize).wrapping_mul(::core::mem::size_of::<XRectangle>() as usize) as Cardinal,
        ) as *mut XRectangle;
        if (*prl).prect.is_null() {
            XtFree(prl as *mut ::core::ffi::c_char);
            prl = ::core::ptr::null_mut::<RList>();
        } else {
            (*prl).allocated = amt as ::core::ffi::c_int;
            (*prl).used = 0 as ::core::ffi::c_int;
        }
    }
    return prl;
}
#[no_mangle]
pub unsafe extern "C" fn FreeRList(mut prl: *mut RList) {
    if !prl.is_null() {
        if !(*prl).prect.is_null() {
            XtFree((*prl).prect as *mut ::core::ffi::c_char);
        }
        XtFree(prl as *mut ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WmDrawString(
    mut dpy: *mut Display,
    mut d: Drawable,
    mut gc: GC,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut string: *mut ::core::ffi::c_char,
    mut length: ::core::ffi::c_uint,
) {
    if (*wmGD.pActiveSD).cleanText != 0 {
        XDrawImageString(dpy, d, gc, x, y, string, length as ::core::ffi::c_int);
    } else {
        XDrawString(dpy, d, gc, x, y, string, length as ::core::ffi::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn WmDrawXmString(
    mut dpy: *mut Display,
    mut w: Window,
    mut xmrendertable: XmRenderTable,
    mut xmstring: XmString,
    mut gc: GC,
    mut x: Position,
    mut y: Position,
    mut width: Dimension,
    mut pbox: *mut XRectangle,
    mut bCenter: Boolean,
) {
    let mut textWidth: Dimension = 0;
    let mut alignment: ::core::ffi::c_int = 0;
    textWidth = XmStringWidth(xmrendertable as XmFontList, xmstring);
    alignment = if bCenter as ::core::ffi::c_int != 0 {
        XmALIGNMENT_CENTER as ::core::ffi::c_int
    } else {
        XmALIGNMENT_BEGINNING as ::core::ffi::c_int
    };
    if textWidth as ::core::ffi::c_int >= (*pbox).width as ::core::ffi::c_int {
        alignment = XmALIGNMENT_BEGINNING as ::core::ffi::c_int;
    }
    if (*wmGD.pActiveSD).cleanText != 0 {
        XmStringDrawImage(
            dpy,
            w,
            xmrendertable as XmFontList,
            xmstring,
            gc,
            x,
            y,
            width,
            alignment as ::core::ffi::c_uchar,
            XmSTRING_DIRECTION_L_TO_R as ::core::ffi::c_int as ::core::ffi::c_uchar,
            pbox,
        );
    } else {
        XmStringDraw(
            dpy,
            w,
            xmrendertable as XmFontList,
            xmstring,
            gc,
            x,
            y,
            width,
            alignment as ::core::ffi::c_uchar,
            XmSTRING_DIRECTION_L_TO_R as ::core::ffi::c_int as ::core::ffi::c_uchar,
            pbox,
        );
    }
    XSetClipMask(dpy, gc, None_0 as Pixmap);
}
#[no_mangle]
pub unsafe extern "C" fn WmInstallBitmapIntoXmCache(
    mut pchName: *mut ::core::ffi::c_uchar,
    mut bitmap: Pixmap,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
) {
    let mut pImage = ::core::ptr::null_mut::<XImage>();
    pImage = XGetImage(
        wmGD.display,
        bitmap as Drawable,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        width,
        height,
        1 as ::core::ffi::c_ulong,
        XYBitmap,
    );
    XmInstallImage(pImage, pchName as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn WmInstallBitmapDataIntoXmCache(
    mut pSD: *mut WmScreenData,
    mut pchName: *mut ::core::ffi::c_uchar,
    mut pData: *mut ::core::ffi::c_char,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
) {
    let mut pImage = ::core::ptr::null_mut::<XImage>();
    pImage = XtMalloc(::core::mem::size_of::<XImage>() as Cardinal) as *mut XImage;
    if !pImage.is_null() {
        (*pImage).width = width as ::core::ffi::c_int;
        (*pImage).height = height as ::core::ffi::c_int;
        (*pImage).xoffset = 0 as ::core::ffi::c_int;
        (*pImage).data = pData;
        (*pImage).format = XYBitmap;
        (*pImage).byte_order = MSBFirst;
        (*pImage).bitmap_pad = 8 as ::core::ffi::c_int;
        (*pImage).bitmap_bit_order = LSBFirst;
        (*pImage).bitmap_unit = 8 as ::core::ffi::c_int;
        (*pImage).depth = 1 as ::core::ffi::c_int;
        (*pImage).bytes_per_line = width.wrapping_div(8 as ::core::ffi::c_uint).wrapping_add(
            (if width.wrapping_rem(8 as ::core::ffi::c_uint) != 0 as ::core::ffi::c_uint {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int;
        (*pImage).obdata = ::core::ptr::null_mut::<::core::ffi::c_char>();
        XmInstallImage(pImage, pchName as *mut ::core::ffi::c_char);
    }
}
