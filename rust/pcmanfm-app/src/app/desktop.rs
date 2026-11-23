// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      desktop.c
 *
 *      Copyright 2010 - 2012 Hong Jen Yee (PCMan) <pcman.tw@gmail.com>
 *      Copyright 2012-2017 Andriy Grytsenko (LStranger) <andrej@rep.kiev.ua>
 *      Copyright 2024 Ingo Brückl
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

#[inline(never)]
unsafe extern "C" fn fm_desktop_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        gtk_window_get_type(),
        g_intern_static_string(b"FmDesktop\0" as *const u8 as *const gchar),
        mem::size_of::<FmDesktopClass>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(mem::transmute::<
            Option<unsafe extern "C" fn(gpointer) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            fm_desktop_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
        ))),
        mem::size_of::<FmDesktop>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmDesktop) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                fm_desktop_init as unsafe extern "C" fn(*mut FmDesktop) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInterfaceInitFunc>(
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolderViewInterface) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                fm_desktop_view_init as unsafe extern "C" fn(*mut FmFolderViewInterface) -> (),
            )),
        ),
        interface_finalize: None,
        interface_data: ptr::null_mut::<c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        fm_folder_view_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
static mut FmDesktop_private_offset: gint = 0;
unsafe extern "C" fn fm_desktop_class_intern_init(mut klass: gpointer) {
    fm_desktop_parent_class = g_type_class_peek_parent(klass);
    if FmDesktop_private_offset != 0i32 {
        g_type_class_adjust_private_offset(klass, &raw mut FmDesktop_private_offset);
    }
    fm_desktop_class_init(klass as *mut FmDesktopClass);
}
#[no_mangle]
pub unsafe extern "C" fn fm_desktop_get_type() -> GType {
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
        let mut g_define_type_id: GType = fm_desktop_get_type_once();
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
static mut fm_desktop_parent_class: gpointer = ptr::null::<c_void>() as *mut c_void;
static mut win_group: *mut GtkWindowGroup = ptr::null::<GtkWindowGroup>() as *mut GtkWindowGroup;
static mut desktops: *mut *mut FmDesktop = ptr::null::<*mut FmDesktop>() as *mut *mut FmDesktop;
static mut n_screens: c_int = 0i32;
static mut icon_theme_changed: guint = 0 as guint;
static mut acc_grp: *mut GtkAccelGroup = ptr::null::<GtkAccelGroup>() as *mut GtkAccelGroup;
static mut XA_NET_WORKAREA: Atom = 0 as Atom;
static mut XA_NET_NUMBER_OF_DESKTOPS: Atom = 0 as Atom;
static mut XA_NET_CURRENT_DESKTOP: Atom = 0 as Atom;
static mut XA_XROOTMAP_ID: Atom = 0 as Atom;
static mut XA_XROOTPMAP_ID: Atom = 0 as Atom;
static mut hand_cursor: *mut GdkCursor = ptr::null::<GdkCursor>() as *mut GdkCursor;
static mut idle_config_save: guint = 0 as guint;
static mut emwm_layout: gboolean = 0 as gboolean;
static mut emwm_right: gboolean = 0 as gboolean;
static mut emwm_output: *mut c_char = ptr::null::<c_char>() as *mut c_char;
static mut emwm_settings_monitor: *mut GFileMonitor =
    ptr::null::<GFileMonitor>() as *mut GFileMonitor;
static mut emwm_refresh_id: guint = 0 as guint;
static mut emwm_on_screen: gint = -(1 as gint);
static mut emwm_have_wallpaper: gboolean = 0 as gboolean;
static mut emwm_wallpaper: *mut c_char = ptr::null::<c_char>() as *mut c_char;
static mut emwm_wallpaper_mode: FmWallpaperMode = FM_WP_COLOR;
unsafe extern "C" fn emwm_icon_monitor(mut screen: *mut GdkScreen) -> gint {
    let mut i: gint = 0;
    let mut count: gint = gdk_screen_get_n_monitors(screen);
    if !emwm_output.is_null() && strcmp(emwm_output, c"primary".as_ptr()) != 0 {
        i = 0 as gint;
        while i < count {
            let mut name: *mut c_char = gdk_screen_get_monitor_plug_name(screen, i) as *mut c_char;
            let mut match_0: gboolean = (g_strcmp0(name, emwm_output) == 0) as c_int;
            g_free(name as gpointer);
            if match_0 != 0 {
                return i;
            }
            i += 1;
        }
    }
    i = gdk_screen_get_primary_monitor(screen);
    return if i >= 0i32 && i < count { i } else { 0 as gint };
}
static mut dnd_targets: [GtkTargetEntry; 1] = [_GtkTargetEntry {
    target: c"application/x-desktop-item".as_ptr() as *mut gchar,
    flags: GTK_TARGET_SAME_WIDGET as c_int as guint,
    info: FM_DND_DEST_DESKTOP_ITEM as c_int as guint,
}];
static mut desktop_atom: GdkAtom = ptr::null::<_GdkAtom>() as *mut _GdkAtom;
static mut documents: *mut FmDesktopExtraItem =
    ptr::null::<FmDesktopExtraItem>() as *mut FmDesktopExtraItem;
static mut trash_can: *mut FmDesktopExtraItem =
    ptr::null::<FmDesktopExtraItem>() as *mut FmDesktopExtraItem;
static mut mounts: *mut GSList = ptr::null::<GSList>() as *mut GSList;
unsafe extern "C" fn get_config_file(
    mut desktop: *mut FmDesktop,
    mut create_dir: gboolean,
) -> *mut c_char {
    let mut dir: *mut c_char = ptr::null_mut::<c_char>();
    let mut path: *mut c_char = ptr::null_mut::<c_char>();
    let mut i: c_int = 0;
    i = 0i32;
    while i < n_screens {
        if *desktops.offset(i as isize) == desktop {
            break;
        }
        i += 1;
    }
    if i >= n_screens {
        return ptr::null_mut::<c_char>();
    }
    dir = pcmanfm_get_profile_dir(create_dir);
    path = g_strdup_printf(
        b"%s/desktop-items-%u.conf\0" as *const u8 as *const gchar,
        dir,
        i,
    ) as *mut c_char;
    if emwm_layout != 0 && (*desktop).monitor >= 0i32 {
        let mut cached: *const c_char = g_object_get_data(
            desktop as *mut c_void as *mut GObject,
            b"emwm-profile\0" as *const u8 as *const gchar,
        ) as *const c_char;
        if cached.is_null() {
            let mut screen: *mut GdkScreen =
                gtk_widget_get_screen(desktop as *mut c_void as *mut GtkWidget);
            let mut name: *mut c_char =
                gdk_screen_get_monitor_plug_name(screen, (*desktop).monitor) as *mut c_char;
            if !name.is_null() && *name as c_int != 0 {
                let mut named: *mut c_char = emwm_named_profile(dir, name, path);
                g_object_set_data_full(
                    desktop as *mut c_void as *mut GObject,
                    b"emwm-profile\0" as *const u8 as *const gchar,
                    named as gpointer,
                    Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
                );
                cached = named;
            }
            g_free(name as gpointer);
        }
        if !cached.is_null() {
            g_free(path as gpointer);
            path = g_strdup_inline(cached);
        }
    }
    g_free(dir as gpointer);
    return path;
}
#[inline]
unsafe extern "C" fn desktop_item_new(
    mut model: *mut FmFolderModel,
    mut it: *mut GtkTreeIter,
) -> *mut FmDesktopItem {
    let mut item: *mut FmDesktopItem = ({
        let mut __s: gsize = mem::size_of::<FmDesktopItem>() as gsize;
        let mut __p: gpointer = ptr::null_mut::<c_void>();
        __p = g_slice_alloc(__s);
        memset(__p as *mut c_void, 0i32, __s as size_t);
        __p
    }) as *mut FmDesktopItem;
    let mut sl: *mut GSList = ptr::null_mut::<GSList>();
    fm_folder_model_set_item_userdata(model, it, item as gpointer);
    gtk_tree_model_get(
        model as *mut c_void as *mut GtkTreeModel,
        it,
        FM_FOLDER_MODEL_COL_INFO as c_int,
        &raw mut (*item).fi,
        -(1i32),
    );
    fm_file_info_ref((*item).fi);
    if !trash_can.is_null() && (*trash_can).fi == (*item).fi
        || !documents.is_null() && (*documents).fi == (*item).fi
    {
        (*item).set_is_special((0i32 == 0) as c_int as gboolean as gboolean);
    } else {
        sl = mounts;
        while !sl.is_null() {
            if (*((*sl).data as *mut FmDesktopExtraItem)).fi == (*item).fi {
                (*item).set_is_special((0i32 == 0) as c_int as gboolean as gboolean);
                (*item).set_is_mount((0i32 == 0) as c_int as gboolean as gboolean);
                break;
            } else {
                sl = (*sl).next;
            }
        }
    }
    return item;
}
#[inline]
unsafe extern "C" fn desktop_item_free(mut item: *mut FmDesktopItem) {
    if !(*item).fi.is_null() {
        fm_file_info_unref((*item).fi);
    }
    g_slice_free1(mem::size_of::<FmDesktopItem>() as gsize, item as gpointer);
}
unsafe extern "C" fn desktop_grid(mut desktop: *mut FmDesktop) -> EmwmGrid {
    let mut right: gboolean = if emwm_layout != 0 {
        emwm_right
    } else {
        (gtk_widget_get_direction(desktop as *mut c_void as *mut GtkWidget) as c_uint
            == GTK_TEXT_DIR_RTL as c_int as c_uint) as c_int
    };
    return EmwmGrid {
        work: (*desktop).working_area,
        x_margin: (*desktop).xmargin,
        y_margin: (*desktop).ymargin,
        cell_width: (*desktop).cell_w as gint,
        cell_height: (*desktop).cell_h as gint,
        right: right as gint,
    };
}
unsafe extern "C" fn item_geometry(mut item: *mut FmDesktopItem) -> EmwmGeometry {
    return EmwmGeometry {
        area: (*item).area,
        icon: (*item).icon_rect,
        text: (*item).text_rect,
    };
}
unsafe extern "C" fn set_geometry(mut item: *mut FmDesktopItem, mut geometry: EmwmGeometry) {
    (*item).area = geometry.area;
    (*item).icon_rect = geometry.icon;
    (*item).text_rect = geometry.text;
}
unsafe extern "C" fn calc_item_size(
    mut desktop: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
    mut icon: *mut GdkPixbuf,
) {
    let mut rc2: PangoRectangle = _PangoRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    pango_layout_set_text((*desktop).pl, ptr::null::<c_char>(), 0i32);
    pango_layout_set_height((*desktop).pl, (*desktop).pango_text_h as c_int);
    pango_layout_set_width((*desktop).pl, (*desktop).pango_text_w as c_int);
    pango_layout_set_text(
        (*desktop).pl,
        fm_file_info_get_disp_name((*item).fi),
        -(1i32),
    );
    pango_layout_get_pixel_extents(
        (*desktop).pl,
        ptr::null_mut::<PangoRectangle>(),
        &raw mut rc2,
    );
    pango_layout_set_text((*desktop).pl, ptr::null::<c_char>(), 0i32);
    let mut metrics: EmwmMetrics = EmwmMetrics {
        cell_width: (*desktop).cell_w as gint,
        y_pad: (*desktop).ypad as gint,
        icon_size: (*fm_config).big_icon_size,
        spacing: (*desktop).spacing as gint,
        pix_width: if !icon.is_null() {
            gdk_pixbuf_get_width(icon) as gint
        } else {
            (*fm_config).big_icon_size
        },
        pix_height: if !icon.is_null() {
            gdk_pixbuf_get_height(icon) as gint
        } else {
            (*fm_config).big_icon_size
        },
        label_width: rc2.width as gint,
        label_height: rc2.height as gint,
        label_y: rc2.y as gint,
    };
    set_geometry(
        item,
        emwm_icon_geometry((*item).area.x as gint, (*item).area.y as gint, metrics),
    );
}
#[inline]
unsafe extern "C" fn load_config(mut desktop: *mut FmDesktop) {
    let mut path: *mut c_char = ptr::null_mut::<c_char>();
    let mut kf: *mut GKeyFile = ptr::null_mut::<GKeyFile>();
    path = get_config_file(desktop, 0 as gboolean);
    if path.is_null() {
        return;
    }
    kf = g_key_file_new();
    if g_key_file_load_from_file(kf, path, G_KEY_FILE_NONE, ptr::null_mut::<*mut GError>()) != 0 {
        fm_app_config_load_desktop_config(kf, c"*".as_ptr(), &raw mut (*desktop).conf);
    }
    g_free(path as gpointer);
    g_key_file_free(kf);
}
#[inline]
unsafe extern "C" fn load_items(mut desktop: *mut FmDesktop) {
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut path: *mut c_char = ptr::null_mut::<c_char>();
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut profile_0: *mut EmwmProfile = ptr::null_mut::<EmwmProfile>();
    if (*desktop).model.is_null() {
        return;
    }
    model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
    if gtk_tree_model_get_iter_first(model, &raw mut it) == 0 {
        return;
    }
    path = get_config_file(desktop, 0 as gboolean);
    if path.is_null() {
        return;
    }
    profile_0 = emwm_profile_load(path);
    if !profile_0.is_null() {
        loop {
            let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
            let mut name: *const c_char = ptr::null::<c_char>();
            let mut icon: *mut GdkPixbuf = ptr::null_mut::<GdkPixbuf>();
            let mut out: c_int = 0;
            item = fm_folder_model_get_item_userdata((*desktop).model, &raw mut it)
                as *mut FmDesktopItem;
            name = fm_file_info_get_name((*item).fi);
            let mut point: EmwmSavedPoint =
                emwm_profile_position(profile_0, name, emwm_layout as gint, desktop_grid(desktop));
            if point.found != 0 {
                gtk_tree_model_get(
                    model,
                    &raw mut it,
                    FM_FOLDER_MODEL_COL_ICON as c_int,
                    &raw mut icon,
                    -(1i32),
                );
                (*desktop).fixed_items = g_list_prepend((*desktop).fixed_items, item as gpointer);
                (*item).set_fixed_pos((0i32 == 0) as c_int as gboolean as gboolean);
                (*item).area.x = point.x as c_int;
                (*item).area.y = point.y as c_int;
                if (*item).area.x < (*desktop).xmargin as c_int + (*desktop).working_area.x {
                    (*item).area.x = (*desktop).xmargin as c_int + (*desktop).working_area.x;
                }
                if (*item).area.y < (*desktop).ymargin as c_int + (*desktop).working_area.y {
                    (*item).area.y = (*desktop).ymargin as c_int + (*desktop).working_area.y;
                }
                calc_item_size(desktop, item, icon);
                out = (*item).area.x + (*item).area.width + (*desktop).xmargin as c_int
                    - (*desktop).working_area.width
                    - (*desktop).working_area.x;
                if out > 0i32 {
                    if out > (*item).area.x as gint - (*desktop).xmargin {
                        out = ((*item).area.x as gint - (*desktop).xmargin) as c_int;
                    }
                    (*item).area.x -= out;
                    (*item).icon_rect.x -= out;
                    (*item).text_rect.x -= out;
                }
                out = (*item).area.y + (*item).area.height + (*desktop).ymargin as c_int
                    - (*desktop).working_area.height
                    - (*desktop).working_area.y;
                if out > 0i32 {
                    if out > (*item).area.y as gint - (*desktop).ymargin {
                        out = ((*item).area.y as gint - (*desktop).ymargin) as c_int;
                    }
                    (*item).area.y -= out;
                    (*item).icon_rect.y -= out;
                    (*item).text_rect.y -= out;
                }
                if !icon.is_null() {
                    g_object_unref(icon as gpointer);
                }
            }
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        }
    }
    g_free(path as gpointer);
    emwm_profile_free(profile_0);
    queue_layout_items(desktop);
}
#[inline]
unsafe extern "C" fn unload_items(mut desktop: *mut FmDesktop) {
    g_list_free((*desktop).fixed_items);
    (*desktop).fixed_items = ptr::null_mut::<GList>();
    (*desktop).focus = ptr::null_mut::<FmDesktopItem>();
    (*desktop).drop_hilight = ptr::null_mut::<FmDesktopItem>();
    (*desktop).hover_item = ptr::null_mut::<FmDesktopItem>();
    g_object_set(
        desktop as *mut c_void as *mut GObject as gpointer,
        b"tooltip-text\0" as *const u8 as *const gchar,
        ptr::null_mut::<c_void>(),
        ptr::null_mut::<c_void>(),
    );
}
#[inline]
unsafe extern "C" fn reload_items(mut desktop: *mut FmDesktop) {
    unload_items(desktop);
    load_items(desktop);
}
unsafe extern "C" fn get_desktop_for_root_window(mut root: *mut GdkWindow) -> gint {
    let mut desktop: gint = -(1 as gint);
    let mut ret_type: Atom = 0;
    let mut len: gulong = 0;
    let mut after: gulong = 0;
    let mut format: c_int = 0;
    let mut prop: *mut guchar = ptr::null_mut::<guchar>();
    if XGetWindowProperty(
        gdk_x11_display_get_xdisplay(gdk_window_get_display(root)),
        gdk_x11_window_get_xid(root),
        XA_NET_CURRENT_DESKTOP,
        0 as c_long,
        1 as c_long,
        0i32,
        6 as Atom,
        &raw mut ret_type,
        &raw mut format,
        &raw mut len,
        &raw mut after,
        &raw mut prop,
    ) == 0i32
        && !prop.is_null()
    {
        desktop = *(prop as *mut guint32) as gint;
        XFree(prop as *mut c_void);
    }
    return desktop;
}
unsafe extern "C" fn save_item_pos(mut desktop: *mut FmDesktop) {
    let mut path: *mut c_char = get_config_file(desktop, (0i32 == 0) as c_int);
    if path.is_null() {
        return;
    }
    let mut config: *mut GString = g_string_sized_new(1024 as gsize);
    if (*desktop).conf.configured() != 0 {
        fm_app_config_save_desktop_config(config, c"*".as_ptr(), &raw mut (*desktop).conf);
    }
    let mut fixed: *mut GArray = g_array_new(
        0 as gboolean,
        0 as gboolean,
        mem::size_of::<EmwmFixedItem>() as guint,
    );
    let mut l: *mut GList = (*desktop).fixed_items;
    while !l.is_null() {
        let mut item: *mut FmDesktopItem = (*l).data as *mut FmDesktopItem;
        let mut file: *mut FmPath = fm_file_info_get_path((*item).fi);
        let mut entry: EmwmFixedItem = EmwmFixedItem {
            basename: fm_path_get_basename(file),
            x: (*item).area.x as gint,
            y: (*item).area.y as gint,
        };
        g_array_append_vals(fixed, &raw mut entry as gconstpointer, 1 as guint);
        l = (*l).next;
    }
    let mut present: *mut GPtrArray = g_ptr_array_new();
    let mut loaded: gboolean = 0 as gboolean;
    if emwm_layout != 0 && !(*desktop).model.is_null() {
        let mut iter: GtkTreeIter = _GtkTreeIter {
            stamp: 0,
            user_data: ptr::null_mut::<c_void>(),
            user_data2: ptr::null_mut::<c_void>(),
            user_data3: ptr::null_mut::<c_void>(),
        };
        let mut model: *mut GtkTreeModel = (*desktop).model as *mut c_void as *mut GtkTreeModel;
        loaded = fm_folder_is_loaded(fm_folder_model_get_folder((*desktop).model));
        if gtk_tree_model_get_iter_first(model, &raw mut iter) != 0 {
            loop {
                let mut item_0: *mut FmDesktopItem =
                    fm_folder_model_get_item_userdata((*desktop).model, &raw mut iter)
                        as *mut FmDesktopItem;
                let mut name: *const c_char = if !item_0.is_null() {
                    fm_file_info_get_name((*item_0).fi)
                } else {
                    ptr::null::<c_char>()
                };
                if !name.is_null() {
                    g_ptr_array_add(present, name as gpointer);
                }
                if !(gtk_tree_model_iter_next(model, &raw mut iter) != 0) {
                    break;
                }
            }
        }
    }
    let mut state: EmwmDesktopState = EmwmDesktopState {
        config: (*config).str_0,
        configured: (*desktop).conf.configured(),
        layout: emwm_layout as gint,
        active: ((*desktop).model != ptr::null_mut::<c_void>() as *mut FmFolderModel) as c_int,
        loaded: loaded as gint,
        grid: desktop_grid(desktop),
        fixed: (*fixed).data as *mut EmwmFixedItem,
        fixed_count: (*fixed).len as gsize,
        present: (*present).pdata as *const *const c_char,
        present_count: (*present).len as gsize,
    };
    emwm_save_positions(path, &raw mut state);
    g_ptr_array_free(present, (0i32 == 0) as c_int);
    g_array_free(fixed, (0i32 == 0) as c_int);
    if 0 != 0 {
        if 0i32 == 0 {
            g_string_free(config, (0i32 == 0) as c_int);
        } else {
            g_string_free_and_steal(config);
        };
    } else {
        g_string_free(config, (0i32 == 0) as c_int);
    };
    g_free(path as gpointer);
    (*desktop).conf.set_changed(0 as gint as gint);
}
unsafe extern "C" fn on_config_save_idle(mut _unused: gpointer) -> gboolean {
    let mut i: c_int = 0;
    if g_source_is_destroyed(g_main_current_source()) != 0 {
        return 0 as gboolean;
    }
    i = 0i32;
    while i < n_screens {
        if (**desktops.offset(i as isize)).conf.changed() != 0 {
            save_item_pos(*desktops.offset(i as isize));
        }
        i += 1;
    }
    idle_config_save = 0 as guint;
    return 0 as gboolean;
}
unsafe extern "C" fn queue_config_save(mut desktop: *mut FmDesktop) {
    (*desktop)
        .conf
        .set_configured((0i32 == 0) as c_int as gint as gint);
    (*desktop)
        .conf
        .set_changed((0i32 == 0) as c_int as gint as gint);
    if idle_config_save == 0 as guint {
        idle_config_save = gdk_threads_add_idle(
            Some(on_config_save_idle as unsafe extern "C" fn(gpointer) -> gboolean),
            ptr::null_mut::<c_void>(),
        );
    }
}
unsafe extern "C" fn get_selected_items(
    mut desktop: *mut FmDesktop,
    mut n_items: *mut c_int,
) -> *mut GList {
    let mut items: *mut GList = ptr::null_mut::<GList>();
    let mut n: c_int = 0i32;
    let mut focus: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut model: *mut GtkTreeModel = if !(*desktop).model.is_null() {
        (*desktop).model as *mut c_void as *mut GtkTreeModel
    } else {
        ptr::null_mut::<GtkTreeModel>()
    };
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if !model.is_null() && gtk_tree_model_get_iter_first(model, &raw mut it) != 0 {
        loop {
            let mut item: *mut FmDesktopItem =
                fm_folder_model_get_item_userdata((*desktop).model, &raw mut it)
                    as *mut FmDesktopItem;
            if (*item).is_selected() != 0 {
                if ({
                    let mut _g_boolean_var_32: c_int = 0 as c_int;
                    if item != (*desktop).focus {
                        _g_boolean_var_32 = 1i32;
                    }
                    _g_boolean_var_32
                }) as c_long
                    != 0
                {
                    items = g_list_prepend(items, item as gpointer);
                    n += 1;
                } else {
                    focus = item;
                }
            }
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        }
    }
    items = g_list_reverse(items);
    if !focus.is_null() {
        items = g_list_prepend(items, focus as gpointer);
        n += 1;
    }
    if !n_items.is_null() {
        *n_items = n;
    }
    return items;
}
unsafe extern "C" fn copy_desktop_config(
    mut dst: *mut FmDesktopConfig,
    mut src: *mut FmDesktopConfig,
) {
    let mut i: c_int = 0;
    (*dst).wallpaper_mode = (*src).wallpaper_mode;
    (*dst).wallpaper = g_strdup_inline((*src).wallpaper);
    if (*src).wallpapers_configured > 0i32 {
        (*dst).wallpapers = ({
            let mut __n: gsize = (*src).wallpapers_configured as gsize;
            let mut __s: gsize = mem::size_of::<*mut c_char>() as gsize;
            let mut __p: gpointer = ptr::null_mut::<c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0
                && (__s == 0 as gsize
                    || __n
                        <= (9223372036854775807 as c_long as gsize)
                            .wrapping_mul(2 as gsize)
                            .wrapping_add(1 as gsize)
                            .wrapping_div(__s))
            {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut c_char;
        i = 0i32;
        while i < (*src).wallpapers_configured {
            let ref mut fresh11 = *(*dst).wallpapers.offset(i as isize);
            *fresh11 = g_strdup_inline(*(*src).wallpapers.offset(i as isize));
            i += 1;
        }
    } else {
        (*dst).wallpapers = ptr::null_mut::<*mut c_char>();
    }
    (*dst).wallpapers_configured = (*src).wallpapers_configured;
    (*dst).wallpaper_common = (*src).wallpaper_common;
    (*dst).show_wm_menu = (*src).show_wm_menu;
    (*dst).set_configured((0i32 == 0) as c_int as gint as gint);
    (*dst).set_changed(0 as gint as gint);
    (*dst).desktop_bg = (*src).desktop_bg;
    (*dst).desktop_fg = (*src).desktop_fg;
    (*dst).desktop_shadow = (*src).desktop_shadow;
    (*dst).desktop_font = g_strdup_inline((*src).desktop_font);
    (*dst).desktop_sort_type = (*src).desktop_sort_type;
    (*dst).desktop_sort_by = (*src).desktop_sort_by;
    (*dst).folder = g_strdup_inline((*src).folder);
    (*dst).show_documents = (*src).show_documents;
    (*dst).show_trash = (*src).show_trash;
    (*dst).show_mounts = (*src).show_mounts;
}
static mut vol_mon: *mut GVolumeMonitor = ptr::null::<GVolumeMonitor>() as *mut GVolumeMonitor;
unsafe extern "C" fn on_idle_extra_item_add(mut user_data: gpointer) -> gboolean {
    let mut item: *mut FmDesktopExtraItem = user_data as *mut FmDesktopExtraItem;
    let mut i: c_int = 0;
    if !(*item).mount.is_null() {
        mounts = g_slist_append(mounts, item as gpointer);
        i = 0i32;
        while i < n_screens {
            if (**desktops.offset(i as isize)).monitor >= 0i32
                && (**desktops.offset(i as isize)).conf.show_mounts != 0
                && !(**desktops.offset(i as isize)).model.is_null()
            {
                fm_folder_model_extra_file_add(
                    (**desktops.offset(i as isize)).model,
                    (*item).fi,
                    FM_FOLDER_MODEL_ITEMPOS_POST,
                );
            }
            i += 1;
        }
    } else if item == documents {
        i = 0i32;
        while i < n_screens {
            if (**desktops.offset(i as isize)).monitor >= 0i32
                && (**desktops.offset(i as isize)).conf.show_documents != 0
                && !(**desktops.offset(i as isize)).model.is_null()
            {
                fm_folder_model_extra_file_add(
                    (**desktops.offset(i as isize)).model,
                    (*item).fi,
                    FM_FOLDER_MODEL_ITEMPOS_PRE,
                );
                reload_items(*desktops.offset(i as isize));
            }
            i += 1;
        }
    } else if item == trash_can {
        i = 0i32;
        while i < n_screens {
            if (**desktops.offset(i as isize)).monitor >= 0i32
                && (**desktops.offset(i as isize)).conf.show_trash != 0
                && !(**desktops.offset(i as isize)).model.is_null()
            {
                fm_folder_model_extra_file_add(
                    (**desktops.offset(i as isize)).model,
                    (*item).fi,
                    FM_FOLDER_MODEL_ITEMPOS_PRE,
                );
                reload_items(*desktops.offset(i as isize));
            }
            i += 1;
        }
    } else {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_CRITICAL,
            b"got file info for unknown desktop item %s\0" as *const u8 as *const gchar,
            fm_path_get_basename((*item).path),
        );
        _free_extra_item(item);
    }
    return 0 as gboolean;
}
static mut trash_is_empty: gboolean = 0 as gboolean;
unsafe extern "C" fn _update_trash_icon(mut item: *mut FmDesktopExtraItem) -> gboolean {
    let mut gf: *mut GFile = fm_file_new_for_uri(c"trash:///".as_ptr());
    let mut inf: *mut GFileInfo = g_file_query_info(
        gf,
        c"trash::item-count".as_ptr(),
        G_FILE_QUERY_INFO_NONE,
        ptr::null_mut::<GCancellable>(),
        ptr::null_mut::<*mut GError>(),
    );
    let mut icon_name: *const c_char = ptr::null::<c_char>();
    let mut icon: *mut GIcon = ptr::null_mut::<GIcon>();
    let mut n: guint32 = 0;
    g_object_unref(gf as gpointer);
    if inf.is_null() {
        return 0 as gboolean;
    }
    n = g_file_info_get_attribute_uint32(inf, c"trash::item-count".as_ptr());
    g_object_unref(inf as gpointer);
    if n > 0 as guint32 && trash_is_empty != 0 {
        icon_name = c"user-trash-full".as_ptr();
    } else if n == 0 as guint32 && trash_is_empty == 0 {
        icon_name = c"user-trash".as_ptr();
    } else {
        return 0 as gboolean;
    }
    trash_is_empty = (n == 0 as guint32) as c_int as gboolean;
    icon = g_themed_icon_new_with_default_fallbacks(icon_name);
    fm_file_info_set_icon((*item).fi, icon);
    g_object_unref(icon as gpointer);
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn on_file_info_job_finished(
    mut job: *mut FmFileInfoJob,
    mut user_data: gpointer,
) {
    let mut item: *mut FmDesktopExtraItem = user_data as *mut FmDesktopExtraItem;
    let mut fi: *mut FmFileInfo = ptr::null_mut::<FmFileInfo>();
    let mut name: *mut c_char = ptr::null_mut::<c_char>();
    let mut icon: *mut GIcon = ptr::null_mut::<GIcon>();
    if fm_file_info_list_is_empty((*job).file_infos) != 0 {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_CRITICAL,
            b"FmFileInfoJob failed on desktop mount update\0" as *const u8 as *const gchar,
        );
        _free_extra_item(item);
        return;
    }
    fi = fm_file_info_list_peek_head((*job).file_infos);
    (*item).fi = fm_file_info_ref(fi);
    (*item).job = ptr::null_mut::<FmFileInfoJob>();
    g_object_unref(job as gpointer);
    if !(*item).mount.is_null() {
        name = g_mount_get_name((*item).mount);
        fm_file_info_set_disp_name(fi, name);
        g_free(name as gpointer);
        icon = g_mount_get_icon((*item).mount);
        fm_file_info_set_icon(fi, icon);
        g_object_unref(icon as gpointer);
    } else if item == trash_can {
        _update_trash_icon(item);
    }
    gdk_threads_add_idle(
        Some(on_idle_extra_item_add as unsafe extern "C" fn(gpointer) -> gboolean),
        item as gpointer,
    );
}
unsafe extern "C" fn _free_extra_item(mut item: *mut FmDesktopExtraItem) {
    if !(*item).mount.is_null() {
        g_object_unref((*item).mount as gpointer);
    }
    fm_path_unref((*item).path);
    if !(*item).fi.is_null() {
        fm_file_info_unref((*item).fi);
    }
    if !(*item).job.is_null() {
        g_signal_handlers_disconnect_matched(
            (*item).job as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFileInfoJob, gpointer) -> ()>,
                gpointer,
            >(Some(
                on_file_info_job_finished
                    as unsafe extern "C" fn(*mut FmFileInfoJob, gpointer) -> (),
            )),
            item as gpointer,
        );
        fm_job_cancel((*item).job as *mut c_void as *mut FmJob);
        g_object_unref((*item).job as gpointer);
    }
    g_slice_free1(
        mem::size_of::<FmDesktopExtraItem>() as gsize,
        item as gpointer,
    );
}
unsafe extern "C" fn on_mount_added(
    mut volume_monitor: *mut GVolumeMonitor,
    mut mount: *mut GMount,
    mut _unused: gpointer,
) {
    let mut file: *mut GFile = ptr::null_mut::<GFile>();
    let mut item: *mut FmDesktopExtraItem = ptr::null_mut::<FmDesktopExtraItem>();
    item = g_slice_alloc(mem::size_of::<FmDesktopExtraItem>() as gsize) as *mut FmDesktopExtraItem;
    (*item).mount = g_object_ref(mount as gpointer) as *mut GMount as *mut GMount;
    file = g_mount_get_root(mount);
    (*item).path = fm_path_new_for_gfile(file);
    g_object_unref(file as gpointer);
    (*item).fi = ptr::null_mut::<FmFileInfo>();
    (*item).job = fm_file_info_job_new(ptr::null_mut::<FmPathList>(), FM_FILE_INFO_JOB_NONE);
    fm_file_info_job_add((*item).job, (*item).path);
    g_signal_connect_data(
        (*item).job as gpointer,
        b"finished\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut FmFileInfoJob, gpointer) -> ()>, GCallback>(
            Some(
                on_file_info_job_finished
                    as unsafe extern "C" fn(*mut FmFileInfoJob, gpointer) -> (),
            ),
        ),
        item as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    if fm_job_run_async((*item).job as *mut c_void as *mut FmJob) == 0 {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_CRITICAL,
            b"fm_job_run_async() failed on desktop mount update\0" as *const u8 as *const gchar,
        );
        _free_extra_item(item);
    }
}
unsafe extern "C" fn on_idle_extra_item_remove(mut user_data: gpointer) -> gboolean {
    let mut mount: *mut GMount = user_data as *mut GMount;
    let mut sl: *mut GSList = ptr::null_mut::<GSList>();
    let mut item: *mut FmDesktopExtraItem = ptr::null_mut::<FmDesktopExtraItem>();
    let mut i: c_int = 0;
    sl = mounts;
    while !sl.is_null() {
        item = (*sl).data as *mut FmDesktopExtraItem;
        if (*item).mount == mount {
            break;
        }
        sl = (*sl).next;
    }
    if !sl.is_null() {
        i = 0i32;
        while i < n_screens {
            if (**desktops.offset(i as isize)).monitor >= 0i32
                && (**desktops.offset(i as isize)).conf.show_mounts != 0
                && !(**desktops.offset(i as isize)).model.is_null()
            {
                fm_folder_model_extra_file_remove(
                    (**desktops.offset(i as isize)).model,
                    (*item).fi,
                );
            }
            i += 1;
        }
        mounts = g_slist_delete_link(mounts, sl);
        _free_extra_item(item);
    } else {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_WARNING,
            b"got unmount for unknown desktop item\0" as *const u8 as *const gchar,
        );
    }
    g_object_unref(mount as gpointer);
    return 0 as gboolean;
}
unsafe extern "C" fn on_mount_removed(
    mut volume_monitor: *mut GVolumeMonitor,
    mut mount: *mut GMount,
    mut _unused: gpointer,
) {
    gdk_threads_add_idle(
        Some(on_idle_extra_item_remove as unsafe extern "C" fn(gpointer) -> gboolean),
        g_object_ref(mount as gpointer) as *mut GMount as gpointer,
    );
}
static mut trash_monitor: *mut GFileMonitor = ptr::null::<GFileMonitor>() as *mut GFileMonitor;
unsafe extern "C" fn on_trash_changed(
    mut monitor: *mut GFileMonitor,
    mut gf: *mut GFile,
    mut other: *mut GFile,
    mut evt: GFileMonitorEvent,
    mut item: *mut FmDesktopExtraItem,
) {
    let mut i: c_int = 0;
    if _update_trash_icon(item) == 0 {
        return;
    }
    i = 0i32;
    while i < n_screens {
        if (**desktops.offset(i as isize)).monitor >= 0i32
            && (**desktops.offset(i as isize)).conf.show_trash != 0
            && !(**desktops.offset(i as isize)).model.is_null()
        {
            fm_folder_model_file_changed((**desktops.offset(i as isize)).model, (*item).fi);
        }
        i += 1;
    }
}
unsafe extern "C" fn _add_extra_item(mut path_str: *const c_char) -> *mut FmDesktopExtraItem {
    let mut item: *mut FmDesktopExtraItem = ptr::null_mut::<FmDesktopExtraItem>();
    if path_str.is_null() {
        return ptr::null_mut::<FmDesktopExtraItem>();
    }
    item = g_slice_alloc(mem::size_of::<FmDesktopExtraItem>() as gsize) as *mut FmDesktopExtraItem;
    (*item).mount = ptr::null_mut::<GMount>();
    (*item).path = fm_path_new_for_str(path_str);
    (*item).fi = ptr::null_mut::<FmFileInfo>();
    (*item).job = fm_file_info_job_new(ptr::null_mut::<FmPathList>(), FM_FILE_INFO_JOB_NONE);
    fm_file_info_job_add((*item).job, (*item).path);
    g_signal_connect_data(
        (*item).job as gpointer,
        b"finished\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut FmFileInfoJob, gpointer) -> ()>, GCallback>(
            Some(
                on_file_info_job_finished
                    as unsafe extern "C" fn(*mut FmFileInfoJob, gpointer) -> (),
            ),
        ),
        item as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    if fm_job_run_async((*item).job as *mut c_void as *mut FmJob) == 0 {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_CRITICAL,
            b"fm_job_run_async() failed on desktop special item update\0" as *const u8
                as *const gchar,
        );
        _free_extra_item(item);
        item = ptr::null_mut::<FmDesktopExtraItem>();
    }
    return item;
}
#[inline(never)]
unsafe extern "C" fn fm_desktop_item_accessible_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        atk_object_get_type(),
        g_intern_static_string(b"FmDesktopItemAccessible\0" as *const u8 as *const gchar),
        mem::size_of::<FmDesktopItemAccessibleClass>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(mem::transmute::<
            Option<unsafe extern "C" fn(gpointer) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            fm_desktop_item_accessible_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
        ))),
        mem::size_of::<FmDesktopItemAccessible>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmDesktopItemAccessible) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                fm_desktop_item_accessible_init
                    as unsafe extern "C" fn(*mut FmDesktopItemAccessible) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInterfaceInitFunc>(
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut AtkComponentIface) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                atk_component_item_interface_init
                    as unsafe extern "C" fn(*mut AtkComponentIface) -> (),
            )),
        ),
        interface_finalize: None,
        interface_data: ptr::null_mut::<c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        atk_component_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInterfaceInitFunc>(
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut AtkActionIface) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                atk_action_item_interface_init as unsafe extern "C" fn(*mut AtkActionIface) -> (),
            )),
        ),
        interface_finalize: None,
        interface_data: ptr::null_mut::<c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        atk_action_get_type(),
        &raw const g_implement_interface_info_0,
    );
    let g_implement_interface_info_1: GInterfaceInfo = _GInterfaceInfo {
        interface_init: mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInterfaceInitFunc>(
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut AtkImageIface) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                atk_image_item_interface_init as unsafe extern "C" fn(*mut AtkImageIface) -> (),
            )),
        ),
        interface_finalize: None,
        interface_data: ptr::null_mut::<c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        atk_image_get_type(),
        &raw const g_implement_interface_info_1,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fm_desktop_item_accessible_get_type() -> GType {
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
        let mut g_define_type_id: GType = fm_desktop_item_accessible_get_type_once();
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
unsafe extern "C" fn fm_desktop_item_accessible_class_intern_init(mut klass: gpointer) {
    fm_desktop_item_accessible_parent_class = g_type_class_peek_parent(klass);
    if FmDesktopItemAccessible_private_offset != 0i32 {
        g_type_class_adjust_private_offset(klass, &raw mut FmDesktopItemAccessible_private_offset);
    }
    fm_desktop_item_accessible_class_init(klass as *mut FmDesktopItemAccessibleClass);
}
static mut FmDesktopItemAccessible_private_offset: gint = 0;
static mut fm_desktop_item_accessible_parent_class: gpointer = ptr::null::<c_void>() as *mut c_void;
unsafe extern "C" fn fm_desktop_item_accessible_init(mut item: *mut FmDesktopItemAccessible) {
    (*item).state_set = atk_state_set_new();
    atk_state_set_add_state((*item).state_set, ATK_STATE_ENABLED);
    atk_state_set_add_state((*item).state_set, ATK_STATE_FOCUSABLE);
    atk_state_set_add_state((*item).state_set, ATK_STATE_SENSITIVE);
    atk_state_set_add_state((*item).state_set, ATK_STATE_SELECTABLE);
    atk_state_set_add_state((*item).state_set, ATK_STATE_VISIBLE);
    (*item).action_idle_handler = 0 as guint;
}
unsafe extern "C" fn fm_desktop_item_accessible_finalize(mut object: *mut GObject) {
    let mut item: *mut FmDesktopItemAccessible = object as *mut FmDesktopItemAccessible;
    if !(*item).widget.is_null() {
        g_object_remove_weak_pointer(
            (*item).widget as *mut c_void as *mut GObject,
            &raw mut (*item).widget as gpointer as *mut gpointer,
        );
    }
    if !(*item).state_set.is_null() {
        g_object_unref((*item).state_set as gpointer);
    }
    if (*item).action_idle_handler != 0 {
        g_source_remove((*item).action_idle_handler);
        (*item).action_idle_handler = 0 as guint;
    }
}
unsafe extern "C" fn fm_desktop_item_accessible_new(
    mut desktop: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
) -> *mut FmDesktopItemAccessible {
    let mut name: *const c_char = ptr::null::<c_char>();
    let mut atk_item: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    name = fm_file_info_get_disp_name((*item).fi);
    atk_item = g_object_new(
        fm_desktop_item_accessible_get_type(),
        b"accessible-name\0" as *const u8 as *const gchar,
        name,
        ptr::null_mut::<c_void>(),
    ) as *mut FmDesktopItemAccessible;
    (*atk_item).item = item;
    (*atk_item).widget = desktop as *mut c_void as *mut GtkWidget;
    g_object_add_weak_pointer(
        desktop as *mut c_void as *mut GObject,
        &raw mut (*atk_item).widget as gpointer as *mut gpointer,
    );
    return atk_item;
}
unsafe extern "C" fn fm_desktop_item_accessible_get_extents(
    mut component: *mut AtkComponent,
    mut x: *mut gint,
    mut y: *mut gint,
    mut width: *mut gint,
    mut height: *mut gint,
    mut coord_type: AtkCoordType,
) {
    let mut item: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    let mut parent_obj: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut l_x: gint = 0;
    let mut l_y: gint = 0;
    item = component as *mut c_void as *mut FmDesktopItemAccessible;
    if (*item).widget.is_null() {
        return;
    }
    if atk_state_set_contains_state((*item).state_set, ATK_STATE_DEFUNCT) != 0 {
        return;
    }
    *width = (*(*item).item).area.width as gint;
    *height = (*(*item).item).area.height as gint;
    parent_obj = gtk_widget_get_accessible((*item).widget);
    atk_component_get_extents(
        parent_obj as *mut c_void as *mut AtkComponent,
        &raw mut l_x,
        &raw mut l_y,
        ptr::null_mut::<gint>(),
        ptr::null_mut::<gint>(),
        coord_type,
    );
    *x = (l_x as c_int + (*(*item).item).area.x) as gint;
    *y = (l_y as c_int + (*(*item).item).area.y) as gint;
}
unsafe extern "C" fn fm_desktop_item_accessible_grab_focus(
    mut component: *mut AtkComponent,
) -> gboolean {
    let mut item: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    item = component as *mut c_void as *mut FmDesktopItemAccessible;
    if (*item).widget.is_null() {
        return 0 as gboolean;
    }
    if atk_state_set_contains_state((*item).state_set, ATK_STATE_DEFUNCT) != 0 {
        return 0 as gboolean;
    }
    gtk_widget_grab_focus((*item).widget);
    set_focused_item(
        (*item).widget as *mut c_void as *mut FmDesktop,
        (*item).item,
    );
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn atk_component_item_interface_init(mut iface: *mut AtkComponentIface) {
    (*iface).get_extents = Some(
        fm_desktop_item_accessible_get_extents
            as unsafe extern "C" fn(
                *mut AtkComponent,
                *mut gint,
                *mut gint,
                *mut gint,
                *mut gint,
                AtkCoordType,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut AtkComponent,
                *mut gint,
                *mut gint,
                *mut gint,
                *mut gint,
                AtkCoordType,
            ) -> (),
        >;
    (*iface).grab_focus = Some(
        fm_desktop_item_accessible_grab_focus
            as unsafe extern "C" fn(*mut AtkComponent) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut AtkComponent) -> gboolean>;
}
unsafe extern "C" fn fm_desktop_item_get_tree_path(
    mut self_0: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
) -> *mut GtkTreePath {
    let mut model: *mut GtkTreeModel = if !(*self_0).model.is_null() {
        (*self_0).model as *mut c_void as *mut GtkTreeModel
    } else {
        ptr::null_mut::<GtkTreeModel>()
    };
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if !model.is_null() && gtk_tree_model_get_iter_first(model, &raw mut it) != 0 {
        loop {
            if fm_folder_model_get_item_userdata((*self_0).model, &raw mut it) == item as gpointer {
                return gtk_tree_model_get_path(model, &raw mut it);
            }
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        }
    }
    return ptr::null_mut::<GtkTreePath>();
}
unsafe extern "C" fn fm_desktop_item_accessible_idle_do_action(mut data: gpointer) -> gboolean {
    let mut item: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    let mut tp: *mut GtkTreePath = ptr::null_mut::<GtkTreePath>();
    if g_source_is_destroyed(g_main_current_source()) != 0 {
        return 0 as gboolean;
    }
    item = data as *mut FmDesktopItemAccessible;
    (*item).action_idle_handler = 0 as guint;
    if !(*item).widget.is_null() {
        tp = fm_desktop_item_get_tree_path(
            (*item).widget as *mut c_void as *mut FmDesktop,
            (*item).item,
        );
        if !tp.is_null() {
            fm_folder_view_item_clicked(
                (*item).widget as *mut c_void as *mut FmFolderView,
                tp,
                (if (*item).action_type == 0i32 {
                    FM_FV_ACTIVATED as c_int
                } else {
                    FM_FV_CONTEXT_MENU as c_int
                }) as FmFolderViewClickType,
            );
            gtk_tree_path_free(tp);
        }
    }
    return 0 as gboolean;
}
unsafe extern "C" fn fm_desktop_item_accessible_action_do_action(
    mut action: *mut AtkAction,
    mut i: gint,
) -> gboolean {
    let mut item: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    if i != 0i32 && i != 1i32 {
        return 0 as gboolean;
    }
    item = action as *mut c_void as *mut FmDesktopItemAccessible;
    if (*item).widget.is_null() {
        return 0 as gboolean;
    }
    if atk_state_set_contains_state((*item).state_set, ATK_STATE_DEFUNCT) != 0 {
        return 0 as gboolean;
    }
    if (*item).action_idle_handler == 0 {
        (*item).action_type = i;
        (*item).action_idle_handler = gdk_threads_add_idle(
            Some(
                fm_desktop_item_accessible_idle_do_action
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            item as gpointer,
        );
    }
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn fm_desktop_item_accessible_action_get_n_actions(
    mut action: *mut AtkAction,
) -> gint {
    return 2 as gint;
}
unsafe extern "C" fn fm_desktop_item_accessible_action_get_description(
    mut action: *mut AtkAction,
    mut i: gint,
) -> *const gchar {
    if i == 0i32 {
        return dcgettext(ptr::null::<c_char>(), c"Activate file".as_ptr(), 5 as c_int);
    } else if i == 1i32 {
        return dcgettext(
            ptr::null::<c_char>(),
            c"Show file menu".as_ptr(),
            5 as c_int,
        );
    }
    return ptr::null::<gchar>();
}
unsafe extern "C" fn fm_desktop_item_accessible_action_get_name(
    mut action: *mut AtkAction,
    mut i: gint,
) -> *const gchar {
    if i == 0i32 {
        return b"activate\0" as *const u8 as *const gchar;
    }
    if i == 1i32 {
        return b"menu\0" as *const u8 as *const gchar;
    }
    return ptr::null::<gchar>();
}
unsafe extern "C" fn atk_action_item_interface_init(mut iface: *mut AtkActionIface) {
    (*iface).do_action = Some(
        fm_desktop_item_accessible_action_do_action
            as unsafe extern "C" fn(*mut AtkAction, gint) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut AtkAction, gint) -> gboolean>;
    (*iface).get_n_actions = Some(
        fm_desktop_item_accessible_action_get_n_actions
            as unsafe extern "C" fn(*mut AtkAction) -> gint,
    ) as Option<unsafe extern "C" fn(*mut AtkAction) -> gint>;
    (*iface).get_description = Some(
        fm_desktop_item_accessible_action_get_description
            as unsafe extern "C" fn(*mut AtkAction, gint) -> *const gchar,
    )
        as Option<unsafe extern "C" fn(*mut AtkAction, gint) -> *const gchar>;
    (*iface).get_name = Some(
        fm_desktop_item_accessible_action_get_name
            as unsafe extern "C" fn(*mut AtkAction, gint) -> *const gchar,
    ) as Option<unsafe extern "C" fn(*mut AtkAction, gint) -> *const gchar>;
}
unsafe extern "C" fn fm_desktop_item_accessible_image_get_image_description(
    mut image: *mut AtkImage,
) -> *const gchar {
    let mut item: *mut FmDesktopItemAccessible =
        image as *mut c_void as *mut FmDesktopItemAccessible;
    return fm_file_info_get_desc((*(*item).item).fi) as *const gchar;
}
unsafe extern "C" fn fm_desktop_item_accessible_image_get_image_size(
    mut image: *mut AtkImage,
    mut width: *mut gint,
    mut height: *mut gint,
) {
    let mut item: *mut FmDesktopItemAccessible =
        image as *mut c_void as *mut FmDesktopItemAccessible;
    if (*item).widget.is_null() {
        return;
    }
    if atk_state_set_contains_state((*item).state_set, ATK_STATE_DEFUNCT) != 0 {
        return;
    }
    *width = (*(*item).item).icon_rect.width as gint;
    *height = (*(*item).item).icon_rect.height as gint;
}
unsafe extern "C" fn fm_desktop_item_accessible_image_get_image_position(
    mut image: *mut AtkImage,
    mut x: *mut gint,
    mut y: *mut gint,
    mut coord_type: AtkCoordType,
) {
    let mut item: *mut FmDesktopItemAccessible =
        image as *mut c_void as *mut FmDesktopItemAccessible;
    let mut parent_obj: *mut AtkObject = ptr::null_mut::<AtkObject>();
    if (*item).widget.is_null() {
        return;
    }
    if atk_state_set_contains_state((*item).state_set, ATK_STATE_DEFUNCT) != 0 {
        return;
    }
    parent_obj = gtk_widget_get_accessible((*item).widget);
    atk_component_get_extents(
        parent_obj as *mut c_void as *mut AtkComponent,
        x,
        y,
        ptr::null_mut::<gint>(),
        ptr::null_mut::<gint>(),
        coord_type,
    );
    *x += (*(*item).item).icon_rect.x - (*(*item).item).area.x;
    *y += (*(*item).item).icon_rect.y - (*(*item).item).area.y;
}
unsafe extern "C" fn atk_image_item_interface_init(mut iface: *mut AtkImageIface) {
    (*iface).get_image_description = Some(
        fm_desktop_item_accessible_image_get_image_description
            as unsafe extern "C" fn(*mut AtkImage) -> *const gchar,
    )
        as Option<unsafe extern "C" fn(*mut AtkImage) -> *const gchar>;
    (*iface).get_image_size = Some(
        fm_desktop_item_accessible_image_get_image_size
            as unsafe extern "C" fn(*mut AtkImage, *mut gint, *mut gint) -> (),
    )
        as Option<unsafe extern "C" fn(*mut AtkImage, *mut gint, *mut gint) -> ()>;
    (*iface).get_image_position = Some(
        fm_desktop_item_accessible_image_get_image_position
            as unsafe extern "C" fn(*mut AtkImage, *mut gint, *mut gint, AtkCoordType) -> (),
    )
        as Option<unsafe extern "C" fn(*mut AtkImage, *mut gint, *mut gint, AtkCoordType) -> ()>;
}
unsafe extern "C" fn fm_desktop_item_accessible_get_parent(
    mut obj: *mut AtkObject,
) -> *mut AtkObject {
    let mut item: *mut FmDesktopItemAccessible = obj as *mut c_void as *mut FmDesktopItemAccessible;
    return if !(*item).widget.is_null() {
        gtk_widget_get_accessible((*item).widget)
    } else {
        ptr::null_mut::<AtkObject>()
    };
}
unsafe extern "C" fn fm_desktop_item_accessible_get_index_in_parent(
    mut obj: *mut AtkObject,
) -> gint {
    let mut item: *mut FmDesktopItemAccessible = obj as *mut c_void as *mut FmDesktopItemAccessible;
    return if !(*item).widget.is_null() {
        fm_desktop_accessible_index((*item).widget, item as gpointer)
    } else {
        -(1 as gint)
    };
}
unsafe extern "C" fn fm_desktop_item_accessible_ref_state_set(
    mut obj: *mut AtkObject,
) -> *mut AtkStateSet {
    let mut item: *mut FmDesktopItemAccessible = obj as *mut c_void as *mut FmDesktopItemAccessible;
    let mut desktop: *mut FmDesktop = ptr::null_mut::<FmDesktop>();
    if !(*item).widget.is_null() {
        desktop = (*item).widget as *mut FmDesktop;
        if (*desktop).focus == (*item).item {
            atk_state_set_add_state((*item).state_set, ATK_STATE_FOCUSED);
        } else {
            atk_state_set_remove_state((*item).state_set, ATK_STATE_FOCUSED);
        }
    }
    if (*(*item).item).is_selected() != 0 {
        atk_state_set_add_state((*item).state_set, ATK_STATE_SELECTED);
    } else {
        atk_state_set_remove_state((*item).state_set, ATK_STATE_SELECTED);
    }
    return g_object_ref((*item).state_set as gpointer) as *mut AtkStateSet;
}
unsafe extern "C" fn fm_desktop_item_accessible_class_init(
    mut klass: *mut FmDesktopItemAccessibleClass,
) {
    let mut gobject_class: *mut GObjectClass = klass as *mut c_void as *mut GObjectClass;
    let mut atk_class: *mut AtkObjectClass = klass as *mut c_void as *mut AtkObjectClass;
    (*gobject_class).finalize =
        Some(fm_desktop_item_accessible_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*atk_class).get_index_in_parent = Some(
        fm_desktop_item_accessible_get_index_in_parent
            as unsafe extern "C" fn(*mut AtkObject) -> gint,
    )
        as Option<unsafe extern "C" fn(*mut AtkObject) -> gint>;
    (*atk_class).get_parent = Some(
        fm_desktop_item_accessible_get_parent
            as unsafe extern "C" fn(*mut AtkObject) -> *mut AtkObject,
    )
        as Option<unsafe extern "C" fn(*mut AtkObject) -> *mut AtkObject>;
    (*atk_class).ref_state_set = Some(
        fm_desktop_item_accessible_ref_state_set
            as unsafe extern "C" fn(*mut AtkObject) -> *mut AtkStateSet,
    )
        as Option<unsafe extern "C" fn(*mut AtkObject) -> *mut AtkStateSet>;
}
unsafe extern "C" fn fm_desktop_item_accessible_add_state(
    mut item: *mut FmDesktopItemAccessible,
    mut state_type: AtkStateType,
) -> gboolean {
    let mut rc: gboolean = 0;
    rc = atk_state_set_add_state((*item).state_set, state_type);
    atk_object_notify_state_change(
        item as *mut c_void as *mut AtkObject,
        state_type as AtkState,
        (0i32 == 0) as c_int,
    );
    if state_type as c_uint == ATK_STATE_VISIBLE as c_int as c_uint {
        g_signal_emit_by_name(
            item as gpointer,
            b"visible-data-changed\0" as *const u8 as *const gchar,
        );
    }
    return rc;
}
static mut fm_desktop_accessible_parent_class: gpointer = ptr::null::<c_void>() as *mut c_void;
unsafe extern "C" fn fm_desktop_accessible_class_intern_init(mut klass: gpointer) {
    fm_desktop_accessible_parent_class = g_type_class_peek_parent(klass);
    fm_desktop_accessible_class_init(klass as *mut FmDesktopAccessibleClass);
}
unsafe extern "C" fn fm_desktop_accessible_get_type() -> GType {
    static mut type_id_volatile: gsize = 0 as gsize;
    if ({
        if 0i32 != 0 {
            type_id_volatile;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut type_id_volatile;
            *(&raw mut gapg_temp_newval as *mut gsize) =
                std::sync::atomic::AtomicUsize::from_ptr(gapg_temp_atomic.cast())
                    .load(std::sync::atomic::Ordering::SeqCst) as _;
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut type_id_volatile as *mut c_void) != 0) as c_int
    }) != 0
    {
        let mut factory: *mut AtkObjectFactory = ptr::null_mut::<AtkObjectFactory>();
        let mut query: GTypeQuery = _GTypeQuery {
            type_0: 0,
            type_name: ptr::null::<gchar>(),
            class_size: 0,
            instance_size: 0,
        };
        let mut derived_atk_type: GType = 0;
        let mut g_define_type_id: GType = 0;
        factory = atk_registry_get_factory(
            atk_get_default_registry(),
            g_type_parent(fm_desktop_get_type()),
        );
        derived_atk_type = atk_object_factory_get_accessible_type(factory);
        g_type_query(derived_atk_type, &raw mut query);
        g_define_type_id = g_type_register_static_simple(
            derived_atk_type,
            g_intern_static_string(b"FmDesktopAccessible\0" as *const u8 as *const gchar),
            query.class_size,
            mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GClassInitFunc>(Some(
                fm_desktop_accessible_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
            query.instance_size,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmDesktopAccessible) -> ()>,
                GInstanceInitFunc,
            >(Some(
                fm_desktop_accessible_init as unsafe extern "C" fn(*mut FmDesktopAccessible) -> (),
            )),
            G_TYPE_FLAG_NONE,
        );
        let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
            interface_init: mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                GInterfaceInitFunc,
            >(mem::transmute::<
                Option<unsafe extern "C" fn(*mut AtkComponentIface) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                atk_component_interface_init as unsafe extern "C" fn(*mut AtkComponentIface) -> (),
            ))),
            interface_finalize: None,
            interface_data: ptr::null_mut::<c_void>(),
        };
        g_type_add_interface_static(
            g_define_type_id,
            atk_component_get_type(),
            &raw const g_implement_interface_info,
        );
        let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
            interface_init: mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                GInterfaceInitFunc,
            >(mem::transmute::<
                Option<unsafe extern "C" fn(*mut AtkSelectionIface) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                atk_selection_interface_init as unsafe extern "C" fn(*mut AtkSelectionIface) -> (),
            ))),
            interface_finalize: None,
            interface_data: ptr::null_mut::<c_void>(),
        };
        g_type_add_interface_static(
            g_define_type_id,
            atk_selection_get_type(),
            &raw const g_implement_interface_info_0,
        );
        let g_implement_interface_info_1: GInterfaceInfo = _GInterfaceInfo {
            interface_init: mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                GInterfaceInitFunc,
            >(mem::transmute::<
                Option<unsafe extern "C" fn(*mut AtkActionIface) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                atk_action_interface_init as unsafe extern "C" fn(*mut AtkActionIface) -> (),
            ))),
            interface_finalize: None,
            interface_data: ptr::null_mut::<c_void>(),
        };
        g_type_add_interface_static(
            g_define_type_id,
            atk_action_get_type(),
            &raw const g_implement_interface_info_1,
        );
        if 0i32 != 0 {
            ptr::write_volatile(
                &mut type_id_volatile as *mut gsize,
                g_define_type_id as gsize,
            );
        } else {
        };
        g_once_init_leave(&raw mut type_id_volatile as *mut c_void, g_define_type_id);
    }
    return type_id_volatile as GType;
}
#[inline]
unsafe extern "C" fn fm_desktop_find_accessible_for_item(
    mut priv_0: *mut FmDesktopAccessiblePriv,
    mut item: *mut FmDesktopItem,
) -> *mut GList {
    let mut items: *mut GList = (*priv_0).items;
    while !items.is_null() {
        if (*((*items).data as *mut FmDesktopItemAccessible)).item == item {
            return items;
        }
        items = (*items).next;
    }
    return ptr::null_mut::<GList>();
}
unsafe extern "C" fn fm_desktop_accessible_ref_accessible_at_point(
    mut component: *mut AtkComponent,
    mut x: gint,
    mut y: gint,
    mut coord_type: AtkCoordType,
) -> *mut AtkObject {
    let mut widget: *mut GtkWidget =
        gtk_accessible_get_widget(component as *mut c_void as *mut GtkAccessible);
    let mut desktop: *mut FmDesktop = ptr::null_mut::<FmDesktop>();
    let mut x_pos: gint = 0;
    let mut y_pos: gint = 0;
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut obj_l: *mut GList = ptr::null_mut::<GList>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if widget.is_null() {
        return ptr::null_mut::<AtkObject>();
    }
    desktop = widget as *mut c_void as *mut FmDesktop;
    atk_component_get_extents(
        component,
        &raw mut x_pos,
        &raw mut y_pos,
        ptr::null_mut::<gint>(),
        ptr::null_mut::<gint>(),
        coord_type,
    );
    item = hit_test(
        desktop,
        &raw mut it,
        x as c_int - x_pos as c_int,
        y as c_int - y_pos as c_int,
    );
    if !item.is_null() {
        obj_l = fm_desktop_find_accessible_for_item(
            g_type_instance_get_private(
                component as *mut GTypeInstance,
                fm_desktop_accessible_get_type(),
            ) as *mut FmDesktopAccessiblePriv,
            item,
        );
    }
    if !obj_l.is_null() {
        return g_object_ref((*obj_l).data) as *mut AtkObject;
    }
    return ptr::null_mut::<AtkObject>();
}
unsafe extern "C" fn atk_component_interface_init(mut iface: *mut AtkComponentIface) {
    (*iface).ref_accessible_at_point = Some(
        fm_desktop_accessible_ref_accessible_at_point
            as unsafe extern "C" fn(*mut AtkComponent, gint, gint, AtkCoordType) -> *mut AtkObject,
    )
        as Option<
            unsafe extern "C" fn(*mut AtkComponent, gint, gint, AtkCoordType) -> *mut AtkObject,
        >;
}
unsafe extern "C" fn fm_desktop_accessible_add_selection(
    mut selection: *mut AtkSelection,
    mut i: gint,
) -> gboolean {
    let mut widget: *mut GtkWidget =
        gtk_accessible_get_widget(selection as *mut c_void as *mut GtkAccessible);
    let mut desktop: *mut FmDesktop = ptr::null_mut::<FmDesktop>();
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    let mut item: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    if widget.is_null() {
        return 0 as gboolean;
    }
    desktop = widget as *mut c_void as *mut FmDesktop;
    priv_0 = g_type_instance_get_private(
        selection as *mut GTypeInstance,
        fm_desktop_accessible_get_type(),
    ) as *mut FmDesktopAccessiblePriv;
    item = g_list_nth_data((*priv_0).items, i as guint) as *mut FmDesktopItemAccessible;
    if item.is_null() {
        return 0 as gboolean;
    }
    (*(*item).item).set_is_selected((0i32 == 0) as c_int as gboolean as gboolean);
    redraw_item(desktop, (*item).item);
    atk_object_notify_state_change(
        item as *mut c_void as *mut AtkObject,
        ATK_STATE_SELECTED as c_int as AtkState,
        (0i32 == 0) as c_int,
    );
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn fm_desktop_accessible_clear_selection(
    mut selection: *mut AtkSelection,
) -> gboolean {
    let mut widget: *mut GtkWidget =
        gtk_accessible_get_widget(selection as *mut c_void as *mut GtkAccessible);
    if widget.is_null() {
        return 0 as gboolean;
    }
    _unselect_all(widget as *mut c_void as *mut FmFolderView);
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn fm_desktop_accessible_ref_selection(
    mut selection: *mut AtkSelection,
    mut i: gint,
) -> *mut AtkObject {
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    let mut item: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    let mut items: *mut GList = ptr::null_mut::<GList>();
    if i < 0i32 {
        return ptr::null_mut::<AtkObject>();
    }
    priv_0 = g_type_instance_get_private(
        selection as *mut GTypeInstance,
        fm_desktop_accessible_get_type(),
    ) as *mut FmDesktopAccessiblePriv;
    items = (*priv_0).items;
    while !items.is_null() {
        item = (*items).data as *mut FmDesktopItemAccessible;
        if (*(*item).item).is_selected() != 0 {
            let fresh10 = i;
            i = i - 1;
            if fresh10 == 0i32 {
                return g_object_ref(item as gpointer) as *mut FmDesktopItemAccessible
                    as *mut AtkObject;
            }
        }
        items = (*items).next;
    }
    return ptr::null_mut::<AtkObject>();
}
unsafe extern "C" fn fm_desktop_accessible_get_selection_count(
    mut selection: *mut AtkSelection,
) -> gint {
    let mut priv_0: *mut FmDesktopAccessiblePriv = g_type_instance_get_private(
        selection as *mut GTypeInstance,
        fm_desktop_accessible_get_type(),
    ) as *mut FmDesktopAccessiblePriv;
    let mut item: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    let mut items: *mut GList = ptr::null_mut::<GList>();
    let mut i: gint = 0 as gint;
    items = (*priv_0).items;
    while !items.is_null() {
        item = (*items).data as *mut FmDesktopItemAccessible;
        if (*(*item).item).is_selected() != 0 {
            i += 1;
        }
        items = (*items).next;
    }
    return i;
}
unsafe extern "C" fn fm_desktop_accessible_is_child_selected(
    mut selection: *mut AtkSelection,
    mut i: gint,
) -> gboolean {
    let mut priv_0: *mut FmDesktopAccessiblePriv = g_type_instance_get_private(
        selection as *mut GTypeInstance,
        fm_desktop_accessible_get_type(),
    ) as *mut FmDesktopAccessiblePriv;
    let mut item: *mut FmDesktopItemAccessible =
        g_list_nth_data((*priv_0).items, i as guint) as *mut FmDesktopItemAccessible;
    if item.is_null() {
        return 0 as gboolean;
    }
    return (*(*item).item).is_selected();
}
unsafe extern "C" fn fm_desktop_accessible_remove_selection(
    mut selection: *mut AtkSelection,
    mut i: gint,
) -> gboolean {
    let mut widget: *mut GtkWidget =
        gtk_accessible_get_widget(selection as *mut c_void as *mut GtkAccessible);
    let mut desktop: *mut FmDesktop = ptr::null_mut::<FmDesktop>();
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    let mut item: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    let mut items: *mut GList = ptr::null_mut::<GList>();
    if i < 0i32 {
        return 0 as gboolean;
    }
    if widget.is_null() {
        return 0 as gboolean;
    }
    desktop = widget as *mut c_void as *mut FmDesktop;
    priv_0 = g_type_instance_get_private(
        selection as *mut GTypeInstance,
        fm_desktop_accessible_get_type(),
    ) as *mut FmDesktopAccessiblePriv;
    items = (*priv_0).items;
    while !items.is_null() {
        item = (*items).data as *mut FmDesktopItemAccessible;
        if (*(*item).item).is_selected() != 0 {
            let fresh9 = i;
            i = i - 1;
            if fresh9 == 0i32 {
                (*(*item).item).set_is_selected(0 as gboolean as gboolean);
                redraw_item(desktop, (*item).item);
                atk_object_notify_state_change(
                    item as *mut c_void as *mut AtkObject,
                    ATK_STATE_SELECTED as c_int as AtkState,
                    0 as gboolean,
                );
                return (0i32 == 0) as c_int;
            }
        }
        items = (*items).next;
    }
    return 0 as gboolean;
}
unsafe extern "C" fn fm_desktop_accessible_select_all_selection(
    mut selection: *mut AtkSelection,
) -> gboolean {
    let mut widget: *mut GtkWidget =
        gtk_accessible_get_widget(selection as *mut c_void as *mut GtkAccessible);
    if widget.is_null() {
        return 0 as gboolean;
    }
    _select_all(widget as *mut c_void as *mut FmFolderView);
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn atk_selection_interface_init(mut iface: *mut AtkSelectionIface) {
    (*iface).add_selection = Some(
        fm_desktop_accessible_add_selection
            as unsafe extern "C" fn(*mut AtkSelection, gint) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut AtkSelection, gint) -> gboolean>;
    (*iface).clear_selection = Some(
        fm_desktop_accessible_clear_selection
            as unsafe extern "C" fn(*mut AtkSelection) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut AtkSelection) -> gboolean>;
    (*iface).ref_selection = Some(
        fm_desktop_accessible_ref_selection
            as unsafe extern "C" fn(*mut AtkSelection, gint) -> *mut AtkObject,
    )
        as Option<unsafe extern "C" fn(*mut AtkSelection, gint) -> *mut AtkObject>;
    (*iface).get_selection_count = Some(
        fm_desktop_accessible_get_selection_count
            as unsafe extern "C" fn(*mut AtkSelection) -> gint,
    ) as Option<unsafe extern "C" fn(*mut AtkSelection) -> gint>;
    (*iface).is_child_selected = Some(
        fm_desktop_accessible_is_child_selected
            as unsafe extern "C" fn(*mut AtkSelection, gint) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut AtkSelection, gint) -> gboolean>;
    (*iface).remove_selection = Some(
        fm_desktop_accessible_remove_selection
            as unsafe extern "C" fn(*mut AtkSelection, gint) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut AtkSelection, gint) -> gboolean>;
    (*iface).select_all_selection = Some(
        fm_desktop_accessible_select_all_selection
            as unsafe extern "C" fn(*mut AtkSelection) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut AtkSelection) -> gboolean>;
}
unsafe extern "C" fn fm_desktop_accessible_idle_do_action(mut data: gpointer) -> gboolean {
    let mut widget: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    if g_source_is_destroyed(g_main_current_source()) != 0 {
        return 0 as gboolean;
    }
    priv_0 =
        g_type_instance_get_private(data as *mut GTypeInstance, fm_desktop_accessible_get_type())
            as *mut FmDesktopAccessiblePriv;
    (*priv_0).action_idle_handler = 0 as guint;
    widget = gtk_accessible_get_widget(data as *mut GtkAccessible);
    if !widget.is_null() {
        fm_folder_view_item_clicked(
            widget as *mut c_void as *mut FmFolderView,
            ptr::null_mut::<GtkTreePath>(),
            FM_FV_CONTEXT_MENU,
        );
    }
    return 0 as gboolean;
}
unsafe extern "C" fn fm_desktop_accessible_action_do_action(
    mut action: *mut AtkAction,
    mut i: gint,
) -> gboolean {
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    if i != 0i32 {
        return 0 as gboolean;
    }
    if gtk_accessible_get_widget(action as *mut c_void as *mut GtkAccessible).is_null() {
        return 0 as gboolean;
    }
    priv_0 = g_type_instance_get_private(
        action as *mut GTypeInstance,
        fm_desktop_accessible_get_type(),
    ) as *mut FmDesktopAccessiblePriv;
    if (*priv_0).action_idle_handler == 0 {
        (*priv_0).action_idle_handler = gdk_threads_add_idle(
            Some(
                fm_desktop_accessible_idle_do_action as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            action as gpointer,
        );
    }
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn fm_desktop_accessible_action_get_n_actions(
    mut action: *mut AtkAction,
) -> gint {
    return 1 as gint;
}
unsafe extern "C" fn fm_desktop_accessible_action_get_description(
    mut action: *mut AtkAction,
    mut i: gint,
) -> *const gchar {
    if i == 0i32 {
        return dcgettext(
            ptr::null::<c_char>(),
            c"Show desktop menu".as_ptr(),
            5 as c_int,
        );
    }
    return ptr::null::<gchar>();
}
unsafe extern "C" fn fm_desktop_accessible_action_get_name(
    mut action: *mut AtkAction,
    mut i: gint,
) -> *const gchar {
    if i == 0i32 {
        return b"menu\0" as *const u8 as *const gchar;
    }
    return ptr::null::<gchar>();
}
unsafe extern "C" fn atk_action_interface_init(mut iface: *mut AtkActionIface) {
    (*iface).do_action = Some(
        fm_desktop_accessible_action_do_action
            as unsafe extern "C" fn(*mut AtkAction, gint) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut AtkAction, gint) -> gboolean>;
    (*iface).get_n_actions = Some(
        fm_desktop_accessible_action_get_n_actions as unsafe extern "C" fn(*mut AtkAction) -> gint,
    ) as Option<unsafe extern "C" fn(*mut AtkAction) -> gint>;
    (*iface).get_description = Some(
        fm_desktop_accessible_action_get_description
            as unsafe extern "C" fn(*mut AtkAction, gint) -> *const gchar,
    )
        as Option<unsafe extern "C" fn(*mut AtkAction, gint) -> *const gchar>;
    (*iface).get_name = Some(
        fm_desktop_accessible_action_get_name
            as unsafe extern "C" fn(*mut AtkAction, gint) -> *const gchar,
    ) as Option<unsafe extern "C" fn(*mut AtkAction, gint) -> *const gchar>;
}
unsafe extern "C" fn fm_desktop_accessible_finalize(mut object: *mut GObject) {
    let mut priv_0: *mut FmDesktopAccessiblePriv = g_type_instance_get_private(
        object as *mut GTypeInstance,
        fm_desktop_accessible_get_type(),
    ) as *mut FmDesktopAccessiblePriv;
    let mut item: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    while !(*priv_0).items.is_null() {
        item = (*(*priv_0).items).data as *mut FmDesktopItemAccessible;
        (*item).item = ptr::null_mut::<FmDesktopItem>();
        fm_desktop_item_accessible_add_state(item, ATK_STATE_DEFUNCT);
        g_signal_emit_by_name(
            item as gpointer,
            b"children-changed::remove\0" as *const u8 as *const gchar,
            0i32,
            ptr::null_mut::<c_void>(),
            ptr::null_mut::<c_void>(),
        );
        (*priv_0).items = g_list_remove_link((*priv_0).items, (*priv_0).items);
        g_object_unref(item as gpointer);
    }
    if (*priv_0).action_idle_handler != 0 {
        g_source_remove((*priv_0).action_idle_handler);
        (*priv_0).action_idle_handler = 0 as guint;
    }
    (*(fm_desktop_accessible_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn fm_desktop_accessible_get_n_children(mut accessible: *mut AtkObject) -> gint {
    let mut priv_0: *mut FmDesktopAccessiblePriv = g_type_instance_get_private(
        accessible as *mut GTypeInstance,
        fm_desktop_accessible_get_type(),
    ) as *mut FmDesktopAccessiblePriv;
    return g_list_length((*priv_0).items) as gint;
}
unsafe extern "C" fn fm_desktop_accessible_ref_child(
    mut accessible: *mut AtkObject,
    mut index: gint,
) -> *mut AtkObject {
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    let mut item: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    if index < 0i32 {
        return ptr::null_mut::<AtkObject>();
    }
    priv_0 = g_type_instance_get_private(
        accessible as *mut GTypeInstance,
        fm_desktop_accessible_get_type(),
    ) as *mut FmDesktopAccessiblePriv;
    item = g_list_nth_data((*priv_0).items, index as guint) as *mut FmDesktopItemAccessible;
    if item.is_null() {
        return ptr::null_mut::<AtkObject>();
    }
    return g_object_ref(item as gpointer) as *mut FmDesktopItemAccessible as *mut AtkObject;
}
unsafe extern "C" fn fm_desktop_accessible_initialize(
    mut accessible: *mut AtkObject,
    mut data: gpointer,
) {
    if (*(fm_desktop_accessible_parent_class as *mut AtkObjectClass))
        .initialize
        .is_some()
    {
        (*(fm_desktop_accessible_parent_class as *mut AtkObjectClass))
            .initialize
            .expect("non-null function pointer")(accessible, data);
    }
    atk_object_set_role(accessible, ATK_ROLE_WINDOW);
    atk_object_set_name(
        accessible,
        dcgettext(ptr::null::<c_char>(), c"Desktop".as_ptr(), 5 as c_int),
    );
}
unsafe extern "C" fn fm_desktop_accessible_init(mut object: *mut FmDesktopAccessible) {
    let mut priv_0: *mut FmDesktopAccessiblePriv = g_type_instance_get_private(
        object as *mut GTypeInstance,
        fm_desktop_accessible_get_type(),
    ) as *mut FmDesktopAccessiblePriv;
    (*priv_0).items = ptr::null_mut::<GList>();
}
unsafe extern "C" fn fm_desktop_accessible_class_init(mut klass: *mut FmDesktopAccessibleClass) {
    let mut gobject_class: *mut GObjectClass = klass as *mut c_void as *mut GObjectClass;
    let mut atk_class: *mut AtkObjectClass = klass as *mut c_void as *mut AtkObjectClass;
    (*gobject_class).finalize =
        Some(fm_desktop_accessible_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*atk_class).get_n_children =
        Some(fm_desktop_accessible_get_n_children as unsafe extern "C" fn(*mut AtkObject) -> gint)
            as Option<unsafe extern "C" fn(*mut AtkObject) -> gint>;
    (*atk_class).ref_child = Some(
        fm_desktop_accessible_ref_child
            as unsafe extern "C" fn(*mut AtkObject, gint) -> *mut AtkObject,
    )
        as Option<unsafe extern "C" fn(*mut AtkObject, gint) -> *mut AtkObject>;
    (*atk_class).initialize = Some(
        fm_desktop_accessible_initialize as unsafe extern "C" fn(*mut AtkObject, gpointer) -> (),
    ) as Option<unsafe extern "C" fn(*mut AtkObject, gpointer) -> ()>;
    g_type_class_add_private(
        klass as gpointer,
        mem::size_of::<FmDesktopAccessiblePriv>() as gsize,
    );
}
#[inline(never)]
unsafe extern "C" fn fm_desktop_accessible_factory_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        atk_object_factory_get_type(),
        g_intern_static_string(b"FmDesktopAccessibleFactory\0" as *const u8 as *const gchar),
        mem::size_of::<FmDesktopAccessibleFactoryClass>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(mem::transmute::<
            Option<unsafe extern "C" fn(gpointer) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            fm_desktop_accessible_factory_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
        ))),
        mem::size_of::<FmDesktopAccessibleFactory>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmDesktopAccessibleFactory) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                fm_desktop_accessible_factory_init
                    as unsafe extern "C" fn(*mut FmDesktopAccessibleFactory) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    atk_registry_set_factory_type(
        atk_get_default_registry(),
        fm_desktop_get_type(),
        g_define_type_id,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fm_desktop_accessible_factory_class_intern_init(mut klass: gpointer) {
    fm_desktop_accessible_factory_parent_class = g_type_class_peek_parent(klass);
    if FmDesktopAccessibleFactory_private_offset != 0i32 {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut FmDesktopAccessibleFactory_private_offset,
        );
    }
    fm_desktop_accessible_factory_class_init(klass as *mut FmDesktopAccessibleFactoryClass);
}
unsafe extern "C" fn fm_desktop_accessible_factory_get_type() -> GType {
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
        let mut g_define_type_id: GType = fm_desktop_accessible_factory_get_type_once();
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
static mut FmDesktopAccessibleFactory_private_offset: gint = 0;
static mut fm_desktop_accessible_factory_parent_class: gpointer =
    ptr::null::<c_void>() as *mut c_void;
unsafe extern "C" fn fm_desktop_accessible_factory_get_accessible_type() -> GType {
    return fm_desktop_accessible_get_type();
}
unsafe extern "C" fn fm_desktop_accessible_factory_create_accessible(
    mut object: *mut GObject,
) -> *mut AtkObject {
    let mut accessible: *mut AtkObject = ptr::null_mut::<AtkObject>();
    accessible =
        g_object_new(fm_desktop_accessible_get_type(), ptr::null::<gchar>()) as *mut AtkObject;
    atk_object_initialize(accessible, object as gpointer);
    return accessible;
}
unsafe extern "C" fn fm_desktop_accessible_factory_class_init(
    mut klass: *mut FmDesktopAccessibleFactoryClass,
) {
    let mut factory_class: *mut AtkObjectFactoryClass = klass as *mut AtkObjectFactoryClass;
    (*factory_class).create_accessible = Some(
        fm_desktop_accessible_factory_create_accessible
            as unsafe extern "C" fn(*mut GObject) -> *mut AtkObject,
    )
        as Option<unsafe extern "C" fn(*mut GObject) -> *mut AtkObject>;
    (*factory_class).get_accessible_type =
        Some(fm_desktop_accessible_factory_get_accessible_type as unsafe extern "C" fn() -> GType)
            as Option<unsafe extern "C" fn() -> GType>;
}
unsafe extern "C" fn fm_desktop_accessible_factory_init(
    mut factory: *mut FmDesktopAccessibleFactory,
) {
}
unsafe extern "C" fn fm_desktop_get_accessible(mut widget: *mut GtkWidget) -> *mut AtkObject {
    fm_desktop_accessible_factory_get_type();
    return (*(fm_desktop_parent_class as *mut GtkWidgetClass))
        .get_accessible
        .expect("non-null function pointer")(widget);
}
#[inline]
unsafe extern "C" fn fm_desktop_accessible_index(
    mut desktop: *mut GtkWidget,
    mut item: gpointer,
) -> gint {
    let mut desktop_atk: *mut AtkObject = gtk_widget_get_accessible(desktop);
    let mut priv_0: *mut FmDesktopAccessiblePriv = g_type_instance_get_private(
        desktop_atk as *mut GTypeInstance,
        fm_desktop_accessible_get_type(),
    ) as *mut FmDesktopAccessiblePriv;
    return g_list_index((*priv_0).items, item as gconstpointer);
}
unsafe extern "C" fn fm_desktop_accessible_item_deleted(
    mut desktop: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
) {
    let mut obj: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    let mut item_atk_l: *mut GList = ptr::null_mut::<GList>();
    let mut item_atk: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    let mut index: gint = 0;
    obj = gtk_widget_get_accessible(desktop as *mut c_void as *mut GtkWidget);
    if !obj.is_null()
        && ({
            let mut __inst: *mut GTypeInstance = obj as *mut GTypeInstance;
            let mut __t: GType = fm_desktop_accessible_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0i32 == 0) as c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
    {
        priv_0 = g_type_instance_get_private(
            obj as *mut GTypeInstance,
            fm_desktop_accessible_get_type(),
        ) as *mut FmDesktopAccessiblePriv;
        item_atk_l = fm_desktop_find_accessible_for_item(priv_0, item);
        item_atk = (*item_atk_l).data as *mut FmDesktopItemAccessible;
        (*item_atk).item = ptr::null_mut::<FmDesktopItem>();
        index = g_list_position((*priv_0).items, item_atk_l);
        fm_desktop_item_accessible_add_state(item_atk, ATK_STATE_DEFUNCT);
        g_signal_emit_by_name(
            obj as gpointer,
            b"children-changed::remove\0" as *const u8 as *const gchar,
            index,
            ptr::null_mut::<c_void>(),
            ptr::null_mut::<c_void>(),
        );
        (*priv_0).items = g_list_remove_link((*priv_0).items, item_atk_l);
        g_object_unref(item_atk as gpointer);
    }
}
unsafe extern "C" fn fm_desktop_accessible_item_added(
    mut desktop: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
    mut index: guint,
) {
    let mut obj: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    let mut item_atk: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    obj = gtk_widget_get_accessible(desktop as *mut c_void as *mut GtkWidget);
    if !obj.is_null()
        && ({
            let mut __inst: *mut GTypeInstance = obj as *mut GTypeInstance;
            let mut __t: GType = fm_desktop_accessible_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0i32 == 0) as c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
    {
        priv_0 = g_type_instance_get_private(
            obj as *mut GTypeInstance,
            fm_desktop_accessible_get_type(),
        ) as *mut FmDesktopAccessiblePriv;
        item_atk = fm_desktop_item_accessible_new(desktop, item);
        if !(({
            let mut _g_boolean_var_33: c_int = 0i32;
            if index <= g_list_length((*priv_0).items) {
                _g_boolean_var_33 = 1i32;
            }
            _g_boolean_var_33
        }) as c_long
            != 0)
        {
            g_warn_message(
                ptr::null_mut::<gchar>(),
                c"/tmp/fm-full/pcmanfm/src/desktop.c".as_ptr(),
                1626 as c_int,
                c"fm_desktop_accessible_item_added".as_ptr(),
                c"index <= g_list_length(priv->items)".as_ptr(),
            );
        }
        (*priv_0).items = g_list_insert(
            (*priv_0).items,
            g_object_ref(item_atk as gpointer) as *mut FmDesktopItemAccessible as gpointer,
            index as gint,
        );
        g_signal_emit_by_name(
            obj as gpointer,
            b"children-changed::add\0" as *const u8 as *const gchar,
            index,
            ptr::null_mut::<c_void>(),
            ptr::null_mut::<c_void>(),
        );
    }
}
unsafe extern "C" fn fm_desktop_accessible_items_reordered(
    mut desktop: *mut FmDesktop,
    mut model: *mut GtkTreeModel,
    mut new_order: *mut gint,
) {
    let mut obj: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    let mut new_list: *mut GList = ptr::null_mut::<GList>();
    let mut length: c_int = 0;
    let mut i: c_int = 0;
    obj = gtk_widget_get_accessible(desktop as *mut c_void as *mut GtkWidget);
    if !obj.is_null()
        && ({
            let mut __inst: *mut GTypeInstance = obj as *mut GTypeInstance;
            let mut __t: GType = fm_desktop_accessible_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0i32 == 0) as c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
    {
        priv_0 = g_type_instance_get_private(
            obj as *mut GTypeInstance,
            fm_desktop_accessible_get_type(),
        ) as *mut FmDesktopAccessiblePriv;
        length = gtk_tree_model_iter_n_children(model, ptr::null_mut::<GtkTreeIter>()) as c_int;
        i = 0i32;
        while i < length {
            new_list = g_list_prepend(
                new_list,
                g_list_nth_data((*priv_0).items, *new_order.offset(i as isize) as guint),
            );
            i += 1;
        }
        g_list_free((*priv_0).items);
        (*priv_0).items = g_list_reverse(new_list);
    }
}
unsafe extern "C" fn fm_desktop_item_selected_changed(
    mut desktop: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
) {
    let mut obj: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    let mut item_atk_l: *mut GList = ptr::null_mut::<GList>();
    obj = gtk_widget_get_accessible(desktop as *mut c_void as *mut GtkWidget);
    if !obj.is_null()
        && ({
            let mut __inst: *mut GTypeInstance = obj as *mut GTypeInstance;
            let mut __t: GType = fm_desktop_accessible_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0i32 == 0) as c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
    {
        priv_0 = g_type_instance_get_private(
            obj as *mut GTypeInstance,
            fm_desktop_accessible_get_type(),
        ) as *mut FmDesktopAccessiblePriv;
        item_atk_l = fm_desktop_find_accessible_for_item(priv_0, item);
        atk_object_notify_state_change(
            (*item_atk_l).data as *mut AtkObject,
            ATK_STATE_SELECTED as c_int as AtkState,
            (*item).is_selected(),
        );
    }
}
unsafe extern "C" fn fm_desktop_accessible_focus_set(
    mut desktop: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
) {
    let mut obj: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    let mut item_atk_l: *mut GList = ptr::null_mut::<GList>();
    obj = gtk_widget_get_accessible(desktop as *mut c_void as *mut GtkWidget);
    if !obj.is_null()
        && ({
            let mut __inst: *mut GTypeInstance = obj as *mut GTypeInstance;
            let mut __t: GType = fm_desktop_accessible_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0i32 == 0) as c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
    {
        priv_0 = g_type_instance_get_private(
            obj as *mut GTypeInstance,
            fm_desktop_accessible_get_type(),
        ) as *mut FmDesktopAccessiblePriv;
        item_atk_l = fm_desktop_find_accessible_for_item(priv_0, item);
        atk_object_notify_state_change(
            (*item_atk_l).data as *mut AtkObject,
            ATK_STATE_FOCUSED as c_int as AtkState,
            (0i32 == 0) as c_int,
        );
    }
}
unsafe extern "C" fn fm_desktop_accessible_focus_unset(
    mut desktop: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
) {
    let mut obj: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    let mut item_atk_l: *mut GList = ptr::null_mut::<GList>();
    obj = gtk_widget_get_accessible(desktop as *mut c_void as *mut GtkWidget);
    if !obj.is_null()
        && ({
            let mut __inst: *mut GTypeInstance = obj as *mut GTypeInstance;
            let mut __t: GType = fm_desktop_accessible_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0i32 == 0) as c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
    {
        priv_0 = g_type_instance_get_private(
            obj as *mut GTypeInstance,
            fm_desktop_accessible_get_type(),
        ) as *mut FmDesktopAccessiblePriv;
        item_atk_l = fm_desktop_find_accessible_for_item(priv_0, item);
        atk_object_notify_state_change(
            (*item_atk_l).data as *mut AtkObject,
            ATK_STATE_FOCUSED as c_int as AtkState,
            0 as gboolean,
        );
    }
}
unsafe extern "C" fn fm_desktop_accessible_model_removed(mut desktop: *mut FmDesktop) {
    let mut obj: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut priv_0: *mut FmDesktopAccessiblePriv = ptr::null_mut::<FmDesktopAccessiblePriv>();
    let mut item_atk: *mut FmDesktopItemAccessible = ptr::null_mut::<FmDesktopItemAccessible>();
    obj = gtk_widget_get_accessible(desktop as *mut c_void as *mut GtkWidget);
    if !obj.is_null()
        && ({
            let mut __inst: *mut GTypeInstance = obj as *mut GTypeInstance;
            let mut __t: GType = fm_desktop_accessible_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0i32 == 0) as c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
    {
        priv_0 = g_type_instance_get_private(
            obj as *mut GTypeInstance,
            fm_desktop_accessible_get_type(),
        ) as *mut FmDesktopAccessiblePriv;
        while !(*priv_0).items.is_null() {
            item_atk = (*(*priv_0).items).data as *mut FmDesktopItemAccessible;
            (*item_atk).item = ptr::null_mut::<FmDesktopItem>();
            fm_desktop_item_accessible_add_state(item_atk, ATK_STATE_DEFUNCT);
            g_signal_emit_by_name(
                obj as gpointer,
                b"children-changed::remove\0" as *const u8 as *const gchar,
                0i32,
                ptr::null_mut::<c_void>(),
                ptr::null_mut::<c_void>(),
            );
            (*priv_0).items = g_list_remove_link((*priv_0).items, (*priv_0).items);
            g_object_unref(item_atk as gpointer);
        }
    }
}
#[inline]
unsafe extern "C" fn get_item_rect(mut item: *mut FmDesktopItem, mut rect: *mut GdkRectangle) {
    gdk_rectangle_union(&raw mut (*item).icon_rect, &raw mut (*item).text_rect, rect);
}
unsafe extern "C" fn layout_items(mut self_0: *mut FmDesktop) {
    let mut model: *mut GtkTreeModel = if !(*self_0).model.is_null() {
        (*self_0).model as *mut c_void as *mut GtkTreeModel
    } else {
        ptr::null_mut::<GtkTreeModel>()
    };
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if !model.is_null() && gtk_tree_model_get_iter_first(model, &raw mut it) != 0 {
        let mut items: *mut GArray = g_array_new(
            0 as gboolean,
            0 as gboolean,
            mem::size_of::<EmwmItem>() as guint,
        );
        loop {
            let mut item: *mut FmDesktopItem =
                fm_folder_model_get_item_userdata((*self_0).model, &raw mut it)
                    as *mut FmDesktopItem;
            let mut icon: *mut GdkPixbuf = ptr::null_mut::<GdkPixbuf>();
            let mut entry: EmwmItem = EmwmItem {
                geometry: EmwmGeometry {
                    area: _cairo_rectangle_int {
                        x: 0,
                        y: 0,
                        width: 0,
                        height: 0,
                    },
                    icon: _cairo_rectangle_int {
                        x: 0,
                        y: 0,
                        width: 0,
                        height: 0,
                    },
                    text: _cairo_rectangle_int {
                        x: 0,
                        y: 0,
                        width: 0,
                        height: 0,
                    },
                },
                old_bounds: _cairo_rectangle_int {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                },
                fixed: (*item).fixed_pos() as gint,
            };
            if entry.fixed != 0 {
                get_item_rect(item, &raw mut entry.old_bounds);
            }
            gtk_tree_model_get(
                model,
                &raw mut it,
                FM_FOLDER_MODEL_COL_ICON as c_int,
                &raw mut icon,
                -(1i32),
            );
            calc_item_size(self_0, item, icon);
            entry.geometry = item_geometry(item);
            g_array_append_vals(items, &raw mut entry as gconstpointer, 1 as guint);
            if !icon.is_null() {
                g_object_unref(icon as gpointer);
            }
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        }
        emwm_layout_icons(
            desktop_grid(self_0),
            (*items).data as *mut EmwmItem,
            (*items).len as gsize,
        );
        gtk_tree_model_get_iter_first(model, &raw mut it);
        let mut index: guint = 0 as guint;
        loop {
            let mut item_0: *mut FmDesktopItem =
                fm_folder_model_get_item_userdata((*self_0).model, &raw mut it)
                    as *mut FmDesktopItem;
            set_geometry(
                item_0,
                (*((*items).data as *mut c_void as *mut EmwmItem).offset(index as isize)).geometry,
            );
            index = index.wrapping_add(1);
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        }
        g_array_free(items, (0i32 == 0) as c_int);
    }
    gtk_widget_queue_draw(self_0 as *mut c_void as *mut GtkWidget);
}
unsafe extern "C" fn on_idle_layout(mut desktop: *mut FmDesktop) -> gboolean {
    (*desktop).idle_layout = 0 as guint;
    (*desktop).set_layout_pending(0 as gboolean as gboolean);
    layout_items(desktop);
    return 0 as gboolean;
}
unsafe extern "C" fn queue_layout_items(mut desktop: *mut FmDesktop) {
    if gtk_widget_get_realized(desktop as *mut c_void as *mut GtkWidget) == 0 {
        (*desktop).set_layout_pending((0i32 == 0) as c_int as gboolean as gboolean);
    } else if 0 as guint == (*desktop).idle_layout {
        (*desktop).idle_layout = gdk_threads_add_idle(
            mem::transmute::<Option<unsafe extern "C" fn(*mut FmDesktop) -> gboolean>, GSourceFunc>(
                Some(on_idle_layout as unsafe extern "C" fn(*mut FmDesktop) -> gboolean),
            ),
            desktop as gpointer,
        );
    }
}
unsafe extern "C" fn paint_item(
    mut self_0: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
    mut cr: *mut cairo_t,
    mut expose_area: *mut GdkRectangle,
    mut icon: *mut GdkPixbuf,
) {
    let mut style: *mut GtkStyleContext = ptr::null_mut::<GtkStyleContext>();
    let mut widget: *mut GtkWidget = self_0 as *mut GtkWidget;
    let mut state: GtkCellRendererState = 0 as GtkCellRendererState;
    let mut rgba: GdkRGBA = _GdkRGBA {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
    };
    let mut text_x: c_int = 0;
    let mut text_y: c_int = 0;
    if (*item).is_selected() != 0 && (*self_0).dragging() != 0 {
        return;
    }
    style = gtk_widget_get_style_context(widget);
    pango_layout_set_text((*self_0).pl, ptr::null::<c_char>(), 0i32);
    pango_layout_set_width((*self_0).pl, (*self_0).pango_text_w as c_int);
    pango_layout_set_height((*self_0).pl, (*self_0).pango_text_h as c_int);
    pango_layout_set_text(
        (*self_0).pl,
        fm_file_info_get_disp_name((*item).fi),
        -(1i32),
    );
    text_x = ((*item).area.x as guint)
        .wrapping_add(
            (*self_0)
                .cell_w
                .wrapping_sub((*self_0).text_w)
                .wrapping_div(2 as guint),
        )
        .wrapping_add(2 as guint) as c_int;
    text_y = (*item).text_rect.y + 2 as c_int;
    if (*item).is_selected() != 0 || item == (*self_0).drop_hilight {
        state = GTK_CELL_RENDERER_SELECTED;
        cairo_save(cr);
        gdk_cairo_rectangle(cr, &raw mut (*item).text_rect);
        gtk_style_context_get_background_color(style, GTK_STATE_FLAG_SELECTED, &raw mut rgba);
        gdk_cairo_set_source_rgba(cr, &raw mut rgba);
        cairo_clip(cr);
        cairo_paint(cr);
        cairo_restore(cr);
        gtk_style_context_get_color(style, GTK_STATE_FLAG_SELECTED, &raw mut rgba);
        gdk_cairo_set_source_rgba(cr, &raw mut rgba);
    } else {
        gdk_cairo_set_source_color(cr, &raw mut (*self_0).conf.desktop_shadow);
        cairo_move_to(cr, (text_x + 1i32) as c_double, (text_y + 1i32) as c_double);
        pango_cairo_show_layout(cr, (*self_0).pl);
        gdk_cairo_set_source_color(cr, &raw mut (*self_0).conf.desktop_fg);
    }
    cairo_move_to(cr, text_x as c_double, text_y as c_double);
    pango_cairo_show_layout(cr, (*self_0).pl);
    pango_layout_set_text((*self_0).pl, ptr::null::<c_char>(), 0i32);
    if item == (*self_0).focus && gtk_widget_has_focus(widget) != 0 {
        gtk_render_focus(
            style,
            cr,
            (*item).text_rect.x as gdouble,
            (*item).text_rect.y as gdouble,
            (*item).text_rect.width as gdouble,
            (*item).text_rect.height as gdouble,
        );
    }
    if item == (*self_0).hover_item {
        g_object_set(
            self_0 as *mut c_void as *mut GObject as gpointer,
            b"tooltip-text\0" as *const u8 as *const gchar,
            fm_file_info_get_disp_name((*item).fi),
            ptr::null_mut::<c_void>(),
        );
    } else {
        g_object_set(
            self_0 as *mut c_void as *mut GObject as gpointer,
            b"tooltip-text\0" as *const u8 as *const gchar,
            ptr::null_mut::<c_void>(),
            ptr::null_mut::<c_void>(),
        );
    }
    g_object_set(
        (*self_0).icon_render as gpointer,
        b"pixbuf\0" as *const u8 as *const gchar,
        icon,
        c"info".as_ptr(),
        (*item).fi,
        ptr::null_mut::<c_void>(),
    );
    gtk_cell_renderer_render(
        (*self_0).icon_render as *mut c_void as *mut GtkCellRenderer,
        cr,
        widget,
        &raw mut (*item).icon_rect,
        &raw mut (*item).icon_rect,
        state,
    );
}
unsafe extern "C" fn redraw_item(mut desktop: *mut FmDesktop, mut item: *mut FmDesktopItem) {
    let mut rect: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    get_item_rect(item, &raw mut rect);
    rect.x -= 1;
    rect.y -= 1;
    rect.width += 2 as c_int;
    rect.height += 2 as c_int;
    gdk_window_invalidate_rect(
        gtk_widget_get_window(desktop as *mut c_void as *mut GtkWidget),
        &raw mut rect,
        0 as gboolean,
    );
}
unsafe extern "C" fn move_item(
    mut desktop: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
    mut x: c_int,
    mut y: c_int,
    mut redraw: gboolean,
) {
    if redraw != 0 {
        redraw_item(desktop, item);
    }
    set_geometry(
        item,
        emwm_move_icon(
            item_geometry(item),
            (*desktop).working_area,
            (*desktop).xmargin,
            (*desktop).ymargin,
            x as gint,
            y as gint,
        ),
    );
    if (*item).fixed_pos() == 0 {
        (*item).set_fixed_pos((0i32 == 0) as c_int as gboolean as gboolean);
        (*desktop).fixed_items = g_list_prepend((*desktop).fixed_items, item as gpointer);
    }
    if redraw != 0 {
        redraw_item(desktop, item);
    }
}
unsafe extern "C" fn calc_rubber_banding_rect(
    mut self_0: *mut FmDesktop,
    mut x: c_int,
    mut y: c_int,
    mut rect: *mut GdkRectangle,
) {
    *rect = emwm_selection_rect(
        (*self_0).drag_start_x,
        (*self_0).drag_start_y,
        x as gint,
        y as gint,
    );
}
unsafe extern "C" fn update_rubberbanding(
    mut self_0: *mut FmDesktop,
    mut newx: c_int,
    mut newy: c_int,
) {
    let mut model: *mut GtkTreeModel = if !(*self_0).model.is_null() {
        (*self_0).model as *mut c_void as *mut GtkTreeModel
    } else {
        ptr::null_mut::<GtkTreeModel>()
    };
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut old_rect: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut new_rect: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut window: *mut GdkWindow = ptr::null_mut::<GdkWindow>();
    window = gtk_widget_get_window(self_0 as *mut c_void as *mut GtkWidget);
    calc_rubber_banding_rect(
        self_0,
        (*self_0).rubber_bending_x as c_int,
        (*self_0).rubber_bending_y as c_int,
        &raw mut old_rect,
    );
    calc_rubber_banding_rect(self_0, newx, newy, &raw mut new_rect);
    gdk_window_invalidate_rect(window, &raw mut old_rect, 0 as gboolean);
    gdk_window_invalidate_rect(window, &raw mut new_rect, 0 as gboolean);
    (*self_0).rubber_bending_x = newx as gint;
    (*self_0).rubber_bending_y = newy as gint;
    if !model.is_null() && gtk_tree_model_get_iter_first(model, &raw mut it) != 0 {
        loop {
            let mut item: *mut FmDesktopItem =
                fm_folder_model_get_item_userdata((*self_0).model, &raw mut it)
                    as *mut FmDesktopItem;
            let mut selected: gboolean = 0;
            if gdk_rectangle_intersect(
                &raw mut new_rect,
                &raw mut (*item).icon_rect,
                ptr::null_mut::<GdkRectangle>(),
            ) != 0
                || gdk_rectangle_intersect(
                    &raw mut new_rect,
                    &raw mut (*item).text_rect,
                    ptr::null_mut::<GdkRectangle>(),
                ) != 0
            {
                selected = (0i32 == 0) as c_int as gboolean;
            } else {
                selected = 0 as gboolean;
            }
            if (*item).is_rubber_banded() != 0 && selected == 0
                || (*item).is_rubber_banded() == 0 && selected != 0
            {
                (*item).set_is_selected(selected as gboolean);
                redraw_item(self_0, item);
                fm_desktop_item_selected_changed(self_0, item);
            }
            (*item).set_is_rubber_banded(
                ((*self_0).rubber_bending() != 0 && selected != 0) as c_int as gboolean as gboolean,
            );
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        }
    }
}
unsafe extern "C" fn paint_rubber_banding_rect(
    mut self_0: *mut FmDesktop,
    mut cr: *mut cairo_t,
    mut expose_area: *mut GdkRectangle,
) {
    let mut widget: *mut GtkWidget = self_0 as *mut GtkWidget;
    let mut rect: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut clr: GdkColor = _GdkColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
    };
    let mut alpha: guchar = 0;
    calc_rubber_banding_rect(
        self_0,
        (*self_0).rubber_bending_x as c_int,
        (*self_0).rubber_bending_y as c_int,
        &raw mut rect,
    );
    if rect.width <= 0i32 || rect.height <= 0i32 {
        return;
    }
    if gdk_rectangle_intersect(expose_area, &raw mut rect, &raw mut rect) == 0 {
        return;
    }
    clr = (*gtk_widget_get_style(widget)).base[GTK_STATE_SELECTED as c_int as usize];
    alpha = 64 as guchar;
    cairo_save(cr);
    cairo_set_source_rgba(
        cr,
        clr.red as c_double / 65535 as c_double,
        clr.green as c_double / 65536 as c_double,
        clr.blue as c_double / 65535 as c_double,
        alpha as c_double / 100 as c_double,
    );
    gdk_cairo_rectangle(cr, &raw mut rect);
    cairo_clip(cr);
    cairo_paint(cr);
    gdk_cairo_set_source_color(cr, &raw mut clr);
    cairo_rectangle(
        cr,
        rect.x as c_double + 0.5f64,
        rect.y as c_double + 0.5f64,
        (rect.width - 1i32) as c_double,
        (rect.height - 1i32) as c_double,
    );
    cairo_stroke(cr);
    cairo_restore(cr);
}
unsafe extern "C" fn _free_cache_image(mut cache: *mut FmBackgroundCache) {
    XFreePixmap(
        cairo_xlib_surface_get_display((*cache).bg),
        cairo_xlib_surface_get_drawable((*cache).bg) as Pixmap,
    );
    cairo_surface_destroy((*cache).bg);
    (*cache).bg = ptr::null_mut::<cairo_surface_t>();
    (*cache).wallpaper_mode = FM_WP_COLOR;
}
unsafe extern "C" fn _clear_bg_cache(mut self_0: *mut FmDesktop) {
    while !(*self_0).cache.is_null() {
        let mut bg: *mut FmBackgroundCache = (*self_0).cache;
        (*self_0).cache = (*bg).next;
        _free_cache_image(bg);
        g_free((*bg).filename as gpointer);
        g_free(bg as gpointer);
    }
}
unsafe extern "C" fn update_background(mut desktop: *mut FmDesktop, mut is_it: c_int) {
    let mut widget: *mut GtkWidget = desktop as *mut GtkWidget;
    let mut pix: *mut GdkPixbuf = ptr::null_mut::<GdkPixbuf>();
    let mut scaled: *mut GdkPixbuf = ptr::null_mut::<GdkPixbuf>();
    let mut cr: *mut cairo_t = ptr::null_mut::<cairo_t>();
    let mut screen: *mut GdkScreen = gtk_widget_get_screen(widget);
    let mut root: *mut GdkWindow = gdk_screen_get_root_window(screen);
    let mut window: *mut GdkWindow = gtk_widget_get_window(widget);
    let mut cache: *mut FmBackgroundCache = ptr::null_mut::<FmBackgroundCache>();
    let mut pattern: *mut cairo_pattern_t = ptr::null_mut::<cairo_pattern_t>();
    let mut xdisplay: *mut Display = ptr::null_mut::<Display>();
    let mut xpixmap: Pixmap = 0;
    let mut xroot: Window = 0;
    let mut screen_num: c_int = gdk_screen_get_number(screen) as c_int;
    let mut wallpaper: *mut c_char = ptr::null_mut::<c_char>();
    if (*desktop).conf.wallpaper_common == 0 {
        let mut cur_desktop: guint32 = (*desktop).cur_desktop as guint32;
        if is_it >= 0i32 {
            let mut i: c_int = 0;
            wallpaper = (*desktop).conf.wallpaper;
            if cur_desktop as gint >= (*desktop).conf.wallpapers_configured {
                (*desktop).conf.wallpapers = ({
                    let mut __n: gsize = cur_desktop.wrapping_add(1 as guint32) as gsize;
                    let mut __s: gsize = mem::size_of::<*mut c_char>() as gsize;
                    let mut __p: gpointer = (*desktop).conf.wallpapers as gpointer;
                    if __s == 1 as gsize {
                        __p = g_realloc(__p, __n);
                    } else if 0 != 0
                        && (__s == 0 as gsize
                            || __n
                                <= (9223372036854775807 as c_long as gsize)
                                    .wrapping_mul(2 as gsize)
                                    .wrapping_add(1 as gsize)
                                    .wrapping_div(__s))
                    {
                        __p = g_realloc(__p, __n.wrapping_mul(__s));
                    } else {
                        __p = g_realloc_n(__p, __n, __s);
                    }
                    __p
                }) as *mut *mut c_char;
                i = if (*desktop).conf.wallpapers_configured > 0i32 {
                    (*desktop).conf.wallpapers_configured
                } else {
                    0i32
                };
                while i < cur_desktop as c_int {
                    let ref mut fresh5 = *(*desktop).conf.wallpapers.offset(i as isize);
                    *fresh5 = g_strdup_inline(wallpaper);
                    i += 1;
                }
                let ref mut fresh6 = *(*desktop).conf.wallpapers.offset(cur_desktop as isize);
                *fresh6 = ptr::null_mut::<c_char>();
                (*desktop).conf.wallpapers_configured =
                    cur_desktop.wrapping_add(1 as guint32) as c_int;
            } else if g_strcmp0(
                *(*desktop).conf.wallpapers.offset(cur_desktop as isize),
                wallpaper,
            ) != 0
            {
                wallpaper = *(*desktop).conf.wallpapers.offset(cur_desktop as isize);
                if !wallpaper.is_null() {
                    i = 0i32;
                    while i < (*desktop).conf.wallpapers_configured {
                        if i != cur_desktop as c_int
                            && g_strcmp0(*(*desktop).conf.wallpapers.offset(i as isize), wallpaper)
                                == 0i32
                        {
                            break;
                        }
                        i += 1;
                    }
                    if i == (*desktop).conf.wallpapers_configured {
                        cache = (*desktop).cache;
                        while !cache.is_null() {
                            if strcmp(wallpaper, (*cache).filename) == 0i32 {
                                break;
                            }
                            cache = (*cache).next;
                        }
                        if !cache.is_null() {
                            _free_cache_image(cache);
                        }
                    }
                    g_free(wallpaper as gpointer);
                }
                wallpaper = (*desktop).conf.wallpaper;
                let ref mut fresh7 = *(*desktop).conf.wallpapers.offset(cur_desktop as isize);
                *fresh7 = g_strdup_inline(wallpaper);
            }
        } else {
            if (cur_desktop as gint) < (*desktop).conf.wallpapers_configured {
                wallpaper = *(*desktop).conf.wallpapers.offset(cur_desktop as isize);
            } else {
                wallpaper = ptr::null_mut::<c_char>();
            }
            if wallpaper.is_null() && !(*desktop).conf.wallpaper.is_null() {
                wallpaper = (*desktop).conf.wallpaper;
                if (cur_desktop as gint) < (*desktop).conf.wallpapers_configured {
                    let ref mut fresh8 = *(*desktop).conf.wallpapers.offset(cur_desktop as isize);
                    *fresh8 = g_strdup_inline(wallpaper);
                }
            } else {
                g_free((*desktop).conf.wallpaper as gpointer);
                (*desktop).conf.wallpaper = g_strdup_inline(wallpaper);
            }
        }
    } else {
        _clear_bg_cache(desktop);
        wallpaper = (*desktop).conf.wallpaper;
    }
    if (*desktop).conf.wallpaper_mode as c_uint != FM_WP_COLOR as c_int as c_uint
        && !wallpaper.is_null()
        && *wallpaper as c_int != 0
    {
        // Match the original second-resolution cache key, using the system
        // stat ABI supplied by std on the target architecture.
        let path = std::ffi::CStr::from_ptr(wallpaper).to_bytes();
        let modified =
            std::fs::metadata(<std::ffi::OsStr as std::os::unix::ffi::OsStrExt>::from_bytes(path))
                .map(|meta| std::os::unix::fs::MetadataExt::mtime(&meta))
                .unwrap_or(0);
        cache = (*desktop).cache;
        while !cache.is_null() {
            if strcmp(wallpaper, (*cache).filename) == 0i32 {
                break;
            }
            cache = (*cache).next;
        }
        if !cache.is_null()
            && (*cache).wallpaper_mode as c_uint == (*desktop).conf.wallpaper_mode as c_uint
            && modified == (*cache).mtime
        {
            pix = ptr::null_mut::<GdkPixbuf>();
        } else {
            pix = gdk_pixbuf_new_from_file(wallpaper, ptr::null_mut::<*mut GError>());
            if !pix.is_null() {
                if !cache.is_null() {
                    _free_cache_image(cache);
                } else if !(*desktop).cache.is_null() {
                    cache = (*desktop).cache;
                    while !(*cache).next.is_null() {
                        cache = (*cache).next;
                    }
                    (*cache).next = ({
                        let mut __n: gsize = 1 as gsize;
                        let mut __s: gsize = mem::size_of::<FmBackgroundCache>() as gsize;
                        let mut __p: gpointer = ptr::null_mut::<c_void>();
                        if __s == 1 as gsize {
                            __p = g_malloc0(__n);
                        } else if 0 != 0
                            && (__s == 0 as gsize
                                || __n
                                    <= (9223372036854775807 as c_long as gsize)
                                        .wrapping_mul(2 as gsize)
                                        .wrapping_add(1 as gsize)
                                        .wrapping_div(__s))
                        {
                            __p = g_malloc0(__n.wrapping_mul(__s));
                        } else {
                            __p = g_malloc0_n(__n, __s);
                        }
                        __p
                    }) as *mut FmBackgroundCache;
                    cache = (*cache).next;
                } else {
                    cache = ({
                        let mut __n: gsize = 1 as gsize;
                        let mut __s: gsize = mem::size_of::<FmBackgroundCache>() as gsize;
                        let mut __p: gpointer = ptr::null_mut::<c_void>();
                        if __s == 1 as gsize {
                            __p = g_malloc0(__n);
                        } else if 0 != 0
                            && (__s == 0 as gsize
                                || __n
                                    <= (9223372036854775807 as c_long as gsize)
                                        .wrapping_mul(2 as gsize)
                                        .wrapping_add(1 as gsize)
                                        .wrapping_div(__s))
                        {
                            __p = g_malloc0(__n.wrapping_mul(__s));
                        } else {
                            __p = g_malloc0_n(__n, __s);
                        }
                        __p
                    }) as *mut FmBackgroundCache;
                    (*desktop).cache = cache;
                }
                if (*cache).filename.is_null() {
                    (*cache).filename = g_strdup_inline(wallpaper);
                }
                (*cache).mtime = modified as time_t;
                g_log(
                    ptr::null_mut::<gchar>(),
                    G_LOG_LEVEL_DEBUG,
                    b"adding new FmBackgroundCache for %s\0" as *const u8 as *const gchar,
                    wallpaper,
                );
            } else {
                cache = ptr::null_mut::<FmBackgroundCache>();
            }
        }
    } else {
        cache = ptr::null_mut::<FmBackgroundCache>();
    }
    if cache.is_null() {
        pattern = cairo_pattern_create_rgb(
            (*desktop).conf.desktop_bg.red as c_int as c_double / 65535.0f64,
            (*desktop).conf.desktop_bg.green as c_int as c_double / 65535.0f64,
            (*desktop).conf.desktop_bg.blue as c_int as c_double / 65535.0f64,
        );
        gdk_window_set_background_pattern(window, pattern);
        cairo_pattern_destroy(pattern);
        gdk_window_invalidate_rect(window, ptr::null::<GdkRectangle>(), (0i32 == 0) as c_int);
        return;
    }
    if (*cache).bg.is_null() {
        let mut src_w: c_int = 0;
        let mut src_h: c_int = 0;
        let mut dest_w: c_int = 0;
        let mut dest_h: c_int = 0;
        let mut x: c_int = 0i32;
        let mut y: c_int = 0i32;
        src_w = gdk_pixbuf_get_width(pix);
        src_h = gdk_pixbuf_get_height(pix);
        if (*desktop).conf.wallpaper_mode as c_uint == FM_WP_TILE as c_int as c_uint {
            dest_w = src_w;
            dest_h = src_h;
        } else {
            let mut geom: GdkRectangle = _cairo_rectangle_int {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            gdk_screen_get_monitor_geometry(screen, (*desktop).monitor, &raw mut geom);
            if (*desktop).conf.wallpaper_mode as c_uint == FM_WP_SCREEN as c_int as c_uint {
                let mut root_attributes: XWindowAttributes = XWindowAttributes {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                    border_width: 0,
                    depth: 0,
                    visual: ptr::null_mut::<Visual>(),
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
                    screen: ptr::null_mut::<Screen>(),
                };
                if emwm_layout != 0
                    && XGetWindowAttributes(
                        gdk_x11_display_get_xdisplay(gdk_window_get_display(root)),
                        gdk_x11_window_get_xid(root),
                        &raw mut root_attributes,
                    ) != 0
                {
                    dest_w = root_attributes.width;
                    dest_h = root_attributes.height;
                } else {
                    dest_w = gdk_screen_get_width(screen) as c_int;
                    dest_h = gdk_screen_get_height(screen) as c_int;
                }
                x = -geom.x;
                y = -geom.y;
            } else {
                dest_w = geom.width;
                dest_h = geom.height;
            }
        }
        xdisplay = gdk_x11_display_get_xdisplay(gdk_window_get_display(root));
        xpixmap = XCreatePixmap(
            xdisplay,
            (*(*(xdisplay as _XPrivDisplay))
                .screens
                .offset(screen_num as isize))
            .root as Drawable,
            dest_w as c_uint,
            dest_h as c_uint,
            (*(*(xdisplay as _XPrivDisplay))
                .screens
                .offset(screen_num as isize))
            .root_depth as c_uint,
        );
        (*cache).bg = cairo_xlib_surface_create(
            xdisplay,
            xpixmap as Drawable,
            gdk_x11_visual_get_xvisual(gdk_screen_get_system_visual(screen)),
            dest_w,
            dest_h,
        );
        cr = cairo_create((*cache).bg);
        if gdk_pixbuf_get_has_alpha(pix) != 0
            || (*desktop).conf.wallpaper_mode as c_uint == FM_WP_CENTER as c_int as c_uint
            || (*desktop).conf.wallpaper_mode as c_uint == FM_WP_FIT as c_int as c_uint
        {
            gdk_cairo_set_source_color(cr, &raw mut (*desktop).conf.desktop_bg);
            cairo_rectangle(
                cr,
                0 as c_double,
                0 as c_double,
                dest_w as c_double,
                dest_h as c_double,
            );
            cairo_fill(cr);
        }
        let mut paint: GdkRectangle = emwm_wallpaper_rect(
            (*desktop).conf.wallpaper_mode as gint,
            src_w as gint,
            src_h as gint,
            dest_w as gint,
            dest_h as gint,
            x as gint,
            y as gint,
        );
        x = paint.x;
        y = paint.y;
        if paint.width != src_w || paint.height != src_h {
            scaled = gdk_pixbuf_scale_simple(pix, paint.width, paint.height, GDK_INTERP_BILINEAR);
            g_object_unref(pix as gpointer);
            pix = scaled;
        }
        gdk_cairo_set_source_pixbuf(cr, pix, x as gdouble, y as gdouble);
        cairo_paint(cr);
        cairo_destroy(cr);
        (*cache).wallpaper_mode = (*desktop).conf.wallpaper_mode;
    }
    pattern = cairo_pattern_create_for_surface((*cache).bg);
    gdk_window_set_background_pattern(window, pattern);
    cairo_pattern_destroy(pattern);
    if emwm_layout != 0 {
        if !pix.is_null() {
            g_object_unref(pix as gpointer);
        }
        gdk_window_invalidate_rect(window, ptr::null::<GdkRectangle>(), (0i32 == 0) as c_int);
        return;
    }
    xdisplay = gdk_x11_display_get_xdisplay(gdk_window_get_display(root));
    xroot = (*(*(xdisplay as _XPrivDisplay))
        .screens
        .offset(screen_num as isize))
    .root;
    xpixmap = cairo_xlib_surface_get_drawable((*cache).bg) as Pixmap;
    XChangeProperty(
        xdisplay,
        gdk_x11_window_get_xid(root),
        XA_XROOTMAP_ID,
        20 as Atom,
        32 as c_int,
        0i32,
        &raw mut xpixmap as *mut guchar,
        1i32,
    );
    XGrabServer(xdisplay);
    XChangeProperty(
        xdisplay,
        xroot,
        XA_XROOTPMAP_ID,
        20 as Atom,
        32 as c_int,
        0i32,
        &raw mut xpixmap as *mut guchar,
        1i32,
    );
    XSetWindowBackgroundPixmap(xdisplay, xroot, xpixmap);
    XClearWindow(xdisplay, xroot);
    XFlush(xdisplay);
    XUngrabServer(xdisplay);
    if !pix.is_null() {
        g_object_unref(pix as gpointer);
    }
    gdk_window_invalidate_rect(window, ptr::null::<GdkRectangle>(), (0i32 == 0) as c_int);
}
unsafe extern "C" fn on_row_deleting(
    mut model: *mut FmFolderModel,
    mut tp: *mut GtkTreePath,
    mut iter: *mut GtkTreeIter,
    mut data: gpointer,
    mut desktop: *mut FmDesktop,
) {
    let mut l: *mut GList = ptr::null_mut::<GList>();
    l = (*desktop).fixed_items;
    while !l.is_null() {
        if (*l).data == data {
            (*desktop).fixed_items = g_list_delete_link((*desktop).fixed_items, l);
            break;
        } else {
            l = (*l).next;
        }
    }
    if (*desktop).focus as gpointer == data {
        let mut it: GtkTreeIter = *iter;
        fm_desktop_accessible_focus_unset(desktop, data as *mut FmDesktopItem);
        if gtk_tree_model_iter_next(model as *mut c_void as *mut GtkTreeModel, &raw mut it) != 0 {
            (*desktop).focus =
                fm_folder_model_get_item_userdata(model, &raw mut it) as *mut FmDesktopItem;
        } else if gtk_tree_path_prev(tp) != 0 {
            gtk_tree_model_get_iter(model as *mut c_void as *mut GtkTreeModel, &raw mut it, tp);
            gtk_tree_path_next(tp);
            (*desktop).focus =
                fm_folder_model_get_item_userdata(model, &raw mut it) as *mut FmDesktopItem;
        } else {
            (*desktop).focus = ptr::null_mut::<FmDesktopItem>();
        }
        if !(*desktop).focus.is_null() {
            fm_desktop_accessible_focus_set(desktop, (*desktop).focus);
        }
    }
    if (*desktop).drop_hilight as gpointer == data {
        (*desktop).drop_hilight = ptr::null_mut::<FmDesktopItem>();
    }
    if (*desktop).hover_item as gpointer == data {
        (*desktop).hover_item = ptr::null_mut::<FmDesktopItem>();
        g_object_set(
            desktop as *mut c_void as *mut GObject as gpointer,
            b"tooltip-text\0" as *const u8 as *const gchar,
            ptr::null_mut::<c_void>(),
            ptr::null_mut::<c_void>(),
        );
    }
    fm_desktop_accessible_item_deleted(desktop, data as *mut FmDesktopItem);
    desktop_item_free(data as *mut FmDesktopItem);
}
unsafe extern "C" fn on_row_inserted(
    mut mod_0: *mut FmFolderModel,
    mut tp: *mut GtkTreePath,
    mut it: *mut GtkTreeIter,
    mut desktop: *mut FmDesktop,
) {
    let mut item: *mut FmDesktopItem = desktop_item_new(mod_0, it);
    let mut indices: *mut gint = gtk_tree_path_get_indices(tp);
    fm_desktop_accessible_item_added(desktop, item, *indices.offset(0 as isize) as guint);
    fm_folder_model_set_item_userdata(mod_0, it, item as gpointer);
    queue_layout_items(desktop);
}
unsafe extern "C" fn on_row_deleted(
    mut mod_0: *mut FmFolderModel,
    mut tp: *mut GtkTreePath,
    mut desktop: *mut FmDesktop,
) {
    queue_layout_items(desktop);
}
unsafe extern "C" fn on_row_changed(
    mut model: *mut FmFolderModel,
    mut tp: *mut GtkTreePath,
    mut it: *mut GtkTreeIter,
    mut desktop: *mut FmDesktop,
) {
    let mut item: *mut FmDesktopItem =
        fm_folder_model_get_item_userdata(model, it) as *mut FmDesktopItem;
    let mut icon: *mut GdkPixbuf = ptr::null_mut::<GdkPixbuf>();
    fm_file_info_unref((*item).fi);
    gtk_tree_model_get(
        model as *mut c_void as *mut GtkTreeModel,
        it,
        FM_FOLDER_MODEL_COL_INFO as c_int,
        &raw mut (*item).fi,
        FM_FOLDER_MODEL_COL_ICON as c_int,
        &raw mut icon,
        -(1i32),
    );
    fm_file_info_ref((*item).fi);
    redraw_item(desktop, item);
    calc_item_size(desktop, item, icon);
    if !icon.is_null() {
        g_object_unref(icon as gpointer);
    }
    redraw_item(desktop, item);
}
unsafe extern "C" fn on_rows_reordered(
    mut model: *mut FmFolderModel,
    mut parent_tp: *mut GtkTreePath,
    mut parent_it: *mut GtkTreeIter,
    mut new_order: gpointer,
    mut desktop: *mut FmDesktop,
) {
    fm_desktop_accessible_items_reordered(
        desktop,
        model as *mut c_void as *mut GtkTreeModel,
        new_order as *mut gint,
    );
    queue_layout_items(desktop);
}
unsafe extern "C" fn update_working_area(mut desktop: *mut FmDesktop) {
    let mut screen: *mut GdkScreen = gtk_widget_get_screen(desktop as *mut GtkWidget);
    let mut geom: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    gdk_screen_get_monitor_geometry(screen, (*desktop).monitor, &raw mut geom);
    if emwm_layout != 0 {
        let mut root: *mut GdkWindow = gdk_screen_get_root_window(screen);
        let mut type_0: Atom = 0;
        let mut format: c_int = 0;
        let mut count: c_ulong = 0;
        let mut after: c_ulong = 0;
        let mut data: *mut c_uchar = ptr::null_mut::<c_uchar>();
        (*desktop).working_area = geom;
        if XGetWindowProperty(
            gdk_x11_display_get_xdisplay(gdk_window_get_display(root)),
            gdk_x11_window_get_xid(root),
            XA_NET_WORKAREA,
            (*desktop).cur_desktop.wrapping_mul(4 as guint) as c_long,
            4 as c_long,
            0i32,
            6 as Atom,
            &raw mut type_0,
            &raw mut format,
            &raw mut count,
            &raw mut after,
            &raw mut data,
        ) == 0i32
            && type_0 == 6 as Atom
            && format == 32 as c_int
            && count == 4 as c_ulong
        {
            let mut v: *mut c_ulong = data as *mut c_ulong;
            let mut area: GdkRectangle = _cairo_rectangle_int {
                x: *v.offset(0 as isize) as c_int,
                y: *v.offset(1 as isize) as c_int,
                width: *v.offset(2 as isize) as c_int,
                height: *v.offset(3 as isize) as c_int,
            };
            let mut clipped: GdkRectangle = _cairo_rectangle_int {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            if gdk_rectangle_intersect(&raw mut geom, &raw mut area, &raw mut clipped) != 0 {
                (*desktop).working_area = clipped;
            }
        }
        if !data.is_null() {
            XFree(data as *mut c_void);
        }
    } else {
        gdk_screen_get_monitor_workarea(
            screen,
            (*desktop).monitor,
            &raw mut (*desktop).working_area,
        );
    }
    (*desktop).working_area.x -= geom.x;
    (*desktop).working_area.y -= geom.y;
    queue_layout_items(desktop);
}
unsafe extern "C" fn on_root_event(
    mut xevent: *mut c_void,
    mut event: *mut GdkEvent,
    mut data: gpointer,
) -> GdkFilterReturn {
    let mut evt: *mut XPropertyEvent = xevent as *mut XPropertyEvent;
    let mut self_0: *mut FmDesktop = data as *mut FmDesktop;
    if (*evt).type_0 == 28 as c_int {
        if (*evt).atom == XA_NET_WORKAREA {
            update_working_area(self_0);
        } else if (*evt).atom == XA_NET_CURRENT_DESKTOP {
            let mut desktop: gint = get_desktop_for_root_window(gdk_screen_get_root_window(
                gtk_widget_get_screen(data as *mut GtkWidget),
            ));
            if desktop >= 0i32 {
                (*self_0).cur_desktop = desktop as guint;
                if (*self_0).conf.wallpaper_common == 0 {
                    update_background(self_0, -(1i32));
                }
            }
        }
    }
    return GDK_FILTER_CONTINUE;
}
unsafe extern "C" fn on_screen_size_changed(
    mut screen: *mut GdkScreen,
    mut desktop: *mut FmDesktop,
) {
    let mut geom: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if emwm_layout != 0 {
        emwm_queue_refresh();
        return;
    }
    if (*desktop).monitor >= gdk_screen_get_n_monitors(screen) {
        let mut i: gint = 0;
        i = 0 as gint;
        while i < n_screens {
            if *desktops.offset(i as isize) == desktop {
                break;
            }
            i += 1;
        }
        if i < n_screens {
            let ref mut fresh1 = *desktops.offset(i as isize);
            *fresh1 = fm_desktop_new(
                screen,
                if (*desktop).monitor != 0 {
                    -(2 as gint)
                } else {
                    -(1 as gint)
                },
            );
        }
        gtk_widget_destroy(desktop as *mut c_void as *mut GtkWidget);
        return;
    }
    gdk_screen_get_monitor_geometry(screen, (*desktop).monitor, &raw mut geom);
    gtk_window_resize(
        desktop as *mut GtkWindow,
        geom.width as gint,
        geom.height as gint,
    );
    gtk_window_move(desktop as *mut GtkWindow, geom.x as gint, geom.y as gint);
}
unsafe extern "C" fn reload_icons() {
    let mut i: c_int = 0;
    i = 0i32;
    while i < n_screens {
        if (**desktops.offset(i as isize)).monitor >= 0i32 {
            gtk_widget_queue_resize(*desktops.offset(i as isize) as *mut c_void as *mut GtkWidget);
        }
        i += 1;
    }
}
unsafe extern "C" fn on_big_icon_size_changed(
    mut cfg: *mut FmConfig,
    mut model: *mut FmFolderModel,
) {
    fm_folder_model_set_icon_size(model, (*fm_config).big_icon_size as guint);
    reload_icons();
}
unsafe extern "C" fn on_icon_theme_changed(mut theme: *mut GtkIconTheme, mut user_data: gpointer) {
    reload_icons();
}
unsafe extern "C" fn fm_desktop_update_popup(
    mut fv: *mut FmFolderView,
    mut window: *mut GtkWindow,
    mut ui: *mut GtkUIManager,
    mut act_grp: *mut GtkActionGroup,
    mut files: *mut FmFileInfoList,
) {
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    act = gtk_action_group_get_action(act_grp, b"Rename\0" as *const u8 as *const gchar);
    gtk_action_set_visible(act, 0 as gboolean);
    gtk_action_set_sensitive(act, 0 as gboolean);
    act = gtk_action_group_get_action(act_grp, b"ShowHidden\0" as *const u8 as *const gchar);
    gtk_action_set_visible(act, 0 as gboolean);
    act = gtk_action_group_get_action(act_grp, b"Prop\0" as *const u8 as *const gchar);
    gtk_action_set_visible(act, 0 as gboolean);
    gtk_action_group_set_translation_domain(act_grp, ptr::null::<gchar>());
    gtk_action_group_add_actions(
        act_grp,
        &raw const desktop_actions as *const GtkActionEntry,
        (mem::size_of::<[GtkActionEntry; 1]>() as usize)
            .wrapping_div(mem::size_of::<GtkActionEntry>() as usize) as guint,
        window as gpointer,
    );
    gtk_ui_manager_add_ui_from_string(
        ui,
        &raw const desktop_menu_xml as *const gchar,
        -(1i32) as gssize,
        ptr::null_mut::<*mut GError>(),
    );
}
unsafe extern "C" fn fm_desktop_update_item_popup(
    mut fv: *mut FmFolderView,
    mut window: *mut GtkWindow,
    mut ui: *mut GtkUIManager,
    mut act_grp: *mut GtkActionGroup,
    mut files: *mut FmFileInfoList,
) {
    let mut fi: *mut FmFileInfo = ptr::null_mut::<FmFileInfo>();
    let mut sel_items: *mut GList = ptr::null_mut::<GList>();
    let mut l: *mut GList = ptr::null_mut::<GList>();
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    let mut all_fixed: gboolean = (0i32 == 0) as c_int;
    let mut has_fixed: gboolean = 0 as gboolean;
    let mut all_native: gboolean = (0i32 == 0) as c_int;
    let mut has_extra: gboolean = 0 as gboolean;
    let mut has_mount: gboolean = 0 as gboolean;
    sel_items = get_selected_items(
        fv as *mut c_void as *mut FmDesktop,
        ptr::null_mut::<c_int>(),
    );
    l = sel_items;
    while !l.is_null() {
        let mut item: *mut FmDesktopItem = (*l).data as *mut FmDesktopItem;
        if (*item).fixed_pos() != 0 {
            has_fixed = (0i32 == 0) as c_int as gboolean;
        } else {
            all_fixed = 0 as gboolean;
        }
        if pcmanfm_can_open_path_in_terminal(fm_file_info_get_path((*item).fi)) == 0 {
            all_native = 0 as gboolean;
        }
        if (*item).is_special() != 0 {
            has_extra = (0i32 == 0) as c_int as gboolean;
        }
        if (*item).is_mount() != 0 {
            has_mount = (0i32 == 0) as c_int as gboolean;
        }
        l = (*l).next;
    }
    g_list_free(sel_items);
    fi = fm_file_info_list_peek_head(files);
    gtk_action_group_set_translation_domain(act_grp, ptr::null::<gchar>());
    if fm_file_info_list_get_length(files) == 1 as guint && fm_file_info_is_dir(fi) != 0 {
        gtk_action_group_add_actions(
            act_grp,
            &raw const folder_menu_actions as *const GtkActionEntry,
            (mem::size_of::<[GtkActionEntry; 4]>() as usize)
                .wrapping_div(mem::size_of::<GtkActionEntry>() as usize) as guint,
            fv as gpointer,
        );
        gtk_ui_manager_add_ui_from_string(
            ui,
            &raw const folder_menu_xml as *const gchar,
            -(1i32) as gssize,
            ptr::null_mut::<*mut GError>(),
        );
        act = gtk_action_group_get_action(act_grp, b"Term\0" as *const u8 as *const gchar);
        gtk_action_set_visible(act, all_native);
    }
    if has_extra != 0 {
        if fm_file_info_list_get_length(files) == 1 as guint {
            gtk_action_group_add_actions(
                act_grp,
                &raw const extra_item_menu_actions as *const GtkActionEntry,
                (mem::size_of::<[GtkActionEntry; 1]>() as usize)
                    .wrapping_div(mem::size_of::<GtkActionEntry>() as usize)
                    as guint,
                fv as gpointer,
            );
            gtk_ui_manager_add_ui_from_string(
                ui,
                &raw const extra_item_menu_xml as *const gchar,
                -(1i32) as gssize,
                ptr::null_mut::<*mut GError>(),
            );
            if has_mount != 0 {
                act =
                    gtk_action_group_get_action(act_grp, b"Disable\0" as *const u8 as *const gchar);
                gtk_action_set_visible(act, 0 as gboolean);
            }
        }
        act = gtk_action_group_get_action(act_grp, b"Cut\0" as *const u8 as *const gchar);
        gtk_action_set_visible(act, 0 as gboolean);
        act = gtk_action_group_get_action(act_grp, b"Del\0" as *const u8 as *const gchar);
        gtk_action_set_visible(act, 0 as gboolean);
        act = gtk_action_group_get_action(act_grp, b"Rename\0" as *const u8 as *const gchar);
        gtk_action_set_visible(act, 0 as gboolean);
    }
    gtk_action_group_add_actions(
        act_grp,
        &raw const desktop_icon_actions as *const GtkActionEntry,
        (mem::size_of::<[GtkActionEntry; 1]>() as usize)
            .wrapping_div(mem::size_of::<GtkActionEntry>() as usize) as guint,
        fv as gpointer,
    );
    act = gtk_action_group_get_action(act_grp, b"Snap\0" as *const u8 as *const gchar);
    gtk_action_set_sensitive(act, has_fixed);
    gtk_action_group_add_toggle_actions(
        act_grp,
        &raw mut desktop_icon_toggle_actions as *mut GtkToggleActionEntry,
        (mem::size_of::<[GtkToggleActionEntry; 1]>() as usize)
            .wrapping_div(mem::size_of::<GtkToggleActionEntry>() as usize) as guint,
        fv as gpointer,
    );
    act = gtk_action_group_get_action(act_grp, b"Fix\0" as *const u8 as *const gchar);
    gtk_toggle_action_set_active(act as *mut c_void as *mut GtkToggleAction, all_fixed);
    gtk_ui_manager_add_ui_from_string(
        ui,
        &raw const desktop_icon_menu_xml as *const gchar,
        -(1i32) as gssize,
        ptr::null_mut::<*mut GError>(),
    );
}
unsafe extern "C" fn on_open_in_new_tab(mut act: *mut GtkAction, mut user_data: gpointer) {
    let mut desktop: *mut FmDesktop = user_data as *mut FmDesktop;
    if !(*desktop).focus.is_null() {
        fm_main_win_open_in_last_active(fm_file_info_get_path((*(*desktop).focus).fi));
    }
}
unsafe extern "C" fn on_open_in_new_win(mut act: *mut GtkAction, mut user_data: gpointer) {
    let mut desktop: *mut FmDesktop = user_data as *mut FmDesktop;
    if !(*desktop).focus.is_null() {
        fm_main_win_add_win(
            ptr::null_mut::<FmMainWin>(),
            fm_file_info_get_path((*(*desktop).focus).fi),
        );
    }
}
unsafe extern "C" fn on_open_folder_in_terminal(mut act: *mut GtkAction, mut user_data: gpointer) {
    let mut desktop: *mut FmDesktop = user_data as *mut FmDesktop;
    if !(*desktop).focus.is_null() {
        pcmanfm_open_folder_in_terminal(
            ptr::null_mut::<GtkWindow>(),
            fm_file_info_get_path((*(*desktop).focus).fi),
        );
    }
}
unsafe extern "C" fn on_fix_pos(mut act: *mut GtkToggleAction, mut user_data: gpointer) {
    let mut desktop: *mut FmDesktop = user_data as *mut FmDesktop;
    let mut items: *mut GList = get_selected_items(desktop, ptr::null_mut::<c_int>());
    let mut l: *mut GList = ptr::null_mut::<GList>();
    if gtk_toggle_action_get_active(act) != 0 {
        l = items;
        while !l.is_null() {
            let mut item: *mut FmDesktopItem = (*l).data as *mut FmDesktopItem;
            if (*item).fixed_pos() == 0 {
                (*item).set_fixed_pos((0i32 == 0) as c_int as gboolean as gboolean);
                (*desktop).fixed_items = g_list_prepend((*desktop).fixed_items, item as gpointer);
            }
            l = (*l).next;
        }
    } else {
        l = items;
        while !l.is_null() {
            let mut item_0: *mut FmDesktopItem = (*l).data as *mut FmDesktopItem;
            (*item_0).set_fixed_pos(0 as gboolean as gboolean);
            (*desktop).fixed_items = g_list_remove((*desktop).fixed_items, item_0 as gconstpointer);
            l = (*l).next;
        }
        queue_layout_items(desktop);
    }
    g_list_free(items);
    queue_config_save(desktop);
}
unsafe extern "C" fn on_disable(mut act: *mut GtkAction, mut user_data: gpointer) {
    let mut desktop: *mut FmDesktop = user_data as *mut FmDesktop;
    let mut items: *mut GList = get_selected_items(desktop, ptr::null_mut::<c_int>());
    let mut item: *mut FmDesktopItem = (*items).data as *mut FmDesktopItem;
    g_list_free(items);
    if !trash_can.is_null() && (*trash_can).fi == (*item).fi {
        (*desktop).conf.show_trash = 0 as gboolean;
    } else if !documents.is_null() && (*documents).fi == (*item).fi {
        (*desktop).conf.show_documents = 0 as gboolean;
    } else {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_WARNING,
            b"invalid item to remove from desktop\0" as *const u8 as *const gchar,
        );
        return;
    }
    queue_config_save(desktop);
    fm_folder_model_extra_file_remove((*desktop).model, (*item).fi);
}
unsafe extern "C" fn on_snap_to_grid(mut act: *mut GtkAction, mut user_data: gpointer) {
    let mut desktop: *mut FmDesktop = user_data as *mut FmDesktop;
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut items: *mut GList = get_selected_items(desktop, ptr::null_mut::<c_int>());
    let mut l: *mut GList = ptr::null_mut::<GList>();
    let mut grid: EmwmGrid = desktop_grid(desktop);
    l = items;
    while !l.is_null() {
        let mut point: GdkPoint = _GdkPoint { x: 0, y: 0 };
        item = (*l).data as *mut FmDesktopItem;
        if !((*item).fixed_pos() == 0) {
            point = emwm_snap_icon(grid, (*item).area.x as gint, (*item).area.y as gint);
            move_item(
                desktop,
                item,
                point.x as c_int,
                point.y as c_int,
                0 as gboolean,
            );
        }
        l = (*l).next;
    }
    g_list_free(items);
    queue_layout_items(desktop);
}
unsafe extern "C" fn hit_test(
    mut self_0: *mut FmDesktop,
    mut it: *mut GtkTreeIter,
    mut x: c_int,
    mut y: c_int,
) -> *mut FmDesktopItem {
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    if (*self_0).model.is_null() {
        return ptr::null_mut::<FmDesktopItem>();
    }
    model = (*self_0).model as *mut c_void as *mut GtkTreeModel;
    if !model.is_null() && gtk_tree_model_get_iter_first(model, it) != 0 {
        loop {
            item = fm_folder_model_get_item_userdata((*self_0).model, it) as *mut FmDesktopItem;
            if !((*item).is_selected() != 0 && (*self_0).dragging() != 0) {
                if emwm_icon_hit(item_geometry(item), x as gint, y as gint) != 0 {
                    return item;
                }
            }
            if !(gtk_tree_model_iter_next(model, it) != 0) {
                break;
            }
        }
    }
    return ptr::null_mut::<FmDesktopItem>();
}
unsafe extern "C" fn get_nearest_item(
    mut desktop: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
    mut dir: GtkDirectionType,
) -> *mut FmDesktopItem {
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut item2: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut ret: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut min_x_dist: guint = 0;
    let mut min_y_dist: guint = 0;
    let mut dist: guint = 0;
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if (*desktop).model.is_null() {
        return ptr::null_mut::<FmDesktopItem>();
    }
    model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
    if gtk_tree_model_get_iter_first(model, &raw mut it) == 0 {
        return ptr::null_mut::<FmDesktopItem>();
    }
    if item.is_null() {
        return fm_folder_model_get_item_userdata((*desktop).model, &raw mut it)
            as *mut FmDesktopItem;
    }
    min_y_dist = -(1i32) as guint;
    min_x_dist = min_y_dist;
    item2 = ptr::null_mut::<FmDesktopItem>();
    match dir as c_uint {
        4 => loop {
            item2 = fm_folder_model_get_item_userdata((*desktop).model, &raw mut it)
                as *mut FmDesktopItem;
            if !((*item2).area.x >= (*item).area.x) {
                dist = ((*item).area.x - (*item2).area.x) as guint;
                if dist < min_x_dist {
                    ret = item2;
                    min_x_dist = dist;
                    min_y_dist = (if (*item).area.y - (*item2).area.y < 0i32 {
                        -((*item).area.y - (*item2).area.y)
                    } else {
                        (*item).area.y - (*item2).area.y
                    }) as guint;
                } else if dist == min_x_dist && item2 != ret {
                    dist = (if (*item2).area.y - (*item).area.y < 0i32 {
                        -((*item2).area.y - (*item).area.y)
                    } else {
                        (*item2).area.y - (*item).area.y
                    }) as guint;
                    if dist < min_y_dist {
                        ret = item2;
                        min_y_dist = dist;
                    }
                }
            }
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        },
        5 => loop {
            item2 = fm_folder_model_get_item_userdata((*desktop).model, &raw mut it)
                as *mut FmDesktopItem;
            if !((*item2).area.x <= (*item).area.x) {
                dist = ((*item2).area.x - (*item).area.x) as guint;
                if dist < min_x_dist {
                    ret = item2;
                    min_x_dist = dist;
                    min_y_dist = (if (*item).area.y - (*item2).area.y < 0i32 {
                        -((*item).area.y - (*item2).area.y)
                    } else {
                        (*item).area.y - (*item2).area.y
                    }) as guint;
                } else if dist == min_x_dist && item2 != ret {
                    dist = (if (*item2).area.y - (*item).area.y < 0i32 {
                        -((*item2).area.y - (*item).area.y)
                    } else {
                        (*item2).area.y - (*item).area.y
                    }) as guint;
                    if dist < min_y_dist {
                        ret = item2;
                        min_y_dist = dist;
                    }
                }
            }
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        },
        2 => loop {
            item2 = fm_folder_model_get_item_userdata((*desktop).model, &raw mut it)
                as *mut FmDesktopItem;
            if !((*item2).area.y >= (*item).area.y) {
                dist = ((*item).area.y - (*item2).area.y) as guint;
                if dist < min_y_dist {
                    ret = item2;
                    min_y_dist = dist;
                    min_x_dist = (if (*item).area.x - (*item2).area.x < 0i32 {
                        -((*item).area.x - (*item2).area.x)
                    } else {
                        (*item).area.x - (*item2).area.x
                    }) as guint;
                } else if dist == min_y_dist && item2 != ret {
                    dist = (if (*item2).area.x - (*item).area.x < 0i32 {
                        -((*item2).area.x - (*item).area.x)
                    } else {
                        (*item2).area.x - (*item).area.x
                    }) as guint;
                    if dist < min_x_dist {
                        ret = item2;
                        min_x_dist = dist;
                    }
                }
            }
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        },
        3 => loop {
            item2 = fm_folder_model_get_item_userdata((*desktop).model, &raw mut it)
                as *mut FmDesktopItem;
            if !((*item2).area.y <= (*item).area.y) {
                dist = ((*item2).area.y - (*item).area.y) as guint;
                if dist < min_y_dist {
                    ret = item2;
                    min_y_dist = dist;
                    min_x_dist = (if (*item).area.x - (*item2).area.x < 0i32 {
                        -((*item).area.x - (*item2).area.x)
                    } else {
                        (*item).area.x - (*item2).area.x
                    }) as guint;
                } else if dist == min_y_dist && item2 != ret {
                    dist = (if (*item2).area.x - (*item).area.x < 0i32 {
                        -((*item2).area.x - (*item).area.x)
                    } else {
                        (*item2).area.x - (*item).area.x
                    }) as guint;
                    if dist < min_x_dist {
                        ret = item2;
                        min_x_dist = dist;
                    }
                }
            }
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        },
        0 | 1 | _ => {}
    }
    return ret;
}
unsafe extern "C" fn set_focused_item(mut desktop: *mut FmDesktop, mut item: *mut FmDesktopItem) {
    if item != (*desktop).focus {
        let mut old_focus: *mut FmDesktopItem = (*desktop).focus;
        (*desktop).focus = item;
        if !old_focus.is_null() {
            redraw_item(desktop, old_focus);
            fm_desktop_accessible_focus_unset(desktop, old_focus);
        }
        if !item.is_null() {
            redraw_item(desktop, item);
            fm_desktop_accessible_focus_set(desktop, item);
        }
    }
}
unsafe extern "C" fn _focus_and_select_focused_item(
    mut desktop: *mut FmDesktop,
    mut item: *mut FmDesktopItem,
) {
    (*item).set_is_selected((0i32 == 0) as c_int as gboolean as gboolean);
    fm_desktop_item_selected_changed(desktop, item);
    set_focused_item(desktop, item);
}
unsafe extern "C" fn forward_event_to_rootwin(
    mut gscreen: *mut GdkScreen,
    mut event: *mut GdkEvent,
) {
    let mut xev: XButtonEvent = XButtonEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: ptr::null_mut::<Display>(),
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
    };
    let mut xev2: XButtonEvent = XButtonEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: ptr::null_mut::<Display>(),
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
    };
    let mut dpy: *mut Display = gdk_x11_display_get_xdisplay(gdk_screen_get_display(gscreen));
    if (*event).type_0 as c_int == GDK_BUTTON_PRESS as c_int
        || (*event).type_0 as c_int == GDK_BUTTON_RELEASE as c_int
    {
        if (*event).type_0 as c_int == GDK_BUTTON_PRESS as c_int {
            xev.type_0 = 4 as c_int;
            XUngrabPointer(dpy, (*event).button.time as Time);
        } else {
            xev.type_0 = 5 as c_int;
        }
        xev.button = (*event).button.button as c_uint;
        xev.x = (*event).button.x as c_int;
        xev.y = (*event).button.y as c_int;
        xev.x_root = (*event).button.x_root as c_int;
        xev.y_root = (*event).button.y_root as c_int;
        xev.state = (*event).button.state as c_uint;
        xev2.type_0 = 0i32;
    } else if (*event).type_0 as c_int == GDK_SCROLL as c_int {
        xev.type_0 = 4 as c_int;
        xev.button = ((*event).scroll.direction as c_uint).wrapping_add(4 as c_uint);
        xev.x = (*event).scroll.x as c_int;
        xev.y = (*event).scroll.y as c_int;
        xev.x_root = (*event).scroll.x_root as c_int;
        xev.y_root = (*event).scroll.y_root as c_int;
        xev.state = (*event).scroll.state as c_uint;
        xev2.type_0 = 5 as c_int;
        xev2.button = xev.button;
    } else {
        return;
    }
    xev.window = gdk_x11_window_get_xid(gdk_screen_get_root_window(gscreen));
    xev.root = xev.window;
    xev.subwindow = 0 as Window;
    xev.time = (*event).button.time as Time;
    xev.same_screen = 1i32;
    XSendEvent(
        dpy,
        xev.window,
        0i32,
        (1 as c_long) << 2 as c_int | (1 as c_long) << 3 as c_int,
        &raw mut xev as *mut XEvent,
    );
    if xev2.type_0 == 0i32 {
        return;
    }
    xev2.window = xev.window;
    xev2.root = xev.root;
    xev2.subwindow = xev.subwindow;
    xev2.time = xev.time;
    xev2.x = xev.x;
    xev2.y = xev.y;
    xev2.x_root = xev.x_root;
    xev2.y_root = xev.y_root;
    xev2.state = xev.state;
    xev2.same_screen = xev.same_screen;
    XSendEvent(
        dpy,
        xev2.window,
        0i32,
        (1 as c_long) << 2 as c_int | (1 as c_long) << 3 as c_int,
        &raw mut xev2 as *mut XEvent,
    );
}
unsafe extern "C" fn on_draw(mut w: *mut GtkWidget, mut cr: *mut cairo_t) -> gboolean {
    let mut self_0: *mut FmDesktop = w as *mut FmDesktop;
    let mut model: *mut GtkTreeModel = if !(*self_0).model.is_null() {
        (*self_0).model as *mut c_void as *mut GtkTreeModel
    } else {
        ptr::null_mut::<GtkTreeModel>()
    };
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut area: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if ({
        let mut _g_boolean_var_34: c_int = 0i32;
        if gtk_cairo_should_draw_window(cr, gtk_widget_get_window(w)) == 0 {
            _g_boolean_var_34 = 1i32;
        }
        _g_boolean_var_34
    }) as c_long
        != 0
    {
        return 0 as gboolean;
    }
    cairo_save(cr);
    gtk_cairo_transform_to_window(cr, w, gtk_widget_get_window(w));
    gdk_cairo_get_clip_rectangle(cr, &raw mut area);
    if (*self_0).rubber_bending() != 0 {
        paint_rubber_banding_rect(self_0, cr, &raw mut area);
    }
    if !model.is_null() && gtk_tree_model_get_iter_first(model, &raw mut it) != 0 {
        loop {
            let mut item: *mut FmDesktopItem =
                fm_folder_model_get_item_userdata((*self_0).model, &raw mut it)
                    as *mut FmDesktopItem;
            let mut intersect: *mut GdkRectangle = ptr::null_mut::<GdkRectangle>();
            let mut tmp: GdkRectangle = _cairo_rectangle_int {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            let mut tmp2: GdkRectangle = _cairo_rectangle_int {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            let mut icon: *mut GdkPixbuf = ptr::null_mut::<GdkPixbuf>();
            if gdk_rectangle_intersect(&raw mut area, &raw mut (*item).icon_rect, &raw mut tmp) != 0
            {
                intersect = &raw mut tmp;
            } else {
                intersect = ptr::null_mut::<GdkRectangle>();
            }
            if gdk_rectangle_intersect(&raw mut area, &raw mut (*item).text_rect, &raw mut tmp2)
                != 0
            {
                if !intersect.is_null() {
                    gdk_rectangle_union(intersect, &raw mut tmp2, intersect);
                } else {
                    intersect = &raw mut tmp2;
                }
            }
            if !intersect.is_null() {
                gtk_tree_model_get(
                    model,
                    &raw mut it,
                    FM_FOLDER_MODEL_COL_ICON as c_int,
                    &raw mut icon,
                    -(1i32),
                );
                paint_item(self_0, item, cr, intersect, icon);
                if !icon.is_null() {
                    g_object_unref(icon as gpointer);
                }
            }
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        }
    }
    cairo_restore(cr);
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn on_size_allocate(mut w: *mut GtkWidget, mut alloc: *mut GtkAllocation) {
    let mut self_0: *mut FmDesktop = w as *mut FmDesktop;
    let mut pc: *mut PangoContext = ptr::null_mut::<PangoContext>();
    let mut metrics: *mut PangoFontMetrics = ptr::null_mut::<PangoFontMetrics>();
    let mut font_h: c_int = 0;
    pc = gtk_widget_get_pango_context(self_0 as *mut GtkWidget);
    metrics = pango_context_get_metrics(
        pc,
        ptr::null::<PangoFontDescription>(),
        ptr::null_mut::<PangoLanguage>(),
    );
    font_h = pango_font_metrics_get_ascent(metrics) + pango_font_metrics_get_descent(metrics);
    pango_font_metrics_unref(metrics);
    font_h /= 1024 as c_int;
    (*self_0).spacing = 2 as guint;
    (*self_0).ypad = 6 as guint;
    (*self_0).xpad = (*self_0).ypad;
    (*self_0).ymargin = 2 as gint;
    (*self_0).xmargin = (*self_0).ymargin;
    (*self_0).text_h = (font_h * 2 as c_int) as guint;
    (*self_0).text_w = 100 as guint;
    if (*fm_config).show_full_names != 0 {
        (*self_0).pango_text_h = -(1i32) as guint;
    } else {
        (*self_0).pango_text_h = (*self_0).text_h.wrapping_mul(1024 as guint);
    }
    (*self_0).pango_text_w = (*self_0).text_w.wrapping_mul(1024 as guint);
    (*self_0).text_h = (*self_0).text_h.wrapping_add(4 as guint);
    (*self_0).text_w = (*self_0).text_w.wrapping_add(4 as guint);
    (*self_0).cell_h = ((*fm_config).big_icon_size as guint)
        .wrapping_add((*self_0).spacing)
        .wrapping_add((*self_0).text_h)
        .wrapping_add((*self_0).ypad.wrapping_mul(2 as guint));
    (*self_0).cell_w = ((if (*self_0).text_w as gint > (*fm_config).big_icon_size {
        (*self_0).text_w as gint
    } else {
        (*fm_config).big_icon_size
    }) as guint)
        .wrapping_add((*self_0).xpad.wrapping_mul(2 as guint));
    update_working_area(self_0);
    if gtk_widget_get_realized(w) != 0 {
        let mut font_desc: *mut PangoFontDescription =
            pango_font_description_from_string((*self_0).conf.desktop_font);
        pango_context_set_font_description(pc, font_desc);
        pango_font_description_free(font_desc);
        _clear_bg_cache(self_0);
        if (*self_0).conf.wallpaper_mode as c_uint != FM_WP_COLOR as c_int as c_uint
            && (*self_0).conf.wallpaper_mode as c_uint != FM_WP_TILE as c_int as c_uint
        {
            update_background(self_0, -(1i32));
        }
    }
    (*(fm_desktop_parent_class as *mut GtkWidgetClass))
        .size_allocate
        .expect("non-null function pointer")(w, alloc);
}
unsafe extern "C" fn on_get_preferred_width(
    mut w: *mut GtkWidget,
    mut minimal_width: *mut gint,
    mut natural_width: *mut gint,
) {
    let mut scr: *mut GdkScreen = gtk_widget_get_screen(w);
    let mut monitor: gint = (*(w as *mut c_void as *mut FmDesktop)).monitor;
    let mut geom: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    gdk_screen_get_monitor_geometry(scr, monitor, &raw mut geom);
    *natural_width = geom.width as gint;
    *minimal_width = *natural_width;
}
unsafe extern "C" fn on_get_preferred_height(
    mut w: *mut GtkWidget,
    mut minimal_height: *mut gint,
    mut natural_height: *mut gint,
) {
    let mut scr: *mut GdkScreen = gtk_widget_get_screen(w);
    let mut monitor: gint = (*(w as *mut c_void as *mut FmDesktop)).monitor;
    let mut geom: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    gdk_screen_get_monitor_geometry(scr, monitor, &raw mut geom);
    *natural_height = geom.height as gint;
    *minimal_height = *natural_height;
}
unsafe extern "C" fn _stop_rubberbanding(mut self_0: *mut FmDesktop, mut x: gint, mut y: gint) {
    let mut drag_data: gpointer = g_object_get_data(
        self_0 as *mut c_void as *mut GObject,
        g_intern_static_string(b"gtk-site-data\0" as *const u8 as *const gchar),
    );
    if ({
        let mut _g_boolean_var_35: c_int = 0i32;
        if !drag_data.is_null() {
            _g_boolean_var_35 = 1i32;
        }
        _g_boolean_var_35
    }) as c_long
        != 0
    {
        g_signal_handlers_unblock_matched(
            self_0 as *mut c_void as *mut GObject as gpointer,
            G_SIGNAL_MATCH_DATA,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            ptr::null_mut::<c_void>(),
            drag_data,
        );
    }
    (*self_0).set_rubber_bending(0 as gboolean as gboolean);
    update_rubberbanding(self_0, x as c_int, y as c_int);
    gtk_grab_remove(self_0 as *mut c_void as *mut GtkWidget);
}
unsafe extern "C" fn on_button_press(
    mut w: *mut GtkWidget,
    mut evt: *mut GdkEventButton,
) -> gboolean {
    let mut self_0: *mut FmDesktop = w as *mut FmDesktop;
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut clicked_item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut clicked: FmFolderViewClickType = FM_FV_CLICK_NONE;
    clicked_item = hit_test(
        w as *mut c_void as *mut FmDesktop,
        &raw mut it,
        (*evt).x as c_int,
        (*evt).y as c_int,
    );
    if (*self_0).single_click_timeout_handler != 0 as guint {
        g_source_remove((*self_0).single_click_timeout_handler);
        (*self_0).single_click_timeout_handler = 0 as guint;
    }
    if (*evt).type_0 as c_int == GDK_BUTTON_PRESS as c_int {
        if (*self_0).button_pressed == 0 as guint {
            (*self_0).button_pressed = (*evt).button;
        }
        if (*evt).button == 1 as guint {
            (*self_0).drag_start_x = (*evt).x as gint;
            (*self_0).drag_start_y = (*evt).y as gint;
        } else if (*self_0).rubber_bending() != 0 {
            _stop_rubberbanding(self_0, (*self_0).drag_start_x, (*self_0).drag_start_y);
        }
        if (*evt).state & (GDK_SHIFT_MASK as c_int | GDK_CONTROL_MASK as c_int) as guint == 0 {
            if clicked_item.is_null() {
                if (*evt).button == 1 as guint {
                    _unselect_all(self_0 as *mut c_void as *mut FmFolderView);
                }
            } else if !(((*evt).button == 1 as guint || (*evt).button == 3 as guint)
                && (*clicked_item).is_selected() != 0)
            {
                _unselect_all(self_0 as *mut c_void as *mut FmFolderView);
            }
        }
        if !clicked_item.is_null() {
            if (*evt).state & (GDK_SHIFT_MASK as c_int | GDK_CONTROL_MASK as c_int) as guint != 0 {
                (*clicked_item).set_is_selected(
                    ((*clicked_item).is_selected() == 0) as c_int as gboolean as gboolean,
                );
            } else {
                (*clicked_item).set_is_selected((0i32 == 0) as c_int as gboolean as gboolean);
            }
            fm_desktop_item_selected_changed(self_0, clicked_item);
            if !(*self_0).focus.is_null() && (*self_0).focus != item {
                let mut old_focus: *mut FmDesktopItem = (*self_0).focus;
                (*self_0).focus = ptr::null_mut::<FmDesktopItem>();
                if !old_focus.is_null() {
                    redraw_item(self_0, old_focus);
                    fm_desktop_accessible_focus_unset(self_0, old_focus);
                }
            }
            (*self_0).focus = clicked_item;
            fm_desktop_accessible_focus_set(self_0, clicked_item);
            redraw_item(self_0, clicked_item);
            if (*evt).button == 3 as guint {
                clicked = FM_FV_CONTEXT_MENU;
            } else if (*evt).button == 2 as guint {
                clicked = FM_FV_MIDDLE_CLICK;
            }
        } else if (*evt).button == 3 as guint {
            if (*self_0).conf.show_wm_menu == 0 {
                clicked = FM_FV_CONTEXT_MENU;
            }
        } else if (*evt).button == 1 as guint {
            let mut drag_data: gpointer = g_object_get_data(
                self_0 as *mut c_void as *mut GObject,
                g_intern_static_string(b"gtk-site-data\0" as *const u8 as *const gchar),
            );
            if ({
                let mut _g_boolean_var_36: c_int = 0i32;
                if !drag_data.is_null() {
                    _g_boolean_var_36 = 1i32;
                }
                _g_boolean_var_36
            }) as c_long
                != 0
            {
                g_signal_handlers_block_matched(
                    self_0 as *mut c_void as *mut GObject as gpointer,
                    G_SIGNAL_MATCH_DATA,
                    0 as guint,
                    0 as GQuark,
                    ptr::null_mut::<GClosure>(),
                    ptr::null_mut::<c_void>(),
                    drag_data,
                );
            }
            (*self_0).set_rubber_bending((0i32 == 0) as c_int as gboolean as gboolean);
            gtk_grab_add(w);
            (*self_0).rubber_bending_x = (*evt).x as gint;
            (*self_0).rubber_bending_y = (*evt).y as gint;
        }
    } else if (*evt).type_0 as c_int == GDK_2BUTTON_PRESS as c_int {
        if !clicked_item.is_null() && (*evt).button == 1 as guint {
            clicked = FM_FV_ACTIVATED;
        }
    }
    if clicked as c_uint != FM_FV_CLICK_NONE as c_int as c_uint {
        let mut tp: *mut GtkTreePath = ptr::null_mut::<GtkTreePath>();
        if !(*self_0).model.is_null() && !clicked_item.is_null() {
            tp = gtk_tree_model_get_path(
                (*self_0).model as *mut c_void as *mut GtkTreeModel,
                &raw mut it,
            );
        }
        fm_folder_view_item_clicked(self_0 as *mut c_void as *mut FmFolderView, tp, clicked);
        if !tp.is_null() {
            gtk_tree_path_free(tp);
        }
        (*self_0).hover_item = ptr::null_mut::<FmDesktopItem>();
    } else if (*evt).button != 1 as guint && (*evt).button == (*self_0).button_pressed {
        (*self_0).set_forward_pending((0i32 == 0) as c_int as gboolean as gboolean);
        forward_event_to_rootwin(gtk_widget_get_screen(w), evt as *mut GdkEvent);
    }
    if gtk_widget_has_focus(w) == 0 {
        gtk_widget_grab_focus(w);
    }
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn on_button_release(
    mut w: *mut GtkWidget,
    mut evt: *mut GdkEventButton,
) -> gboolean {
    let mut self_0: *mut FmDesktop = w as *mut FmDesktop;
    if (*self_0).rubber_bending() != 0 {
        _stop_rubberbanding(self_0, (*evt).x as gint, (*evt).y as gint);
    } else if (*self_0).dragging() != 0 {
        (*self_0).set_dragging(0 as gboolean as gboolean);
        queue_layout_items(self_0);
    } else if (*fm_config).single_click != 0 && (*evt).button == 1 as guint
        || (*fm_config).c2rust_unnamed_0.middle_click != 0 && (*evt).button == 2 as guint
    {
        let mut it: GtkTreeIter = _GtkTreeIter {
            stamp: 0,
            user_data: ptr::null_mut::<c_void>(),
            user_data2: ptr::null_mut::<c_void>(),
            user_data3: ptr::null_mut::<c_void>(),
        };
        let mut clicked_item: *mut FmDesktopItem =
            hit_test(self_0, &raw mut it, (*evt).x as c_int, (*evt).y as c_int);
        if !clicked_item.is_null() {
            fm_launch_file_simple(
                w as *mut c_void as *mut GtkWindow,
                ptr::null_mut::<GAppLaunchContext>(),
                (*clicked_item).fi,
                Some(
                    pcmanfm_open_folder
                        as unsafe extern "C" fn(
                            *mut GAppLaunchContext,
                            *mut GList,
                            gpointer,
                            *mut *mut GError,
                        ) -> gboolean,
                ),
                w as gpointer,
            );
        }
    }
    if (*self_0).button_pressed == (*evt).button {
        if (*self_0).forward_pending() != 0 {
            forward_event_to_rootwin(gtk_widget_get_screen(w), evt as *mut GdkEvent);
        }
        (*self_0).button_pressed = 0 as guint;
        (*self_0).set_forward_pending(0 as gboolean as gboolean);
    }
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn on_single_click_timeout(mut user_data: gpointer) -> gboolean {
    let mut self_0: *mut FmDesktop = user_data as *mut FmDesktop;
    let mut w: *mut GtkWidget = self_0 as *mut GtkWidget;
    let mut window: *mut GdkWindow = ptr::null_mut::<GdkWindow>();
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut state: GdkModifierType = 0 as GdkModifierType;
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if g_source_is_destroyed(g_main_current_source()) != 0 {
        return 0 as gboolean;
    }
    (*self_0).single_click_timeout_handler = 0 as guint;
    if (*fm_config).single_click == 0 || (*self_0).hover_item.is_null() {
        return 0 as gboolean;
    }
    window = gtk_widget_get_window(w);
    gdk_window_get_pointer(window, &raw mut x, &raw mut y, &raw mut state);
    item = hit_test(self_0, &raw mut it, x, y);
    if item != (*self_0).hover_item {
        return 0 as gboolean;
    }
    state = mem::transmute::<c_uint, GdkModifierType>(
        state as c_uint & gtk_accelerator_get_default_mod_mask() as c_uint,
    );
    if state as c_uint == 0 as c_uint {
        _unselect_all(self_0 as *mut c_void as *mut FmFolderView);
        (*item).set_is_selected((0i32 == 0) as c_int as gboolean as gboolean);
    } else if state as c_uint == GDK_CONTROL_MASK as c_int as c_uint {
        (*item).set_is_selected(((*item).is_selected() == 0) as c_int as gboolean as gboolean);
    } else {
        return 0 as gboolean;
    }
    fm_desktop_item_selected_changed(self_0, item);
    set_focused_item(self_0, item);
    return 0 as gboolean;
}
unsafe extern "C" fn on_motion_notify(
    mut w: *mut GtkWidget,
    mut evt: *mut GdkEventMotion,
) -> gboolean {
    let mut self_0: *mut FmDesktop = w as *mut FmDesktop;
    if (*self_0).button_pressed == 0 {
        if (*fm_config).single_click != 0 {
            let mut it: GtkTreeIter = _GtkTreeIter {
                stamp: 0,
                user_data: ptr::null_mut::<c_void>(),
                user_data2: ptr::null_mut::<c_void>(),
                user_data3: ptr::null_mut::<c_void>(),
            };
            let mut item: *mut FmDesktopItem =
                hit_test(self_0, &raw mut it, (*evt).x as c_int, (*evt).y as c_int);
            let mut hover_item: *mut FmDesktopItem = (*self_0).hover_item;
            let mut window: *mut GdkWindow = ptr::null_mut::<GdkWindow>();
            if item != hover_item {
                if 0 as guint != (*self_0).single_click_timeout_handler {
                    g_source_remove((*self_0).single_click_timeout_handler);
                    (*self_0).single_click_timeout_handler = 0 as guint;
                }
                window = gtk_widget_get_window(w);
                (*self_0).hover_item = item;
                if !hover_item.is_null() {
                    redraw_item(self_0, hover_item);
                }
                if !item.is_null() {
                    redraw_item(self_0, item);
                    gdk_window_set_cursor(window, hand_cursor);
                    if (*fm_config).auto_selection_delay > 0i32 {
                        (*self_0).single_click_timeout_handler = gdk_threads_add_timeout(
                            (*fm_config).auto_selection_delay as guint,
                            Some(
                                on_single_click_timeout
                                    as unsafe extern "C" fn(gpointer) -> gboolean,
                            ),
                            self_0 as gpointer,
                        );
                    }
                } else {
                    gdk_window_set_cursor(window, ptr::null_mut::<GdkCursor>());
                }
            }
        } else {
            let mut it_0: GtkTreeIter = _GtkTreeIter {
                stamp: 0,
                user_data: ptr::null_mut::<c_void>(),
                user_data2: ptr::null_mut::<c_void>(),
                user_data3: ptr::null_mut::<c_void>(),
            };
            let mut item_0: *mut FmDesktopItem =
                hit_test(self_0, &raw mut it_0, (*evt).x as c_int, (*evt).y as c_int);
            let mut hover_item_0: *mut FmDesktopItem = (*self_0).hover_item;
            if item_0 != hover_item_0 {
                (*self_0).hover_item = item_0;
                if !hover_item_0.is_null() {
                    redraw_item(self_0, hover_item_0);
                }
                if !item_0.is_null() {
                    redraw_item(self_0, item_0);
                }
            }
        }
        return (0i32 == 0) as c_int;
    }
    if !((*self_0).dragging() != 0) {
        if (*self_0).rubber_bending() != 0 {
            update_rubberbanding(self_0, (*evt).x as c_int, (*evt).y as c_int);
        }
    }
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn on_leave_notify(
    mut w: *mut GtkWidget,
    mut evt: *mut GdkEventCrossing,
) -> gboolean {
    let mut self_0: *mut FmDesktop = w as *mut FmDesktop;
    if (*self_0).single_click_timeout_handler != 0 {
        g_source_remove((*self_0).single_click_timeout_handler);
        (*self_0).single_click_timeout_handler = 0 as guint;
    }
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn get_focused_item(
    mut focus: *mut FmDesktopItem,
    mut model: *mut GtkTreeModel,
    mut it: *mut GtkTreeIter,
) -> gboolean {
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    if gtk_tree_model_get_iter_first(model, it) != 0 {
        loop {
            item = fm_folder_model_get_item_userdata(model as *mut c_void as *mut FmFolderModel, it)
                as *mut FmDesktopItem;
            if item == focus {
                return (*item).is_selected();
            }
            if !(gtk_tree_model_iter_next(model, it) != 0) {
                break;
            }
        }
    }
    return 0 as gboolean;
}
unsafe extern "C" fn send_focus_change(mut widget: *mut GtkWidget, mut in_0: gboolean) {
    let mut fevent: *mut GdkEvent = gdk_event_new(GDK_FOCUS_CHANGE);
    (*fevent).focus_change.type_0 = GDK_FOCUS_CHANGE;
    (*fevent).focus_change.window =
        g_object_ref(gtk_widget_get_window(widget) as gpointer) as *mut GdkWindow as *mut GdkWindow;
    (*fevent).focus_change.in_0 = in_0 as gint16;
    gtk_widget_send_focus_change(widget, fevent);
    gdk_event_free(fevent);
}
unsafe extern "C" fn desktop_search_dialog_hide(
    mut search_dialog: *mut GtkWidget,
    mut desktop: *mut FmDesktop,
) {
    if (*desktop).search_entry_changed_id != 0 as guint {
        g_signal_handler_disconnect(
            (*desktop).search_entry as gpointer,
            (*desktop).search_entry_changed_id as gulong,
        );
        (*desktop).search_entry_changed_id = 0 as guint;
    }
    if (*desktop).search_timeout_id != 0 as guint {
        g_source_remove((*desktop).search_timeout_id);
        (*desktop).search_timeout_id = 0 as guint;
    }
    send_focus_change((*desktop).search_entry, 0 as gboolean);
    gtk_widget_hide(search_dialog);
    gtk_entry_set_text(
        (*desktop).search_entry as *mut c_void as *mut GtkEntry,
        b"\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn desktop_search_delete_event(
    mut widget: *mut GtkWidget,
    mut evt: *mut GdkEventAny,
    mut desktop: *mut FmDesktop,
) -> gboolean {
    desktop_search_dialog_hide(widget, desktop);
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn desktop_search_timeout(mut user_data: gpointer) -> gboolean {
    let mut desktop: *mut FmDesktop = user_data as *mut FmDesktop;
    if g_source_is_destroyed(g_main_current_source()) == 0 {
        desktop_search_dialog_hide((*desktop).search_window, desktop);
    }
    return 0 as gboolean;
}
unsafe extern "C" fn desktop_search_timeout_destroy(mut user_data: gpointer) {
    (*(user_data as *mut FmDesktop)).search_timeout_id = 0 as guint;
}
unsafe extern "C" fn desktop_search_move(
    mut widget: *mut GtkWidget,
    mut desktop: *mut FmDesktop,
    mut move_up: gboolean,
) {
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut text: *const gchar = ptr::null::<gchar>();
    let mut casefold: *mut c_char = ptr::null_mut::<c_char>();
    let mut key: *mut c_char = ptr::null_mut::<c_char>();
    let mut name: *mut c_char = ptr::null_mut::<c_char>();
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut found: gboolean = 0 as gboolean;
    if (*desktop).model.is_null() {
        return;
    }
    model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
    text = gtk_entry_get_text((*desktop).search_entry as *mut c_void as *mut GtkEntry);
    if ({
        let mut _g_boolean_var_37: c_int = 0i32;
        if text.is_null() || *text.offset(0 as isize) as c_int == 0 {
            _g_boolean_var_37 = 1i32;
        }
        _g_boolean_var_37
    }) as c_long
        != 0
    {
        return;
    }
    if get_focused_item((*desktop).focus, model, &raw mut it) == 0 {
        return;
    }
    casefold = g_utf8_casefold(text, -(1i32) as gssize) as *mut c_char;
    key = g_utf8_normalize(casefold, -(1i32) as gssize, G_NORMALIZE_ALL) as *mut c_char;
    g_free(casefold as gpointer);
    if move_up != 0 {
        while found == 0 && gtk_tree_model_iter_previous(model, &raw mut it) != 0 {
            item = fm_folder_model_get_item_userdata((*desktop).model, &raw mut it)
                as *mut FmDesktopItem;
            casefold = g_utf8_casefold(
                fm_file_info_get_disp_name((*item).fi) as *const gchar,
                -(1i32) as gssize,
            ) as *mut c_char;
            name = g_utf8_normalize(casefold, -(1i32) as gssize, G_NORMALIZE_ALL) as *mut c_char;
            g_free(casefold as gpointer);
            found = (strncmp(name, key, strlen(key)) == 0i32) as c_int as gboolean;
            g_free(name as gpointer);
        }
    } else {
        while found == 0 && gtk_tree_model_iter_next(model, &raw mut it) != 0 {
            item = fm_folder_model_get_item_userdata((*desktop).model, &raw mut it)
                as *mut FmDesktopItem;
            casefold = g_utf8_casefold(
                fm_file_info_get_disp_name((*item).fi) as *const gchar,
                -(1i32) as gssize,
            ) as *mut c_char;
            name = g_utf8_normalize(casefold, -(1i32) as gssize, G_NORMALIZE_ALL) as *mut c_char;
            g_free(casefold as gpointer);
            found = (strncmp(name, key, strlen(key)) == 0i32) as c_int as gboolean;
            g_free(name as gpointer);
        }
    }
    g_free(key as gpointer);
    if found == 0 {
        return;
    }
    _unselect_all(desktop as *mut c_void as *mut FmFolderView);
    _focus_and_select_focused_item(desktop, item);
}
unsafe extern "C" fn desktop_search_scroll_event(
    mut widget: *mut GtkWidget,
    mut evt: *mut GdkEventScroll,
    mut desktop: *mut FmDesktop,
) -> gboolean {
    if (*evt).direction as c_uint == GDK_SCROLL_UP as c_int as c_uint {
        desktop_search_move(widget, desktop, (0i32 == 0) as c_int);
    } else if (*evt).direction as c_uint == GDK_SCROLL_DOWN as c_int as c_uint {
        desktop_search_move(widget, desktop, 0 as gboolean);
    } else {
        return 0 as gboolean;
    }
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn desktop_search_update_timeout(mut desktop: *mut FmDesktop) {
    if (*desktop).search_timeout_id == 0 as guint {
        return;
    }
    g_source_remove((*desktop).search_timeout_id);
    (*desktop).search_timeout_id = gdk_threads_add_timeout_full(
        300 as gint,
        5000 as guint,
        Some(desktop_search_timeout as unsafe extern "C" fn(gpointer) -> gboolean),
        desktop as gpointer,
        Some(desktop_search_timeout_destroy as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn desktop_search_key_press_event(
    mut widget: *mut GtkWidget,
    mut evt: *mut GdkEventKey,
    mut desktop: *mut FmDesktop,
) -> gboolean {
    if (*evt).keyval == 0xff1b as guint || (*evt).keyval == 0xff09 as guint {
        desktop_search_dialog_hide(widget, desktop);
    } else if (*evt).keyval == 0xff52 as guint || (*evt).keyval == 0xff97 as guint {
        desktop_search_move(widget, desktop, (0i32 == 0) as c_int);
    } else if (*evt).state & (GDK_CONTROL_MASK as c_int | GDK_SHIFT_MASK as c_int) as guint
        == (GDK_CONTROL_MASK as c_int | GDK_SHIFT_MASK as c_int) as guint
        && ((*evt).keyval == 0x67 as guint || (*evt).keyval == 0x47 as guint)
    {
        desktop_search_move(widget, desktop, (0i32 == 0) as c_int);
    } else if (*evt).keyval == 0xff54 as guint || (*evt).keyval == 0xff99 as guint {
        desktop_search_move(widget, desktop, 0 as gboolean);
    } else if (*evt).state & (GDK_CONTROL_MASK as c_int | GDK_SHIFT_MASK as c_int) as guint
        == GDK_CONTROL_MASK as c_int as guint
        && ((*evt).keyval == 0x67 as guint || (*evt).keyval == 0x47 as guint)
    {
        desktop_search_move(widget, desktop, 0 as gboolean);
    } else {
        return 0 as gboolean;
    }
    desktop_search_update_timeout(desktop);
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn desktop_search_activate(
    mut entry: *mut GtkEntry,
    mut desktop: *mut FmDesktop,
) {
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut tp: *mut GtkTreePath = ptr::null_mut::<GtkTreePath>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    desktop_search_dialog_hide((*desktop).search_window, desktop);
    if !(*desktop).focus.is_null() {
        if (*(*desktop).focus).is_selected() != 0 {
            model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
            if get_focused_item((*desktop).focus, model, &raw mut it) != 0 {
                tp = gtk_tree_model_get_path(model, &raw mut it);
                fm_folder_view_item_clicked(
                    desktop as *mut c_void as *mut FmFolderView,
                    tp,
                    FM_FV_ACTIVATED,
                );
                if !tp.is_null() {
                    gtk_tree_path_free(tp);
                }
            }
        }
    }
}
unsafe extern "C" fn desktop_search_preedit_changed(
    mut entry: *mut GtkEntry,
    mut preedit: *mut gchar,
    mut desktop: *mut FmDesktop,
) {
    (*desktop).set_search_imcontext_changed((0i32 == 0) as c_int as gboolean as gboolean);
    desktop_search_update_timeout(desktop);
}
unsafe extern "C" fn desktop_search_position(mut desktop: *mut FmDesktop) {
    let mut requisition: GtkRequisition = _GtkRequisition {
        width: 0,
        height: 0,
    };
    let mut geom: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut x: gint = 0;
    let mut y: gint = 0;
    gtk_widget_realize((*desktop).search_window);
    gtk_widget_get_preferred_size(
        (*desktop).search_window,
        ptr::null_mut::<GtkRequisition>(),
        &raw mut requisition,
    );
    gdk_screen_get_monitor_geometry(
        gtk_widget_get_screen(desktop as *mut GtkWidget),
        (*desktop).monitor,
        &raw mut geom,
    );
    x = geom.x as gint + (*desktop).working_area.x as gint + (*desktop).working_area.width as gint
        - requisition.width;
    y = (geom.y + (*desktop).working_area.y) as gint;
    gtk_window_move(
        (*desktop).search_window as *mut c_void as *mut GtkWindow,
        x,
        y,
    );
}
unsafe extern "C" fn desktop_search_init(
    mut search_entry: *mut GtkWidget,
    mut desktop: *mut FmDesktop,
) {
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut text: *const gchar = ptr::null::<gchar>();
    let mut casefold: *mut c_char = ptr::null_mut::<c_char>();
    let mut key: *mut c_char = ptr::null_mut::<c_char>();
    let mut name: *mut c_char = ptr::null_mut::<c_char>();
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut found: gboolean = 0 as gboolean;
    if (*desktop).model.is_null() {
        return;
    }
    model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
    desktop_search_update_timeout(desktop);
    text = gtk_entry_get_text((*desktop).search_entry as *mut c_void as *mut GtkEntry);
    if ({
        let mut _g_boolean_var_38: c_int = 0i32;
        if text.is_null() || *text.offset(0 as isize) as c_int == 0 {
            _g_boolean_var_38 = 1i32;
        }
        _g_boolean_var_38
    }) as c_long
        != 0
    {
        return;
    }
    _unselect_all(desktop as *mut c_void as *mut FmFolderView);
    casefold = g_utf8_casefold(text, -(1i32) as gssize) as *mut c_char;
    key = g_utf8_normalize(casefold, -(1i32) as gssize, G_NORMALIZE_ALL) as *mut c_char;
    g_free(casefold as gpointer);
    if gtk_tree_model_get_iter_first(model, &raw mut it) != 0 {
        loop {
            item = fm_folder_model_get_item_userdata((*desktop).model, &raw mut it)
                as *mut FmDesktopItem;
            casefold = g_utf8_casefold(
                fm_file_info_get_disp_name((*item).fi) as *const gchar,
                -(1i32) as gssize,
            ) as *mut c_char;
            name = g_utf8_normalize(casefold, -(1i32) as gssize, G_NORMALIZE_ALL) as *mut c_char;
            g_free(casefold as gpointer);
            found = (strncmp(name, key, strlen(key)) == 0i32) as c_int as gboolean;
            g_free(name as gpointer);
            if !(found == 0 && gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        }
    }
    g_free(key as gpointer);
    if found == 0 {
        return;
    }
    _focus_and_select_focused_item(desktop, item);
}
unsafe extern "C" fn desktop_search_ensure_window(mut desktop: *mut FmDesktop) {
    let mut window: *mut GtkWindow = ptr::null_mut::<GtkWindow>();
    let mut frame: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
    let mut vbox: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
    if ({
        let mut _g_boolean_var_39: c_int = 0i32;
        if !(*desktop).search_window.is_null() {
            _g_boolean_var_39 = 1i32;
        }
        _g_boolean_var_39
    }) as c_long
        != 0
    {
        return;
    }
    (*desktop).search_window = gtk_window_new(GTK_WINDOW_POPUP);
    window = (*desktop).search_window as *mut c_void as *mut GtkWindow;
    gtk_window_group_add_window(win_group, window);
    gtk_window_set_modal(window, (0i32 == 0) as c_int);
    gtk_window_set_screen(
        window,
        gtk_widget_get_screen(desktop as *mut c_void as *mut GtkWidget),
    );
    g_signal_connect_data(
        window as gpointer,
        b"delete-event\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventAny, *mut FmDesktop) -> gboolean,
            >,
            GCallback,
        >(Some(
            desktop_search_delete_event
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventAny,
                    *mut FmDesktop,
                ) -> gboolean,
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        window as gpointer,
        b"scroll-event\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventScroll,
                    *mut FmDesktop,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            desktop_search_scroll_event
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventScroll,
                    *mut FmDesktop,
                ) -> gboolean,
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        window as gpointer,
        b"key-press-event\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventKey, *mut FmDesktop) -> gboolean,
            >,
            GCallback,
        >(Some(
            desktop_search_key_press_event
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventKey,
                    *mut FmDesktop,
                ) -> gboolean,
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    frame = g_object_new(
        gtk_frame_get_type(),
        b"shadow-type\0" as *const u8 as *const gchar,
        GTK_SHADOW_ETCHED_IN as c_int,
        ptr::null_mut::<c_void>(),
    ) as *mut GtkWidget;
    gtk_container_add(window as *mut c_void as *mut GtkContainer, frame);
    gtk_widget_show(frame);
    vbox = g_object_new(
        gtk_vbox_get_type(),
        b"border-width\0" as *const u8 as *const gchar,
        3 as c_int,
        ptr::null_mut::<c_void>(),
    ) as *mut GtkWidget;
    gtk_container_add(frame as *mut c_void as *mut GtkContainer, vbox);
    gtk_widget_show(vbox);
    (*desktop).search_entry = gtk_entry_new();
    g_signal_connect_data(
        (*desktop).search_entry as gpointer,
        b"activate\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkEntry, *mut FmDesktop) -> ()>,
            GCallback,
        >(Some(
            desktop_search_activate as unsafe extern "C" fn(*mut GtkEntry, *mut FmDesktop) -> (),
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*desktop).search_entry as gpointer,
        b"preedit-changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkEntry, *mut gchar, *mut FmDesktop) -> ()>,
            GCallback,
        >(Some(
            desktop_search_preedit_changed
                as unsafe extern "C" fn(*mut GtkEntry, *mut gchar, *mut FmDesktop) -> (),
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    gtk_box_pack_start(
        vbox as *mut c_void as *mut GtkBox,
        (*desktop).search_entry,
        (0i32 == 0) as c_int,
        (0i32 == 0) as c_int,
        0 as guint,
    );
    gtk_widget_realize((*desktop).search_entry);
    gtk_widget_show((*desktop).search_entry);
}
unsafe extern "C" fn desktop_search_start(
    mut desktop: *mut FmDesktop,
    mut keybinding: gboolean,
) -> gboolean {
    let mut klass: *mut GTypeClass = ptr::null_mut::<GTypeClass>();
    if !(*desktop).search_window.is_null() && gtk_widget_get_visible((*desktop).search_window) != 0
    {
        return (0i32 == 0) as c_int;
    }
    desktop_search_ensure_window(desktop);
    if ({
        let mut _g_boolean_var_40: c_int = 0i32;
        if keybinding != 0 {
            _g_boolean_var_40 = 1i32;
        }
        _g_boolean_var_40
    }) as c_long
        != 0
    {
        gtk_entry_set_text(
            (*desktop).search_entry as *mut c_void as *mut GtkEntry,
            b"\0" as *const u8 as *const gchar,
        );
    }
    desktop_search_position(desktop);
    gtk_widget_show((*desktop).search_window);
    if ({
        let mut _g_boolean_var_41: c_int = 0i32;
        if (*desktop).search_entry_changed_id == 0 as guint {
            _g_boolean_var_41 = 1i32;
        }
        _g_boolean_var_41
    }) as c_long
        != 0
    {
        (*desktop).search_entry_changed_id = g_signal_connect_data(
            (*desktop).search_entry as *mut c_void as *mut GObject as gpointer,
            b"changed\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkWidget, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                desktop_search_init as unsafe extern "C" fn(*mut GtkWidget, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        ) as guint;
    }
    (*desktop).search_timeout_id = gdk_threads_add_timeout_full(
        300 as gint,
        5000 as guint,
        Some(desktop_search_timeout as unsafe extern "C" fn(gpointer) -> gboolean),
        desktop as gpointer,
        Some(desktop_search_timeout_destroy as unsafe extern "C" fn(gpointer) -> ()),
    );
    klass = g_type_class_peek_parent(
        (*((*desktop).search_entry as *mut GTypeInstance)).g_class as *mut GtkEntryClass
            as gpointer,
    ) as *mut GTypeClass;
    Some(
        (*(klass as *mut c_void as *mut GtkWidgetClass))
            .grab_focus
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")((*desktop).search_entry);
    send_focus_change((*desktop).search_entry, (0i32 == 0) as c_int);
    desktop_search_init((*desktop).search_entry, desktop);
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn on_key_press(mut w: *mut GtkWidget, mut evt: *mut GdkEventKey) -> gboolean {
    let mut desktop: *mut FmDesktop = w as *mut FmDesktop;
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut modifier: c_int =
        ((*evt).state as c_uint & gtk_accelerator_get_default_mod_mask() as c_uint) as c_int;
    let mut sels: *mut FmPathList = ptr::null_mut::<FmPathList>();
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut tp: *mut GtkTreePath = ptr::null_mut::<GtkTreePath>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut old_text: *mut c_char = ptr::null_mut::<c_char>();
    let mut new_text: *mut c_char = ptr::null_mut::<c_char>();
    let mut screen: *mut GdkScreen = ptr::null_mut::<GdkScreen>();
    let mut new_event: *mut GdkEvent = ptr::null_mut::<GdkEvent>();
    let mut popup_menu_id: guint = 0;
    let mut retval: gboolean = 0;
    match (*evt).keyval {
        65307 => {
            if (*desktop).rubber_bending() != 0 {
                _stop_rubberbanding(desktop, (*desktop).drag_start_x, (*desktop).drag_start_y);
                return (0i32 == 0) as c_int;
            }
        }
        65361 => {
            item = get_nearest_item(desktop, (*desktop).focus, GTK_DIR_LEFT);
            if !item.is_null() {
                if 0i32 == modifier {
                    _unselect_all(desktop as *mut c_void as *mut FmFolderView);
                    (*item).set_is_selected((0i32 == 0) as c_int as gboolean as gboolean);
                    fm_desktop_item_selected_changed(desktop, item);
                }
                set_focused_item(desktop, item);
            }
            return (0i32 == 0) as c_int;
        }
        65363 => {
            item = get_nearest_item(desktop, (*desktop).focus, GTK_DIR_RIGHT);
            if !item.is_null() {
                if 0i32 == modifier {
                    _unselect_all(desktop as *mut c_void as *mut FmFolderView);
                    (*item).set_is_selected((0i32 == 0) as c_int as gboolean as gboolean);
                    fm_desktop_item_selected_changed(desktop, item);
                }
                set_focused_item(desktop, item);
            }
            return (0i32 == 0) as c_int;
        }
        65362 => {
            item = get_nearest_item(desktop, (*desktop).focus, GTK_DIR_UP);
            if !item.is_null() {
                if 0i32 == modifier {
                    _unselect_all(desktop as *mut c_void as *mut FmFolderView);
                    (*item).set_is_selected((0i32 == 0) as c_int as gboolean as gboolean);
                    fm_desktop_item_selected_changed(desktop, item);
                }
                set_focused_item(desktop, item);
            }
            return (0i32 == 0) as c_int;
        }
        65364 => {
            item = get_nearest_item(desktop, (*desktop).focus, GTK_DIR_DOWN);
            if !item.is_null() {
                if 0i32 == modifier {
                    _unselect_all(desktop as *mut c_void as *mut FmFolderView);
                    (*item).set_is_selected((0i32 == 0) as c_int as gboolean as gboolean);
                    fm_desktop_item_selected_changed(desktop, item);
                }
                set_focused_item(desktop, item);
            }
            return (0i32 == 0) as c_int;
        }
        32 => {
            if modifier & GDK_CONTROL_MASK as c_int != 0 {
                if !(*desktop).focus.is_null() {
                    (*(*desktop).focus).set_is_selected(
                        ((*(*desktop).focus).is_selected() == 0) as c_int as gboolean as gboolean,
                    );
                    redraw_item(desktop, (*desktop).focus);
                    fm_desktop_item_selected_changed(desktop, (*desktop).focus);
                }
                return (0i32 == 0) as c_int;
            }
        }
        65471 => {
            sels = _dup_selected_file_paths(desktop as *mut c_void as *mut FmFolderView);
            if !sels.is_null() {
                fm_rename_file(
                    desktop as *mut c_void as *mut GtkWindow,
                    fm_path_list_peek_head(sels),
                );
                fm_path_list_unref(sels);
                return (0i32 == 0) as c_int;
            }
        }
        65293 | 65076 | 65421 => {
            if modifier == 0i32 && !(*desktop).focus.is_null() {
                model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
                if get_focused_item((*desktop).focus, model, &raw mut it) != 0 {
                    tp = gtk_tree_model_get_path(model, &raw mut it);
                    fm_folder_view_item_clicked(
                        desktop as *mut c_void as *mut FmFolderView,
                        tp,
                        FM_FV_ACTIVATED,
                    );
                    if !tp.is_null() {
                        gtk_tree_path_free(tp);
                    }
                }
                return (0i32 == 0) as c_int;
            }
        }
        70 | 102 => {
            if modifier == GDK_CONTROL_MASK as c_int {
                return desktop_search_start(desktop, (0i32 == 0) as c_int);
            }
        }
        _ => {}
    }
    if (*(fm_desktop_parent_class as *mut GtkWidgetClass))
        .key_press_event
        .expect("non-null function pointer")(w, evt)
        != 0
    {
        return (0i32 == 0) as c_int;
    }
    desktop_search_ensure_window(desktop);
    gtk_widget_realize((*desktop).search_window);
    old_text = gtk_editable_get_chars(
        (*desktop).search_entry as *mut c_void as *mut GtkEditable,
        0 as gint,
        -(1 as gint),
    ) as *mut c_char;
    popup_menu_id = g_signal_connect_data(
        (*desktop).search_entry as gpointer,
        b"popup-menu\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn() -> gboolean>, GCallback>(Some(
            gtk_true as unsafe extern "C" fn() -> gboolean,
        )),
        ptr::null_mut::<c_void>(),
        None,
        G_CONNECT_DEFAULT,
    ) as guint;
    screen = gtk_widget_get_screen(w);
    gtk_window_move(
        (*desktop).search_window as *mut c_void as *mut GtkWindow,
        gdk_screen_get_width(screen) + 1 as gint,
        gdk_screen_get_height(screen) + 1 as gint,
    );
    gtk_widget_show((*desktop).search_window);
    new_event = gdk_event_copy(evt as *mut GdkEvent);
    g_object_unref((*new_event).key.window as gpointer);
    (*new_event).key.window =
        g_object_ref(gtk_widget_get_window((*desktop).search_entry) as gpointer) as *mut GdkWindow
            as *mut GdkWindow;
    (*desktop).set_search_imcontext_changed(0 as gboolean as gboolean);
    retval = gtk_widget_event((*desktop).search_entry, new_event);
    gtk_widget_hide((*desktop).search_window);
    gdk_event_free(new_event);
    g_signal_handler_disconnect((*desktop).search_entry as gpointer, popup_menu_id as gulong);
    new_text = gtk_editable_get_chars(
        (*desktop).search_entry as *mut c_void as *mut GtkEditable,
        0 as gint,
        -(1 as gint),
    ) as *mut c_char;
    retval = (retval != 0 && strcmp(new_text, old_text) != 0i32) as c_int as gboolean;
    g_free(old_text as gpointer);
    g_free(new_text as gpointer);
    if (*desktop).search_imcontext_changed() != 0 || retval != 0 {
        if desktop_search_start(desktop, 0 as gboolean) != 0 {
            gtk_widget_grab_focus(w);
            return (0i32 == 0) as c_int;
        } else {
            gtk_entry_set_text(
                (*desktop).search_entry as *mut c_void as *mut GtkEntry,
                b"\0" as *const u8 as *const gchar,
            );
            return 0 as gboolean;
        }
    }
    return 0 as gboolean;
}
unsafe extern "C" fn on_direction_changed(mut w: *mut GtkWidget, mut prev: GtkTextDirection) {
    let mut self_0: *mut FmDesktop = w as *mut FmDesktop;
    pango_layout_context_changed((*self_0).pl);
    queue_layout_items(self_0);
}
unsafe extern "C" fn on_realize(mut w: *mut GtkWidget) {
    let mut self_0: *mut FmDesktop = w as *mut FmDesktop;
    let mut font_desc: *mut PangoFontDescription = ptr::null_mut::<PangoFontDescription>();
    let mut pc: *mut PangoContext = ptr::null_mut::<PangoContext>();
    let mut css_data: *mut c_char = ptr::null_mut::<c_char>();
    (*(fm_desktop_parent_class as *mut GtkWidgetClass))
        .realize
        .expect("non-null function pointer")(w);
    gtk_window_set_skip_pager_hint(w as *mut c_void as *mut GtkWindow, (0i32 == 0) as c_int);
    gtk_window_set_skip_taskbar_hint(w as *mut c_void as *mut GtkWindow, (0i32 == 0) as c_int);
    gtk_window_set_resizable(w as *mut GtkWindow, 0 as gboolean);
    load_config(self_0);
    if (*self_0).conf.configured() == 0 {
        copy_desktop_config(
            &raw mut (*self_0).conf,
            &raw mut (*(fm_config as *mut FmAppConfig)).desktop_section,
        );
        queue_config_save(self_0);
    } else if (*(fm_config as *mut FmAppConfig))
        .desktop_section
        .configured()
        == 0
    {
        copy_desktop_config(
            &raw mut (*(fm_config as *mut FmAppConfig)).desktop_section,
            &raw mut (*self_0).conf,
        );
    }
    update_background(self_0, -(1i32));
    if (*self_0).conf.desktop_font.is_null() {
        (*self_0).conf.desktop_font = g_strdup_inline(c"Sans 12".as_ptr());
    }
    font_desc = pango_font_description_from_string((*self_0).conf.desktop_font);
    pc = gtk_widget_get_pango_context(w);
    pango_context_set_font_description(pc, font_desc);
    pango_font_description_free(font_desc);
    css_data = g_strdup_printf(
        b"FmDesktop {\nbackground-color: #%02x%02x%02x\n}\0" as *const u8 as *const gchar,
        (*self_0).conf.desktop_bg.red as c_int / 256 as c_int,
        (*self_0).conf.desktop_bg.green as c_int / 256 as c_int,
        (*self_0).conf.desktop_bg.blue as c_int / 256 as c_int,
    ) as *mut c_char;
    gtk_css_provider_load_from_data(
        (*self_0).css,
        css_data,
        -(1i32) as gssize,
        ptr::null_mut::<*mut GError>(),
    );
    g_free(css_data as gpointer);
    if (*self_0).layout_pending() != 0 {
        queue_layout_items(self_0);
    }
}
unsafe extern "C" fn on_focus_in(mut w: *mut GtkWidget, mut evt: *mut GdkEventFocus) -> gboolean {
    let mut self_0: *mut FmDesktop = w as *mut FmDesktop;
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if (*self_0).focus.is_null()
        && !(*self_0).model.is_null()
        && gtk_tree_model_get_iter_first(
            (*self_0).model as *mut c_void as *mut GtkTreeModel,
            &raw mut it,
        ) != 0
    {
        (*self_0).focus =
            fm_folder_model_get_item_userdata((*self_0).model, &raw mut it) as *mut FmDesktopItem;
        fm_desktop_accessible_focus_set(self_0, (*self_0).focus);
    }
    if !(*self_0).focus.is_null() {
        redraw_item(self_0, (*self_0).focus);
    }
    return 0 as gboolean;
}
unsafe extern "C" fn on_focus_out(mut w: *mut GtkWidget, mut evt: *mut GdkEventFocus) -> gboolean {
    let mut self_0: *mut FmDesktop = w as *mut FmDesktop;
    if !(*self_0).focus.is_null() {
        redraw_item(self_0, (*self_0).focus);
    }
    return 0 as gboolean;
}
unsafe extern "C" fn on_drag_motion(
    mut dest_widget: *mut GtkWidget,
    mut drag_context: *mut GdkDragContext,
    mut x: gint,
    mut y: gint,
    mut time: guint,
) -> gboolean {
    let mut target: GdkAtom = ptr::null_mut::<_GdkAtom>();
    let mut action: GdkDragAction = 0 as GdkDragAction;
    let mut desktop: *mut FmDesktop = dest_widget as *mut c_void as *mut FmDesktop;
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if (*desktop).model.is_null() {
        fm_dnd_dest_set_dest_file((*desktop).dnd_dest, ptr::null_mut::<FmFileInfo>());
        gdk_drag_status(drag_context, action, time as guint32);
        return 0 as gboolean;
    }
    item = hit_test(desktop, &raw mut it, x as c_int, y as c_int);
    if item.is_null() {
        if !g_list_find(
            gdk_drag_context_list_targets(drag_context),
            desktop_atom as gconstpointer,
        )
        .is_null()
            && gdk_drag_context_get_actions(drag_context) as c_uint
                & GDK_ACTION_MOVE as c_int as c_uint
                != 0
        {
            action = GDK_ACTION_MOVE;
            fm_dnd_dest_set_dest_file((*desktop).dnd_dest, ptr::null_mut::<FmFileInfo>());
        }
    }
    if action as u64 == 0 {
        fm_dnd_dest_set_dest_file(
            (*desktop).dnd_dest,
            if !item.is_null() {
                (*item).fi
            } else {
                fm_folder_get_info(fm_folder_model_get_folder((*desktop).model))
            },
        );
        target = fm_dnd_dest_find_target((*desktop).dnd_dest, drag_context);
        if !target.is_null() && fm_dnd_dest_is_target_supported((*desktop).dnd_dest, target) != 0 {
            action = fm_dnd_dest_get_default_action((*desktop).dnd_dest, drag_context, target);
        }
    }
    gdk_drag_status(drag_context, action, time as guint32);
    if (*desktop).drop_hilight != item {
        let mut old_drop: *mut FmDesktopItem = (*desktop).drop_hilight;
        if action as u64 != 0 {
            (*desktop).drop_hilight = item;
        }
        if !old_drop.is_null() {
            redraw_item(desktop, old_drop);
        }
        if !item.is_null() && action as c_uint != 0 {
            redraw_item(desktop, item);
        }
    }
    return (action as c_uint != 0 as c_uint) as c_int;
}
unsafe extern "C" fn on_drag_leave(
    mut dest_widget: *mut GtkWidget,
    mut drag_context: *mut GdkDragContext,
    mut time: guint,
) {
    let mut desktop: *mut FmDesktop = dest_widget as *mut c_void as *mut FmDesktop;
    if !(*desktop).drop_hilight.is_null() {
        let mut old_drop: *mut FmDesktopItem = (*desktop).drop_hilight;
        (*desktop).drop_hilight = ptr::null_mut::<FmDesktopItem>();
        redraw_item(desktop, old_drop);
    }
}
unsafe extern "C" fn on_drag_drop(
    mut dest_widget: *mut GtkWidget,
    mut drag_context: *mut GdkDragContext,
    mut x: gint,
    mut y: gint,
    mut time: guint,
) -> gboolean {
    let mut desktop: *mut FmDesktop = dest_widget as *mut c_void as *mut FmDesktop;
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    item = hit_test(desktop, &raw mut it, x as c_int, y as c_int);
    if item.is_null() {
        if !g_list_find(
            gdk_drag_context_list_targets(drag_context),
            desktop_atom as gconstpointer,
        )
        .is_null()
            && gdk_drag_context_get_actions(drag_context) as c_uint
                & GDK_ACTION_MOVE as c_int as c_uint
                != 0
        {
            gtk_drag_get_data(dest_widget, drag_context, desktop_atom, time as guint32);
            return (0i32 == 0) as c_int;
        }
    }
    return 0 as gboolean;
}
unsafe extern "C" fn on_drag_data_received(
    mut dest_widget: *mut GtkWidget,
    mut drag_context: *mut GdkDragContext,
    mut x: gint,
    mut y: gint,
    mut sel_data: *mut GtkSelectionData,
    mut info: guint,
    mut time: guint,
) {
    let mut desktop: *mut FmDesktop = dest_widget as *mut c_void as *mut FmDesktop;
    let mut items: *mut GList = ptr::null_mut::<GList>();
    let mut l: *mut GList = ptr::null_mut::<GList>();
    let mut offset_x: c_int = 0;
    let mut offset_y: c_int = 0;
    if info != FM_DND_DEST_DESKTOP_ITEM as c_int as guint {
        return;
    }
    items = get_selected_items(desktop, ptr::null_mut::<c_int>());
    offset_x = (x - (*desktop).drag_start_x) as c_int;
    offset_y = (y - (*desktop).drag_start_y) as c_int;
    l = items;
    while !l.is_null() {
        let mut item: *mut FmDesktopItem = (*l).data as *mut FmDesktopItem;
        move_item(
            desktop,
            item,
            (*item).area.x + offset_x,
            (*item).area.y + offset_y,
            0 as gboolean,
        );
        l = (*l).next;
    }
    g_list_free(items);
    queue_config_save(desktop);
    queue_layout_items(desktop);
    gtk_drag_finish(
        drag_context,
        (0i32 == 0) as c_int,
        0 as gboolean,
        time as guint32,
    );
}
unsafe extern "C" fn _create_drag_icon(
    mut desktop: *mut FmDesktop,
    mut x: *mut gint,
    mut y: *mut gint,
) -> *mut GdkPixbuf {
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut item: *mut FmDesktopItem = ptr::null_mut::<FmDesktopItem>();
    let mut s: *mut cairo_surface_t = ptr::null_mut::<cairo_surface_t>();
    let mut pixbuf: *mut GdkPixbuf = ptr::null_mut::<GdkPixbuf>();
    let mut cr: *mut cairo_t = ptr::null_mut::<cairo_t>();
    let mut icon: *mut GdkPixbuf = ptr::null_mut::<GdkPixbuf>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut area: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut icon_rect: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if (*desktop).model.is_null() {
        return ptr::null_mut::<GdkPixbuf>();
    }
    model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
    if gtk_tree_model_get_iter_first(model, &raw mut it) == 0 {
        return ptr::null_mut::<GdkPixbuf>();
    }
    area.width = 0i32;
    loop {
        item =
            fm_folder_model_get_item_userdata((*desktop).model, &raw mut it) as *mut FmDesktopItem;
        if !((*item).is_selected() == 0) {
            if area.width == 0i32 {
                area = (*item).icon_rect;
            } else {
                if (*item).icon_rect.x < area.x {
                    area.width += area.x - (*item).icon_rect.x;
                    area.x = (*item).icon_rect.x;
                }
                if (*item).icon_rect.x + (*item).icon_rect.width > area.x + area.width {
                    area.width = (*item).icon_rect.x + (*item).icon_rect.width - area.x;
                }
                if (*item).icon_rect.y < area.y {
                    area.height += area.y - (*item).icon_rect.y;
                    area.y = (*item).icon_rect.y;
                }
                if (*item).icon_rect.y + (*item).icon_rect.height > area.y + area.height {
                    area.height = (*item).icon_rect.y + (*item).icon_rect.height - area.y;
                }
            }
        }
        if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
            break;
        }
    }
    if area.width == 0i32 {
        return ptr::null_mut::<GdkPixbuf>();
    }
    area.width += 2 as c_int;
    area.height += 2 as c_int;
    s = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, area.width, area.height);
    cr = cairo_create(s);
    gtk_tree_model_get_iter_first(model, &raw mut it);
    loop {
        item =
            fm_folder_model_get_item_userdata((*desktop).model, &raw mut it) as *mut FmDesktopItem;
        if !((*item).is_selected() == 0) {
            icon = ptr::null_mut::<GdkPixbuf>();
            gtk_tree_model_get(
                model,
                &raw mut it,
                FM_FOLDER_MODEL_COL_ICON as c_int,
                &raw mut icon,
                -(1i32),
            );
            if !icon.is_null() {
                icon_rect.x = (*item).icon_rect.x - area.x + 1i32;
                icon_rect.width = (*item).icon_rect.width;
                icon_rect.y = (*item).icon_rect.y - area.y + 1i32;
                icon_rect.height = (*item).icon_rect.height;
                gdk_cairo_set_source_pixbuf(
                    cr,
                    icon,
                    icon_rect.x as gdouble,
                    icon_rect.y as gdouble,
                );
                gdk_cairo_rectangle(cr, &raw mut icon_rect);
                cairo_fill(cr);
                g_object_unref(icon as gpointer);
            }
        }
        if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
            break;
        }
    }
    cairo_destroy(cr);
    pixbuf = gdk_pixbuf_get_from_surface(
        s,
        0 as gint,
        0 as gint,
        area.width as gint,
        area.height as gint,
    );
    cairo_surface_destroy(s);
    *x = area.x as gint;
    *y = area.y as gint;
    return pixbuf;
}
unsafe extern "C" fn on_drag_begin(
    mut widget: *mut GtkWidget,
    mut drag_context: *mut GdkDragContext,
) {
    let mut desktop: *mut FmDesktop = widget as *mut c_void as *mut FmDesktop;
    let mut x: gint = 0;
    let mut y: gint = 0;
    let mut icon_x: gint = 0;
    let mut icon_y: gint = 0;
    let mut pix: *mut GdkPixbuf = _create_drag_icon(desktop, &raw mut x, &raw mut y);
    (*desktop).set_dragging((0i32 == 0) as c_int as gboolean as gboolean);
    if !pix.is_null() {
        icon_x = (*desktop).drag_start_x - x;
        icon_y = (*desktop).drag_start_y - y;
        gtk_drag_set_icon_pixbuf(drag_context, pix, icon_x, icon_y);
        g_object_unref(pix as gpointer);
    }
    queue_layout_items(desktop);
}
unsafe extern "C" fn on_drag_end(mut widget: *mut GtkWidget, mut context: *mut GdkDragContext) {
    let mut desktop: *mut FmDesktop = widget as *mut c_void as *mut FmDesktop;
    let mut parent: *mut GtkWidgetClass = fm_desktop_parent_class as *mut GtkWidgetClass;
    if (*parent).drag_end.is_some() {
        (*parent).drag_end.expect("non-null function pointer")(widget, context);
    }
    if (*desktop).dragging() != 0 {
        (*desktop).set_dragging(0 as gboolean as gboolean);
        queue_layout_items(desktop);
    }
}
unsafe extern "C" fn on_dnd_src_data_get(mut ds: *mut FmDndSrc, mut desktop: *mut FmDesktop) {
    let mut files: *mut FmFileInfoList =
        _dup_selected_files(desktop as *mut c_void as *mut FmFolderView);
    if !files.is_null() {
        fm_dnd_src_set_files(ds, files);
        fm_file_info_list_unref(files);
    }
}
unsafe extern "C" fn on_folder_start_loading(mut folder: *mut FmFolder, mut user_data: gpointer) {}
unsafe extern "C" fn on_folder_finish_loading(mut folder: *mut FmFolder, mut user_data: gpointer) {
    let mut desktop: *mut FmDesktop = user_data as *mut FmDesktop;
    if (*desktop).monitor < 0i32 {
        return;
    }
    fm_folder_view_add_popup(
        desktop as *mut c_void as *mut FmFolderView,
        desktop as *mut c_void as *mut GtkWindow,
        Some(
            fm_desktop_update_popup
                as unsafe extern "C" fn(
                    *mut FmFolderView,
                    *mut GtkWindow,
                    *mut GtkUIManager,
                    *mut GtkActionGroup,
                    *mut FmFileInfoList,
                ) -> (),
        ),
    );
    reload_items(desktop);
}
unsafe extern "C" fn on_folder_error(
    mut folder: *mut FmFolder,
    mut err: *mut GError,
    mut severity: FmJobErrorSeverity,
    mut user_data: gpointer,
) -> FmJobErrorAction {
    if (*err).domain == g_io_error_quark() {
        if (*err).code == G_IO_ERROR_NOT_MOUNTED as c_int
            && (severity as c_uint) < FM_JOB_ERROR_CRITICAL as c_int as c_uint
        {
            let mut path: *mut FmPath = fm_folder_get_path(folder);
            if fm_mount_path(ptr::null_mut::<GtkWindow>(), path, (0i32 == 0) as c_int) != 0 {
                return FM_JOB_RETRY;
            }
        }
    }
    fm_show_error(
        ptr::null_mut::<GtkWindow>(),
        ptr::null::<c_char>(),
        (*err).message,
    );
    return FM_JOB_CONTINUE;
}
unsafe extern "C" fn on_sort_changed(mut model: *mut GtkTreeSortable, mut desktop: *mut FmDesktop) {
    let mut by: FmFolderModelCol = FM_FOLDER_MODEL_COL_GICON;
    let mut type_0: FmSortMode = FM_SORT_ASCENDING;
    if fm_folder_model_get_sort(
        model as *mut c_void as *mut FmFolderModel,
        &raw mut by,
        &raw mut type_0,
    ) == 0
    {
        return;
    }
    if type_0 as c_uint == (*desktop).conf.desktop_sort_type as c_uint
        && by as c_uint == (*desktop).conf.desktop_sort_by as c_uint
    {
        return;
    }
    (*desktop).conf.desktop_sort_type = type_0;
    (*desktop).conf.desktop_sort_by = by;
    queue_config_save(desktop);
}
#[inline]
unsafe extern "C" fn connect_model(mut desktop: *mut FmDesktop, mut folder: *mut FmFolder) {
    (*desktop).model = fm_folder_model_new(folder, 0 as gboolean);
    g_signal_connect_data(
        folder as gpointer,
        b"start-loading\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut FmFolder, gpointer) -> ()>, GCallback>(
            Some(on_folder_start_loading as unsafe extern "C" fn(*mut FmFolder, gpointer) -> ()),
        ),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        folder as gpointer,
        b"finish-loading\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut FmFolder, gpointer) -> ()>, GCallback>(
            Some(on_folder_finish_loading as unsafe extern "C" fn(*mut FmFolder, gpointer) -> ()),
        ),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        folder as gpointer,
        b"error\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolder,
                    *mut GError,
                    FmJobErrorSeverity,
                    gpointer,
                ) -> FmJobErrorAction,
            >,
            GCallback,
        >(Some(
            on_folder_error
                as unsafe extern "C" fn(
                    *mut FmFolder,
                    *mut GError,
                    FmJobErrorSeverity,
                    gpointer,
                ) -> FmJobErrorAction,
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    fm_folder_model_set_icon_size((*desktop).model, (*fm_config).big_icon_size as guint);
    g_signal_connect_data(
        fm_config as *mut FmAppConfig as gpointer,
        b"changed::big_icon_size\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmConfig, *mut FmFolderModel) -> ()>,
            GCallback,
        >(Some(
            on_big_icon_size_changed
                as unsafe extern "C" fn(*mut FmConfig, *mut FmFolderModel) -> (),
        )),
        (*desktop).model as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*desktop).model as gpointer,
        b"row-deleting\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    gpointer,
                    *mut FmDesktop,
                ) -> (),
            >,
            GCallback,
        >(Some(
            on_row_deleting
                as unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    gpointer,
                    *mut FmDesktop,
                ) -> (),
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*desktop).model as gpointer,
        b"row-inserted\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    *mut FmDesktop,
                ) -> (),
            >,
            GCallback,
        >(Some(
            on_row_inserted
                as unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    *mut FmDesktop,
                ) -> (),
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*desktop).model as gpointer,
        b"row-deleted\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut FmFolderModel, *mut GtkTreePath, *mut FmDesktop) -> (),
            >,
            GCallback,
        >(Some(
            on_row_deleted
                as unsafe extern "C" fn(*mut FmFolderModel, *mut GtkTreePath, *mut FmDesktop) -> (),
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*desktop).model as gpointer,
        b"row-changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    *mut FmDesktop,
                ) -> (),
            >,
            GCallback,
        >(Some(
            on_row_changed
                as unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    *mut FmDesktop,
                ) -> (),
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*desktop).model as gpointer,
        b"rows-reordered\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    gpointer,
                    *mut FmDesktop,
                ) -> (),
            >,
            GCallback,
        >(Some(
            on_rows_reordered
                as unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    gpointer,
                    *mut FmDesktop,
                ) -> (),
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    fm_folder_model_set_sort(
        (*desktop).model,
        (*desktop).conf.desktop_sort_by,
        (*desktop).conf.desktop_sort_type,
    );
    g_signal_connect_data(
        (*desktop).model as gpointer,
        b"sort-column-changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkTreeSortable, *mut FmDesktop) -> ()>,
            GCallback,
        >(Some(
            on_sort_changed as unsafe extern "C" fn(*mut GtkTreeSortable, *mut FmDesktop) -> (),
        )),
        desktop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    on_folder_start_loading(folder, desktop as gpointer);
    if fm_folder_is_loaded(folder) != 0 {
        on_folder_finish_loading(folder, desktop as gpointer);
    }
}
#[inline]
unsafe extern "C" fn disconnect_model(mut desktop: *mut FmDesktop) {
    let mut folder: *mut FmFolder = ptr::null_mut::<FmFolder>();
    if (*desktop).model.is_null() {
        return;
    }
    folder = fm_folder_model_get_folder((*desktop).model);
    g_signal_handlers_disconnect_matched(
        folder as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<Option<unsafe extern "C" fn(*mut FmFolder, gpointer) -> ()>, gpointer>(
            Some(on_folder_start_loading as unsafe extern "C" fn(*mut FmFolder, gpointer) -> ()),
        ),
        desktop as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        folder as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<Option<unsafe extern "C" fn(*mut FmFolder, gpointer) -> ()>, gpointer>(
            Some(on_folder_finish_loading as unsafe extern "C" fn(*mut FmFolder, gpointer) -> ()),
        ),
        desktop as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        folder as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolder,
                    *mut GError,
                    FmJobErrorSeverity,
                    gpointer,
                ) -> FmJobErrorAction,
            >,
            gpointer,
        >(Some(
            on_folder_error
                as unsafe extern "C" fn(
                    *mut FmFolder,
                    *mut GError,
                    FmJobErrorSeverity,
                    gpointer,
                ) -> FmJobErrorAction,
        )),
        desktop as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        fm_config as *mut FmAppConfig as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmConfig, *mut FmFolderModel) -> ()>,
            gpointer,
        >(Some(
            on_big_icon_size_changed
                as unsafe extern "C" fn(*mut FmConfig, *mut FmFolderModel) -> (),
        )),
        (*desktop).model as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*desktop).model as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    gpointer,
                    *mut FmDesktop,
                ) -> (),
            >,
            gpointer,
        >(Some(
            on_row_deleting
                as unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    gpointer,
                    *mut FmDesktop,
                ) -> (),
        )),
        desktop as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*desktop).model as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    *mut FmDesktop,
                ) -> (),
            >,
            gpointer,
        >(Some(
            on_row_inserted
                as unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    *mut FmDesktop,
                ) -> (),
        )),
        desktop as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*desktop).model as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut FmFolderModel, *mut GtkTreePath, *mut FmDesktop) -> (),
            >,
            gpointer,
        >(Some(
            on_row_deleted
                as unsafe extern "C" fn(*mut FmFolderModel, *mut GtkTreePath, *mut FmDesktop) -> (),
        )),
        desktop as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*desktop).model as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    *mut FmDesktop,
                ) -> (),
            >,
            gpointer,
        >(Some(
            on_row_changed
                as unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    *mut FmDesktop,
                ) -> (),
        )),
        desktop as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*desktop).model as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    gpointer,
                    *mut FmDesktop,
                ) -> (),
            >,
            gpointer,
        >(Some(
            on_rows_reordered
                as unsafe extern "C" fn(
                    *mut FmFolderModel,
                    *mut GtkTreePath,
                    *mut GtkTreeIter,
                    gpointer,
                    *mut FmDesktop,
                ) -> (),
        )),
        desktop as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*desktop).model as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkTreeSortable, *mut FmDesktop) -> ()>,
            gpointer,
        >(Some(
            on_sort_changed as unsafe extern "C" fn(*mut GtkTreeSortable, *mut FmDesktop) -> (),
        )),
        desktop as gpointer,
    );
    fm_desktop_accessible_model_removed(desktop);
    if emwm_layout != 0 {
        let mut iter: GtkTreeIter = _GtkTreeIter {
            stamp: 0,
            user_data: ptr::null_mut::<c_void>(),
            user_data2: ptr::null_mut::<c_void>(),
            user_data3: ptr::null_mut::<c_void>(),
        };
        let mut model: *mut GtkTreeModel = (*desktop).model as *mut c_void as *mut GtkTreeModel;
        if gtk_tree_model_get_iter_first(model, &raw mut iter) != 0 {
            loop {
                let mut item: *mut FmDesktopItem =
                    fm_folder_model_get_item_userdata((*desktop).model, &raw mut iter)
                        as *mut FmDesktopItem;
                if !item.is_null() {
                    desktop_item_free(item);
                }
                fm_folder_model_set_item_userdata(
                    (*desktop).model,
                    &raw mut iter,
                    ptr::null_mut::<c_void>(),
                );
                if !(gtk_tree_model_iter_next(model, &raw mut iter) != 0) {
                    break;
                }
            }
        }
    }
    g_object_unref((*desktop).model as gpointer);
    (*desktop).model = ptr::null_mut::<FmFolderModel>();
    fm_folder_view_add_popup(
        desktop as *mut c_void as *mut FmFolderView,
        desktop as *mut c_void as *mut GtkWindow,
        Some(
            fm_desktop_update_popup
                as unsafe extern "C" fn(
                    *mut FmFolderView,
                    *mut GtkWindow,
                    *mut GtkUIManager,
                    *mut GtkActionGroup,
                    *mut FmFileInfoList,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn on_show_full_names_changed(
    mut cfg: *mut FmConfig,
    mut self_0: *mut FmDesktop,
) {
    if (*fm_config).show_full_names != 0 {
        (*self_0).pango_text_h = -(1i32) as guint;
        pango_layout_set_ellipsize((*self_0).pl, PANGO_ELLIPSIZE_NONE);
    } else {
        (*self_0).pango_text_h = (*self_0).text_h.wrapping_mul(1024 as guint);
        pango_layout_set_ellipsize((*self_0).pl, PANGO_ELLIPSIZE_END);
    }
    queue_layout_items(self_0);
}
unsafe extern "C" fn fm_desktop_destroy(mut object: *mut GtkWidget) {
    let mut self_0: *mut FmDesktop = ptr::null_mut::<FmDesktop>();
    let mut screen: *mut GdkScreen = ptr::null_mut::<GdkScreen>();
    self_0 = object as *mut c_void as *mut FmDesktop;
    if emwm_layout != 0 && (*self_0).conf.changed() != 0 {
        save_item_pos(self_0);
    }
    if !(*self_0).icon_render.is_null() {
        screen = gtk_widget_get_screen(self_0 as *mut GtkWidget);
        gdk_window_remove_filter(
            gdk_screen_get_root_window(screen),
            Some(
                on_root_event
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *mut GdkEvent,
                        gpointer,
                    ) -> GdkFilterReturn,
            ),
            self_0 as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            screen as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GdkScreen, *mut FmDesktop) -> ()>,
                gpointer,
            >(Some(
                on_screen_size_changed
                    as unsafe extern "C" fn(*mut GdkScreen, *mut FmDesktop) -> (),
            )),
            self_0 as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            fm_config as *mut FmAppConfig as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmConfig, *mut FmDesktop) -> ()>,
                gpointer,
            >(Some(
                on_show_full_names_changed
                    as unsafe extern "C" fn(*mut FmConfig, *mut FmDesktop) -> (),
            )),
            self_0 as gpointer,
        );
        gtk_window_group_remove_window(win_group, self_0 as *mut GtkWindow);
        if !(*self_0).model.is_null() {
            disconnect_model(self_0);
        }
        unload_items(self_0);
        g_object_unref((*self_0).icon_render as gpointer);
        (*self_0).icon_render = ptr::null_mut::<FmCellRendererPixbuf>();
        g_object_unref((*self_0).pl as gpointer);
        if (*self_0).single_click_timeout_handler != 0 {
            g_source_remove((*self_0).single_click_timeout_handler);
        }
        if (*self_0).idle_layout != 0 {
            g_source_remove((*self_0).idle_layout);
        }
        g_signal_handlers_disconnect_matched(
            (*self_0).dnd_src as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmDndSrc, *mut FmDesktop) -> ()>,
                gpointer,
            >(Some(
                on_dnd_src_data_get as unsafe extern "C" fn(*mut FmDndSrc, *mut FmDesktop) -> (),
            )),
            self_0 as gpointer,
        );
        g_object_unref((*self_0).dnd_src as gpointer);
        g_object_unref((*self_0).dnd_dest as gpointer);
    }
    if (*self_0).conf.configured() != 0 {
        (*self_0).conf.set_configured(0 as gint as gint);
        if (*self_0).conf.changed() != 0 {
            save_item_pos(self_0);
        }
        g_free((*self_0).conf.wallpaper as gpointer);
        if (*self_0).conf.wallpapers_configured > 0i32 {
            let mut i: c_int = 0;
            i = 0i32;
            while i < (*self_0).conf.wallpapers_configured {
                g_free(*(*self_0).conf.wallpapers.offset(i as isize) as gpointer);
                i += 1;
            }
            g_free((*self_0).conf.wallpapers as gpointer);
        }
        g_free((*self_0).conf.desktop_font as gpointer);
        g_free((*self_0).conf.folder as gpointer);
    }
    _clear_bg_cache(self_0);
    if ({
        let mut _g_boolean_var_42: c_int = 0i32;
        if (*self_0).search_timeout_id != 0 {
            _g_boolean_var_42 = 1i32;
        }
        _g_boolean_var_42
    }) as c_long
        != 0
    {
        g_source_remove((*self_0).search_timeout_id);
        (*self_0).search_timeout_id = 0 as guint;
    }
    if ({
        let mut _g_boolean_var_43: c_int = 0i32;
        if !(*self_0).search_window.is_null() {
            _g_boolean_var_43 = 1i32;
        }
        _g_boolean_var_43
    }) as c_long
        != 0
    {
        g_signal_handlers_disconnect_matched(
            (*self_0).search_window as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventAny,
                        *mut FmDesktop,
                    ) -> gboolean,
                >,
                gpointer,
            >(Some(
                desktop_search_delete_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventAny,
                        *mut FmDesktop,
                    ) -> gboolean,
            )),
            self_0 as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*self_0).search_window as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventScroll,
                        *mut FmDesktop,
                    ) -> gboolean,
                >,
                gpointer,
            >(Some(
                desktop_search_scroll_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventScroll,
                        *mut FmDesktop,
                    ) -> gboolean,
            )),
            self_0 as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*self_0).search_window as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventKey,
                        *mut FmDesktop,
                    ) -> gboolean,
                >,
                gpointer,
            >(Some(
                desktop_search_key_press_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventKey,
                        *mut FmDesktop,
                    ) -> gboolean,
            )),
            self_0 as gpointer,
        );
        gtk_widget_destroy((*self_0).search_window);
        (*self_0).search_entry = ptr::null_mut::<GtkWidget>();
        (*self_0).search_window = ptr::null_mut::<GtkWidget>();
    }
    (*(fm_desktop_parent_class as *mut GtkWidgetClass))
        .destroy
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn fm_desktop_init(mut self_0: *mut FmDesktop) {
    (*self_0).css = gtk_css_provider_new();
    gtk_style_context_add_provider(
        gtk_widget_get_style_context(self_0 as *mut GtkWidget),
        (*self_0).css as *mut c_void as *mut GtkStyleProvider,
        800 as guint,
    );
}
unsafe extern "C" fn fm_desktop_constructor(
    mut type_0: GType,
    mut n_construct_properties: guint,
    mut construct_properties: *mut GObjectConstructParam,
) -> *mut GObject {
    let mut object: *mut GObject = (*(fm_desktop_parent_class as *mut GObjectClass))
        .constructor
        .expect("non-null function pointer")(
        type_0, n_construct_properties, construct_properties
    );
    let mut self_0: *mut FmDesktop = object as *mut FmDesktop;
    let mut screen: *mut GdkScreen = gtk_widget_get_screen(self_0 as *mut GtkWidget);
    let mut root: *mut GdkWindow = ptr::null_mut::<GdkWindow>();
    let mut i: guint = 0;
    let mut n: gint = 0;
    let mut geom: GdkRectangle = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    i = 0 as guint;
    while i < n_construct_properties {
        if strcmp(
            (*(*construct_properties.offset(i as isize)).pspec).name as *const c_char,
            c"monitor".as_ptr(),
        ) == 0
            && ({
                let mut __val: *const GValue =
                    (*construct_properties.offset(i as isize)).value as *const GValue;
                let mut __t: GType = ((6 as c_int) << 2 as c_int) as GType;
                let mut __r: gboolean = 0;
                if __val.is_null() {
                    __r = 0 as gboolean;
                } else if (*__val).g_type == __t {
                    __r = (0i32 == 0) as c_int as gboolean;
                } else {
                    __r = g_type_check_value_holds(__val, __t);
                }
                __r
            }) != 0
        {
            (*self_0).monitor = g_value_get_int((*construct_properties.offset(i as isize)).value);
        }
        i = i.wrapping_add(1);
    }
    if (*self_0).monitor < 0i32 {
        return object;
    }
    g_log(
        ptr::null_mut::<gchar>(),
        G_LOG_LEVEL_DEBUG,
        b"fm_desktop_constructor for monitor %d\0" as *const u8 as *const gchar,
        (*self_0).monitor,
    );
    gdk_screen_get_monitor_geometry(screen, (*self_0).monitor, &raw mut geom);
    gtk_window_set_default_size(
        self_0 as *mut GtkWindow,
        geom.width as gint,
        geom.height as gint,
    );
    gtk_window_move(
        self_0 as *mut c_void as *mut GtkWindow,
        geom.x as gint,
        geom.y as gint,
    );
    gtk_widget_set_app_paintable(self_0 as *mut GtkWidget, (0i32 == 0) as c_int);
    gtk_window_set_type_hint(
        self_0 as *mut c_void as *mut GtkWindow,
        GDK_WINDOW_TYPE_HINT_DESKTOP,
    );
    gtk_widget_add_events(
        self_0 as *mut GtkWidget,
        GDK_POINTER_MOTION_MASK as c_int as gint
            | GDK_BUTTON_PRESS_MASK as c_int as gint
            | GDK_BUTTON_RELEASE_MASK as c_int as gint
            | GDK_KEY_PRESS_MASK as c_int as gint
            | GDK_PROPERTY_CHANGE_MASK as c_int as gint,
    );
    (*self_0).icon_render = fm_cell_renderer_pixbuf_new();
    g_object_set(
        (*self_0).icon_render as gpointer,
        b"follow-state\0" as *const u8 as *const gchar,
        (0i32 == 0) as c_int,
        ptr::null_mut::<c_void>(),
    );
    g_object_ref_sink((*self_0).icon_render as gpointer);
    fm_cell_renderer_pixbuf_set_fixed_size(
        (*self_0).icon_render as *mut c_void as *mut FmCellRendererPixbuf,
        (*fm_config).big_icon_size,
        (*fm_config).big_icon_size,
    );
    (*self_0).pl = gtk_widget_create_pango_layout(self_0 as *mut GtkWidget, ptr::null::<gchar>());
    pango_layout_set_alignment((*self_0).pl, PANGO_ALIGN_CENTER);
    if (*fm_config).show_full_names != 0 {
        pango_layout_set_ellipsize((*self_0).pl, PANGO_ELLIPSIZE_NONE);
    } else {
        pango_layout_set_ellipsize((*self_0).pl, PANGO_ELLIPSIZE_END);
    }
    pango_layout_set_wrap((*self_0).pl, PANGO_WRAP_WORD_CHAR);
    g_signal_connect_data(
        fm_config as *mut FmAppConfig as gpointer,
        b"changed::show_full_names\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmConfig, *mut FmDesktop) -> ()>,
            GCallback,
        >(Some(
            on_show_full_names_changed as unsafe extern "C" fn(*mut FmConfig, *mut FmDesktop) -> (),
        )),
        self_0 as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    root = gdk_screen_get_root_window(screen);
    gdk_window_set_events(
        root,
        (gdk_window_get_events(root) as c_uint | GDK_PROPERTY_CHANGE_MASK as c_int as c_uint)
            as GdkEventMask,
    );
    gdk_window_add_filter(
        root,
        Some(
            on_root_event
                as unsafe extern "C" fn(*mut c_void, *mut GdkEvent, gpointer) -> GdkFilterReturn,
        ),
        self_0 as gpointer,
    );
    g_signal_connect_data(
        screen as gpointer,
        b"monitors-changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GdkScreen, *mut FmDesktop) -> ()>,
            GCallback,
        >(Some(
            on_screen_size_changed as unsafe extern "C" fn(*mut GdkScreen, *mut FmDesktop) -> (),
        )),
        self_0 as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    n = get_desktop_for_root_window(root);
    if n < 0i32 {
        n = 0 as gint;
    }
    (*self_0).cur_desktop = n as guint;
    (*self_0).dnd_src = fm_dnd_src_new(self_0 as *mut GtkWidget);
    gtk_target_list_add_table(
        gtk_drag_source_get_target_list(self_0 as *mut GtkWidget),
        &raw mut dnd_targets as *mut GtkTargetEntry,
        (mem::size_of::<[GtkTargetEntry; 1]>() as usize)
            .wrapping_div(mem::size_of::<GtkTargetEntry>() as usize) as guint,
    );
    g_signal_connect_data(
        (*self_0).dnd_src as gpointer,
        b"data-get\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmDndSrc, *mut FmDesktop) -> ()>,
            GCallback,
        >(Some(
            on_dnd_src_data_get as unsafe extern "C" fn(*mut FmDndSrc, *mut FmDesktop) -> (),
        )),
        self_0 as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*self_0).dnd_dest = fm_dnd_dest_new_with_handlers(self_0 as *mut GtkWidget);
    gtk_target_list_add_table(
        gtk_drag_dest_get_target_list(self_0 as *mut GtkWidget),
        &raw mut dnd_targets as *mut GtkTargetEntry,
        (mem::size_of::<[GtkTargetEntry; 1]>() as usize)
            .wrapping_div(mem::size_of::<GtkTargetEntry>() as usize) as guint,
    );
    gtk_window_group_add_window(win_group, self_0 as *mut c_void as *mut GtkWindow);
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn fm_desktop_new(
    mut screen: *mut GdkScreen,
    mut monitor: gint,
) -> *mut FmDesktop {
    return g_object_new(
        fm_desktop_get_type(),
        b"screen\0" as *const u8 as *const gchar,
        screen,
        c"monitor".as_ptr(),
        monitor,
        ptr::null_mut::<c_void>(),
    ) as *mut FmDesktop;
}
unsafe extern "C" fn fm_desktop_set_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    match property_id {
        1 => {
            (*(object as *mut c_void as *mut FmDesktop)).monitor = g_value_get_int(value);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = property_id;
            g_log(
                ptr::null_mut::<gchar>(),
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                c"/tmp/fm-full/pcmanfm/src/desktop.c".as_ptr(),
                4961i32,
                c"property".as_ptr(),
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn fm_desktop_get_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    match property_id {
        1 => {
            g_value_set_int(value, (*(object as *mut c_void as *mut FmDesktop)).monitor);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = property_id;
            g_log(
                ptr::null_mut::<gchar>(),
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                c"/tmp/fm-full/pcmanfm/src/desktop.c".as_ptr(),
                4974 as c_int,
                c"property".as_ptr(),
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn fm_desktop_class_init(mut klass: *mut FmDesktopClass) {
    let mut widget_class: *mut GtkWidgetClass = klass as *mut c_void as *mut GtkWidgetClass;
    let mut atom_names: [*mut c_char; 5] = [
        c"_NET_WORKAREA".as_ptr() as *mut c_char,
        c"_NET_NUMBER_OF_DESKTOPS".as_ptr() as *mut c_char,
        c"_NET_CURRENT_DESKTOP".as_ptr() as *mut c_char,
        c"_XROOTMAP_ID".as_ptr() as *mut c_char,
        c"_XROOTPMAP_ID".as_ptr() as *mut c_char,
    ];
    let mut atoms: [Atom; 5] = [0 as Atom, 0, 0, 0, 0];
    let mut object_class: *mut GObjectClass = klass as *mut c_void as *mut GObjectClass;
    (*widget_class).destroy = Some(fm_desktop_destroy as unsafe extern "C" fn(*mut GtkWidget) -> ())
        as Option<unsafe extern "C" fn(*mut GtkWidget) -> ()>;
    (*widget_class).draw =
        Some(on_draw as unsafe extern "C" fn(*mut GtkWidget, *mut cairo_t) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GtkWidget, *mut cairo_t) -> gboolean>;
    (*widget_class).get_preferred_width = Some(
        on_get_preferred_width as unsafe extern "C" fn(*mut GtkWidget, *mut gint, *mut gint) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GtkWidget, *mut gint, *mut gint) -> ()>;
    (*widget_class).get_preferred_height = Some(
        on_get_preferred_height as unsafe extern "C" fn(*mut GtkWidget, *mut gint, *mut gint) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GtkWidget, *mut gint, *mut gint) -> ()>;
    (*widget_class).size_allocate =
        Some(on_size_allocate as unsafe extern "C" fn(*mut GtkWidget, *mut GtkAllocation) -> ())
            as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GtkAllocation) -> ()>;
    (*widget_class).button_press_event = Some(
        on_button_press as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventButton) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventButton) -> gboolean>;
    (*widget_class).button_release_event = Some(
        on_button_release as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventButton) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventButton) -> gboolean>;
    (*widget_class).motion_notify_event = Some(
        on_motion_notify as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventMotion) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventMotion) -> gboolean>;
    (*widget_class).leave_notify_event = Some(
        on_leave_notify as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventCrossing) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventCrossing) -> gboolean>;
    (*widget_class).key_press_event =
        Some(on_key_press as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventKey) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventKey) -> gboolean>;
    (*widget_class).direction_changed =
        Some(on_direction_changed as unsafe extern "C" fn(*mut GtkWidget, GtkTextDirection) -> ())
            as Option<unsafe extern "C" fn(*mut GtkWidget, GtkTextDirection) -> ()>;
    (*widget_class).realize = Some(on_realize as unsafe extern "C" fn(*mut GtkWidget) -> ())
        as Option<unsafe extern "C" fn(*mut GtkWidget) -> ()>;
    (*widget_class).focus_in_event =
        Some(on_focus_in as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventFocus) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventFocus) -> gboolean>;
    (*widget_class).focus_out_event =
        Some(on_focus_out as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventFocus) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventFocus) -> gboolean>;
    (*widget_class).delete_event =
        mem::transmute::<Option<unsafe extern "C" fn() -> gboolean>, DeleteEvtHandler>(Some(
            gtk_true as unsafe extern "C" fn() -> gboolean,
        )) as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventAny) -> gboolean>;
    (*widget_class).get_accessible =
        Some(fm_desktop_get_accessible as unsafe extern "C" fn(*mut GtkWidget) -> *mut AtkObject)
            as Option<unsafe extern "C" fn(*mut GtkWidget) -> *mut AtkObject>;
    (*widget_class).drag_motion = Some(
        on_drag_motion
            as unsafe extern "C" fn(
                *mut GtkWidget,
                *mut GdkDragContext,
                gint,
                gint,
                guint,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GtkWidget,
                *mut GdkDragContext,
                gint,
                gint,
                guint,
            ) -> gboolean,
        >;
    (*widget_class).drag_drop = Some(
        on_drag_drop
            as unsafe extern "C" fn(
                *mut GtkWidget,
                *mut GdkDragContext,
                gint,
                gint,
                guint,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GtkWidget,
                *mut GdkDragContext,
                gint,
                gint,
                guint,
            ) -> gboolean,
        >;
    (*widget_class).drag_data_received = Some(
        on_drag_data_received
            as unsafe extern "C" fn(
                *mut GtkWidget,
                *mut GdkDragContext,
                gint,
                gint,
                *mut GtkSelectionData,
                guint,
                guint,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GtkWidget,
                *mut GdkDragContext,
                gint,
                gint,
                *mut GtkSelectionData,
                guint,
                guint,
            ) -> (),
        >;
    (*widget_class).drag_leave = Some(
        on_drag_leave as unsafe extern "C" fn(*mut GtkWidget, *mut GdkDragContext, guint) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkDragContext, guint) -> ()>;
    (*widget_class).drag_end =
        Some(on_drag_end as unsafe extern "C" fn(*mut GtkWidget, *mut GdkDragContext) -> ())
            as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkDragContext) -> ()>;
    (*widget_class).drag_begin =
        Some(on_drag_begin as unsafe extern "C" fn(*mut GtkWidget, *mut GdkDragContext) -> ())
            as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkDragContext) -> ()>;
    if XInternAtoms(
        gdk_x11_get_default_xdisplay(),
        &raw mut atom_names as *mut *mut c_char,
        (mem::size_of::<[*mut c_char; 5]>() as usize)
            .wrapping_div(mem::size_of::<*mut c_char>() as usize) as c_int,
        0i32,
        &raw mut atoms as *mut Atom,
    ) != 0
    {
        XA_NET_WORKAREA = atoms[0 as usize];
        XA_NET_NUMBER_OF_DESKTOPS = atoms[1 as usize];
        XA_NET_CURRENT_DESKTOP = atoms[2 as usize];
        XA_XROOTMAP_ID = atoms[3 as usize];
        XA_XROOTPMAP_ID = atoms[4 as usize];
    }
    (*object_class).constructor = Some(
        fm_desktop_constructor
            as unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject,
    )
        as Option<unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject>;
    (*object_class).set_property = Some(
        fm_desktop_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).get_property = Some(
        fm_desktop_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        object_class,
        PROP_MONITOR as c_int as guint,
        g_param_spec_int(
            b"monitor\0" as *const u8 as *const gchar,
            b"Monitor\0" as *const u8 as *const gchar,
            b"Monitor number where desktop is\0" as *const u8 as *const gchar,
            -(2 as gint),
            127 as gint,
            0 as gint,
            (G_PARAM_CONSTRUCT_ONLY as c_int | G_PARAM_WRITABLE as c_int) as GParamFlags,
        ),
    );
    desktop_atom = gdk_atom_intern_static_string(dnd_targets[0 as usize].target);
}
unsafe extern "C" fn _set_sel_mode(mut fv: *mut FmFolderView, mut mode: GtkSelectionMode) {}
unsafe extern "C" fn _get_sel_mode(mut fv: *mut FmFolderView) -> GtkSelectionMode {
    return GTK_SELECTION_MULTIPLE;
}
unsafe extern "C" fn _set_show_hidden(mut fv: *mut FmFolderView, mut show: gboolean) {}
unsafe extern "C" fn _get_show_hidden(mut fv: *mut FmFolderView) -> gboolean {
    return 0 as gboolean;
}
unsafe extern "C" fn _get_folder(mut fv: *mut FmFolderView) -> *mut FmFolder {
    let mut desktop: *mut FmDesktop = fv as *mut c_void as *mut FmDesktop;
    return if !(*desktop).model.is_null() {
        fm_folder_model_get_folder((*desktop).model)
    } else {
        ptr::null_mut::<FmFolder>()
    };
}
unsafe extern "C" fn _set_model(mut fv: *mut FmFolderView, mut model: *mut FmFolderModel) {}
unsafe extern "C" fn _get_model(mut fv: *mut FmFolderView) -> *mut FmFolderModel {
    return (*(fv as *mut c_void as *mut FmDesktop)).model;
}
unsafe extern "C" fn _count_selected_files(mut fv: *mut FmFolderView) -> gint {
    let mut desktop: *mut FmDesktop = fv as *mut c_void as *mut FmDesktop;
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut n: gint = 0 as gint;
    if (*desktop).model.is_null() {
        return 0 as gint;
    }
    model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
    if gtk_tree_model_get_iter_first(model, &raw mut it) == 0 {
        return 0 as gint;
    }
    loop {
        let mut item: *mut FmDesktopItem =
            fm_folder_model_get_item_userdata((*desktop).model, &raw mut it) as *mut FmDesktopItem;
        if (*item).is_selected() != 0 {
            n += 1;
        }
        if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
            break;
        }
    }
    return n;
}
unsafe extern "C" fn _dup_selected_files(mut fv: *mut FmFolderView) -> *mut FmFileInfoList {
    let mut desktop: *mut FmDesktop = fv as *mut c_void as *mut FmDesktop;
    let mut files: *mut FmFileInfoList = ptr::null_mut::<FmFileInfoList>();
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if (*desktop).model.is_null() {
        return ptr::null_mut::<FmFileInfoList>();
    }
    model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
    if gtk_tree_model_get_iter_first(model, &raw mut it) == 0 {
        return ptr::null_mut::<FmFileInfoList>();
    }
    loop {
        let mut item: *mut FmDesktopItem =
            fm_folder_model_get_item_userdata((*desktop).model, &raw mut it) as *mut FmDesktopItem;
        if (*item).is_selected() != 0 {
            if files.is_null() {
                files = fm_file_info_list_new();
            }
            fm_file_info_list_push_tail(files, (*item).fi);
        }
        if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
            break;
        }
    }
    return files;
}
unsafe extern "C" fn _dup_selected_file_paths(mut fv: *mut FmFolderView) -> *mut FmPathList {
    let mut desktop: *mut FmDesktop = fv as *mut c_void as *mut FmDesktop;
    let mut files: *mut FmPathList = ptr::null_mut::<FmPathList>();
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if (*desktop).model.is_null() {
        return ptr::null_mut::<FmPathList>();
    }
    model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
    if gtk_tree_model_get_iter_first(model, &raw mut it) == 0 {
        return ptr::null_mut::<FmPathList>();
    }
    loop {
        let mut item: *mut FmDesktopItem =
            fm_folder_model_get_item_userdata((*desktop).model, &raw mut it) as *mut FmDesktopItem;
        if (*item).is_selected() != 0 {
            if files.is_null() {
                files = fm_path_list_new();
            }
            fm_path_list_push_tail(files, fm_file_info_get_path((*item).fi));
        }
        if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
            break;
        }
    }
    return files;
}
unsafe extern "C" fn _select_all(mut fv: *mut FmFolderView) {
    let mut desktop: *mut FmDesktop = fv as *mut c_void as *mut FmDesktop;
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    if (*desktop).model.is_null() {
        return;
    }
    model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
    if gtk_tree_model_get_iter_first(model, &raw mut it) == 0 {
        return;
    }
    loop {
        let mut item: *mut FmDesktopItem =
            fm_folder_model_get_item_userdata((*desktop).model, &raw mut it) as *mut FmDesktopItem;
        if (*item).is_selected() == 0 {
            (*item).set_is_selected((0i32 == 0) as c_int as gboolean as gboolean);
            redraw_item(desktop, item);
            fm_desktop_item_selected_changed(desktop, item);
        }
        if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
            break;
        }
    }
}
unsafe extern "C" fn _unselect_all(mut fv: *mut FmFolderView) {
    let mut desktop: *mut FmDesktop = fv as *mut c_void as *mut FmDesktop;
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    if (*desktop).model.is_null() {
        return;
    }
    model = (*desktop).model as *mut c_void as *mut GtkTreeModel;
    if gtk_tree_model_get_iter_first(model, &raw mut it) == 0 {
        return;
    }
    loop {
        let mut item: *mut FmDesktopItem =
            fm_folder_model_get_item_userdata((*desktop).model, &raw mut it) as *mut FmDesktopItem;
        if (*item).is_selected() != 0 {
            (*item).set_is_selected(0 as gboolean as gboolean);
            redraw_item(desktop, item);
            fm_desktop_item_selected_changed(desktop, item);
        }
        if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
            break;
        }
    }
}
unsafe extern "C" fn _select_invert(mut fv: *mut FmFolderView) {}
unsafe extern "C" fn _select_file_path(mut fv: *mut FmFolderView, mut path: *mut FmPath) {}
unsafe extern "C" fn _get_custom_menu_callbacks(
    mut fv: *mut FmFolderView,
    mut popup: *mut FmFolderViewUpdatePopup,
    mut launch: *mut FmLaunchFolderFunc,
) {
    if !popup.is_null() {
        *popup = Some(
            fm_desktop_update_item_popup
                as unsafe extern "C" fn(
                    *mut FmFolderView,
                    *mut GtkWindow,
                    *mut GtkUIManager,
                    *mut GtkActionGroup,
                    *mut FmFileInfoList,
                ) -> (),
        ) as FmFolderViewUpdatePopup;
    }
    if !launch.is_null() {
        *launch = Some(
            pcmanfm_open_folder
                as unsafe extern "C" fn(
                    *mut GAppLaunchContext,
                    *mut GList,
                    gpointer,
                    *mut *mut GError,
                ) -> gboolean,
        ) as FmLaunchFolderFunc;
    }
}
unsafe extern "C" fn fm_desktop_view_init(mut iface: *mut FmFolderViewInterface) {
    (*iface).set_sel_mode =
        Some(_set_sel_mode as unsafe extern "C" fn(*mut FmFolderView, GtkSelectionMode) -> ())
            as Option<unsafe extern "C" fn(*mut FmFolderView, GtkSelectionMode) -> ()>;
    (*iface).get_sel_mode =
        Some(_get_sel_mode as unsafe extern "C" fn(*mut FmFolderView) -> GtkSelectionMode)
            as Option<unsafe extern "C" fn(*mut FmFolderView) -> GtkSelectionMode>;
    (*iface).set_show_hidden =
        Some(_set_show_hidden as unsafe extern "C" fn(*mut FmFolderView, gboolean) -> ())
            as Option<unsafe extern "C" fn(*mut FmFolderView, gboolean) -> ()>;
    (*iface).get_show_hidden =
        Some(_get_show_hidden as unsafe extern "C" fn(*mut FmFolderView) -> gboolean)
            as Option<unsafe extern "C" fn(*mut FmFolderView) -> gboolean>;
    (*iface).get_folder =
        Some(_get_folder as unsafe extern "C" fn(*mut FmFolderView) -> *mut FmFolder)
            as Option<unsafe extern "C" fn(*mut FmFolderView) -> *mut FmFolder>;
    (*iface).set_model =
        Some(_set_model as unsafe extern "C" fn(*mut FmFolderView, *mut FmFolderModel) -> ())
            as Option<unsafe extern "C" fn(*mut FmFolderView, *mut FmFolderModel) -> ()>;
    (*iface).get_model =
        Some(_get_model as unsafe extern "C" fn(*mut FmFolderView) -> *mut FmFolderModel)
            as Option<unsafe extern "C" fn(*mut FmFolderView) -> *mut FmFolderModel>;
    (*iface).count_selected_files =
        Some(_count_selected_files as unsafe extern "C" fn(*mut FmFolderView) -> gint)
            as Option<unsafe extern "C" fn(*mut FmFolderView) -> gint>;
    (*iface).dup_selected_files =
        Some(_dup_selected_files as unsafe extern "C" fn(*mut FmFolderView) -> *mut FmFileInfoList)
            as Option<unsafe extern "C" fn(*mut FmFolderView) -> *mut FmFileInfoList>;
    (*iface).dup_selected_file_paths = Some(
        _dup_selected_file_paths as unsafe extern "C" fn(*mut FmFolderView) -> *mut FmPathList,
    )
        as Option<unsafe extern "C" fn(*mut FmFolderView) -> *mut FmPathList>;
    (*iface).select_all = Some(_select_all as unsafe extern "C" fn(*mut FmFolderView) -> ())
        as Option<unsafe extern "C" fn(*mut FmFolderView) -> ()>;
    (*iface).select_invert = Some(_select_invert as unsafe extern "C" fn(*mut FmFolderView) -> ())
        as Option<unsafe extern "C" fn(*mut FmFolderView) -> ()>;
    (*iface).select_file_path =
        Some(_select_file_path as unsafe extern "C" fn(*mut FmFolderView, *mut FmPath) -> ())
            as Option<unsafe extern "C" fn(*mut FmFolderView, *mut FmPath) -> ()>;
    (*iface).get_custom_menu_callbacks = Some(
        _get_custom_menu_callbacks
            as unsafe extern "C" fn(
                *mut FmFolderView,
                *mut FmFolderViewUpdatePopup,
                *mut FmLaunchFolderFunc,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut FmFolderView,
                *mut FmFolderViewUpdatePopup,
                *mut FmLaunchFolderFunc,
            ) -> (),
        >;
}
static mut desktop_pref_dlg: *mut GtkWindow = ptr::null::<GtkWindow>() as *mut GtkWindow;
unsafe extern "C" fn desktop_on_response(
    mut dlg: *mut GtkDialog,
    mut res: c_int,
    mut pdlg: *mut *mut GtkWindow,
) {
    *pdlg = ptr::null_mut::<GtkWindow>();
    gtk_widget_destroy(dlg as *mut c_void as *mut GtkWidget);
}
unsafe extern "C" fn on_wallpaper_set(
    mut btn: *mut GtkFileChooserButton,
    mut desktop: *mut FmDesktop,
) {
    let mut file: *mut c_char =
        gtk_file_chooser_get_filename(btn as *mut c_void as *mut GtkFileChooser) as *mut c_char;
    g_free((*desktop).conf.wallpaper as gpointer);
    (*desktop).conf.wallpaper = file;
    queue_config_save(desktop);
    update_background(desktop, 0i32);
}
unsafe extern "C" fn on_update_img_preview(
    mut chooser: *mut GtkFileChooser,
    mut img: *mut GtkImage,
) {
    let mut file: *mut c_char = gtk_file_chooser_get_preview_filename(chooser);
    let mut pix: *mut GdkPixbuf = ptr::null_mut::<GdkPixbuf>();
    if !file.is_null() {
        pix = gdk_pixbuf_new_from_file_at_scale(
            file,
            128 as c_int,
            128 as c_int,
            (0i32 == 0) as c_int,
            ptr::null_mut::<*mut GError>(),
        );
        g_free(file as gpointer);
    }
    if !pix.is_null() {
        gtk_file_chooser_set_preview_widget_active(chooser, (0i32 == 0) as c_int);
        gtk_image_set_from_pixbuf(img, pix);
        g_object_unref(pix as gpointer);
    } else {
        gtk_image_clear(img);
        gtk_file_chooser_set_preview_widget_active(chooser, 0 as gboolean);
    };
}
unsafe extern "C" fn on_wallpaper_mode_changed(
    mut combo: *mut GtkComboBox,
    mut desktop: *mut FmDesktop,
) {
    let mut sel: c_int = gtk_combo_box_get_active(combo) as c_int;
    if sel >= 0i32 && sel != (*desktop).conf.wallpaper_mode as c_int {
        (*desktop).conf.wallpaper_mode = sel as FmWallpaperMode;
        queue_config_save(desktop);
        update_background(desktop, 0i32);
    }
}
unsafe extern "C" fn on_wallpaper_mode_changed2(
    mut combo: *mut GtkComboBox,
    mut wallpaper_box: *mut GtkWidget,
) {
    if !wallpaper_box.is_null() {
        gtk_widget_set_sensitive(
            wallpaper_box,
            (gtk_combo_box_get_active(combo) != FM_WP_COLOR as c_int) as c_int,
        );
    }
}
unsafe extern "C" fn on_bg_color_set(mut btn: *mut GtkColorButton, mut desktop: *mut FmDesktop) {
    let mut new_val: GdkColor = _GdkColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
    };
    gtk_color_button_get_color(btn, &raw mut new_val);
    if gdk_color_equal(&raw mut (*desktop).conf.desktop_bg, &raw mut new_val) == 0 {
        let mut css_data: *mut c_char = g_strdup_printf(
            b"FmDesktop {\nbackground-color: #%02x%02x%02x\n}\0" as *const u8 as *const gchar,
            new_val.red as c_int / 256 as c_int,
            new_val.green as c_int / 256 as c_int,
            new_val.blue as c_int / 256 as c_int,
        ) as *mut c_char;
        gtk_css_provider_load_from_data(
            (*desktop).css,
            css_data,
            -(1i32) as gssize,
            ptr::null_mut::<*mut GError>(),
        );
        g_free(css_data as gpointer);
        (*desktop).conf.desktop_bg = new_val;
        queue_config_save(desktop);
        update_background(desktop, 0i32);
    }
}
unsafe extern "C" fn on_wallpaper_common_toggled(
    mut btn: *mut GtkToggleButton,
    mut desktop: *mut FmDesktop,
) {
    let mut new_val: gboolean = gtk_toggle_button_get_active(btn);
    if (*desktop).conf.wallpaper_common != new_val {
        (*desktop).conf.wallpaper_common = new_val;
        queue_config_save(desktop);
        update_background(desktop, 0i32);
    }
}
unsafe extern "C" fn on_fg_color_set(mut btn: *mut GtkColorButton, mut desktop: *mut FmDesktop) {
    let mut new_val: GdkColor = _GdkColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
    };
    gtk_color_button_get_color(btn, &raw mut new_val);
    if gdk_color_equal(&raw mut (*desktop).conf.desktop_fg, &raw mut new_val) == 0 {
        (*desktop).conf.desktop_fg = new_val;
        queue_config_save(desktop);
        gtk_widget_queue_draw(desktop as *mut c_void as *mut GtkWidget);
    }
}
unsafe extern "C" fn on_shadow_color_set(
    mut btn: *mut GtkColorButton,
    mut desktop: *mut FmDesktop,
) {
    let mut new_val: GdkColor = _GdkColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
    };
    gtk_color_button_get_color(btn, &raw mut new_val);
    if gdk_color_equal(&raw mut (*desktop).conf.desktop_shadow, &raw mut new_val) == 0 {
        (*desktop).conf.desktop_shadow = new_val;
        queue_config_save(desktop);
        gtk_widget_queue_draw(desktop as *mut c_void as *mut GtkWidget);
    }
}
unsafe extern "C" fn on_wm_menu_toggled(
    mut btn: *mut GtkToggleButton,
    mut desktop: *mut FmDesktop,
) {
    let mut new_val: gboolean = gtk_toggle_button_get_active(btn);
    if (*desktop).conf.show_wm_menu != new_val {
        (*desktop).conf.show_wm_menu = new_val;
        queue_config_save(desktop);
    }
}
unsafe extern "C" fn on_desktop_font_set(mut btn: *mut GtkFontButton, mut desktop: *mut FmDesktop) {
    let mut font: *const c_char = gtk_font_button_get_font_name(btn) as *const c_char;
    if !font.is_null() {
        let mut font_desc: *mut PangoFontDescription = ptr::null_mut::<PangoFontDescription>();
        g_free((*desktop).conf.desktop_font as gpointer);
        (*desktop).conf.desktop_font = g_strdup_inline(font);
        queue_config_save(desktop);
        if (*desktop).monitor < 0i32 {
            return;
        }
        font_desc = pango_font_description_from_string((*desktop).conf.desktop_font);
        if !font_desc.is_null() {
            let mut pc: *mut PangoContext = gtk_widget_get_pango_context(desktop as *mut GtkWidget);
            pango_context_set_font_description(pc, font_desc);
            pango_layout_context_changed((*desktop).pl);
            gtk_widget_queue_resize(desktop as *mut c_void as *mut GtkWidget);
            pango_font_description_free(font_desc);
        }
    }
}
unsafe extern "C" fn on_desktop_folder_new_win_toggled(
    mut btn: *mut GtkToggleButton,
    mut desktop: *mut FmDesktop,
) {
    (*(fm_config as *mut FmAppConfig)).desktop_folder_new_win = gtk_toggle_button_get_active(btn);
    pcmanfm_save_config(0 as gboolean);
}
unsafe extern "C" fn on_show_documents_toggled(
    mut btn: *mut GtkToggleButton,
    mut desktop: *mut FmDesktop,
) {
    let mut new_val: gboolean = gtk_toggle_button_get_active(btn);
    if (*desktop).conf.show_documents != new_val {
        (*desktop).conf.show_documents = new_val;
        queue_config_save(desktop);
        if !documents.is_null() && !(*documents).fi.is_null() && !(*desktop).model.is_null() {
            if new_val != 0 {
                fm_folder_model_extra_file_add(
                    (*desktop).model,
                    (*documents).fi,
                    FM_FOLDER_MODEL_ITEMPOS_PRE,
                );
            } else {
                fm_folder_model_extra_file_remove((*desktop).model, (*documents).fi);
            }
        }
    }
}
unsafe extern "C" fn on_show_trash_toggled(
    mut btn: *mut GtkToggleButton,
    mut desktop: *mut FmDesktop,
) {
    let mut new_val: gboolean = gtk_toggle_button_get_active(btn);
    if (*desktop).conf.show_trash != new_val {
        (*desktop).conf.show_trash = new_val;
        queue_config_save(desktop);
        if !trash_can.is_null() && !(*trash_can).fi.is_null() && !(*desktop).model.is_null() {
            if new_val != 0 {
                fm_folder_model_extra_file_add(
                    (*desktop).model,
                    (*trash_can).fi,
                    FM_FOLDER_MODEL_ITEMPOS_PRE,
                );
            } else {
                fm_folder_model_extra_file_remove((*desktop).model, (*trash_can).fi);
            }
        }
    }
}
unsafe extern "C" fn on_show_mounts_toggled(
    mut btn: *mut GtkToggleButton,
    mut desktop: *mut FmDesktop,
) {
    let mut msl: *mut GSList = ptr::null_mut::<GSList>();
    let mut new_val: gboolean = gtk_toggle_button_get_active(btn);
    if (*desktop).conf.show_mounts != new_val {
        (*desktop).conf.show_mounts = new_val;
        queue_config_save(desktop);
        if !(*desktop).model.is_null() {
            msl = mounts;
            while !msl.is_null() {
                let mut mount: *mut FmDesktopExtraItem = (*msl).data as *mut FmDesktopExtraItem;
                if new_val != 0 {
                    fm_folder_model_extra_file_add(
                        (*desktop).model,
                        (*mount).fi,
                        FM_FOLDER_MODEL_ITEMPOS_POST,
                    );
                } else {
                    fm_folder_model_extra_file_remove((*desktop).model, (*mount).fi);
                }
                msl = (*msl).next;
            }
        }
    }
}
unsafe extern "C" fn on_desktop_folder_set_toggled(
    mut btn: *mut GtkToggleButton,
    mut chooser: *mut GtkFileChooserButton,
) {
    let mut active: gboolean = gtk_toggle_button_get_active(btn);
    gtk_widget_set_sensitive(chooser as *mut c_void as *mut GtkWidget, active);
    if active != 0 {
        g_signal_emit_by_name(
            chooser as gpointer,
            b"file-set\0" as *const u8 as *const gchar,
        );
    } else {
        let mut path: *mut c_char = fm_path_to_str(fm_path_get_desktop());
        gtk_file_chooser_set_filename(chooser as *mut c_void as *mut GtkFileChooser, path);
        g_free(path as gpointer);
    };
}
unsafe extern "C" fn _chooser_timeout(mut user_data: gpointer) -> gboolean {
    let mut data: *mut FolderChooserData = ptr::null_mut::<FolderChooserData>();
    let mut gf: *mut GFile = ptr::null_mut::<GFile>();
    let mut folder: *mut FmFolder = ptr::null_mut::<FmFolder>();
    if g_source_is_destroyed(g_main_current_source()) != 0 {
        return 0 as gboolean;
    }
    data = user_data as *mut FolderChooserData;
    (*data).timeout_handler = 0 as guint;
    if gtk_widget_get_sensitive((*data).desktop_folder_box) == 0 {
        return 0 as gboolean;
    }
    gf = gtk_file_chooser_get_file((*data).chooser as *mut GtkFileChooser);
    if gf.is_null() {
        return 0 as gboolean;
    }
    folder = fm_folder_from_gfile(gf);
    if (*(*data).desktop).model.is_null()
        || fm_folder_model_get_folder((*(*data).desktop).model) != folder
    {
        disconnect_model((*data).desktop);
        if !folder.is_null() {
            connect_model((*data).desktop, folder);
        } else {
            queue_layout_items((*data).desktop);
        }
    }
    g_free((*(*data).desktop).conf.folder as gpointer);
    if ({
        let mut _g_boolean_var_44: c_int = 0i32;
        if (*(*data).desktop).model.is_null() {
            _g_boolean_var_44 = 1i32;
        }
        _g_boolean_var_44
    }) as c_long
        != 0
    {
        (*(*data).desktop).conf.folder = g_strdup_inline(c"".as_ptr());
    } else if fm_path_equal(fm_folder_get_path(folder), fm_path_get_desktop()) == 0 {
        (*(*data).desktop).conf.folder = g_file_get_path(gf);
    } else {
        (*(*data).desktop).conf.folder = ptr::null_mut::<c_char>();
    }
    queue_config_save((*data).desktop);
    if !folder.is_null() {
        g_object_unref(folder as gpointer);
    }
    g_object_unref(gf as gpointer);
    return 0 as gboolean;
}
unsafe extern "C" fn on_desktop_folder_set(
    mut btn: *mut GtkFileChooserButton,
    mut data: *mut FolderChooserData,
) {
    if (*data).timeout_handler == 0 as guint {
        (*data).timeout_handler = gdk_threads_add_timeout(
            100 as guint,
            Some(_chooser_timeout as unsafe extern "C" fn(gpointer) -> gboolean),
            data as gpointer,
        );
    }
}
unsafe extern "C" fn _data_destroy(mut data: gpointer, mut dlg: *mut GObject) {
    let mut d: *mut FolderChooserData = data as *mut FolderChooserData;
    if (*d).timeout_handler != 0 {
        g_source_remove((*d).timeout_handler);
    }
    g_free(data);
}
unsafe extern "C" fn on_use_desktop_folder_toggled(
    mut btn: *mut GtkToggleButton,
    mut data: *mut FolderChooserData,
) {
    let mut active: gboolean = gtk_toggle_button_get_active(btn);
    gtk_widget_set_sensitive((*data).desktop_folder_box, active);
    gtk_widget_set_visible((*data).icons_page, active);
    if active != 0 {
        g_signal_emit_by_name((*data).chooser, b"file-set\0" as *const u8 as *const gchar);
    } else {
        disconnect_model((*data).desktop);
        queue_layout_items((*data).desktop);
        g_free((*(*data).desktop).conf.folder as gpointer);
        (*(*data).desktop).conf.folder = g_strdup_inline(c"".as_ptr());
        queue_config_save((*data).desktop);
    };
}
unsafe extern "C" fn emwm_open_preferences(mut button: *mut GtkButton, mut data: gpointer) {
    let mut error: *mut GError = ptr::null_mut::<GError>();
    if g_spawn_command_line_async(
        b"emwm-appearance --desktop\0" as *const u8 as *const gchar,
        &raw mut error,
    ) == 0
    {
        fm_show_error(
            ptr::null_mut::<GtkWindow>(),
            ptr::null::<c_char>(),
            (*error).message,
        );
        g_error_free(error);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fm_desktop_preference(
    mut act: *mut GtkAction,
    mut desktop: *mut FmDesktop,
) {
    if desktop.is_null() {
        return;
    }
    if desktop_pref_dlg.is_null() {
        let mut builder: *mut GtkBuilder = ptr::null_mut::<GtkBuilder>();
        let mut item: *mut GObject = ptr::null_mut::<GObject>();
        let mut img_preview: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
        let mut data: *mut FolderChooserData = ptr::null_mut::<FolderChooserData>();
        let mut path_str: *mut c_char = ptr::null_mut::<c_char>();
        builder = gtk_builder_new();
        gtk_builder_add_from_file(
            builder,
            b"/usr/share/pcmanfm/ui/desktop-pref.ui\0" as *const u8 as *const gchar,
            ptr::null_mut::<*mut GError>(),
        );
        desktop_pref_dlg = gtk_builder_get_object(builder, b"dlg\0" as *const u8 as *const gchar)
            as *mut c_void as *mut GtkWindow;
        if emwm_layout != 0 {
            let mut button: *mut GtkWidget = gtk_button_new_with_label(
                b"Monitor e alinhamento dos \xC3\xADcones\xE2\x80\xA6\0" as *const u8
                    as *const gchar,
            );
            let mut content: *mut GtkWidget =
                gtk_dialog_get_content_area(desktop_pref_dlg as *mut c_void as *mut GtkDialog);
            gtk_box_pack_start(
                content as *mut c_void as *mut GtkBox,
                button,
                0 as gboolean,
                0 as gboolean,
                6 as guint,
            );
            g_signal_connect_data(
                button as gpointer,
                b"clicked\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkButton, gpointer) -> ()>,
                    GCallback,
                >(Some(
                    emwm_open_preferences as unsafe extern "C" fn(*mut GtkButton, gpointer) -> (),
                )),
                ptr::null_mut::<c_void>(),
                None,
                G_CONNECT_DEFAULT,
            );
            gtk_widget_show(button);
            g_signal_connect_data(
                desktop_pref_dlg as gpointer,
                b"destroy\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkWidget, *mut *mut GtkWidget) -> ()>,
                    GCallback,
                >(Some(
                    gtk_widget_destroyed
                        as unsafe extern "C" fn(*mut GtkWidget, *mut *mut GtkWidget) -> (),
                )),
                &raw mut desktop_pref_dlg as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
        }
        item = gtk_builder_get_object(builder, b"wallpaper\0" as *const u8 as *const gchar);
        g_signal_connect_data(
            item as gpointer,
            b"file-set\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkFileChooserButton, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_wallpaper_set
                    as unsafe extern "C" fn(*mut GtkFileChooserButton, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        img_preview = gtk_image_new();
        gtk_misc_set_alignment(img_preview as *mut c_void as *mut GtkMisc, 0.5f32, 0.0f32);
        gtk_widget_set_size_request(img_preview, 128 as gint, 128 as gint);
        gtk_file_chooser_set_preview_widget(
            item as *mut c_void as *mut GtkFileChooser,
            img_preview,
        );
        g_signal_connect_data(
            item as gpointer,
            b"update-preview\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkFileChooser, *mut GtkImage) -> ()>,
                GCallback,
            >(Some(
                on_update_img_preview
                    as unsafe extern "C" fn(*mut GtkFileChooser, *mut GtkImage) -> (),
            )),
            img_preview as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        if !(*desktop).conf.wallpaper.is_null() {
            gtk_file_chooser_set_filename(
                item as *mut c_void as *mut GtkFileChooser,
                (*desktop).conf.wallpaper,
            );
        }
        item = gtk_builder_get_object(builder, b"wallpaper_mode\0" as *const u8 as *const gchar);
        gtk_combo_box_set_active(
            item as *mut c_void as *mut GtkComboBox,
            (*desktop).conf.wallpaper_mode as gint,
        );
        g_signal_connect_data(
            item as gpointer,
            b"changed\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkComboBox, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_wallpaper_mode_changed
                    as unsafe extern "C" fn(*mut GtkComboBox, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        item = gtk_builder_get_object(builder, b"wallpaper_box\0" as *const u8 as *const gchar);
        if !item.is_null() {
            g_signal_connect_data(
                gtk_builder_get_object(builder, b"wallpaper_mode\0" as *const u8 as *const gchar)
                    as gpointer,
                b"changed\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkComboBox, *mut GtkWidget) -> ()>,
                    GCallback,
                >(Some(
                    on_wallpaper_mode_changed2
                        as unsafe extern "C" fn(*mut GtkComboBox, *mut GtkWidget) -> (),
                )),
                item as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
            gtk_widget_set_sensitive(
                item as *mut c_void as *mut GtkWidget,
                ((*desktop).conf.wallpaper_mode as c_uint != FM_WP_COLOR as c_int as c_uint)
                    as c_int,
            );
        }
        item = gtk_builder_get_object(builder, b"desktop_bg\0" as *const u8 as *const gchar);
        gtk_color_button_set_color(
            item as *mut c_void as *mut GtkColorButton,
            &raw mut (*desktop).conf.desktop_bg,
        );
        g_signal_connect_data(
            item as gpointer,
            b"color-set\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkColorButton, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_bg_color_set as unsafe extern "C" fn(*mut GtkColorButton, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        item = gtk_builder_get_object(builder, b"wallpaper_common\0" as *const u8 as *const gchar);
        gtk_toggle_button_set_active(
            item as *mut c_void as *mut GtkToggleButton,
            (*desktop).conf.wallpaper_common,
        );
        g_signal_connect_data(
            item as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_wallpaper_common_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        item = gtk_builder_get_object(builder, b"desktop_fg\0" as *const u8 as *const gchar);
        gtk_color_button_set_color(
            item as *mut c_void as *mut GtkColorButton,
            &raw mut (*desktop).conf.desktop_fg,
        );
        g_signal_connect_data(
            item as gpointer,
            b"color-set\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkColorButton, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_fg_color_set as unsafe extern "C" fn(*mut GtkColorButton, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        item = gtk_builder_get_object(builder, b"desktop_shadow\0" as *const u8 as *const gchar);
        gtk_color_button_set_color(
            item as *mut c_void as *mut GtkColorButton,
            &raw mut (*desktop).conf.desktop_shadow,
        );
        g_signal_connect_data(
            item as gpointer,
            b"color-set\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkColorButton, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_shadow_color_set
                    as unsafe extern "C" fn(*mut GtkColorButton, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        item = gtk_builder_get_object(builder, b"show_wm_menu\0" as *const u8 as *const gchar);
        gtk_toggle_button_set_active(
            item as *mut c_void as *mut GtkToggleButton,
            (*desktop).conf.show_wm_menu,
        );
        g_signal_connect_data(
            item as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_wm_menu_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        item = gtk_builder_get_object(builder, b"desktop_font\0" as *const u8 as *const gchar);
        if !(*desktop).conf.desktop_font.is_null() {
            gtk_font_button_set_font_name(
                item as *mut c_void as *mut GtkFontButton,
                (*desktop).conf.desktop_font,
            );
        }
        g_signal_connect_data(
            item as gpointer,
            b"font-set\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkFontButton, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_desktop_font_set
                    as unsafe extern "C" fn(*mut GtkFontButton, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        data = ({
            let mut __n: gsize = 1 as gsize;
            let mut __s: gsize = mem::size_of::<FolderChooserData>() as gsize;
            let mut __p: gpointer = ptr::null_mut::<c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0
                && (__s == 0 as gsize
                    || __n
                        <= (9223372036854775807 as c_long as gsize)
                            .wrapping_mul(2 as gsize)
                            .wrapping_add(1 as gsize)
                            .wrapping_div(__s))
            {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut FolderChooserData;
        (*data).desktop = desktop;
        (*data).desktop_folder_box = gtk_builder_get_object(
            builder,
            b"desktop_folder_box\0" as *const u8 as *const gchar,
        ) as *mut c_void as *mut GtkWidget;
        (*data).chooser =
            gtk_builder_get_object(builder, b"desktop_folder\0" as *const u8 as *const gchar)
                as gpointer;
        (*data).timeout_handler = gdk_threads_add_timeout(
            500 as guint,
            Some(_chooser_timeout as unsafe extern "C" fn(gpointer) -> gboolean),
            data as gpointer,
        );
        g_object_weak_ref(
            (*data).chooser as *mut GObject,
            Some(_data_destroy as unsafe extern "C" fn(gpointer, *mut GObject) -> ()),
            data as gpointer,
        );
        item = gtk_builder_get_object(
            builder,
            b"use_desktop_folder\0" as *const u8 as *const gchar,
        );
        gtk_toggle_button_set_active(
            item as *mut c_void as *mut GtkToggleButton,
            ((*desktop).model != ptr::null_mut::<c_void>() as *mut FmFolderModel) as c_int,
        );
        gtk_widget_set_sensitive(
            (*data).desktop_folder_box,
            ((*desktop).model != ptr::null_mut::<c_void>() as *mut FmFolderModel) as c_int,
        );
        g_signal_connect_data(
            item as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut FolderChooserData) -> ()>,
                GCallback,
            >(Some(
                on_use_desktop_folder_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, *mut FolderChooserData) -> (),
            )),
            data as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        item = gtk_builder_get_object(
            builder,
            b"desktop_folder_set\0" as *const u8 as *const gchar,
        );
        if (*desktop).model.is_null()
            || fm_path_equal(
                fm_folder_model_get_folder_path((*desktop).model),
                fm_path_get_desktop(),
            ) != 0
        {
            gtk_toggle_button_set_active(
                gtk_builder_get_object(
                    builder,
                    b"desktop_folder_default\0" as *const u8 as *const gchar,
                ) as *mut c_void as *mut GtkToggleButton,
                (0i32 == 0) as c_int,
            );
            gtk_widget_set_sensitive((*data).chooser as *mut GtkWidget, 0 as gboolean);
        } else {
            gtk_toggle_button_set_active(
                item as *mut c_void as *mut GtkToggleButton,
                (0i32 == 0) as c_int,
            );
            gtk_widget_set_sensitive((*data).chooser as *mut GtkWidget, (0i32 == 0) as c_int);
        }
        g_signal_connect_data(
            item as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut GtkFileChooserButton) -> ()>,
                GCallback,
            >(Some(
                on_desktop_folder_set_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, *mut GtkFileChooserButton) -> (),
            )),
            (*data).chooser,
            None,
            G_CONNECT_DEFAULT,
        );
        if !(*desktop).model.is_null() {
            path_str = fm_path_to_uri(fm_folder_model_get_folder_path((*desktop).model));
        } else {
            path_str = fm_path_to_uri(fm_path_get_desktop());
        }
        gtk_file_chooser_set_current_folder_uri((*data).chooser as *mut GtkFileChooser, path_str);
        g_free(path_str as gpointer);
        g_signal_connect_data(
            (*data).chooser,
            b"file-set\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut GtkFileChooserButton, *mut FolderChooserData) -> (),
                >,
                GCallback,
            >(Some(
                on_desktop_folder_set
                    as unsafe extern "C" fn(
                        *mut GtkFileChooserButton,
                        *mut FolderChooserData,
                    ) -> (),
            )),
            data as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        g_signal_connect_data(
            (*data).chooser,
            b"selection-changed\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut GtkFileChooserButton, *mut FolderChooserData) -> (),
                >,
                GCallback,
            >(Some(
                on_desktop_folder_set
                    as unsafe extern "C" fn(
                        *mut GtkFileChooserButton,
                        *mut FolderChooserData,
                    ) -> (),
            )),
            data as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        gtk_widget_show(gtk_builder_get_object(
            builder,
            b"desktop_folder_chooser\0" as *const u8 as *const gchar,
        ) as *mut c_void as *mut GtkWidget);
        item = gtk_builder_get_object(
            builder,
            b"desktop_folder_new_win\0" as *const u8 as *const gchar,
        );
        gtk_toggle_button_set_active(
            item as *mut c_void as *mut GtkToggleButton,
            (*(fm_config as *mut FmAppConfig)).desktop_folder_new_win,
        );
        g_signal_connect_data(
            item as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_desktop_folder_new_win_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        item = gtk_builder_get_object(builder, b"show_documents\0" as *const u8 as *const gchar);
        gtk_toggle_button_set_active(
            item as *mut c_void as *mut GtkToggleButton,
            (*desktop).conf.show_documents,
        );
        gtk_widget_set_sensitive(
            item as *mut c_void as *mut GtkWidget,
            (documents != ptr::null_mut::<c_void>() as *mut FmDesktopExtraItem) as c_int,
        );
        g_signal_connect_data(
            item as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_show_documents_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        item = gtk_builder_get_object(builder, b"show_trash\0" as *const u8 as *const gchar);
        gtk_toggle_button_set_active(
            item as *mut c_void as *mut GtkToggleButton,
            (*desktop).conf.show_trash,
        );
        gtk_widget_set_sensitive(
            item as *mut c_void as *mut GtkWidget,
            (trash_can != ptr::null_mut::<c_void>() as *mut FmDesktopExtraItem) as c_int,
        );
        g_signal_connect_data(
            item as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_show_trash_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        item = gtk_builder_get_object(builder, b"show_mounts\0" as *const u8 as *const gchar);
        gtk_toggle_button_set_active(
            item as *mut c_void as *mut GtkToggleButton,
            (*desktop).conf.show_mounts,
        );
        g_signal_connect_data(
            item as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> ()>,
                GCallback,
            >(Some(
                on_show_mounts_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, *mut FmDesktop) -> (),
            )),
            desktop as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        (*data).icons_page =
            gtk_builder_get_object(builder, b"icons_page\0" as *const u8 as *const gchar)
                as *mut c_void as *mut GtkWidget;
        gtk_widget_set_visible(
            (*data).icons_page,
            ((*desktop).model != ptr::null_mut::<c_void>() as *mut FmFolderModel) as c_int,
        );
        g_signal_connect_data(
            desktop_pref_dlg as gpointer,
            b"response\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkDialog, c_int, *mut *mut GtkWindow) -> ()>,
                GCallback,
            >(Some(
                desktop_on_response
                    as unsafe extern "C" fn(*mut GtkDialog, c_int, *mut *mut GtkWindow) -> (),
            )),
            &raw mut desktop_pref_dlg as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        g_object_unref(builder as gpointer);
        pcmanfm_ref();
        g_signal_connect_data(
            desktop_pref_dlg as gpointer,
            b"destroy\0" as *const u8 as *const gchar,
            mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GCallback>(Some(pcmanfm_unref)),
            ptr::null_mut::<c_void>(),
            None,
            G_CONNECT_DEFAULT,
        );
        gtk_window_set_transient_for(desktop_pref_dlg, desktop as *mut c_void as *mut GtkWindow);
        gtk_window_set_destroy_with_parent(desktop_pref_dlg, (0i32 == 0) as c_int);
    }
    gtk_window_present(desktop_pref_dlg);
}
unsafe extern "C" fn emwm_refresh(mut unused: gpointer) -> gboolean {
    let mut i: gint = 0;
    let mut screen: gint = emwm_on_screen;
    i = 0 as gint;
    while i < n_screens {
        if (**desktops.offset(i as isize)).dragging() != 0
            || (**desktops.offset(i as isize)).rubber_bending() != 0
        {
            return (0i32 == 0) as c_int;
        }
        i += 1;
    }
    emwm_refresh_id = 0 as guint;
    pcmanfm_ref();
    fm_desktop_manager_finalize();
    fm_desktop_manager_init(screen);
    pcmanfm_unref();
    return 0 as gboolean;
}
unsafe extern "C" fn emwm_queue_refresh() {
    if emwm_refresh_id == 0 {
        emwm_refresh_id = g_timeout_add(
            250 as guint,
            Some(emwm_refresh as unsafe extern "C" fn(gpointer) -> gboolean),
            ptr::null_mut::<c_void>(),
        );
    }
}
unsafe extern "C" fn emwm_settings_changed(
    mut monitor: *mut GFileMonitor,
    mut file: *mut GFile,
    mut other: *mut GFile,
    mut event: GFileMonitorEvent,
    mut data: gpointer,
) {
    let mut name: *mut c_char = g_file_get_basename(file);
    let mut other_name: *mut c_char = if !other.is_null() {
        g_file_get_basename(other)
    } else {
        ptr::null_mut::<c_char>()
    };
    if g_strcmp0(name, c"desktop.conf".as_ptr()) == 0
        || g_strcmp0(other_name, c"desktop.conf".as_ptr()) == 0
    {
        emwm_queue_refresh();
    }
    g_free(name as gpointer);
    g_free(other_name as gpointer);
}
unsafe extern "C" fn emwm_load_settings(mut on_screen: gint) {
    let mut directory: *mut c_char = ptr::null_mut::<c_char>();
    let mut path: *mut c_char = ptr::null_mut::<c_char>();
    let mut anchor: *mut c_char = ptr::null_mut::<c_char>();
    let mut settings: *mut GKeyFile = ptr::null_mut::<GKeyFile>();
    let mut file: *mut GFile = ptr::null_mut::<GFile>();
    emwm_layout = (g_strcmp0(
        g_getenv(b"EMWM_DESKTOP_LAYOUT\0" as *const u8 as *const gchar) as *const c_char,
        c"1".as_ptr(),
    ) == 0) as c_int as gboolean;
    if emwm_layout == 0 {
        return;
    }
    emwm_on_screen = on_screen;
    directory = g_build_filename(
        g_get_user_config_dir(),
        c"emwm-desktop".as_ptr(),
        ptr::null_mut::<c_void>(),
    ) as *mut c_char;
    g_mkdir_with_parents(directory, 0o700 as gint);
    path = g_build_filename(
        directory,
        c"desktop.conf".as_ptr(),
        ptr::null_mut::<c_void>(),
    ) as *mut c_char;
    settings = g_key_file_new();
    g_key_file_load_from_file(
        settings,
        path,
        G_KEY_FILE_NONE,
        ptr::null_mut::<*mut GError>(),
    );
    g_free(emwm_output as gpointer);
    emwm_output = g_key_file_get_string(
        settings,
        b"desktop\0" as *const u8 as *const gchar,
        b"monitor\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    ) as *mut c_char;
    anchor = g_key_file_get_string(
        settings,
        b"desktop\0" as *const u8 as *const gchar,
        b"anchor\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    ) as *mut c_char;
    emwm_right = (g_strcmp0(anchor, c"right".as_ptr()) == 0) as c_int as gboolean;
    g_free(anchor as gpointer);
    g_key_file_free(settings);
    file = g_file_new_for_path(directory);
    emwm_settings_monitor = g_file_monitor_directory(
        file,
        G_FILE_MONITOR_WATCH_MOVES,
        ptr::null_mut::<GCancellable>(),
        ptr::null_mut::<*mut GError>(),
    );
    if !emwm_settings_monitor.is_null() {
        g_signal_connect_data(
            emwm_settings_monitor as gpointer,
            b"changed\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        gpointer,
                    ) -> (),
                >,
                GCallback,
            >(Some(
                emwm_settings_changed
                    as unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        gpointer,
                    ) -> (),
            )),
            ptr::null_mut::<c_void>(),
            None,
            G_CONNECT_DEFAULT,
        );
    }
    g_object_unref(file as gpointer);
    g_free(path as gpointer);
    g_free(directory as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn fm_desktop_manager_init(mut on_screen: gint) {
    let mut gdpy: *mut GdkDisplay = ptr::null_mut::<GdkDisplay>();
    let mut i: c_int = 0;
    let mut n_scr: c_int = 0;
    let mut n_mon: c_int = 0;
    let mut scr: c_int = 0;
    let mut mon: c_int = 0;
    let mut desktop_path: *const c_char = ptr::null::<c_char>();
    let mut gf: *mut GFile = ptr::null_mut::<GFile>();
    emwm_load_settings(on_screen);
    if win_group.is_null() {
        win_group = gtk_window_group_new();
    }
    desktop_path = g_get_user_special_dir(G_USER_DIRECTORY_DESKTOP) as *const c_char;
    g_mkdir_with_parents(desktop_path as *const gchar, 0o700 as gint);
    gdpy = gdk_display_get_default();
    n_scr = gdk_display_get_n_screens(gdpy) as c_int;
    n_screens = 0i32;
    i = 0i32;
    while i < n_scr {
        n_screens += gdk_screen_get_n_monitors(gdk_display_get_screen(gdpy, i as gint)) as c_int;
        i += 1;
    }
    desktops = ({
        let mut __n: gsize = n_screens as gsize;
        let mut __s: gsize = mem::size_of::<*mut FmDesktop>() as gsize;
        let mut __p: gpointer = ptr::null_mut::<c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0
            && (__s == 0 as gsize
                || __n
                    <= (9223372036854775807 as c_long as gsize)
                        .wrapping_mul(2 as gsize)
                        .wrapping_add(1 as gsize)
                        .wrapping_div(__s))
        {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut *mut FmDesktop;
    scr = 0i32;
    i = 0i32;
    while scr < n_scr {
        let mut screen: *mut GdkScreen = gdk_display_get_screen(gdpy, scr as gint);
        n_mon = gdk_screen_get_n_monitors(screen) as c_int;
        mon = 0i32;
        while mon < n_mon {
            let mut mon_init: gint = if on_screen < 0i32 || on_screen == scr {
                mon
            } else if mon != 0 {
                -(2 as gint)
            } else {
                -(1 as gint)
            };
            let mut desktop: *mut FmDesktop = fm_desktop_new(screen, mon_init);
            let mut widget: *mut GtkWidget = desktop as *mut c_void as *mut GtkWidget;
            let mut desktop_folder: *mut FmFolder = ptr::null_mut::<FmFolder>();
            let fresh2 = i;
            i = i + 1;
            let ref mut fresh3 = *desktops.offset(fresh2 as isize);
            *fresh3 = desktop;
            if !(mon_init < 0i32) {
                gtk_widget_realize(widget);
                if emwm_layout != 0 && mon != emwm_icon_monitor(screen) {
                    desktop_folder = ptr::null_mut::<FmFolder>();
                } else if !(*desktop).conf.folder.is_null() {
                    if *(*desktop).conf.folder.offset(0 as isize) != 0 {
                        desktop_folder = fm_folder_from_path_name((*desktop).conf.folder);
                    } else {
                        desktop_folder = ptr::null_mut::<FmFolder>();
                    }
                } else {
                    desktop_folder = fm_folder_from_path(fm_path_get_desktop());
                }
                if !desktop_folder.is_null() {
                    connect_model(desktop, desktop_folder);
                    g_object_unref(desktop_folder as gpointer);
                } else {
                    fm_folder_view_add_popup(
                        desktop as *mut c_void as *mut FmFolderView,
                        desktop as *mut c_void as *mut GtkWindow,
                        Some(
                            fm_desktop_update_popup
                                as unsafe extern "C" fn(
                                    *mut FmFolderView,
                                    *mut GtkWindow,
                                    *mut GtkUIManager,
                                    *mut GtkActionGroup,
                                    *mut FmFileInfoList,
                                ) -> (),
                        ),
                    );
                }
                if !(*desktop).model.is_null() {
                    fm_folder_model_set_sort(
                        (*desktop).model,
                        (*desktop).conf.desktop_sort_by,
                        (*desktop).conf.desktop_sort_type,
                    );
                }
                gtk_widget_show_all(widget);
                gdk_window_lower(gtk_widget_get_window(widget));
            }
            mon += 1;
        }
        scr += 1;
    }
    if emwm_layout != 0 && emwm_have_wallpaper != 0 {
        i = 0i32;
        while i < n_screens {
            if (**desktops.offset(i as isize)).monitor >= 0i32 {
                g_free((**desktops.offset(i as isize)).conf.wallpaper as gpointer);
                let ref mut fresh4 = (**desktops.offset(i as isize)).conf.wallpaper;
                *fresh4 = g_strdup_inline(emwm_wallpaper);
                (**desktops.offset(i as isize)).conf.wallpaper_mode = emwm_wallpaper_mode;
                fm_desktop_wallpaper_changed(*desktops.offset(i as isize));
                break;
            } else {
                i += 1;
            }
        }
    }
    icon_theme_changed = g_signal_connect_data(
        gtk_icon_theme_get_default() as gpointer,
        b"changed\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkIconTheme, gpointer) -> ()>, GCallback>(
            Some(on_icon_theme_changed as unsafe extern "C" fn(*mut GtkIconTheme, gpointer) -> ()),
        ),
        ptr::null_mut::<c_void>(),
        None,
        G_CONNECT_DEFAULT,
    ) as guint;
    hand_cursor = gdk_cursor_new(GDK_HAND2);
    gf = fm_file_new_for_uri(c"trash:///".as_ptr());
    if g_file_query_exists(gf, ptr::null_mut::<GCancellable>()) != 0 {
        trash_can = _add_extra_item(c"trash:///".as_ptr());
    } else {
        trash_can = ptr::null_mut::<FmDesktopExtraItem>();
    }
    if ({
        let mut _g_boolean_var_45: c_int = 0i32;
        if !trash_can.is_null() {
            _g_boolean_var_45 = 1i32;
        }
        _g_boolean_var_45
    }) as c_long
        != 0
    {
        trash_monitor = fm_monitor_directory(gf, ptr::null_mut::<*mut GError>());
        g_signal_connect_data(
            trash_monitor as gpointer,
            b"changed\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        *mut FmDesktopExtraItem,
                    ) -> (),
                >,
                GCallback,
            >(Some(
                on_trash_changed
                    as unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        *mut FmDesktopExtraItem,
                    ) -> (),
            )),
            trash_can as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
    }
    g_object_unref(gf as gpointer);
    documents =
        _add_extra_item(g_get_user_special_dir(G_USER_DIRECTORY_DOCUMENTS) as *const c_char);
    vol_mon = g_volume_monitor_get();
    if ({
        let mut _g_boolean_var_46: c_int = 0i32;
        if !vol_mon.is_null() {
            _g_boolean_var_46 = 1i32;
        }
        _g_boolean_var_46
    }) as c_long
        != 0
    {
        let mut ml: *mut GList = g_volume_monitor_get_mounts(vol_mon);
        let mut l: *mut GList = ptr::null_mut::<GList>();
        l = ml;
        while !l.is_null() {
            let mut mount: *mut GMount = (*l).data as *mut GMount;
            on_mount_added(vol_mon, mount, ptr::null_mut::<c_void>());
            g_object_unref(mount as gpointer);
            l = (*l).next;
        }
        g_list_free(ml);
        g_signal_connect_data(
            vol_mon as gpointer,
            b"mount-added\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount, gpointer) -> ()>,
                GCallback,
            >(Some(
                on_mount_added
                    as unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount, gpointer) -> (),
            )),
            ptr::null_mut::<c_void>(),
            None,
            G_CONNECT_DEFAULT,
        );
        g_signal_connect_data(
            vol_mon as gpointer,
            b"mount-removed\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount, gpointer) -> ()>,
                GCallback,
            >(Some(
                on_mount_removed
                    as unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount, gpointer) -> (),
            )),
            ptr::null_mut::<c_void>(),
            None,
            G_CONNECT_DEFAULT,
        );
    }
    pcmanfm_ref();
}
#[no_mangle]
pub unsafe extern "C" fn fm_desktop_manager_finalize() {
    let mut i: c_int = 0;
    if emwm_refresh_id != 0 {
        g_source_remove(emwm_refresh_id);
        emwm_refresh_id = 0 as guint;
    }
    if !emwm_settings_monitor.is_null() {
        g_file_monitor_cancel(emwm_settings_monitor);
        g_object_unref(emwm_settings_monitor as gpointer);
        emwm_settings_monitor = ptr::null_mut::<GFileMonitor>();
    }
    if idle_config_save != 0 {
        g_source_remove(idle_config_save);
        idle_config_save = 0 as guint;
    }
    i = 0i32;
    while i < n_screens {
        gtk_widget_destroy(*desktops.offset(i as isize) as *mut c_void as *mut GtkWidget);
        i += 1;
    }
    g_free(desktops as gpointer);
    n_screens = 0i32;
    g_object_unref(win_group as gpointer);
    win_group = ptr::null_mut::<GtkWindowGroup>();
    g_signal_handler_disconnect(
        gtk_icon_theme_get_default() as gpointer,
        icon_theme_changed as gulong,
    );
    if !acc_grp.is_null() {
        g_object_unref(acc_grp as gpointer);
    }
    acc_grp = ptr::null_mut::<GtkAccelGroup>();
    if !hand_cursor.is_null() {
        gdk_cursor_unref(hand_cursor);
        hand_cursor = ptr::null_mut::<GdkCursor>();
    }
    if ({
        let mut _g_boolean_var_47: c_int = 0i32;
        if !documents.is_null() {
            _g_boolean_var_47 = 1i32;
        }
        _g_boolean_var_47
    }) as c_long
        != 0
    {
        _free_extra_item(documents);
        documents = ptr::null_mut::<FmDesktopExtraItem>();
    }
    if ({
        let mut _g_boolean_var_48: c_int = 0i32;
        if !trash_can.is_null() {
            _g_boolean_var_48 = 1i32;
        }
        _g_boolean_var_48
    }) as c_long
        != 0
    {
        g_signal_handlers_disconnect_matched(
            trash_monitor as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        *mut FmDesktopExtraItem,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                on_trash_changed
                    as unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        *mut FmDesktopExtraItem,
                    ) -> (),
            )),
            trash_can as gpointer,
        );
        g_object_unref(trash_monitor as gpointer);
        _free_extra_item(trash_can);
        trash_can = ptr::null_mut::<FmDesktopExtraItem>();
    }
    if ({
        let mut _g_boolean_var_49: c_int = 0i32;
        if !vol_mon.is_null() {
            _g_boolean_var_49 = 1i32;
        }
        _g_boolean_var_49
    }) as c_long
        != 0
    {
        g_signal_handlers_disconnect_matched(
            vol_mon as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount, gpointer) -> ()>,
                gpointer,
            >(Some(
                on_mount_added
                    as unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount, gpointer) -> (),
            )),
            ptr::null_mut::<c_void>(),
        );
        g_signal_handlers_disconnect_matched(
            vol_mon as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount, gpointer) -> ()>,
                gpointer,
            >(Some(
                on_mount_removed
                    as unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount, gpointer) -> (),
            )),
            ptr::null_mut::<c_void>(),
        );
        g_object_unref(vol_mon as gpointer);
        vol_mon = ptr::null_mut::<GVolumeMonitor>();
    }
    while !mounts.is_null() {
        _free_extra_item((*mounts).data as *mut FmDesktopExtraItem);
        mounts = g_slist_delete_link(mounts, mounts);
    }
    pcmanfm_unref();
}
#[no_mangle]
pub unsafe extern "C" fn fm_desktop_get(mut screen: gint, mut monitor: gint) -> *mut FmDesktop {
    let mut i: c_int = 0i32;
    let mut n: c_int = 0i32;
    while i < n_screens && n <= screen {
        if n == screen && (**desktops.offset(i as isize)).monitor == monitor {
            return *desktops.offset(i as isize);
        }
        i += 1;
        if i < n_screens
            && ((**desktops.offset(i as isize)).monitor == 0i32
                || (**desktops.offset(i as isize)).monitor == -(1i32))
        {
            n += 1;
        }
    }
    return ptr::null_mut::<FmDesktop>();
}
#[no_mangle]
pub unsafe extern "C" fn fm_desktop_wallpaper_changed(mut desktop: *mut FmDesktop) {
    let mut i: c_int = 0;
    if emwm_layout != 0 {
        let mut current: *mut c_char = g_strdup_inline((*desktop).conf.wallpaper);
        g_free(emwm_wallpaper as gpointer);
        emwm_wallpaper = current;
        emwm_wallpaper_mode = (*desktop).conf.wallpaper_mode;
        emwm_have_wallpaper = (0i32 == 0) as c_int as gboolean;
        i = 0i32;
        while i < n_screens {
            let mut target: *mut FmDesktop = *desktops.offset(i as isize);
            if !((*target).monitor < 0i32) {
                if target != desktop {
                    g_free((*target).conf.wallpaper as gpointer);
                    (*target).conf.wallpaper = g_strdup_inline((*desktop).conf.wallpaper);
                    (*target).conf.wallpaper_mode = (*desktop).conf.wallpaper_mode;
                }
                (*target).conf.wallpaper_common = (0i32 == 0) as c_int as gboolean;
                queue_config_save(target);
                update_background(target, 0i32);
            }
            i += 1;
        }
        return;
    }
    queue_config_save(desktop);
    update_background(desktop, 0i32);
}
