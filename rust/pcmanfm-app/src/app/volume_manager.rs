// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      volume-manager.c
 *
 *      Copyright 2010 Hong Jen Yee (PCMan) <pcman.tw@gmail.com>
 *      Copyright 2013 Andriy Grytsenko (LStranger) <andrej@rep.kiev.ua>
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

static mut on_idle_handler: guint = 0 as guint;
unsafe extern "C" fn on_unmount(mut mount: *mut GMount, mut data: *mut AutoRun) {
    g_log(
        ptr::null_mut::<gchar>(),
        G_LOG_LEVEL_DEBUG,
        b"on umount\0" as *const u8 as *const gchar,
    );
    on_dlg_response((*data).dlg, GTK_RESPONSE_CLOSE as c_int, data as gpointer);
}
unsafe extern "C" fn on_row_activated(
    mut view: *mut GtkTreeView,
    mut tp: *mut GtkTreePath,
    mut col: *mut GtkTreeViewColumn,
    mut user_data: gpointer,
) {
    let mut data: *mut AutoRun = user_data as *mut AutoRun;
    on_dlg_response((*data).dlg, GTK_RESPONSE_OK as c_int, data as gpointer);
}
unsafe extern "C" fn _run_app(mut app: *mut GAppInfo, mut mount: *mut GMount) {
    let mut gf: *mut GFile = g_mount_get_root(mount);
    if !app.is_null() {
        let mut filelist: *mut GList = g_list_prepend(ptr::null_mut::<GList>(), gf as gpointer);
        g_app_info_launch(
            app,
            filelist,
            ptr::null_mut::<GAppLaunchContext>(),
            ptr::null_mut::<*mut GError>(),
        );
        g_list_free(filelist);
        g_object_unref(app as gpointer);
    } else {
        let mut path: *mut FmPath = fm_path_new_for_gfile(gf);
        if (*(fm_config as *mut FmAppConfig)).media_in_new_tab != 0 {
            fm_main_win_open_in_last_active(path);
        } else {
            fm_main_win_add_win(ptr::null_mut::<FmMainWin>(), path);
        }
        fm_path_unref(path);
    }
    g_object_unref(gf as gpointer);
}
unsafe extern "C" fn on_dlg_response(
    mut dlg: *mut GtkDialog,
    mut res: c_int,
    mut user_data: gpointer,
) {
    let mut data: *mut AutoRun = user_data as *mut AutoRun;
    let mut dont_ask: gboolean = 0 as gboolean;
    g_cancellable_cancel((*data).cancel);
    if !(*data).dont_ask.is_null() {
        dont_ask = gtk_toggle_button_get_active((*data).dont_ask);
    }
    if !((*data).skipped != 0) {
        if res == GTK_RESPONSE_OK as c_int {
            let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
            let mut sel: *mut GtkTreeSelection = gtk_tree_view_get_selection((*data).view);
            let mut it: GtkTreeIter = _GtkTreeIter {
                stamp: 0,
                user_data: ptr::null_mut::<c_void>(),
                user_data2: ptr::null_mut::<c_void>(),
                user_data3: ptr::null_mut::<c_void>(),
            };
            if gtk_tree_selection_get_selected(sel, &raw mut model, &raw mut it) != 0 {
                let mut app: *mut GAppInfo = ptr::null_mut::<GAppInfo>();
                gtk_tree_model_get(model, &raw mut it, 2 as c_int, &raw mut app, -(1i32));
                fm_app_config_set_autorun_choice(
                    fm_config as *mut FmAppConfig,
                    (*data).content_type,
                    if !app.is_null() {
                        g_app_info_get_id(app)
                    } else {
                        c"pcmanfm".as_ptr()
                    },
                    dont_ask,
                );
                _run_app(app, (*data).mount);
                pcmanfm_save_config(0 as gboolean);
            }
        } else if dont_ask != 0 {
            fm_app_config_set_autorun_choice(
                fm_config as *mut FmAppConfig,
                (*data).content_type,
                ptr::null::<c_char>(),
                (0i32 == 0) as c_int,
            );
            pcmanfm_save_config(0 as gboolean);
        }
    }
    g_signal_handlers_disconnect_matched(
        dlg as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkDialog, c_int, gpointer) -> ()>,
            gpointer,
        >(Some(
            on_dlg_response as unsafe extern "C" fn(*mut GtkDialog, c_int, gpointer) -> (),
        )),
        data as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*data).view as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkTreeView,
                    *mut GtkTreePath,
                    *mut GtkTreeViewColumn,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            on_row_activated
                as unsafe extern "C" fn(
                    *mut GtkTreeView,
                    *mut GtkTreePath,
                    *mut GtkTreeViewColumn,
                    gpointer,
                ) -> (),
        )),
        data as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*data).mount as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<Option<unsafe extern "C" fn(*mut GMount, *mut AutoRun) -> ()>, gpointer>(
            Some(on_unmount as unsafe extern "C" fn(*mut GMount, *mut AutoRun) -> ()),
        ),
        data as gpointer,
    );
    gtk_widget_destroy(dlg as *mut c_void as *mut GtkWidget);
    g_object_unref((*data).cancel as gpointer);
    g_object_unref((*data).store as gpointer);
    g_object_unref((*data).mount as gpointer);
    g_free((*data).content_type as gpointer);
    g_slice_free1(mem::size_of::<AutoRun>() as gsize, data as gpointer);
    pcmanfm_unref();
}
unsafe extern "C" fn on_content_type_finished(
    mut src_obj: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut AutoRun = user_data as *mut AutoRun;
    let mut mount: *mut GMount = src_obj as *mut c_void as *mut GMount;
    let mut types: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
    let mut desc: *mut c_char = ptr::null_mut::<c_char>();
    let mut def_type: *const c_char = ptr::null::<c_char>();
    let mut app: *mut GAppInfo = ptr::null_mut::<GAppInfo>();
    let mut mixed_content: gboolean = 0 as gboolean;
    types = g_mount_guess_content_type_finish(mount, res, ptr::null_mut::<*mut GError>())
        as *mut *mut c_char;
    if !types.is_null() {
        let mut it: GtkTreeIter = _GtkTreeIter {
            stamp: 0,
            user_data: ptr::null_mut::<c_void>(),
            user_data2: ptr::null_mut::<c_void>(),
            user_data3: ptr::null_mut::<c_void>(),
        };
        let mut apps: *mut GList = ptr::null_mut::<GList>();
        let mut l: *mut GList = ptr::null_mut::<GList>();
        let mut type_0: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
        let mut pos: c_int = 0i32;
        if !(*types.offset(0 as isize)).is_null() {
            let mut current_block_18: u64;
            let mut choice: *mut FmAutorunChoice = ptr::null_mut::<FmAutorunChoice>();
            choice = g_hash_table_lookup(
                (*(fm_config as *mut FmAppConfig)).autorun_choices,
                *types.offset(0 as isize) as gconstpointer,
            ) as *mut FmAutorunChoice;
            (*data).content_type = g_strdup_inline(*types.offset(0 as isize));
            if !choice.is_null() {
                def_type = (*choice).last_used;
                if (*choice).dont_ask != 0 {
                    if !def_type.is_null() {
                        if strcmp(def_type, c"pcmanfm".as_ptr()) == 0i32 {
                            _run_app(ptr::null_mut::<GAppInfo>(), (*data).mount);
                            current_block_18 = 12599329904712511516;
                        } else {
                            app = g_desktop_app_info_new(def_type) as *mut c_void as *mut GAppInfo;
                            if !app.is_null() {
                                _run_app(app, (*data).mount);
                                current_block_18 = 12599329904712511516;
                            } else {
                                current_block_18 = 9829309749924503094;
                            }
                        }
                    } else {
                        current_block_18 = 12599329904712511516;
                    }
                    match current_block_18 {
                        9829309749924503094 => {}
                        _ => {
                            g_strfreev(types as *mut *mut gchar);
                            (*data).skipped = (0i32 == 0) as c_int as gboolean;
                            on_dlg_response(
                                (*data).dlg,
                                GTK_RESPONSE_CLOSE as c_int,
                                data as gpointer,
                            );
                            return;
                        }
                    }
                }
            }
            type_0 = types;
            while !(*type_0).is_null() {
                l = g_app_info_get_all_for_type(*type_0);
                if !l.is_null() {
                    apps = g_list_concat(apps, l);
                }
                type_0 = type_0.offset(1);
            }
            if !(*types.offset(1 as isize)).is_null() {
                mixed_content = (0i32 == 0) as c_int as gboolean;
            } else {
                desc = g_content_type_get_description(*types.offset(0 as isize)) as *mut c_char;
            }
        }
        g_strfreev(types as *mut *mut gchar);
        if !def_type.is_null() {
            if strcmp(def_type, c"pcmanfm".as_ptr()) == 0i32 {
                pos += 1;
            } else {
                app = g_desktop_app_info_new(def_type) as *mut c_void as *mut GAppInfo;
                if !app.is_null() {
                    let fresh37 = pos;
                    pos = pos + 1;
                    gtk_list_store_insert_with_values(
                        (*data).store,
                        &raw mut it,
                        fresh37,
                        0i32,
                        g_app_info_get_icon(app),
                        1i32,
                        g_app_info_get_name(app),
                        2 as c_int,
                        app,
                        -(1i32),
                    );
                    g_object_unref(app as gpointer);
                }
            }
        }
        if !apps.is_null() {
            let mut tp: *mut GtkTreePath = ptr::null_mut::<GtkTreePath>();
            l = apps;
            while !l.is_null() {
                app = (*l).data as *mut GAppInfo;
                if g_strcmp0(def_type, g_app_info_get_id(app)) != 0i32 {
                    gtk_list_store_insert_with_values(
                        (*data).store,
                        &raw mut it,
                        pos as gint,
                        0i32,
                        g_app_info_get_icon(app),
                        1i32,
                        g_app_info_get_name(app),
                        2 as c_int,
                        app,
                        -(1i32),
                    );
                }
                g_object_unref(app as gpointer);
                l = (*l).next;
                pos += 1;
            }
            g_list_free(apps);
            gtk_tree_model_get_iter_first(
                (*data).store as *mut c_void as *mut GtkTreeModel,
                &raw mut it,
            );
            gtk_tree_selection_select_iter(gtk_tree_view_get_selection((*data).view), &raw mut it);
            tp = gtk_tree_path_new_first();
            gtk_tree_view_set_cursor(
                (*data).view,
                tp,
                ptr::null_mut::<GtkTreeViewColumn>(),
                0 as gboolean,
            );
            gtk_tree_path_free(tp);
        }
    }
    if mixed_content != 0 {
        gtk_label_set_text(
            (*data).type_0,
            dcgettext(ptr::null::<c_char>(), c"mixed content".as_ptr(), 5 as c_int),
        );
    } else if !desc.is_null() {
        if !(*data).dont_ask.is_null() {
            gtk_widget_show((*data).dont_ask as *mut c_void as *mut GtkWidget);
        }
        gtk_label_set_text((*data).type_0, desc);
        g_free(desc as gpointer);
    } else {
        gtk_label_set_text(
            (*data).type_0,
            dcgettext(
                ptr::null::<c_char>(),
                c"removable disk".as_ptr(),
                5 as c_int,
            ),
        );
    };
}
#[inline]
unsafe extern "C" fn show_autorun_dlg(mut vol: *mut GVolume, mut mount: *mut GMount) {
    let mut builder: *mut GtkBuilder = ptr::null_mut::<GtkBuilder>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut col: *mut GtkTreeViewColumn = ptr::null_mut::<GtkTreeViewColumn>();
    let mut tree_sel: *mut GtkTreeSelection = ptr::null_mut::<GtkTreeSelection>();
    let mut render: *mut GtkCellRenderer = ptr::null_mut::<GtkCellRenderer>();
    let mut icon: *mut GtkImage = ptr::null_mut::<GtkImage>();
    let mut gicon: *mut GIcon = ptr::null_mut::<GIcon>();
    let mut data: *mut AutoRun = ptr::null_mut::<AutoRun>();
    data = g_slice_alloc(mem::size_of::<AutoRun>() as gsize) as *mut AutoRun;
    (*data).cancel = g_cancellable_new();
    builder = gtk_builder_new();
    gtk_builder_add_from_file(
        builder,
        b"/usr/share/pcmanfm/ui/autorun.ui\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    );
    (*data).dlg = gtk_builder_get_object(builder, b"dlg\0" as *const u8 as *const gchar)
        as *mut c_void as *mut GtkDialog;
    (*data).view = gtk_builder_get_object(builder, b"listview\0" as *const u8 as *const gchar)
        as *mut c_void as *mut GtkTreeView;
    (*data).type_0 = gtk_builder_get_object(builder, b"type\0" as *const u8 as *const gchar)
        as *mut c_void as *mut GtkLabel;
    (*data).dont_ask = gtk_builder_get_object(builder, b"dont_ask\0" as *const u8 as *const gchar)
        as *mut GtkToggleButton;
    (*data).content_type = ptr::null_mut::<c_char>();
    (*data).skipped = 0 as gboolean;
    icon = gtk_builder_get_object(builder, b"icon\0" as *const u8 as *const gchar) as *mut c_void
        as *mut GtkImage;
    g_object_unref(builder as gpointer);
    gicon = g_volume_get_icon(vol);
    gtk_image_set_from_gicon(icon, gicon, GTK_ICON_SIZE_DIALOG);
    g_object_unref(gicon as gpointer);
    gtk_dialog_set_default_response((*data).dlg, GTK_RESPONSE_OK as c_int as gint);
    gtk_dialog_set_alternative_button_order(
        (*data).dlg,
        GTK_RESPONSE_OK as c_int as gint,
        GTK_RESPONSE_CANCEL as c_int,
        -(1i32),
    );
    tree_sel = gtk_tree_view_get_selection((*data).view);
    gtk_tree_selection_set_mode(tree_sel, GTK_SELECTION_BROWSE);
    col = gtk_tree_view_column_new();
    render = gtk_cell_renderer_pixbuf_new();
    gtk_tree_view_column_pack_start(col, render, 0 as gboolean);
    gtk_tree_view_column_set_attributes(
        col,
        render,
        c"gicon".as_ptr(),
        0i32,
        ptr::null_mut::<c_void>(),
    );
    render = gtk_cell_renderer_text_new();
    gtk_tree_view_column_pack_start(col, render, 0 as gboolean);
    gtk_tree_view_column_set_attributes(
        col,
        render,
        c"text".as_ptr(),
        1i32,
        ptr::null_mut::<c_void>(),
    );
    gtk_tree_view_append_column((*data).view, col);
    (*data).store = gtk_list_store_new(
        3 as gint,
        g_icon_get_type(),
        ((16 as c_int) << 2 as c_int) as GType,
        ((20i32) << 2 as c_int) as GType,
    );
    (*data).mount = g_object_ref(mount as gpointer) as *mut GMount;
    gtk_list_store_append((*data).store, &raw mut it);
    gicon = g_themed_icon_new(c"system-file-manager".as_ptr());
    gtk_list_store_set(
        (*data).store,
        &raw mut it,
        0i32,
        gicon,
        1i32,
        dcgettext(
            ptr::null::<c_char>(),
            c"Open in File Manager".as_ptr(),
            5 as c_int,
        ),
        -(1i32),
    );
    g_object_unref(gicon as gpointer);
    gtk_tree_view_set_model(
        (*data).view,
        (*data).store as *mut c_void as *mut GtkTreeModel,
    );
    gtk_tree_model_get_iter_first(
        (*data).store as *mut c_void as *mut GtkTreeModel,
        &raw mut it,
    );
    gtk_tree_selection_select_iter(tree_sel, &raw mut it);
    g_signal_connect_data(
        (*data).view as gpointer,
        b"row-activated\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkTreeView,
                    *mut GtkTreePath,
                    *mut GtkTreeViewColumn,
                    gpointer,
                ) -> (),
            >,
            GCallback,
        >(Some(
            on_row_activated
                as unsafe extern "C" fn(
                    *mut GtkTreeView,
                    *mut GtkTreePath,
                    *mut GtkTreeViewColumn,
                    gpointer,
                ) -> (),
        )),
        data as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*data).dlg as gpointer,
        b"response\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkDialog, c_int, gpointer) -> ()>,
            GCallback,
        >(Some(
            on_dlg_response as unsafe extern "C" fn(*mut GtkDialog, c_int, gpointer) -> (),
        )),
        data as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*data).mount as gpointer,
        b"unmounted\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GMount, *mut AutoRun) -> ()>, GCallback>(
            Some(on_unmount as unsafe extern "C" fn(*mut GMount, *mut AutoRun) -> ()),
        ),
        data as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    gtk_window_set_keep_above(
        (*data).dlg as *mut c_void as *mut GtkWindow,
        (0i32 == 0) as c_int,
    );
    gtk_window_present((*data).dlg as *mut c_void as *mut GtkWindow);
    g_mount_guess_content_type(
        mount,
        (0i32 == 0) as c_int,
        (*data).cancel,
        Some(
            on_content_type_finished
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        data as gpointer,
    );
    pcmanfm_ref();
}
#[inline]
unsafe extern "C" fn automount_volume(mut vol: *mut GVolume, mut silent: gboolean) -> gboolean {
    let mut mount: *mut GMount = ptr::null_mut::<GMount>();
    if g_volume_should_automount(vol) == 0 || g_volume_can_mount(vol) == 0 {
        return 0 as gboolean;
    }
    mount = g_volume_get_mount(vol);
    if mount.is_null() {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_DEBUG,
            b"try automount\0" as *const u8 as *const gchar,
        );
        if fm_mount_volume(ptr::null_mut::<GtkWindow>(), vol, (silent == 0) as c_int) == 0 {
            return 0 as gboolean;
        }
        if silent != 0 {
            return (0i32 == 0) as c_int;
        }
        mount = g_volume_get_mount(vol);
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_DEBUG,
            b"mount = %p\0" as *const u8 as *const gchar,
            mount,
        );
    }
    if !mount.is_null() {
        if silent == 0 && (*(fm_config as *mut FmAppConfig)).autorun != 0 {
            show_autorun_dlg(vol, mount);
        }
        g_object_unref(mount as gpointer);
    }
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn on_vol_added_timeout(mut vol: gpointer) -> gboolean {
    automount_volume(vol as *mut GVolume, 0 as gboolean);
    g_object_unref(vol);
    return 0 as gboolean;
}
unsafe extern "C" fn on_vol_added(
    mut vm: *mut GVolumeMonitor,
    mut vol: *mut GVolume,
    mut user_data: gpointer,
) {
    if (*(fm_config as *mut FmAppConfig)).mount_removable != 0 {
        gdk_threads_add_timeout(
            100 as guint,
            Some(on_vol_added_timeout as unsafe extern "C" fn(gpointer) -> gboolean),
            g_object_ref(vol as gpointer) as *mut GVolume as gpointer,
        );
    }
}
unsafe extern "C" fn fm_volume_manager_delay_init(mut user_data: gpointer) -> gboolean {
    let mut vols: *mut GList = ptr::null_mut::<GList>();
    let mut l: *mut GList = ptr::null_mut::<GList>();
    volume_manager_vol_mon = g_volume_monitor_get();
    if !(({
        let mut _g_boolean_var_66: c_int = 0i32;
        if volume_manager_vol_mon.is_null() {
            _g_boolean_var_66 = 1i32;
        }
        _g_boolean_var_66
    }) as c_long
        != 0)
    {
        g_signal_connect_data(
            volume_manager_vol_mon as gpointer,
            b"volume-added\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GVolume, gpointer) -> ()>,
                GCallback,
            >(Some(
                on_vol_added
                    as unsafe extern "C" fn(*mut GVolumeMonitor, *mut GVolume, gpointer) -> (),
            )),
            ptr::null_mut::<c_void>(),
            None,
            G_CONNECT_DEFAULT,
        );
        if (*(fm_config as *mut FmAppConfig)).mount_on_startup != 0 {
            vols = g_volume_monitor_get_volumes(volume_manager_vol_mon);
            l = vols;
            while !l.is_null() {
                let mut vol: *mut GVolume = (*l).data as *mut GVolume;
                if g_volume_should_automount(vol) != 0 {
                    automount_volume(vol, (0i32 == 0) as c_int);
                }
                g_object_unref(vol as gpointer);
                l = (*l).next;
            }
            g_list_free(vols);
        }
    }
    on_idle_handler = 0 as guint;
    return 0 as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn fm_volume_manager_init() {
    on_idle_handler = gdk_threads_add_idle_full(
        300 as gint,
        Some(fm_volume_manager_delay_init as unsafe extern "C" fn(gpointer) -> gboolean),
        ptr::null_mut::<c_void>(),
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fm_volume_manager_finalize() {
    if ({
        let mut _g_boolean_var_67: c_int = 0i32;
        if !volume_manager_vol_mon.is_null() {
            _g_boolean_var_67 = 1i32;
        }
        _g_boolean_var_67
    }) as c_long
        != 0
    {
        g_signal_handlers_disconnect_matched(
            volume_manager_vol_mon as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GVolume, gpointer) -> ()>,
                gpointer,
            >(Some(
                on_vol_added
                    as unsafe extern "C" fn(*mut GVolumeMonitor, *mut GVolume, gpointer) -> (),
            )),
            ptr::null_mut::<c_void>(),
        );
        g_object_unref(volume_manager_vol_mon as gpointer);
        volume_manager_vol_mon = ptr::null_mut::<GVolumeMonitor>();
    }
    if on_idle_handler != 0 {
        g_source_remove(on_idle_handler);
    }
    on_idle_handler = 0 as guint;
}
static mut tab_page_folder_menu_xml: [c_char; 133] = unsafe {
    mem::transmute::<
        [u8; 133],
        [c_char; 133],
    >(
        *b"<popup><placeholder name='ph1'><menuitem action='NewTab'/><menuitem action='NewWin'/><menuitem action='Term'/></placeholder></popup>\0",
    )
};
static mut tab_page_folder_menu_actions: [GtkActionEntry; 4] = unsafe {
    [
        _GtkActionEntry {
            name: b"NewTab\0" as *const u8 as *const gchar,
            stock_id: b"tab-new\0" as *const u8 as *const gchar,
            label: b"Open in New T_ab\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                tab_page_on_open_in_new_tab
                    as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"NewWin\0" as *const u8 as *const gchar,
            stock_id: c"gtk-new".as_ptr() as GtkStock as *const gchar,
            label: b"Open in New Win_dow\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                tab_page_on_open_in_new_win
                    as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Search\0" as *const u8 as *const gchar,
            stock_id: c"gtk-find".as_ptr() as GtkStock as *const gchar,
            label: ptr::null::<gchar>(),
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"Term\0" as *const u8 as *const gchar,
            stock_id: b"utilities-terminal\0" as *const u8 as *const gchar,
            label: b"Open in Termina_l\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                tab_page_on_open_folder_in_terminal
                    as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
    ]
};
