// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      app-config.c
 *
 *      Copyright 2010 PCMan <pcman.tw@gmail.com>
 *      Copyright 2012-2014 Andriy Grytsenko (LStranger) <andrej@rep.kiev.ua>
 *
 *      This program is free software; you can redistribute it and/or modify
 *      it under the terms of the GNU General Public License as published by
 *      the Free Software Foundation; either version 2 of the License, or
 *      (at your option) any later version.
 *
 *      This program is distributed in the hope that it will be useful,
 *      but WITHOUT ANY WARRANTY; without even the implied warranty of
 *      MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *      GNU General Public License for more details.
 *
 *      You should have received a copy of the GNU General Public License
 *      along with this program; if not, write to the Free Software
 *      Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 *      MA 02110-1301, USA.
 */

unsafe extern "C" fn _parse_sort(
    mut kf: *mut GKeyFile,
    mut group: *const c_char,
    mut mode: *mut FmSortMode,
    mut col: *mut FmFolderModelCol,
) {
    let mut tmp_int: c_int = 0;
    let mut sort: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
    sort = g_key_file_get_string_list(
        kf,
        group as *const gchar,
        b"sort\0" as *const u8 as *const gchar,
        ptr::null_mut::<gsize>(),
        ptr::null_mut::<*mut GError>(),
    ) as *mut *mut c_char;
    if !sort.is_null() {
        let mut tmp_mode: FmSortMode = FM_SORT_ASCENDING;
        let mut tmp_col: FmFolderModelCol = 4294967295 as FmFolderModelCol;
        tmp_int = 0i32;
        while !(*sort.offset(tmp_int as isize)).is_null() {
            if tmp_int == 0i32 {
                tmp_col = fm_folder_model_get_col_by_name(*sort.offset(tmp_int as isize));
            } else if strcmp(*sort.offset(tmp_int as isize), c"ascending".as_ptr()) == 0i32 {
                tmp_mode = (tmp_mode as c_uint & !(FM_SORT_ORDER_MASK as c_int) as c_uint
                    | FM_SORT_ASCENDING as c_int as c_uint)
                    as FmSortMode;
            } else if strcmp(*sort.offset(tmp_int as isize), c"descending".as_ptr()) == 0i32 {
                tmp_mode = (tmp_mode as c_uint & !(FM_SORT_ORDER_MASK as c_int) as c_uint
                    | FM_SORT_DESCENDING as c_int as c_uint)
                    as FmSortMode;
            } else if strcmp(*sort.offset(tmp_int as isize), c"case".as_ptr()) == 0i32 {
                tmp_mode = mem::transmute::<c_uint, FmSortMode>(
                    tmp_mode as c_uint | FM_SORT_CASE_SENSITIVE as c_int as c_uint,
                );
            } else if strcmp(*sort.offset(tmp_int as isize), c"mingle".as_ptr()) == 0i32 {
                tmp_mode = mem::transmute::<c_uint, FmSortMode>(
                    tmp_mode as c_uint | FM_SORT_NO_FOLDER_FIRST as c_int as c_uint,
                );
            }
            tmp_int += 1;
        }
        *mode = tmp_mode;
        if tmp_col as c_uint != 4294967295 as FmFolderModelCol as c_uint {
            *col = tmp_col;
        }
        g_strfreev(sort as *mut *mut gchar);
        return;
    }
    if fm_key_file_get_int(kf, group, c"sort_type".as_ptr(), &raw mut tmp_int) != 0
        && tmp_int == GTK_SORT_DESCENDING as c_int
    {
        *mode = FM_SORT_DESCENDING;
    } else {
        *mode = FM_SORT_ASCENDING;
    }
    if fm_key_file_get_int(kf, group, c"sort_by".as_ptr(), &raw mut tmp_int) != 0
        && fm_folder_model_col_is_valid(tmp_int as guint as FmFolderModelCol) != 0
    {
        *col = tmp_int as FmFolderModelCol;
    }
}
unsafe extern "C" fn _save_sort(
    mut buf: *mut GString,
    mut mode: FmSortMode,
    mut col: FmFolderModelCol,
) {
    let mut name: *const c_char = fm_folder_model_col_get_name(col);
    if name.is_null() {
        name = fm_folder_model_col_get_name(FM_FOLDER_MODEL_COL_NAME);
    }
    g_string_append_printf(
        buf,
        b"sort=%s;%s;\0" as *const u8 as *const gchar,
        name,
        if mode as c_uint & FM_SORT_ORDER_MASK as c_int as c_uint
            == FM_SORT_ASCENDING as c_int as c_uint
        {
            c"ascending".as_ptr()
        } else {
            c"descending".as_ptr()
        },
    );
    if mode as c_uint & FM_SORT_CASE_SENSITIVE as c_int as c_uint != 0 {
        if 0 != 0 {
            ({
                let __val: *const c_char = c"case;".as_ptr();
                g_string_append_len_inline(
                    buf,
                    __val,
                    if ({
                        let mut _g_boolean_var_10: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_10 = 1i32;
                        }
                        _g_boolean_var_10
                    }) as c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as isize)) as gssize
                    } else {
                        -(1i32) as gssize
                    },
                );
            });
        } else {
            g_string_append_len_inline(buf, c"case;".as_ptr(), -(1i32) as gssize);
        };
    }
    if mode as c_uint & FM_SORT_NO_FOLDER_FIRST as c_int as c_uint != 0 {
        if 0 != 0 {
            ({
                let __val: *const c_char = c"mingle;".as_ptr();
                g_string_append_len_inline(
                    buf,
                    __val,
                    if ({
                        let mut _g_boolean_var_11: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_11 = 1i32;
                        }
                        _g_boolean_var_11
                    }) as c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as isize)) as gssize
                    } else {
                        -(1i32) as gssize
                    },
                );
            });
        } else {
            g_string_append_len_inline(buf, c"mingle;".as_ptr(), -(1i32) as gssize);
        };
    }
    g_string_append_c_inline(buf, '\n' as i32 as gchar);
}
unsafe extern "C" fn _parse_config_for_path(
    mut fc: *mut FmFolderConfig,
    mut mode: *mut FmSortMode,
    mut by: *mut FmFolderModelCol,
    mut view_mode: *mut FmStandardViewMode,
    mut show_hidden: *mut gboolean,
    mut columns: *mut *mut *mut c_char,
) -> gboolean {
    let mut tmp_int: c_int = 0;
    let mut tmp: *mut c_char = ptr::null_mut::<c_char>();
    let mut sort: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
    sort = fm_folder_config_get_string_list(fc, c"Sort".as_ptr(), ptr::null_mut::<gsize>());
    if !sort.is_null() {
        let mut tmp_mode: FmSortMode = FM_SORT_ASCENDING;
        let mut tmp_col: FmFolderModelCol = 4294967295 as FmFolderModelCol;
        tmp_int = 0i32;
        while !(*sort.offset(tmp_int as isize)).is_null() {
            if tmp_int == 0i32 {
                tmp_col = fm_folder_model_get_col_by_name(*sort.offset(tmp_int as isize));
            } else if strcmp(*sort.offset(tmp_int as isize), c"ascending".as_ptr()) == 0i32 {
                tmp_mode = (tmp_mode as c_uint & !(FM_SORT_ORDER_MASK as c_int) as c_uint
                    | FM_SORT_ASCENDING as c_int as c_uint)
                    as FmSortMode;
            } else if strcmp(*sort.offset(tmp_int as isize), c"descending".as_ptr()) == 0i32 {
                tmp_mode = (tmp_mode as c_uint & !(FM_SORT_ORDER_MASK as c_int) as c_uint
                    | FM_SORT_DESCENDING as c_int as c_uint)
                    as FmSortMode;
            } else if strcmp(*sort.offset(tmp_int as isize), c"case".as_ptr()) == 0i32 {
                tmp_mode = mem::transmute::<c_uint, FmSortMode>(
                    tmp_mode as c_uint | FM_SORT_CASE_SENSITIVE as c_int as c_uint,
                );
            } else if strcmp(*sort.offset(tmp_int as isize), c"mingle".as_ptr()) == 0i32 {
                tmp_mode = mem::transmute::<c_uint, FmSortMode>(
                    tmp_mode as c_uint | FM_SORT_NO_FOLDER_FIRST as c_int as c_uint,
                );
            }
            tmp_int += 1;
        }
        *mode = tmp_mode;
        if tmp_col as c_uint != 4294967295 as FmFolderModelCol as c_uint {
            *by = tmp_col;
        }
        g_strfreev(sort as *mut *mut gchar);
    } else {
        if fm_folder_config_get_integer(fc, c"sort_type".as_ptr(), &raw mut tmp_int) != 0
            && tmp_int == GTK_SORT_DESCENDING as c_int
        {
            *mode = FM_SORT_DESCENDING;
        } else {
            *mode = FM_SORT_ASCENDING;
        }
        if fm_folder_config_get_integer(fc, c"sort_by".as_ptr(), &raw mut tmp_int) != 0
            && fm_folder_model_col_is_valid(tmp_int as guint as FmFolderModelCol) != 0
        {
            *by = tmp_int as FmFolderModelCol;
        }
    }
    if !view_mode.is_null() && {
        tmp = fm_folder_config_get_string(fc, c"ViewMode".as_ptr());
        !tmp.is_null()
    } {
        *view_mode = fm_standard_view_mode_from_str(tmp);
        g_free(tmp as gpointer);
    }
    if !show_hidden.is_null() {
        fm_folder_config_get_boolean(fc, c"ShowHidden".as_ptr(), show_hidden);
    }
    if !columns.is_null() {
        *columns =
            fm_folder_config_get_string_list(fc, c"Columns".as_ptr(), ptr::null_mut::<gsize>());
    }
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn fm_app_config_class_intern_init(mut klass: gpointer) {
    fm_app_config_parent_class = g_type_class_peek_parent(klass);
    if FmAppConfig_private_offset != 0i32 {
        g_type_class_adjust_private_offset(klass, &raw mut FmAppConfig_private_offset);
    }
    fm_app_config_class_init(klass as *mut FmAppConfigClass);
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_config_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0i32 != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval =
                std::sync::atomic::AtomicUsize::from_ptr(gapg_temp_atomic.cast())
                    .load(std::sync::atomic::Ordering::SeqCst) as _;
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(&raw mut static_g_define_type_id as *mut c_void) != 0)
            as c_int
    }) != 0
    {
        let mut g_define_type_id: GType = fm_app_config_get_type_once();
        if 0i32 != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn fm_app_config_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fm_config_get_type(),
        g_intern_static_string(b"FmAppConfig\0" as *const u8 as *const gchar),
        mem::size_of::<FmAppConfigClass>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(mem::transmute::<
            Option<unsafe extern "C" fn(gpointer) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            fm_app_config_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
        ))),
        mem::size_of::<FmAppConfig>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmAppConfig) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                fm_app_config_init as unsafe extern "C" fn(*mut FmAppConfig) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut FmAppConfig_private_offset: gint = 0;
static mut fm_app_config_parent_class: gpointer = ptr::null::<c_void>() as *mut c_void;
unsafe extern "C" fn fm_app_config_class_init(mut klass: *mut FmAppConfigClass) {
    let mut g_object_class: *mut GObjectClass = ptr::null_mut::<GObjectClass>();
    g_object_class = klass as *mut c_void as *mut GObjectClass;
    (*g_object_class).finalize =
        Some(fm_app_config_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn fm_app_config_finalize(mut object: *mut GObject) {
    let mut cfg: *mut FmAppConfig = ptr::null_mut::<FmAppConfig>();
    cfg = object as *mut FmAppConfig;
    g_strfreev((*cfg).columns as *mut *mut gchar);
    if (*cfg).desktop_section.configured() != 0 {
        if (*cfg).desktop_section.wallpapers_configured > 0i32 {
            let mut i: c_int = 0;
            i = 0i32;
            while i < (*cfg).desktop_section.wallpapers_configured {
                g_free(*(*cfg).desktop_section.wallpapers.offset(i as isize) as gpointer);
                i += 1;
            }
            g_free((*cfg).desktop_section.wallpapers as gpointer);
        }
        g_free((*cfg).desktop_section.wallpaper as gpointer);
        g_free((*cfg).desktop_section.desktop_font as gpointer);
        g_free((*cfg).desktop_section.folder as gpointer);
    }
    g_hash_table_unref((*cfg).autorun_choices);
    g_free((*cfg).home_path as gpointer);
    (*(fm_app_config_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn _free_archoice(mut data: gpointer) {
    let mut choice: *mut FmAutorunChoice = data as *mut FmAutorunChoice;
    g_free((*choice).last_used as gpointer);
    g_slice_free1(
        mem::size_of::<FmAutorunChoice>() as gsize,
        choice as gpointer,
    );
}
unsafe extern "C" fn fm_app_config_init(mut cfg: *mut FmAppConfig) {
    fm_config_load_from_file(cfg as *mut FmConfig, ptr::null::<c_char>());
    (*cfg).bm_open_method = FM_OPEN_IN_CURRENT_TAB as c_int;
    (*cfg).mount_on_startup = (0i32 == 0) as c_int as gboolean;
    (*cfg).mount_removable = (0i32 == 0) as c_int as gboolean;
    (*cfg).autorun = (0i32 == 0) as c_int as gboolean;
    (*cfg).desktop_section.desktop_fg.blue = 65535 as guint16;
    (*cfg).desktop_section.desktop_fg.green = (*cfg).desktop_section.desktop_fg.blue;
    (*cfg).desktop_section.desktop_fg.red = (*cfg).desktop_section.desktop_fg.green;
    (*cfg).win_width = 640i32;
    (*cfg).win_height = 480i32;
    (*cfg).splitter_pos = 150i32;
    (*cfg).max_tab_chars = 32 as c_int;
    (*cfg).media_in_new_tab = 0 as gboolean;
    (*cfg).desktop_folder_new_win = 0 as gboolean;
    (*cfg).side_pane_mode = FM_SP_PLACES;
    (*cfg).view_mode = FM_FV_ICON_VIEW as c_int as guint;
    (*cfg).show_hidden = 0 as gboolean;
    (*cfg).sort_type = FM_SORT_ASCENDING;
    (*cfg).sort_by = FM_FOLDER_MODEL_COL_NAME;
    (*cfg).desktop_section.desktop_sort_type =
        (FM_SORT_ASCENDING as c_int | FM_SORT_NO_FOLDER_FIRST as c_int) as FmSortMode;
    (*cfg).desktop_section.desktop_sort_by = FM_FOLDER_MODEL_COL_MTIME;
    (*cfg).desktop_section.wallpaper_common = (0i32 == 0) as c_int as gboolean;
    (*cfg).desktop_section.show_documents = 0 as gboolean;
    (*cfg).desktop_section.show_trash = (0i32 == 0) as c_int as gboolean;
    (*cfg).desktop_section.show_mounts = 0 as gboolean;
    (*cfg).desktop_section.folder = ptr::null_mut::<c_char>();
    let mut rhs_1 = {
        let mut rhs_0 = {
            let mut rhs = {
                (*cfg)
                    .tb
                    .set_home((0i32 == 0) as c_int as gboolean as gboolean);
                (*cfg).tb.home()
            } as gboolean;
            (*cfg).tb.set_nav(rhs);
            (*cfg).tb.nav()
        } as gboolean;
        (*cfg).tb.set_new_tab(rhs_0);
        (*cfg).tb.new_tab()
    } as gboolean;
    (*cfg).tb.set_visible(rhs_1);
    (*cfg).tb.set_new_win(0 as gboolean as gboolean);
    (*cfg).autorun_choices = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(_free_archoice as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*cfg).show_statusbar = (0i32 == 0) as c_int as gboolean;
    (*cfg).home_path = ptr::null_mut::<c_char>();
    (*cfg).focus_previous = 0 as gboolean;
    (*cfg).change_tab_on_drop = (0i32 == 0) as c_int as gboolean;
    (*cfg).close_on_unmount = (0i32 == 0) as c_int as gboolean;
    (*cfg).maximized = 0 as gboolean;
    (*cfg).pathbar_mode_buttons = 0 as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_config_new() -> *mut FmConfig {
    return g_object_new(fm_app_config_get_type(), ptr::null::<gchar>()) as *mut FmConfig;
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_config_load_desktop_config(
    mut kf: *mut GKeyFile,
    mut group: *const c_char,
    mut cfg: *mut FmDesktopConfig,
) {
    let mut tmp: *mut c_char = ptr::null_mut::<c_char>();
    if g_key_file_has_group(kf, group as *const gchar) == 0 {
        return;
    }
    (*cfg).desktop_fg.blue = 65535 as guint16;
    (*cfg).desktop_fg.green = (*cfg).desktop_fg.blue;
    (*cfg).desktop_fg.red = (*cfg).desktop_fg.green;
    (*cfg).desktop_sort_type =
        (FM_SORT_ASCENDING as c_int | FM_SORT_NO_FOLDER_FIRST as c_int) as FmSortMode;
    (*cfg).desktop_sort_by = FM_FOLDER_MODEL_COL_MTIME;
    (*cfg).wallpaper_common = (0i32 == 0) as c_int as gboolean;
    (*cfg).show_trash = (0i32 == 0) as c_int as gboolean;
    (*cfg).set_configured((0i32 == 0) as c_int as gint as gint);
    tmp = g_key_file_get_string(
        kf,
        group as *const gchar,
        b"wallpaper_mode\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    ) as *mut c_char;
    if !tmp.is_null() {
        if *tmp.offset(0 as isize) as c_int >= '0' as i32
            && *tmp.offset(0 as isize) as c_int <= '4' as i32
        {
            (*cfg).wallpaper_mode =
                (*tmp.offset(0 as isize) as c_int - '0' as i32) as FmWallpaperMode;
        } else {
            (*cfg).wallpaper_mode = fm_app_wallpaper_get_mode_by_name(tmp);
        }
        if (*cfg).wallpaper_mode as c_uint == 4294967295 as FmWallpaperMode as c_uint {
            (*cfg).wallpaper_mode = FM_WP_COLOR;
        }
        g_free(tmp as gpointer);
    }
    if (*cfg).wallpapers_configured > 0i32 {
        let mut i: c_int = 0;
        i = 0i32;
        while i < (*cfg).wallpapers_configured {
            g_free(*(*cfg).wallpapers.offset(i as isize) as gpointer);
            i += 1;
        }
        g_free((*cfg).wallpapers as gpointer);
    }
    g_free((*cfg).wallpaper as gpointer);
    (*cfg).wallpaper = ptr::null_mut::<c_char>();
    fm_key_file_get_int(
        kf,
        group,
        c"wallpapers_configured".as_ptr(),
        &raw mut (*cfg).wallpapers_configured,
    );
    if (*cfg).wallpapers_configured > 0i32 {
        let mut wpn_buf: [c_char; 32] = [0; 32];
        let mut i_0: c_int = 0;
        (*cfg).wallpapers = g_malloc(
            ((*cfg).wallpapers_configured as gsize)
                .wrapping_mul(mem::size_of::<*mut c_char>() as gsize),
        ) as *mut *mut c_char;
        i_0 = 0i32;
        while i_0 < (*cfg).wallpapers_configured {
            snprintf(
                &raw mut wpn_buf as *mut c_char,
                mem::size_of::<[c_char; 32]>() as size_t,
                c"wallpaper%d".as_ptr(),
                i_0,
            );
            tmp = g_key_file_get_string(
                kf,
                group as *const gchar,
                &raw mut wpn_buf as *mut c_char,
                ptr::null_mut::<*mut GError>(),
            ) as *mut c_char;
            let ref mut fresh12 = *(*cfg).wallpapers.offset(i_0 as isize);
            *fresh12 = tmp;
            i_0 += 1;
        }
    }
    fm_key_file_get_bool(
        kf,
        group,
        c"wallpaper_common".as_ptr(),
        &raw mut (*cfg).wallpaper_common,
    );
    if (*cfg).wallpaper_common != 0 {
        tmp = g_key_file_get_string(
            kf,
            group as *const gchar,
            b"wallpaper\0" as *const u8 as *const gchar,
            ptr::null_mut::<*mut GError>(),
        ) as *mut c_char;
        if !tmp.is_null() && *tmp.offset(0 as isize) as c_int == 0 {
            g_free(tmp as gpointer);
            tmp = ptr::null_mut::<c_char>();
        }
        g_free((*cfg).wallpaper as gpointer);
        (*cfg).wallpaper = tmp;
    }
    tmp = g_key_file_get_string(
        kf,
        group as *const gchar,
        b"desktop_bg\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    ) as *mut c_char;
    if !tmp.is_null() {
        gdk_color_parse(tmp, &raw mut (*cfg).desktop_bg);
        g_free(tmp as gpointer);
    }
    tmp = g_key_file_get_string(
        kf,
        group as *const gchar,
        b"desktop_fg\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    ) as *mut c_char;
    if !tmp.is_null() {
        gdk_color_parse(tmp, &raw mut (*cfg).desktop_fg);
        g_free(tmp as gpointer);
    }
    tmp = g_key_file_get_string(
        kf,
        group as *const gchar,
        b"desktop_shadow\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    ) as *mut c_char;
    if !tmp.is_null() {
        gdk_color_parse(tmp, &raw mut (*cfg).desktop_shadow);
        g_free(tmp as gpointer);
    }
    tmp = g_key_file_get_string(
        kf,
        group as *const gchar,
        b"desktop_font\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    ) as *mut c_char;
    g_free((*cfg).desktop_font as gpointer);
    (*cfg).desktop_font = tmp;
    g_free((*cfg).folder as gpointer);
    (*cfg).folder = g_key_file_get_string(
        kf,
        group as *const gchar,
        b"folder\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    ) as *mut c_char;
    fm_key_file_get_bool(
        kf,
        group,
        c"show_wm_menu".as_ptr(),
        &raw mut (*cfg).show_wm_menu,
    );
    _parse_sort(
        kf,
        group,
        &raw mut (*cfg).desktop_sort_type,
        &raw mut (*cfg).desktop_sort_by,
    );
    fm_key_file_get_bool(
        kf,
        group,
        c"show_documents".as_ptr(),
        &raw mut (*cfg).show_documents,
    );
    fm_key_file_get_bool(
        kf,
        group,
        c"show_trash".as_ptr(),
        &raw mut (*cfg).show_trash,
    );
    fm_key_file_get_bool(
        kf,
        group,
        c"show_mounts".as_ptr(),
        &raw mut (*cfg).show_mounts,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_config_load_from_key_file(
    mut cfg: *mut FmAppConfig,
    mut kf: *mut GKeyFile,
) {
    let mut tmp: *mut c_char = ptr::null_mut::<c_char>();
    let mut tmpv: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
    let mut tmp_int: c_int = 0;
    let mut i: c_int = 0;
    fm_key_file_get_int(
        kf,
        c"config".as_ptr(),
        c"bm_open_method".as_ptr(),
        &raw mut (*cfg).bm_open_method,
    );
    g_free((*cfg).home_path as gpointer);
    (*cfg).home_path = g_key_file_get_string(
        kf,
        b"config\0" as *const u8 as *const gchar,
        b"home_path\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    ) as *mut c_char;
    fm_key_file_get_bool(
        kf,
        c"volume".as_ptr(),
        c"mount_on_startup".as_ptr(),
        &raw mut (*cfg).mount_on_startup,
    );
    fm_key_file_get_bool(
        kf,
        c"volume".as_ptr(),
        c"mount_removable".as_ptr(),
        &raw mut (*cfg).mount_removable,
    );
    fm_key_file_get_bool(
        kf,
        c"volume".as_ptr(),
        c"autorun".as_ptr(),
        &raw mut (*cfg).autorun,
    );
    fm_app_config_load_desktop_config(kf, c"desktop".as_ptr(), &raw mut (*cfg).desktop_section);
    fm_key_file_get_bool(
        kf,
        c"ui".as_ptr(),
        c"always_show_tabs".as_ptr(),
        &raw mut (*cfg).always_show_tabs,
    );
    fm_key_file_get_int(
        kf,
        c"ui".as_ptr(),
        c"hide_close_btn".as_ptr(),
        &raw mut (*cfg).hide_close_btn,
    );
    fm_key_file_get_int(
        kf,
        c"ui".as_ptr(),
        c"max_tab_chars".as_ptr(),
        &raw mut (*cfg).max_tab_chars,
    );
    fm_key_file_get_int(
        kf,
        c"ui".as_ptr(),
        c"win_width".as_ptr(),
        &raw mut (*cfg).win_width,
    );
    fm_key_file_get_int(
        kf,
        c"ui".as_ptr(),
        c"win_height".as_ptr(),
        &raw mut (*cfg).win_height,
    );
    fm_key_file_get_bool(
        kf,
        c"ui".as_ptr(),
        c"maximized".as_ptr(),
        &raw mut (*cfg).maximized,
    );
    fm_key_file_get_int(
        kf,
        c"ui".as_ptr(),
        c"splitter_pos".as_ptr(),
        &raw mut (*cfg).splitter_pos,
    );
    fm_key_file_get_bool(
        kf,
        c"ui".as_ptr(),
        c"media_in_new_tab".as_ptr(),
        &raw mut (*cfg).media_in_new_tab,
    );
    fm_key_file_get_bool(
        kf,
        c"ui".as_ptr(),
        c"desktop_folder_new_win".as_ptr(),
        &raw mut (*cfg).desktop_folder_new_win,
    );
    fm_key_file_get_bool(
        kf,
        c"ui".as_ptr(),
        c"change_tab_on_drop".as_ptr(),
        &raw mut (*cfg).change_tab_on_drop,
    );
    fm_key_file_get_bool(
        kf,
        c"ui".as_ptr(),
        c"close_on_unmount".as_ptr(),
        &raw mut (*cfg).close_on_unmount,
    );
    fm_key_file_get_bool(
        kf,
        c"ui".as_ptr(),
        c"focus_previous".as_ptr(),
        &raw mut (*cfg).focus_previous,
    );
    tmp_int = FM_SP_NONE as c_int;
    tmpv = g_key_file_get_string_list(
        kf,
        b"ui\0" as *const u8 as *const gchar,
        b"side_pane_mode\0" as *const u8 as *const gchar,
        ptr::null_mut::<gsize>(),
        ptr::null_mut::<*mut GError>(),
    ) as *mut *mut c_char;
    if !tmpv.is_null() {
        i = 0i32;
        while !(*tmpv.offset(i as isize)).is_null() {
            tmp = *tmpv.offset(i as isize);
            if strcmp(tmp, c"hidden".as_ptr()) == 0i32 {
                tmp_int |= 0x20i32;
            } else {
                tmp_int &= !(0x1f as c_int);
                if *tmp.offset(0 as isize) as c_int >= '0' as i32
                    && *tmp.offset(0 as isize) as c_int <= '9' as i32
                {
                    tmp_int |= atoi(tmp);
                } else {
                    tmp_int =
                        (tmp_int as c_uint | fm_side_pane_get_mode_by_name(tmp) as c_uint) as c_int;
                }
            }
            i += 1;
        }
        g_strfreev(tmpv as *mut *mut gchar);
    }
    if tmp_int & 0x1f as c_int != FM_SP_NONE as c_int {
        (*cfg).side_pane_mode = tmp_int as FmSidePaneMode;
    }
    tmp = g_key_file_get_string(
        kf,
        b"ui\0" as *const u8 as *const gchar,
        b"view_mode\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    ) as *mut c_char;
    if !tmp.is_null() {
        if *tmp.offset(0 as isize) as c_int >= '0' as i32
            && *tmp.offset(0 as isize) as c_int <= '9' as i32
        {
            tmp_int = atoi(tmp);
        } else {
            tmp_int = fm_standard_view_mode_from_str(tmp) as c_int;
        }
        g_free(tmp as gpointer);
    }
    if !tmp.is_null() && tmp_int as guint <= FM_FV_LIST_VIEW as c_int as guint {
        (*cfg).view_mode = tmp_int as guint;
    }
    fm_key_file_get_bool(
        kf,
        c"ui".as_ptr(),
        c"show_hidden".as_ptr(),
        &raw mut (*cfg).show_hidden,
    );
    _parse_sort(
        kf,
        c"ui".as_ptr(),
        &raw mut (*cfg).sort_type,
        &raw mut (*cfg).sort_by,
    );
    tmpv = g_key_file_get_string_list(
        kf,
        b"ui\0" as *const u8 as *const gchar,
        b"columns\0" as *const u8 as *const gchar,
        ptr::null_mut::<gsize>(),
        ptr::null_mut::<*mut GError>(),
    ) as *mut *mut c_char;
    if !tmpv.is_null() {
        g_strfreev((*cfg).columns as *mut *mut gchar);
        (*cfg).columns = tmpv;
    }
    tmpv = g_key_file_get_string_list(
        kf,
        b"ui\0" as *const u8 as *const gchar,
        b"toolbar\0" as *const u8 as *const gchar,
        ptr::null_mut::<gsize>(),
        ptr::null_mut::<*mut GError>(),
    ) as *mut *mut c_char;
    if !tmpv.is_null() {
        (*cfg)
            .tb
            .set_visible((0i32 == 0) as c_int as gboolean as gboolean);
        let mut rhs_1 = {
            let mut rhs_0 = {
                let mut rhs = {
                    (*cfg).tb.set_home(0 as gboolean as gboolean);
                    (*cfg).tb.home()
                } as gboolean;
                (*cfg).tb.set_nav(rhs);
                (*cfg).tb.nav()
            } as gboolean;
            (*cfg).tb.set_new_tab(rhs_0);
            (*cfg).tb.new_tab()
        } as gboolean;
        (*cfg).tb.set_new_win(rhs_1);
        i = 0i32;
        while !(*tmpv.offset(i as isize)).is_null() {
            tmp = *tmpv.offset(i as isize);
            if (*cfg).tb.visible() != 0 && strcmp(tmp, c"hidden".as_ptr()) == 0i32 {
                (*cfg).tb.set_visible(0 as gboolean as gboolean);
            } else if (*cfg).tb.new_win() == 0 && strcmp(tmp, c"newwin".as_ptr()) == 0i32 {
                (*cfg)
                    .tb
                    .set_new_win((0i32 == 0) as c_int as gboolean as gboolean);
            } else if (*cfg).tb.new_tab() == 0 && strcmp(tmp, c"newtab".as_ptr()) == 0i32 {
                (*cfg)
                    .tb
                    .set_new_tab((0i32 == 0) as c_int as gboolean as gboolean);
            } else if (*cfg).tb.nav() == 0 && strcmp(tmp, c"navigation".as_ptr()) == 0i32 {
                (*cfg)
                    .tb
                    .set_nav((0i32 == 0) as c_int as gboolean as gboolean);
            } else if (*cfg).tb.home() == 0 && strcmp(tmp, c"home".as_ptr()) == 0i32 {
                (*cfg)
                    .tb
                    .set_home((0i32 == 0) as c_int as gboolean as gboolean);
            }
            i += 1;
        }
        g_strfreev(tmpv as *mut *mut gchar);
    }
    fm_key_file_get_bool(
        kf,
        c"ui".as_ptr(),
        c"show_statusbar".as_ptr(),
        &raw mut (*cfg).show_statusbar,
    );
    if g_key_file_has_group(kf, b"autorun\0" as *const u8 as *const gchar) != 0 {
        tmpv = g_key_file_get_keys(
            kf,
            b"autorun\0" as *const u8 as *const gchar,
            ptr::null_mut::<gsize>(),
            ptr::null_mut::<*mut GError>(),
        ) as *mut *mut c_char;
        i = 0i32;
        while !(*tmpv.offset(i as isize)).is_null() {
            tmp = g_key_file_get_string(
                kf,
                b"autorun\0" as *const u8 as *const gchar,
                *tmpv.offset(i as isize),
                ptr::null_mut::<*mut GError>(),
            ) as *mut c_char;
            if !tmp.is_null() && *tmp.offset(0 as isize) as c_int != 0 {
                if *tmp.offset(0 as isize) as c_int == '*' as i32 {
                    fm_app_config_set_autorun_choice(
                        cfg,
                        *tmpv.offset(i as isize),
                        if *tmp.offset(1 as isize) as c_int != 0 {
                            tmp.offset(1 as isize) as *mut c_char
                        } else {
                            ptr::null_mut::<c_char>()
                        },
                        (0i32 == 0) as c_int,
                    );
                } else {
                    fm_app_config_set_autorun_choice(
                        cfg,
                        *tmpv.offset(i as isize),
                        tmp,
                        0 as gboolean,
                    );
                }
            }
            g_free(tmp as gpointer);
            i += 1;
        }
        g_strfreev(tmpv as *mut *mut gchar);
    }
    fm_key_file_get_bool(
        kf,
        c"ui".as_ptr(),
        c"pathbar_mode_buttons".as_ptr(),
        &raw mut (*cfg).pathbar_mode_buttons,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_config_load_from_profile(
    mut cfg: *mut FmAppConfig,
    mut name: *const c_char,
) {
    let mut dirs: *const *const gchar = ptr::null::<*const gchar>();
    let mut dir: *const *const gchar = ptr::null::<*const gchar>();
    let mut path: *mut c_char = ptr::null_mut::<c_char>();
    let mut kf: *mut GKeyFile = g_key_file_new();
    let mut old_name: *const c_char = name;
    if name.is_null() || *name == 0 {
        name = c"default".as_ptr();
        old_name = c"pcmanfm".as_ptr();
    }
    dirs = g_get_system_config_dirs();
    dir = dirs;
    while !(*dir).is_null() {
        path = g_build_filename(
            *dir,
            c"pcmanfm".as_ptr(),
            name,
            c"pcmanfm.conf".as_ptr(),
            ptr::null_mut::<c_void>(),
        ) as *mut c_char;
        if g_key_file_load_from_file(kf, path, G_KEY_FILE_NONE, ptr::null_mut::<*mut GError>()) != 0
        {
            fm_app_config_load_from_key_file(cfg, kf);
        }
        g_free(path as gpointer);
        dir = dir.offset(1);
    }
    path = g_strconcat(
        g_get_user_config_dir(),
        c"/pcmanfm/".as_ptr(),
        old_name,
        c".conf".as_ptr(),
        ptr::null_mut::<c_void>(),
    ) as *mut c_char;
    if ({
        let mut _g_boolean_var_12: c_int = 0i32;
        if g_key_file_load_from_file(kf, path, G_KEY_FILE_NONE, ptr::null_mut::<*mut GError>()) != 0
        {
            _g_boolean_var_12 = 1i32;
        }
        _g_boolean_var_12
    }) as c_long
        != 0
    {
        let mut new_dir: *mut c_char = ptr::null_mut::<c_char>();
        fm_app_config_load_from_key_file(cfg, kf);
        new_dir = g_build_filename(
            g_get_user_config_dir(),
            c"pcmanfm".as_ptr(),
            name,
            ptr::null_mut::<c_void>(),
        ) as *mut c_char;
        if g_mkdir_with_parents(new_dir, 0o700 as gint) == 0i32 {
            let mut new_path: *mut c_char =
                g_build_filename(new_dir, c"pcmanfm.conf".as_ptr(), ptr::null_mut::<c_void>())
                    as *mut c_char;
            rename(path, new_path);
            g_free(new_path as gpointer);
        }
        g_free(new_dir as gpointer);
    } else {
        g_free(path as gpointer);
        path = g_build_filename(
            g_get_user_config_dir(),
            c"pcmanfm".as_ptr(),
            name,
            c"pcmanfm.conf".as_ptr(),
            ptr::null_mut::<c_void>(),
        ) as *mut c_char;
        if g_key_file_load_from_file(kf, path, G_KEY_FILE_NONE, ptr::null_mut::<*mut GError>()) != 0
        {
            fm_app_config_load_from_key_file(cfg, kf);
        }
    }
    g_free(path as gpointer);
    g_key_file_free(kf);
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_config_get_config_for_path(
    mut path: *mut FmPath,
    mut mode: *mut FmSortMode,
    mut by: *mut FmFolderModelCol,
    mut view_mode: *mut FmStandardViewMode,
    mut show_hidden: *mut gboolean,
    mut columns: *mut *mut *mut c_char,
) -> gboolean {
    let mut sub_path: *mut FmPath = ptr::null_mut::<FmPath>();
    let mut fc: *mut FmFolderConfig = ptr::null_mut::<FmFolderConfig>();
    let mut ret: gboolean = (0i32 == 0) as c_int;
    if !mode.is_null() {
        *mode = (*(fm_config as *mut FmAppConfig)).sort_type;
    }
    if !by.is_null() {
        *by = (*(fm_config as *mut FmAppConfig)).sort_by;
    }
    if !view_mode.is_null() {
        *view_mode = (*(fm_config as *mut FmAppConfig)).view_mode as FmStandardViewMode;
    }
    if !show_hidden.is_null() {
        *show_hidden = (*(fm_config as *mut FmAppConfig)).show_hidden;
    }
    if !columns.is_null() {
        *columns = (*(fm_config as *mut FmAppConfig)).columns;
    }
    fc = fm_folder_config_open(path);
    if fm_folder_config_is_empty(fc) == 0 {
        _parse_config_for_path(fc, mode, by, view_mode, show_hidden, columns);
    } else if !(fm_path_get_flags(path) as c_uint & FM_PATH_IS_NATIVE as c_int as c_uint
        != 0 as c_uint)
    {
        sub_path = fm_path_get_scheme_path(path);
        fm_folder_config_close(fc, ptr::null_mut::<*mut GError>());
        fc = fm_folder_config_open(sub_path);
        if fm_folder_config_is_empty(fc) == 0 {
            _parse_config_for_path(fc, mode, by, view_mode, show_hidden, columns);
        } else if strncmp(
            fm_path_get_basename(sub_path),
            c"search:".as_ptr(),
            7 as size_t,
        ) == 0i32
        {
            if !view_mode.is_null() {
                *view_mode = FM_FV_LIST_VIEW;
            }
            if !show_hidden.is_null() {
                *show_hidden = (0i32 == 0) as c_int as gboolean;
            }
            if !columns.is_null() {
                static mut def: [*mut c_char; 6] = [
                    c"name".as_ptr() as *mut c_char,
                    c"desc".as_ptr() as *mut c_char,
                    c"dirname".as_ptr() as *mut c_char,
                    c"size".as_ptr() as *mut c_char,
                    c"mtime".as_ptr() as *mut c_char,
                    ptr::null::<c_char>() as *mut c_char,
                ];
                *columns = &raw mut def as *mut *mut c_char;
            }
        } else {
            ret = 0 as gboolean;
        }
    } else {
        ret = 0 as gboolean;
    }
    fm_folder_config_close(fc, ptr::null_mut::<*mut GError>());
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_config_save_config_for_path(
    mut path: *mut FmPath,
    mut mode: FmSortMode,
    mut by: FmFolderModelCol,
    mut view_mode: FmStandardViewMode,
    mut show_hidden: gboolean,
    mut columns: *mut *mut c_char,
) {
    let mut sub_path: *mut FmPath = ptr::null_mut::<FmPath>();
    let mut fc: *mut FmFolderConfig = ptr::null_mut::<FmFolderConfig>();
    let mut list: [*const c_char; 5] = [ptr::null::<c_char>(); 5];
    let mut n: c_int = 2 as c_int;
    if path.is_null() {
        return;
    }
    sub_path = fm_path_get_scheme_path(path);
    if strncmp(
        fm_path_get_basename(sub_path),
        c"search:".as_ptr(),
        7 as size_t,
    ) == 0i32
    {
        path = fm_path_new_for_uri(c"search:///".as_ptr());
        fc = fm_folder_config_open(path);
        fm_path_unref(path);
        if fm_folder_config_is_empty(fc) != 0
            && (!columns.is_null() && !(*columns.offset(0 as isize)).is_null())
        {
            fm_folder_config_close(fc, ptr::null_mut::<*mut GError>());
            return;
        }
        view_mode = FM_FV_LIST_VIEW;
    } else {
        fc = fm_folder_config_open(path);
    }
    list[0 as usize] = fm_folder_model_col_get_name(by);
    if list[0 as usize].is_null() {
        list[0 as usize] = fm_folder_model_col_get_name(FM_FOLDER_MODEL_COL_NAME);
    }
    list[1 as usize] = if mode as c_uint & FM_SORT_ORDER_MASK as c_int as c_uint
        == FM_SORT_ASCENDING as c_int as c_uint
    {
        c"ascending".as_ptr()
    } else {
        c"descending".as_ptr()
    };
    if mode as c_uint & FM_SORT_CASE_SENSITIVE as c_int as c_uint != 0 {
        let fresh28 = n;
        n = n + 1;
        list[fresh28 as usize] = c"case".as_ptr();
    }
    if mode as c_uint & FM_SORT_NO_FOLDER_FIRST as c_int as c_uint != 0 {
        let fresh29 = n;
        n = n + 1;
        list[fresh29 as usize] = c"mingle".as_ptr();
    }
    list[n as usize] = ptr::null::<c_char>();
    fm_folder_config_set_string_list(
        fc,
        c"Sort".as_ptr(),
        &raw mut list as *mut *const c_char as *const *const gchar,
        n as gsize,
    );
    if view_mode as guint <= FM_FV_LIST_VIEW as c_int as guint {
        fm_folder_config_set_string(
            fc,
            c"ViewMode".as_ptr(),
            fm_standard_view_mode_to_str(view_mode),
        );
    }
    if !columns.is_null() && !(*columns.offset(0 as isize)).is_null() {
        fm_folder_config_set_string_list(
            fc,
            c"Columns".as_ptr(),
            columns as *const *const gchar,
            g_strv_length(columns as *mut *mut gchar) as gsize,
        );
    } else if !columns.is_null() {
        fm_folder_config_remove_key(fc, c"Columns".as_ptr());
    }
    fm_folder_config_set_boolean(fc, c"ShowHidden".as_ptr(), show_hidden);
    fm_folder_config_close(fc, ptr::null_mut::<*mut GError>());
    pcmanfm_save_config(0 as gboolean);
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_config_clear_config_for_path(mut path: *mut FmPath) {
    let mut fc: *mut FmFolderConfig = fm_folder_config_open(path);
    fm_folder_config_purge(fc);
    fm_folder_config_close(fc, ptr::null_mut::<*mut GError>());
    pcmanfm_save_config(0 as gboolean);
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_config_save_desktop_config(
    mut buf: *mut GString,
    mut group: *const c_char,
    mut cfg: *mut FmDesktopConfig,
) {
    g_string_append_printf(
        buf,
        b"[%s]\nwallpaper_mode=%s\n\0" as *const u8 as *const gchar,
        group,
        fm_app_wallpaper_get_mode_name((*cfg).wallpaper_mode),
    );
    g_string_append_printf(
        buf,
        b"wallpaper_common=%d\n\0" as *const u8 as *const gchar,
        (*cfg).wallpaper_common,
    );
    if !(*cfg).wallpapers.is_null() && (*cfg).wallpapers_configured > 0i32 {
        let mut i: c_int = 0;
        g_string_append_printf(
            buf,
            b"wallpapers_configured=%d\n\0" as *const u8 as *const gchar,
            (*cfg).wallpapers_configured,
        );
        i = 0i32;
        while i < (*cfg).wallpapers_configured {
            if !(*(*cfg).wallpapers.offset(i as isize)).is_null() {
                g_string_append_printf(
                    buf,
                    b"wallpaper%d=%s\n\0" as *const u8 as *const gchar,
                    i,
                    *(*cfg).wallpapers.offset(i as isize),
                );
            }
            i += 1;
        }
    }
    if (*cfg).wallpaper_common != 0 && !(*cfg).wallpaper.is_null() {
        g_string_append_printf(
            buf,
            b"wallpaper=%s\n\0" as *const u8 as *const gchar,
            (*cfg).wallpaper,
        );
    }
    g_string_append_printf(
        buf,
        b"desktop_bg=#%02x%02x%02x\n\0" as *const u8 as *const gchar,
        (*cfg).desktop_bg.red as c_int / 257 as c_int,
        (*cfg).desktop_bg.green as c_int / 257 as c_int,
        (*cfg).desktop_bg.blue as c_int / 257 as c_int,
    );
    g_string_append_printf(
        buf,
        b"desktop_fg=#%02x%02x%02x\n\0" as *const u8 as *const gchar,
        (*cfg).desktop_fg.red as c_int / 257 as c_int,
        (*cfg).desktop_fg.green as c_int / 257 as c_int,
        (*cfg).desktop_fg.blue as c_int / 257 as c_int,
    );
    g_string_append_printf(
        buf,
        b"desktop_shadow=#%02x%02x%02x\n\0" as *const u8 as *const gchar,
        (*cfg).desktop_shadow.red as c_int / 257 as c_int,
        (*cfg).desktop_shadow.green as c_int / 257 as c_int,
        (*cfg).desktop_shadow.blue as c_int / 257 as c_int,
    );
    if !(*cfg).desktop_font.is_null() && *(*cfg).desktop_font as c_int != 0 {
        g_string_append_printf(
            buf,
            b"desktop_font=%s\n\0" as *const u8 as *const gchar,
            (*cfg).desktop_font,
        );
    }
    if !(*cfg).folder.is_null() {
        g_string_append_printf(
            buf,
            b"folder=%s\n\0" as *const u8 as *const gchar,
            (*cfg).folder,
        );
    }
    g_string_append_printf(
        buf,
        b"show_wm_menu=%d\n\0" as *const u8 as *const gchar,
        (*cfg).show_wm_menu,
    );
    _save_sort(buf, (*cfg).desktop_sort_type, (*cfg).desktop_sort_by);
    g_string_append_printf(
        buf,
        b"show_documents=%d\n\0" as *const u8 as *const gchar,
        (*cfg).show_documents,
    );
    g_string_append_printf(
        buf,
        b"show_trash=%d\n\0" as *const u8 as *const gchar,
        (*cfg).show_trash,
    );
    g_string_append_printf(
        buf,
        b"show_mounts=%d\n\0" as *const u8 as *const gchar,
        (*cfg).show_mounts,
    );
}
unsafe extern "C" fn _save_choice(mut key: gpointer, mut val: gpointer, mut buf: gpointer) {
    let mut choice: *mut FmAutorunChoice = val as *mut FmAutorunChoice;
    if (*choice).dont_ask == 0 && (*choice).last_used.is_null() {
        return;
    }
    g_string_append_printf(
        buf as *mut GString,
        b"%s=%s%s\n\0" as *const u8 as *const gchar,
        key as *mut c_char,
        if (*choice).dont_ask != 0 {
            c"*".as_ptr()
        } else {
            c"".as_ptr()
        },
        if !(*choice).last_used.is_null() {
            (*choice).last_used as *const c_char
        } else {
            c"".as_ptr()
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_config_save_profile(
    mut cfg: *mut FmAppConfig,
    mut name: *const c_char,
) {
    let mut path: *mut c_char = ptr::null_mut::<c_char>();
    let mut dir_path: *mut c_char = ptr::null_mut::<c_char>();
    if name.is_null() || *name == 0 {
        name = c"default".as_ptr();
    }
    dir_path = g_build_filename(
        g_get_user_config_dir(),
        c"pcmanfm".as_ptr(),
        name,
        ptr::null_mut::<c_void>(),
    ) as *mut c_char;
    if g_mkdir_with_parents(dir_path, 0o700 as gint) != -(1i32) {
        let mut buf: *mut GString = g_string_sized_new(1024 as gsize);
        if 0 != 0 {
            ({
                let __val: *const c_char = c"[config]\n".as_ptr();
                g_string_append_len_inline(
                    buf,
                    __val,
                    if ({
                        let mut _g_boolean_var_13: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_13 = 1i32;
                        }
                        _g_boolean_var_13
                    }) as c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as isize)) as gssize
                    } else {
                        -(1i32) as gssize
                    },
                );
            });
        } else {
            g_string_append_len_inline(buf, c"[config]\n".as_ptr(), -(1i32) as gssize);
        };
        g_string_append_printf(
            buf,
            b"bm_open_method=%d\n\0" as *const u8 as *const gchar,
            (*cfg).bm_open_method,
        );
        if !(*cfg).home_path.is_null()
            && *(*cfg).home_path.offset(0 as isize) as c_int != 0
            && strcmp((*cfg).home_path, fm_get_home_dir()) != 0i32
        {
            g_string_append_printf(
                buf,
                b"home_path=%s\n\0" as *const u8 as *const gchar,
                (*cfg).home_path,
            );
        }
        if 0 != 0 {
            ({
                let __val: *const c_char = c"\n[volume]\n".as_ptr();
                g_string_append_len_inline(
                    buf,
                    __val,
                    if ({
                        let mut _g_boolean_var_14: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_14 = 1i32;
                        }
                        _g_boolean_var_14
                    }) as c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as isize)) as gssize
                    } else {
                        -(1i32) as gssize
                    },
                );
            });
        } else {
            g_string_append_len_inline(buf, c"\n[volume]\n".as_ptr(), -(1i32) as gssize);
        };
        g_string_append_printf(
            buf,
            b"mount_on_startup=%d\n\0" as *const u8 as *const gchar,
            (*cfg).mount_on_startup,
        );
        g_string_append_printf(
            buf,
            b"mount_removable=%d\n\0" as *const u8 as *const gchar,
            (*cfg).mount_removable,
        );
        g_string_append_printf(
            buf,
            b"autorun=%d\n\0" as *const u8 as *const gchar,
            (*cfg).autorun,
        );
        if g_hash_table_size((*cfg).autorun_choices) > 0 as guint {
            if 0 != 0 {
                ({
                    let __val: *const c_char = c"\n[autorun]\n".as_ptr();
                    g_string_append_len_inline(
                        buf,
                        __val,
                        if ({
                            let mut _g_boolean_var_15: c_int = 0 as c_int;
                            if !__val.is_null() {
                                _g_boolean_var_15 = 1i32;
                            }
                            _g_boolean_var_15
                        }) as c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as isize)) as gssize
                        } else {
                            -(1i32) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(buf, c"\n[autorun]\n".as_ptr(), -(1i32) as gssize);
            };
            g_hash_table_foreach(
                (*cfg).autorun_choices,
                Some(_save_choice as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()),
                buf as gpointer,
            );
        }
        if 0 != 0 {
            ({
                let __val: *const c_char = c"\n[ui]\n".as_ptr();
                g_string_append_len_inline(
                    buf,
                    __val,
                    if ({
                        let mut _g_boolean_var_16: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_16 = 1i32;
                        }
                        _g_boolean_var_16
                    }) as c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as isize)) as gssize
                    } else {
                        -(1i32) as gssize
                    },
                );
            });
        } else {
            g_string_append_len_inline(buf, c"\n[ui]\n".as_ptr(), -(1i32) as gssize);
        };
        g_string_append_printf(
            buf,
            b"always_show_tabs=%d\n\0" as *const u8 as *const gchar,
            (*cfg).always_show_tabs,
        );
        g_string_append_printf(
            buf,
            b"max_tab_chars=%d\n\0" as *const u8 as *const gchar,
            (*cfg).max_tab_chars,
        );
        g_string_append_printf(
            buf,
            b"win_width=%d\n\0" as *const u8 as *const gchar,
            (*cfg).win_width,
        );
        g_string_append_printf(
            buf,
            b"win_height=%d\n\0" as *const u8 as *const gchar,
            (*cfg).win_height,
        );
        if (*cfg).maximized != 0 {
            if 0 != 0 {
                ({
                    let __val: *const c_char = c"maximized=1\n".as_ptr();
                    g_string_append_len_inline(
                        buf,
                        __val,
                        if ({
                            let mut _g_boolean_var_17: c_int = 0 as c_int;
                            if !__val.is_null() {
                                _g_boolean_var_17 = 1i32;
                            }
                            _g_boolean_var_17
                        }) as c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as isize)) as gssize
                        } else {
                            -(1i32) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(buf, c"maximized=1\n".as_ptr(), -(1i32) as gssize);
            };
        }
        g_string_append_printf(
            buf,
            b"splitter_pos=%d\n\0" as *const u8 as *const gchar,
            (*cfg).splitter_pos,
        );
        g_string_append_printf(
            buf,
            b"media_in_new_tab=%d\n\0" as *const u8 as *const gchar,
            (*cfg).media_in_new_tab,
        );
        g_string_append_printf(
            buf,
            b"desktop_folder_new_win=%d\n\0" as *const u8 as *const gchar,
            (*cfg).desktop_folder_new_win,
        );
        g_string_append_printf(
            buf,
            b"change_tab_on_drop=%d\n\0" as *const u8 as *const gchar,
            (*cfg).change_tab_on_drop,
        );
        g_string_append_printf(
            buf,
            b"close_on_unmount=%d\n\0" as *const u8 as *const gchar,
            (*cfg).close_on_unmount,
        );
        g_string_append_printf(
            buf,
            b"focus_previous=%d\n\0" as *const u8 as *const gchar,
            (*cfg).focus_previous,
        );
        if 0 != 0 {
            ({
                let __val: *const c_char = c"side_pane_mode=".as_ptr();
                g_string_append_len_inline(
                    buf,
                    __val,
                    if ({
                        let mut _g_boolean_var_18: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_18 = 1i32;
                        }
                        _g_boolean_var_18
                    }) as c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as isize)) as gssize
                    } else {
                        -(1i32) as gssize
                    },
                );
            });
        } else {
            g_string_append_len_inline(buf, c"side_pane_mode=".as_ptr(), -(1i32) as gssize);
        };
        if (*cfg).side_pane_mode as c_uint & 0x20 as c_uint != 0 {
            if 0 != 0 {
                ({
                    let __val: *const c_char = c"hidden;".as_ptr();
                    g_string_append_len_inline(
                        buf,
                        __val,
                        if ({
                            let mut _g_boolean_var_19: c_int = 0 as c_int;
                            if !__val.is_null() {
                                _g_boolean_var_19 = 1i32;
                            }
                            _g_boolean_var_19
                        }) as c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as isize)) as gssize
                        } else {
                            -(1i32) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(buf, c"hidden;".as_ptr(), -(1i32) as gssize);
            };
        }
        g_string_append_printf(
            buf,
            b"%s\n\0" as *const u8 as *const gchar,
            fm_side_pane_get_mode_name(
                ((*cfg).side_pane_mode as c_uint & 0x1f as c_uint) as FmSidePaneMode,
            ),
        );
        g_string_append_printf(
            buf,
            b"view_mode=%s\n\0" as *const u8 as *const gchar,
            fm_standard_view_mode_to_str((*cfg).view_mode as FmStandardViewMode),
        );
        g_string_append_printf(
            buf,
            b"show_hidden=%d\n\0" as *const u8 as *const gchar,
            (*cfg).show_hidden,
        );
        _save_sort(buf, (*cfg).sort_type, (*cfg).sort_by);
        if !(*cfg).columns.is_null() && !(*(*cfg).columns.offset(0 as isize)).is_null() {
            let mut colptr: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
            if 0 != 0 {
                ({
                    let __val: *const c_char = c"columns=".as_ptr();
                    g_string_append_len_inline(
                        buf,
                        __val,
                        if ({
                            let mut _g_boolean_var_20: c_int = 0 as c_int;
                            if !__val.is_null() {
                                _g_boolean_var_20 = 1i32;
                            }
                            _g_boolean_var_20
                        }) as c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as isize)) as gssize
                        } else {
                            -(1i32) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(buf, c"columns=".as_ptr(), -(1i32) as gssize);
            };
            colptr = (*cfg).columns;
            while !(*colptr).is_null() {
                g_string_append_printf(buf, b"%s;\0" as *const u8 as *const gchar, *colptr);
                colptr = colptr.offset(1);
            }
            g_string_append_c_inline(buf, '\n' as i32 as gchar);
        }
        if 0 != 0 {
            ({
                let __val: *const c_char = c"toolbar=".as_ptr();
                g_string_append_len_inline(
                    buf,
                    __val,
                    if ({
                        let mut _g_boolean_var_21: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_21 = 1i32;
                        }
                        _g_boolean_var_21
                    }) as c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as isize)) as gssize
                    } else {
                        -(1i32) as gssize
                    },
                );
            });
        } else {
            g_string_append_len_inline(buf, c"toolbar=".as_ptr(), -(1i32) as gssize);
        };
        if (*cfg).tb.visible() == 0 {
            if 0 != 0 {
                ({
                    let __val: *const c_char = c"hidden;".as_ptr();
                    g_string_append_len_inline(
                        buf,
                        __val,
                        if ({
                            let mut _g_boolean_var_22: c_int = 0 as c_int;
                            if !__val.is_null() {
                                _g_boolean_var_22 = 1i32;
                            }
                            _g_boolean_var_22
                        }) as c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as isize)) as gssize
                        } else {
                            -(1i32) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(buf, c"hidden;".as_ptr(), -(1i32) as gssize);
            };
        }
        if (*cfg).tb.new_win() != 0 {
            if 0 != 0 {
                ({
                    let __val: *const c_char = c"newwin;".as_ptr();
                    g_string_append_len_inline(
                        buf,
                        __val,
                        if ({
                            let mut _g_boolean_var_23: c_int = 0 as c_int;
                            if !__val.is_null() {
                                _g_boolean_var_23 = 1i32;
                            }
                            _g_boolean_var_23
                        }) as c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as isize)) as gssize
                        } else {
                            -(1i32) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(buf, c"newwin;".as_ptr(), -(1i32) as gssize);
            };
        }
        if (*cfg).tb.new_tab() != 0 {
            if 0 != 0 {
                ({
                    let __val: *const c_char = c"newtab;".as_ptr();
                    g_string_append_len_inline(
                        buf,
                        __val,
                        if ({
                            let mut _g_boolean_var_24: c_int = 0 as c_int;
                            if !__val.is_null() {
                                _g_boolean_var_24 = 1i32;
                            }
                            _g_boolean_var_24
                        }) as c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as isize)) as gssize
                        } else {
                            -(1i32) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(buf, c"newtab;".as_ptr(), -(1i32) as gssize);
            };
        }
        if (*cfg).tb.nav() != 0 {
            if 0 != 0 {
                ({
                    let __val: *const c_char = c"navigation;".as_ptr();
                    g_string_append_len_inline(
                        buf,
                        __val,
                        if ({
                            let mut _g_boolean_var_25: c_int = 0 as c_int;
                            if !__val.is_null() {
                                _g_boolean_var_25 = 1i32;
                            }
                            _g_boolean_var_25
                        }) as c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as isize)) as gssize
                        } else {
                            -(1i32) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(buf, c"navigation;".as_ptr(), -(1i32) as gssize);
            };
        }
        if (*cfg).tb.home() != 0 {
            if 0 != 0 {
                ({
                    let __val: *const c_char = c"home;".as_ptr();
                    g_string_append_len_inline(
                        buf,
                        __val,
                        if ({
                            let mut _g_boolean_var_26: c_int = 0 as c_int;
                            if !__val.is_null() {
                                _g_boolean_var_26 = 1i32;
                            }
                            _g_boolean_var_26
                        }) as c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as isize)) as gssize
                        } else {
                            -(1i32) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(buf, c"home;".as_ptr(), -(1i32) as gssize);
            };
        }
        g_string_append_c_inline(buf, '\n' as i32 as gchar);
        g_string_append_printf(
            buf,
            b"show_statusbar=%d\n\0" as *const u8 as *const gchar,
            (*cfg).show_statusbar,
        );
        g_string_append_printf(
            buf,
            b"pathbar_mode_buttons=%d\n\0" as *const u8 as *const gchar,
            (*cfg).pathbar_mode_buttons,
        );
        path = g_build_filename(
            dir_path,
            c"pcmanfm.conf".as_ptr(),
            ptr::null_mut::<c_void>(),
        ) as *mut c_char;
        g_file_set_contents(
            path,
            (*buf).str_0,
            (*buf).len as gssize,
            ptr::null_mut::<*mut GError>(),
        );
        g_free(path as gpointer);
        if 0 != 0 {
            if 0i32 == 0 {
                g_string_free(buf, (0i32 == 0) as c_int);
            } else {
                g_string_free_and_steal(buf);
            };
        } else {
            g_string_free(buf, (0i32 == 0) as c_int);
        };
        fm_folder_config_save_cache();
    }
    g_free(dir_path as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_config_set_autorun_choice(
    mut cfg: *mut FmAppConfig,
    mut content_type: *const c_char,
    mut app: *const c_char,
    mut dont_ask: gboolean,
) {
    let mut choice: *mut FmAutorunChoice = ptr::null_mut::<FmAutorunChoice>();
    if content_type.is_null() {
        return;
    }
    choice = g_hash_table_lookup((*cfg).autorun_choices, content_type as gconstpointer)
        as *mut FmAutorunChoice;
    if !choice.is_null() {
        g_free((*choice).last_used as gpointer);
    } else {
        choice = g_slice_alloc(mem::size_of::<FmAutorunChoice>() as gsize) as *mut FmAutorunChoice;
        g_hash_table_insert(
            (*cfg).autorun_choices,
            g_strdup_inline(content_type) as gpointer,
            choice as gpointer,
        );
    }
    (*choice).last_used = g_strdup_inline(app);
    (*choice).dont_ask = dont_ask;
}
static mut _wp_modes: [_WPModeDesc; 7] = [
    _WPModeDesc {
        name: c"color".as_ptr(),
        mode: FM_WP_COLOR,
    },
    _WPModeDesc {
        name: c"stretch".as_ptr(),
        mode: FM_WP_STRETCH,
    },
    _WPModeDesc {
        name: c"fit".as_ptr(),
        mode: FM_WP_FIT,
    },
    _WPModeDesc {
        name: c"center".as_ptr(),
        mode: FM_WP_CENTER,
    },
    _WPModeDesc {
        name: c"tile".as_ptr(),
        mode: FM_WP_TILE,
    },
    _WPModeDesc {
        name: c"crop".as_ptr(),
        mode: FM_WP_CROP,
    },
    _WPModeDesc {
        name: c"screen".as_ptr(),
        mode: FM_WP_SCREEN,
    },
];
#[no_mangle]
pub unsafe extern "C" fn fm_app_wallpaper_get_mode_by_name(
    mut name: *const c_char,
) -> FmWallpaperMode {
    let mut i: guint = 0;
    if !name.is_null() {
        i = 0 as guint;
        while (i as usize)
            < (mem::size_of::<[_WPModeDesc; 7]>() as usize)
                .wrapping_div(mem::size_of::<_WPModeDesc>() as usize)
        {
            if strcmp(_wp_modes[i as usize].name, name) == 0i32 {
                return _wp_modes[i as usize].mode;
            }
            i = i.wrapping_add(1);
        }
    }
    return 4294967295 as FmWallpaperMode;
}
#[no_mangle]
pub unsafe extern "C" fn fm_app_wallpaper_get_mode_name(
    mut mode: FmWallpaperMode,
) -> *const c_char {
    let mut i: guint = 0;
    i = 0 as guint;
    while (i as usize)
        < (mem::size_of::<[_WPModeDesc; 7]>() as usize)
            .wrapping_div(mem::size_of::<_WPModeDesc>() as usize)
    {
        if _wp_modes[i as usize].mode as c_uint == mode as c_uint {
            return _wp_modes[i as usize].name;
        }
        i = i.wrapping_add(1);
    }
    return ptr::null::<c_char>();
}
