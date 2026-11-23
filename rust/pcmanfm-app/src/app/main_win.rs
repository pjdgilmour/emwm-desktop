// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      main-win.c
 *
 *      Copyright 2009 - 2012 Hong Jen Yee (PCMan) <pcman.tw@gmail.com>
 *      Copyright 2012-2018 Andriy Grytsenko (LStranger) <andrej@rep.kiev.ua>
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

unsafe extern "C" fn fm_main_win_class_intern_init(mut klass: gpointer) {
    fm_main_win_parent_class = g_type_class_peek_parent(klass);
    if FmMainWin_private_offset != 0i32 {
        g_type_class_adjust_private_offset(klass, &raw mut FmMainWin_private_offset);
    }
    fm_main_win_class_init(klass as *mut FmMainWinClass);
}
static mut fm_main_win_parent_class: gpointer = ptr::null::<c_void>() as *mut c_void;
static mut FmMainWin_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn fm_main_win_get_type() -> GType {
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
        let mut g_define_type_id: GType = fm_main_win_get_type_once();
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
unsafe extern "C" fn fm_main_win_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        gtk_window_get_type(),
        g_intern_static_string(b"FmMainWin\0" as *const u8 as *const gchar),
        mem::size_of::<FmMainWinClass>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(mem::transmute::<
            Option<unsafe extern "C" fn(gpointer) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            fm_main_win_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
        ))),
        mem::size_of::<FmMainWin>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmMainWin) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                fm_main_win_init as unsafe extern "C" fn(*mut FmMainWin) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut all_wins: *mut GSList = ptr::null::<GSList>() as *mut GSList;
static mut about_dlg: *mut GtkAboutDialog = ptr::null::<GtkAboutDialog>() as *mut GtkAboutDialog;
static mut key_nav_list_dlg: *mut GtkWidget = ptr::null::<GtkWidget>() as *mut GtkWidget;
static mut main_win_qdata: GQuark = 0;
unsafe extern "C" fn fm_main_win_class_init(mut klass: *mut FmMainWinClass) {
    let mut g_object_class: *mut GObjectClass = klass as *mut c_void as *mut GObjectClass;
    let mut widget_class: *mut GtkWidgetClass = klass as *mut c_void as *mut GtkWidgetClass;
    (*widget_class).destroy =
        Some(fm_main_win_destroy as unsafe extern "C" fn(*mut GtkWidget) -> ())
            as Option<unsafe extern "C" fn(*mut GtkWidget) -> ()>;
    g_object_class = klass as *mut c_void as *mut GObjectClass;
    (*g_object_class).finalize =
        Some(fm_main_win_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    widget_class = klass as *mut GtkWidgetClass;
    (*widget_class).focus_in_event = Some(
        main_win_on_focus_in
            as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventFocus) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventFocus) -> gboolean>;
    (*widget_class).key_press_event = Some(
        on_key_press_event as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventKey) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventKey) -> gboolean>;
    (*widget_class).button_press_event = Some(
        on_button_press_event
            as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventButton) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventButton) -> gboolean>;
    (*widget_class).scroll_event = Some(
        on_scroll_event as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventScroll) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventScroll) -> gboolean>;
    (*widget_class).unrealize = Some(on_unrealize as unsafe extern "C" fn(*mut GtkWidget) -> ())
        as Option<unsafe extern "C" fn(*mut GtkWidget) -> ()>;
    fm_main_win_parent_class =
        g_type_class_peek(gtk_window_get_type()) as *mut GtkWindowClass as gpointer;
    main_win_qdata = g_quark_from_static_string(b"FmMainWin::data\0" as *const u8 as *const gchar);
}
unsafe extern "C" fn idle_focus_view(mut user_data: gpointer) -> gboolean {
    let mut win: *mut FmMainWin = user_data as *mut FmMainWin;
    if g_source_is_destroyed(g_main_current_source()) != 0 {
        return 0 as gboolean;
    }
    if !(*win).folder_view.is_null() {
        gtk_widget_grab_focus((*win).folder_view as *mut c_void as *mut GtkWidget);
    }
    (*win).idle_handler = 0 as guint;
    return 0 as gboolean;
}
unsafe extern "C" fn on_location_activate(mut entry: *mut GtkEntry, mut win: *mut FmMainWin) {
    let mut path: *mut FmPath = fm_path_entry_get_path(entry as *mut c_void as *mut FmPathEntry);
    fm_path_ref(path);
    fm_main_win_chdir(win, path);
    fm_path_unref(path);
    if (*(fm_config as *mut FmAppConfig)).pathbar_mode_buttons != 0 {
        gtk_widget_hide((*win).location as *mut c_void as *mut GtkWidget);
        gtk_widget_hide(gtk_ui_manager_get_widget(
            (*win).ui,
            b"/toolbar/Go\0" as *const u8 as *const gchar,
        ));
        gtk_widget_show((*win).path_bar as *mut c_void as *mut GtkWidget);
    }
    if (*win).idle_handler == 0 as guint {
        (*win).idle_handler = gdk_threads_add_idle_full(
            300 as gint,
            Some(idle_focus_view as unsafe extern "C" fn(gpointer) -> gboolean),
            win as gpointer,
            None,
        );
    }
}
unsafe extern "C" fn on_path_bar_chdir(
    mut bar: *mut FmPathBar,
    mut path: *mut FmPath,
    mut win: *mut FmMainWin,
) {
    fm_main_win_chdir(win, path);
}
unsafe extern "C" fn on_path_bar_mode(
    mut act: *mut GtkRadioAction,
    mut cur: *mut GtkRadioAction,
    mut win: *mut FmMainWin,
) {
    let mut mode: c_int = gtk_radio_action_get_current_value(cur) as c_int;
    if (*(fm_config as *mut FmAppConfig)).pathbar_mode_buttons != mode {
        (*(fm_config as *mut FmAppConfig)).pathbar_mode_buttons = mode as gboolean;
        pcmanfm_save_config(0 as gboolean);
    }
    gtk_widget_set_visible(
        (*win).location as *mut c_void as *mut GtkWidget,
        (mode == 0i32) as c_int,
    );
    gtk_widget_set_visible(
        gtk_ui_manager_get_widget((*win).ui, b"/toolbar/Go\0" as *const u8 as *const gchar),
        (mode == 0i32) as c_int,
    );
    gtk_widget_set_visible(
        (*win).path_bar as *mut c_void as *mut GtkWidget,
        mode as gboolean,
    );
}
unsafe extern "C" fn update_sort_menu(mut win: *mut FmMainWin) {
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    let mut fv: *mut FmFolderView = (*win).folder_view;
    let mut type_0: GtkSortType = GTK_SORT_ASCENDING;
    let mut by: FmFolderModelCol = FM_FOLDER_MODEL_COL_GICON;
    let mut mode: FmSortMode = FM_SORT_ASCENDING;
    if (*win).in_update != 0 {
        return;
    }
    (*win).in_update = (0i32 == 0) as c_int as gboolean;
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/SavePerFolder\0" as *const u8 as *const gchar,
    );
    gtk_toggle_action_set_active(
        act as *mut c_void as *mut GtkToggleAction,
        (*(*win).current_page).own_config(),
    );
    (*win).in_update = 0 as gboolean;
    if fv.is_null() || fm_folder_view_get_model(fv).is_null() {
        return;
    }
    if fm_folder_model_get_sort(fm_folder_view_get_model(fv), &raw mut by, &raw mut mode) == 0 {
        return;
    }
    type_0 = (if mode as c_uint & FM_SORT_ORDER_MASK as c_int as c_uint
        == FM_SORT_ASCENDING as c_int as c_uint
    {
        GTK_SORT_ASCENDING as c_int
    } else {
        GTK_SORT_DESCENDING as c_int
    }) as GtkSortType;
    if mode as c_uint != (*(*win).current_page).sort_type as c_uint {
        (*(*win).current_page).sort_type = mode;
        if (*(*win).current_page).own_config() != 0 {
            fm_app_config_save_config_for_path(
                fm_folder_view_get_cwd(fv),
                mode,
                by,
                4294967295 as FmStandardViewMode,
                (*(*win).current_page).show_hidden(),
                ptr::null_mut::<*mut c_char>(),
            );
        } else {
            (*(fm_config as *mut FmAppConfig)).sort_type = mode;
            pcmanfm_save_config(0 as gboolean);
        }
    }
    (*win).in_update = (0i32 == 0) as c_int as gboolean;
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Sort/Asc\0" as *const u8 as *const gchar,
    );
    gtk_radio_action_set_current_value(act as *mut c_void as *mut GtkRadioAction, type_0 as gint);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Sort/ByName\0" as *const u8 as *const gchar,
    );
    if by as c_uint == 4294967295 as FmFolderModelCol as c_uint {
        by = FM_FOLDER_MODEL_COL_NAME;
    }
    gtk_radio_action_set_current_value(act as *mut c_void as *mut GtkRadioAction, by as gint);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Sort/SortIgnoreCase\0" as *const u8 as *const gchar,
    );
    gtk_toggle_action_set_active(
        act as *mut c_void as *mut GtkToggleAction,
        (mode as c_uint & FM_SORT_CASE_SENSITIVE as c_int as c_uint == 0 as c_uint) as c_int,
    );
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Sort/MingleDirs\0" as *const u8 as *const gchar,
    );
    gtk_toggle_action_set_active(
        act as *mut c_void as *mut GtkToggleAction,
        (mode as c_uint & FM_SORT_NO_FOLDER_FIRST as c_int as c_uint != 0 as c_uint) as c_int,
    );
    (*win).in_update = 0 as gboolean;
}
unsafe extern "C" fn update_view_menu(mut win: *mut FmMainWin) {
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    let mut fv: *mut FmFolderView = (*win).folder_view;
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/ShowHidden\0" as *const u8 as *const gchar,
    );
    if (*win).in_update != 0 {
        return;
    }
    (*win).in_update = (0i32 == 0) as c_int as gboolean;
    gtk_toggle_action_set_active(
        act as *mut c_void as *mut GtkToggleAction,
        fm_folder_view_get_show_hidden(fv),
    );
    gtk_radio_action_set_current_value(
        (*win).first_view_mode,
        fm_standard_view_get_mode(fv as *mut c_void as *mut FmStandardView) as gint,
    );
    (*win).in_update = 0 as gboolean;
}
unsafe extern "C" fn update_file_menu(mut win: *mut FmMainWin, mut path: *mut FmPath) {
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    let mut can_term: gboolean = pcmanfm_can_open_path_in_terminal(path);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ToolMenu/Term\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, (!path.is_null() && can_term != 0) as c_int);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ToolMenu/Launch\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, (!path.is_null() && can_term != 0) as c_int);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/GoMenu/Up\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(
        act,
        (!path.is_null() && !fm_path_get_parent(path).is_null()) as c_int,
    );
}
unsafe extern "C" fn on_folder_view_sort_changed(
    mut fv: *mut FmFolderView,
    mut win: *mut FmMainWin,
) {
    if fv != (*win).folder_view {
        return;
    }
    update_sort_menu(win);
}
unsafe extern "C" fn on_folder_view_sel_changed(
    mut fv: *mut FmFolderView,
    mut n_sel: gint,
    mut win: *mut FmMainWin,
) {
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    let mut has_selected: gboolean = (n_sel > 0i32) as c_int;
    if fv != (*win).folder_view {
        return;
    }
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/EditMenu/Open\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, has_selected);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/EditMenu/Cut\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, has_selected);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/EditMenu/Copy\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, has_selected);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/EditMenu/ToTrash\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, has_selected);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/EditMenu/Del\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, has_selected);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/EditMenu/CopyPath\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, has_selected);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/EditMenu/Link\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, has_selected);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/EditMenu/CopyTo\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, has_selected);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/EditMenu/MoveTo\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, has_selected);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/EditMenu/FileProp\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, has_selected);
    has_selected = 0 as gboolean;
    if n_sel == 1i32 {
        let mut files: *mut FmFileInfoList = fm_folder_view_dup_selected_files(fv);
        let mut fi: *mut FmFileInfo = fm_file_info_list_peek_head(files);
        if fm_file_info_can_set_name(fi) != 0 {
            if fm_file_info_is_shortcut(fi) == 0 && fm_file_info_is_desktop_entry(fi) == 0 {
                has_selected = (0i32 == 0) as c_int as gboolean;
            }
        }
        fm_file_info_list_unref(files);
    }
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/EditMenu/Rename\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, has_selected);
}
unsafe extern "C" fn on_view_key_press_event(
    mut fv: *mut FmFolderView,
    mut evt: *mut GdkEventKey,
    mut win: *mut FmMainWin,
) -> gboolean {
    match (*evt).keyval {
        65288 => {
            on_go_up(ptr::null_mut::<GtkAction>(), win);
        }
        _ => {}
    }
    return 0 as gboolean;
}
unsafe extern "C" fn on_bookmark(mut mi: *mut GtkMenuItem, mut win: *mut FmMainWin) {
    let mut path: *mut FmPath =
        g_object_get_qdata(mi as *mut c_void as *mut GObject, main_win_qdata) as *mut FmPath;
    match (*(fm_config as *mut FmAppConfig)).bm_open_method {
        0 => {
            fm_main_win_chdir(win, path);
        }
        1 => {
            fm_main_win_add_tab(win, path);
        }
        2 => {
            fm_main_win_add_win(win, path);
        }
        _ => {}
    };
}
unsafe extern "C" fn create_bookmarks_menu(mut win: *mut FmMainWin) {
    let mut list: *const GList = ptr::null::<GList>();
    let mut l: *const GList = ptr::null::<GList>();
    let mut mi: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
    let mut i: c_int = 0i32;
    list = fm_bookmarks_get_all((*win).bookmarks);
    l = list;
    while !l.is_null() {
        let mut item: *mut FmBookmarkItem = (*l).data as *mut FmBookmarkItem;
        mi = gtk_image_menu_item_new_with_label((*item).name);
        gtk_widget_show(mi);
        g_object_set_qdata_full(
            mi as *mut c_void as *mut GObject,
            main_win_qdata,
            fm_path_ref((*item).path) as gpointer,
            mem::transmute::<Option<unsafe extern "C" fn(*mut FmPath) -> ()>, GDestroyNotify>(
                Some(fm_path_unref as unsafe extern "C" fn(*mut FmPath) -> ()),
            ),
        );
        g_signal_connect_data(
            mi as gpointer,
            b"activate\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkMenuItem, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_bookmark as unsafe extern "C" fn(*mut GtkMenuItem, *mut FmMainWin) -> (),
            )),
            win as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        gtk_menu_shell_insert((*win).bookmarks_menu, mi, i as gint);
        i += 1;
        l = (*l).next;
    }
    g_list_free_full(
        list as *mut GList,
        mem::transmute::<Option<unsafe extern "C" fn(*mut FmBookmarkItem) -> ()>, GDestroyNotify>(
            Some(fm_bookmark_item_unref as unsafe extern "C" fn(*mut FmBookmarkItem) -> ()),
        ),
    );
    if i > 0i32 {
        mi = gtk_separator_menu_item_new();
        gtk_widget_show(mi);
        gtk_menu_shell_insert((*win).bookmarks_menu, mi, i as gint);
    }
}
unsafe extern "C" fn on_bookmarks_changed(mut bm: *mut FmBookmarks, mut win: *mut FmMainWin) {
    let mut mis: *mut GList =
        gtk_container_get_children((*win).bookmarks_menu as *mut c_void as *mut GtkContainer);
    let mut l: *mut GList = ptr::null_mut::<GList>();
    l = mis;
    while !l.is_null() {
        let mut item: *mut GtkWidget = (*l).data as *mut GtkWidget;
        if !g_object_get_qdata(item as *mut c_void as *mut GObject, main_win_qdata).is_null() {
            g_signal_handlers_disconnect_matched(
                item as gpointer,
                (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
                0 as guint,
                0 as GQuark,
                ptr::null_mut::<GClosure>(),
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkMenuItem, *mut FmMainWin) -> ()>,
                    gpointer,
                >(Some(
                    on_bookmark as unsafe extern "C" fn(*mut GtkMenuItem, *mut FmMainWin) -> (),
                )),
                win as gpointer,
            );
            gtk_widget_destroy(item);
            l = (*l).next;
        } else {
            if ({
                let mut __inst: *mut GTypeInstance = item as *mut GTypeInstance;
                let mut __t: GType = gtk_separator_menu_item_get_type();
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
                gtk_widget_destroy(item);
            }
            break;
        }
    }
    g_list_free(mis);
    create_bookmarks_menu(win);
}
unsafe extern "C" fn load_bookmarks(mut win: *mut FmMainWin, mut ui: *mut GtkUIManager) {
    let mut mi: *mut GtkWidget =
        gtk_ui_manager_get_widget(ui, b"/menubar/BookmarksMenu\0" as *const u8 as *const gchar);
    (*win).bookmarks_menu = gtk_menu_item_get_submenu(mi as *mut c_void as *mut GtkMenuItem)
        as *mut c_void as *mut GtkMenuShell;
    (*win).bookmarks = fm_bookmarks_dup();
    g_signal_connect_data(
        (*win).bookmarks as gpointer,
        b"changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmBookmarks, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_bookmarks_changed as unsafe extern "C" fn(*mut FmBookmarks, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    create_bookmarks_menu(win);
}
unsafe extern "C" fn on_history_item(mut mi: *mut GtkMenuItem, mut win: *mut FmMainWin) {
    let mut page: *mut FmTabPage = (*win).current_page;
    let mut l: guint =
        g_object_get_qdata(mi as *mut c_void as *mut GObject, main_win_qdata) as gulong as guint;
    fm_tab_page_history(page, l);
    _update_hist_buttons(win);
}
unsafe extern "C" fn disconnect_history_item(mut mi: *mut GtkWidget, mut win: gpointer) {
    g_signal_handlers_disconnect_matched(
        mi as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkMenuItem, *mut FmMainWin) -> ()>,
            gpointer,
        >(Some(
            on_history_item as unsafe extern "C" fn(*mut GtkMenuItem, *mut FmMainWin) -> (),
        )),
        win,
    );
    gtk_widget_destroy(mi);
}
unsafe extern "C" fn on_history_selection_done(mut menu: *mut GtkMenuShell, mut win: gpointer) {
    g_log(
        ptr::null_mut::<gchar>(),
        G_LOG_LEVEL_DEBUG,
        b"history selection done\0" as *const u8 as *const gchar,
    );
    g_signal_handlers_disconnect_matched(
        menu as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkMenuShell, gpointer) -> ()>, gpointer>(
            Some(
                on_history_selection_done
                    as unsafe extern "C" fn(*mut GtkMenuShell, gpointer) -> (),
            ),
        ),
        ptr::null_mut::<c_void>(),
    );
    gtk_container_foreach(
        menu as *mut c_void as *mut GtkContainer,
        Some(disconnect_history_item as unsafe extern "C" fn(*mut GtkWidget, gpointer) -> ()),
        win,
    );
}
unsafe extern "C" fn on_show_history_menu(mut btn: *mut FmMenuToolItem, mut win: *mut FmMainWin) {
    let mut menu: *mut GtkMenuShell = fm_menu_tool_item_get_menu(btn) as *mut GtkMenuShell;
    let mut i: guint = 0;
    let mut cur: guint = fm_nav_history_get_cur_index((*win).nav_history);
    let mut path: *mut FmPath = ptr::null_mut::<FmPath>();
    gtk_container_foreach(
        menu as *mut c_void as *mut GtkContainer,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkWidget) -> ()>, GtkCallback>(Some(
            gtk_widget_destroy as unsafe extern "C" fn(*mut GtkWidget) -> (),
        )),
        ptr::null_mut::<c_void>(),
    );
    i = 0 as guint;
    loop {
        path = fm_nav_history_get_nth_path((*win).nav_history, i);
        if path.is_null() {
            break;
        }
        let mut str: *mut c_char = fm_path_display_name(path, (0i32 == 0) as c_int);
        let mut mi: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
        if i == cur {
            mi = gtk_check_menu_item_new_with_label(str);
            gtk_check_menu_item_set_draw_as_radio(
                mi as *mut c_void as *mut GtkCheckMenuItem,
                (0i32 == 0) as c_int,
            );
            gtk_check_menu_item_set_active(
                mi as *mut c_void as *mut GtkCheckMenuItem,
                (0i32 == 0) as c_int,
            );
        } else {
            mi = gtk_menu_item_new_with_label(str);
        }
        g_free(str as gpointer);
        g_object_set_qdata(
            mi as *mut c_void as *mut GObject,
            main_win_qdata,
            i as gulong as gpointer,
        );
        g_signal_connect_data(
            mi as gpointer,
            b"activate\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkMenuItem, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_history_item as unsafe extern "C" fn(*mut GtkMenuItem, *mut FmMainWin) -> (),
            )),
            win as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        gtk_menu_shell_append(menu, mi);
        i = i.wrapping_add(1);
    }
    g_signal_connect_data(
        menu as gpointer,
        b"selection-done\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkMenuShell, gpointer) -> ()>, GCallback>(
            Some(
                on_history_selection_done
                    as unsafe extern "C" fn(*mut GtkMenuShell, gpointer) -> (),
            ),
        ),
        ptr::null_mut::<c_void>(),
        None,
        G_CONNECT_DEFAULT,
    );
    gtk_widget_show_all(menu as *mut c_void as *mut GtkWidget);
}
unsafe extern "C" fn on_tab_page_splitter_pos_changed(
    mut paned: *mut GtkPaned,
    mut ps: *mut GParamSpec,
    mut win: *mut FmMainWin,
) {
    let mut children: *mut GList = ptr::null_mut::<GList>();
    let mut child: *mut GList = ptr::null_mut::<GList>();
    if paned != (*win).current_page as *mut GtkPaned {
        return;
    }
    (*(fm_config as *mut FmAppConfig)).splitter_pos = gtk_paned_get_position(paned) as c_int;
    pcmanfm_save_config(0 as gboolean);
    children = gtk_container_get_children((*win).notebook as *mut c_void as *mut GtkContainer);
    child = children;
    while !child.is_null() {
        let mut page: *mut FmTabPage = (*child).data as *mut FmTabPage;
        if page as *mut GtkPaned != paned {
            gtk_paned_set_position(
                page as *mut c_void as *mut GtkPaned,
                (*(fm_config as *mut FmAppConfig)).splitter_pos as gint,
            );
        }
        child = (*child).next;
    }
    g_list_free(children);
}
unsafe extern "C" fn on_side_pane_chdir(
    mut sp: *mut FmSidePane,
    mut button: guint,
    mut path: *mut FmPath,
    mut win: *mut FmMainWin,
) {
    if sp != (*win).side_pane {
        return;
    }
    if button == 2 as guint {
        let mut prev_path: *mut FmPath = ptr::null_mut::<FmPath>();
        if !(*win).folder_view.is_null() {
            prev_path = fm_folder_view_get_cwd((*win).folder_view);
        }
        fm_main_win_add_tab(win, path);
        if !prev_path.is_null() {
            fm_side_pane_chdir(sp, prev_path);
        }
    } else {
        fm_main_win_chdir(win, path);
    }
    if !(*win).folder_view.is_null() {
        gtk_widget_grab_focus((*win).folder_view as *mut c_void as *mut GtkWidget);
    }
}
unsafe extern "C" fn on_side_pane_mode_changed(mut sp: *mut FmSidePane, mut win: *mut FmMainWin) {
    let mut mode: FmSidePaneMode = FM_SP_NONE;
    if sp != (*win).side_pane {
        return;
    }
    mode = fm_side_pane_get_mode(sp);
    gtk_radio_action_set_current_value((*win).first_side_pane_mode, mode as gint);
    if mode as c_uint
        != (*(fm_config as *mut FmAppConfig)).side_pane_mode as c_uint & 0x1f as c_uint
    {
        (*(fm_config as *mut FmAppConfig)).side_pane_mode = (mode as c_uint
            | (*(fm_config as *mut FmAppConfig)).side_pane_mode as c_uint
                & !(0x1f as c_int) as c_uint)
            as FmSidePaneMode;
        fm_config_emit_changed(
            fm_config as *mut FmAppConfig as *mut c_void as *mut FmConfig,
            c"side_pane_mode".as_ptr(),
        );
        pcmanfm_save_config(0 as gboolean);
    }
}
unsafe extern "C" fn on_always_show_tabs_changed(
    mut cfg: *mut FmAppConfig,
    mut win: *mut FmMainWin,
) {
    if gtk_notebook_get_n_pages((*win).notebook) == 1i32 {
        gtk_notebook_set_show_tabs(
            (*win).notebook,
            (*(fm_config as *mut FmAppConfig)).always_show_tabs,
        );
    }
}
unsafe extern "C" fn on_toolsbar_changed(mut cfg: *mut FmAppConfig, mut win: *mut FmMainWin) {
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    let mut toolitem: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
    let mut n: c_int = 0;
    let mut active: gboolean = 0;
    if (*win).in_update != 0 {
        return;
    }
    (*win).in_update = (0i32 == 0) as c_int as gboolean;
    active = (*cfg).tb.visible();
    gtk_widget_set_visible((*win).toolbar as *mut c_void as *mut GtkWidget, active);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Toolbar/ToolbarNewWin\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, active);
    if gtk_toggle_action_get_active(act as *mut c_void as *mut GtkToggleAction)
        != (*(fm_config as *mut FmAppConfig)).tb.new_win()
    {
        gtk_toggle_action_set_active(
            act as *mut c_void as *mut GtkToggleAction,
            (*(fm_config as *mut FmAppConfig)).tb.new_win(),
        );
    }
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Toolbar/ToolbarNewTab\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, active);
    if gtk_toggle_action_get_active(act as *mut c_void as *mut GtkToggleAction)
        != (*(fm_config as *mut FmAppConfig)).tb.new_tab()
    {
        gtk_toggle_action_set_active(
            act as *mut c_void as *mut GtkToggleAction,
            (*(fm_config as *mut FmAppConfig)).tb.new_tab(),
        );
    }
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Toolbar/ToolbarNav\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, active);
    if gtk_toggle_action_get_active(act as *mut c_void as *mut GtkToggleAction)
        != (*(fm_config as *mut FmAppConfig)).tb.nav()
    {
        gtk_toggle_action_set_active(
            act as *mut c_void as *mut GtkToggleAction,
            (*(fm_config as *mut FmAppConfig)).tb.nav(),
        );
    }
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Toolbar/ToolbarHome\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, active);
    if gtk_toggle_action_get_active(act as *mut c_void as *mut GtkToggleAction)
        != (*(fm_config as *mut FmAppConfig)).tb.home()
    {
        gtk_toggle_action_set_active(
            act as *mut c_void as *mut GtkToggleAction,
            (*(fm_config as *mut FmAppConfig)).tb.home(),
        );
    }
    if active != 0 {
        toolitem =
            gtk_ui_manager_get_widget((*win).ui, b"/toolbar/New\0" as *const u8 as *const gchar);
        gtk_widget_set_visible(toolitem, (*cfg).tb.new_win());
        toolitem =
            gtk_ui_manager_get_widget((*win).ui, b"/toolbar/NewTab\0" as *const u8 as *const gchar);
        gtk_widget_set_visible(toolitem, (*cfg).tb.new_tab());
        active = (*cfg).tb.nav();
        toolitem =
            gtk_ui_manager_get_widget((*win).ui, b"/toolbar/Next\0" as *const u8 as *const gchar);
        gtk_widget_set_visible(toolitem, active);
        n = gtk_toolbar_get_item_index((*win).toolbar, toolitem as *mut c_void as *mut GtkToolItem)
            as c_int;
        toolitem = gtk_toolbar_get_nth_item((*win).toolbar, n as gint - 1 as gint) as *mut c_void
            as *mut GtkWidget;
        gtk_widget_set_visible(toolitem, active);
        toolitem = gtk_toolbar_get_nth_item((*win).toolbar, n as gint - 2 as gint) as *mut c_void
            as *mut GtkWidget;
        gtk_widget_set_visible(toolitem, active);
        toolitem = gtk_toolbar_get_nth_item((*win).toolbar, n as gint + 1 as gint) as *mut c_void
            as *mut GtkWidget;
        gtk_widget_set_visible(toolitem, active);
        toolitem =
            gtk_ui_manager_get_widget((*win).ui, b"/toolbar/Home\0" as *const u8 as *const gchar);
        gtk_widget_set_visible(toolitem, (*cfg).tb.home());
    }
    (*win).in_update = 0 as gboolean;
}
unsafe extern "C" fn on_statusbar_changed(mut cfg: *mut FmAppConfig, mut win: *mut FmMainWin) {
    gtk_widget_set_visible(
        (*win).statusbar as *mut c_void as *mut GtkWidget,
        (*cfg).show_statusbar,
    );
    update_statusbar(win);
}
static mut settings_hack_done: gboolean = 0 as gboolean;
static mut old_gtk_timeout_expand: gint = 0 as gint;
unsafe extern "C" fn on_change_tab_on_drop_changed(
    mut cfg: *mut FmAppConfig,
    mut _unused: gpointer,
) {
    if (*(fm_config as *mut FmAppConfig)).change_tab_on_drop != 0 {
        g_object_set(
            gtk_settings_get_default() as gpointer,
            b"gtk-timeout-expand\0" as *const u8 as *const gchar,
            old_gtk_timeout_expand,
            ptr::null_mut::<c_void>(),
        );
    } else {
        g_object_set(
            gtk_settings_get_default() as gpointer,
            b"gtk-timeout-expand\0" as *const u8 as *const gchar,
            600000i32,
            ptr::null_mut::<c_void>(),
        );
    };
}
unsafe extern "C" fn _do_settings_hack() {
    let mut settings: *mut GtkSettings = ptr::null_mut::<GtkSettings>();
    if settings_hack_done != 0 {
        return;
    }
    settings_hack_done = (0i32 == 0) as c_int as gboolean;
    settings = gtk_settings_get_default();
    g_object_get(
        settings as gpointer,
        b"gtk-timeout-expand\0" as *const u8 as *const gchar,
        &raw mut old_gtk_timeout_expand,
        ptr::null_mut::<c_void>(),
    );
    if (*(fm_config as *mut FmAppConfig)).change_tab_on_drop == 0 {
        g_object_set(
            settings as gpointer,
            b"gtk-timeout-expand\0" as *const u8 as *const gchar,
            600000i32,
            ptr::null_mut::<c_void>(),
        );
    }
    g_signal_connect_data(
        fm_config as *mut FmAppConfig as gpointer,
        b"changed::change_tab_on_drop\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut FmAppConfig, gpointer) -> ()>, GCallback>(
            Some(
                on_change_tab_on_drop_changed
                    as unsafe extern "C" fn(*mut FmAppConfig, gpointer) -> (),
            ),
        ),
        ptr::null_mut::<c_void>(),
        None,
        G_CONNECT_DEFAULT,
    );
}
unsafe extern "C" fn fm_main_win_init(mut win: *mut FmMainWin) {
    let mut vbox: *mut GtkBox = ptr::null_mut::<GtkBox>();
    let mut menubar: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
    let mut pathbox: *mut GtkBox = ptr::null_mut::<GtkBox>();
    let mut toolitem: *mut GtkToolItem = ptr::null_mut::<GtkToolItem>();
    let mut ui: *mut GtkUIManager = ptr::null_mut::<GtkUIManager>();
    let mut act_grp: *mut GtkActionGroup = ptr::null_mut::<GtkActionGroup>();
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    let mut accel_grp: *mut GtkAccelGroup = ptr::null_mut::<GtkAccelGroup>();
    let mut atk_obj: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut atk_view: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut relation: *mut AtkRelation = ptr::null_mut::<AtkRelation>();
    let mut mode_action: *mut GtkRadioAction = ptr::null_mut::<GtkRadioAction>();
    let mut radio_group: *mut GSList = ptr::null_mut::<GSList>();
    let mut str: *mut GString = ptr::null_mut::<GString>();
    let mut xml: *mut GString = ptr::null_mut::<GString>();
    static mut accel_str: [c_char; 8] =
        unsafe { mem::transmute::<[u8; 8], [c_char; 8]>(*b"<Ctrl>1\0") };
    let mut i: c_int = 0;
    let mut is_first: gboolean = 0;
    let mut shadow_type: GtkShadowType = GTK_SHADOW_NONE;
    pcmanfm_ref();
    all_wins = g_slist_prepend(all_wins, win as gpointer);
    (*win).win_group = gtk_window_group_new();
    gtk_window_group_add_window((*win).win_group, win as *mut c_void as *mut GtkWindow);
    gtk_window_set_icon_name(
        win as *mut c_void as *mut GtkWindow,
        b"folder\0" as *const u8 as *const gchar,
    );
    vbox = gtk_vbox_new(0 as gboolean, 0 as gint) as *mut GtkBox;
    ui = gtk_ui_manager_new();
    act_grp = gtk_action_group_new(b"Main\0" as *const u8 as *const gchar);
    gtk_action_group_set_translation_domain(act_grp, ptr::null::<gchar>());
    gtk_action_group_add_actions(
        act_grp,
        &raw mut main_win_actions as *mut GtkActionEntry,
        (mem::size_of::<[GtkActionEntry; 66]>() as usize)
            .wrapping_div(mem::size_of::<GtkActionEntry>() as usize) as guint,
        win as gpointer,
    );
    gtk_action_group_add_toggle_actions(
        act_grp,
        &raw mut main_win_toggle_actions as *mut GtkToggleActionEntry,
        (mem::size_of::<[GtkToggleActionEntry; 13]>() as usize)
            .wrapping_div(mem::size_of::<GtkToggleActionEntry>() as usize) as guint,
        win as gpointer,
    );
    radio_group = ptr::null_mut::<GSList>();
    is_first = (0i32 == 0) as c_int as gboolean;
    str = g_string_new(b"ViewMode:\0" as *const u8 as *const gchar);
    xml = g_string_new(
        b"<menubar><menu action='ViewMenu'><menu action='FolderView'><placeholder name='ViewModes'>\0"
            as *const u8 as *const gchar,
    );
    accel_str[6 as usize] = '1' as i32 as c_char;
    i = 0i32;
    while i < fm_standard_view_get_n_modes() {
        if !fm_standard_view_get_mode_label(i as FmStandardViewMode).is_null() {
            if 0 != 0 {
                ({
                    let __val: *const c_char =
                        fm_standard_view_mode_to_str(i as FmStandardViewMode) as *const c_char;
                    g_string_append_len_inline(
                        str,
                        __val,
                        if ({
                            let mut _g_boolean_var_50: c_int = 0 as c_int;
                            if !__val.is_null() {
                                _g_boolean_var_50 = 1i32;
                            }
                            _g_boolean_var_50
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
                g_string_append_len_inline(
                    str,
                    fm_standard_view_mode_to_str(i as FmStandardViewMode),
                    -(1i32) as gssize,
                );
            };
            mode_action = gtk_radio_action_new(
                (*str).str_0,
                fm_standard_view_get_mode_label(i as FmStandardViewMode) as *const gchar,
                fm_standard_view_get_mode_tooltip(i as FmStandardViewMode) as *const gchar,
                fm_standard_view_get_mode_icon(i as FmStandardViewMode) as *const gchar,
                i as gint,
            );
            gtk_radio_action_set_group(mode_action, radio_group);
            radio_group = gtk_radio_action_get_group(mode_action);
            gtk_action_group_add_action_with_accel(
                act_grp,
                mode_action as *mut c_void as *mut GtkAction,
                &raw mut accel_str as *mut c_char,
            );
            if is_first != 0 {
                (*win).first_view_mode = mode_action;
                g_signal_connect_data(
                    mode_action as gpointer,
                    b"changed\0" as *const u8 as *const gchar,
                    mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut GtkRadioAction,
                                *mut GtkRadioAction,
                                *mut FmMainWin,
                            ) -> (),
                        >,
                        GCallback,
                    >(Some(
                        on_change_mode
                            as unsafe extern "C" fn(
                                *mut GtkRadioAction,
                                *mut GtkRadioAction,
                                *mut FmMainWin,
                            ) -> (),
                    )),
                    win as gpointer,
                    None,
                    G_CONNECT_DEFAULT,
                );
            }
            is_first = 0 as gboolean;
            g_object_unref(mode_action as gpointer);
            g_string_append_printf(
                xml,
                b"<menuitem action='%s'/>\0" as *const u8 as *const gchar,
                (*str).str_0,
            );
            accel_str[6 as usize] += 1;
            g_string_truncate_inline(str, 9 as gsize);
        }
        i += 1;
    }
    if 0 != 0 {
        ({
            let __val: *const c_char = c"</placeholder></menu>".as_ptr();
            g_string_append_len_inline(
                xml,
                __val,
                if ({
                    let mut _g_boolean_var_51: c_int = 0 as c_int;
                    if !__val.is_null() {
                        _g_boolean_var_51 = 1i32;
                    }
                    _g_boolean_var_51
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
        g_string_append_len_inline(xml, c"</placeholder></menu>".as_ptr(), -(1i32) as gssize);
    };
    gtk_action_group_add_radio_actions(
        act_grp,
        &raw mut main_win_sort_type_actions as *mut GtkRadioActionEntry,
        (mem::size_of::<[GtkRadioActionEntry; 2]>() as usize)
            .wrapping_div(mem::size_of::<GtkRadioActionEntry>() as usize) as guint,
        if (*(fm_config as *mut FmAppConfig)).sort_type as c_uint
            & FM_SORT_ORDER_MASK as c_int as c_uint
            == FM_SORT_ASCENDING as c_int as c_uint
        {
            GTK_SORT_ASCENDING as c_int as gint
        } else {
            GTK_SORT_DESCENDING as c_int as gint
        },
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkRadioAction,
                    *mut GtkRadioAction,
                    *mut FmMainWin,
                ) -> (),
            >,
            GCallback,
        >(Some(
            on_sort_type
                as unsafe extern "C" fn(
                    *mut GtkRadioAction,
                    *mut GtkRadioAction,
                    *mut FmMainWin,
                ) -> (),
        )),
        win as gpointer,
    );
    gtk_action_group_add_radio_actions(
        act_grp,
        &raw mut main_win_sort_by_actions as *mut GtkRadioActionEntry,
        (mem::size_of::<[GtkRadioActionEntry; 5]>() as usize)
            .wrapping_div(mem::size_of::<GtkRadioActionEntry>() as usize) as guint,
        (*(fm_config as *mut FmAppConfig)).sort_by as gint,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkRadioAction,
                    *mut GtkRadioAction,
                    *mut FmMainWin,
                ) -> (),
            >,
            GCallback,
        >(Some(
            on_sort_by
                as unsafe extern "C" fn(
                    *mut GtkRadioAction,
                    *mut GtkRadioAction,
                    *mut FmMainWin,
                ) -> (),
        )),
        win as gpointer,
    );
    radio_group = ptr::null_mut::<GSList>();
    is_first = (0i32 == 0) as c_int as gboolean;
    g_string_assign(str, b"SidePaneMode:\0" as *const u8 as *const gchar);
    if 0 != 0 {
        ({
            let __val: *const c_char =
                c"<menu action='SidePane'><placeholder name='SidePaneModes'>".as_ptr();
            g_string_append_len_inline(
                xml,
                __val,
                if ({
                    let mut _g_boolean_var_52: c_int = 0 as c_int;
                    if !__val.is_null() {
                        _g_boolean_var_52 = 1i32;
                    }
                    _g_boolean_var_52
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
        g_string_append_len_inline(
            xml,
            c"<menu action='SidePane'><placeholder name='SidePaneModes'>".as_ptr(),
            -(1i32) as gssize,
        );
    };
    accel_str[6 as usize] = '6' as i32 as c_char;
    i = 1i32;
    while i <= fm_side_pane_get_n_modes() {
        if !fm_side_pane_get_mode_label(i as FmSidePaneMode).is_null() {
            if 0 != 0 {
                ({
                    let __val: *const c_char =
                        fm_side_pane_get_mode_name(i as FmSidePaneMode) as *const c_char;
                    g_string_append_len_inline(
                        str,
                        __val,
                        if ({
                            let mut _g_boolean_var_53: c_int = 0 as c_int;
                            if !__val.is_null() {
                                _g_boolean_var_53 = 1i32;
                            }
                            _g_boolean_var_53
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
                g_string_append_len_inline(
                    str,
                    fm_side_pane_get_mode_name(i as FmSidePaneMode),
                    -(1i32) as gssize,
                );
            };
            mode_action = gtk_radio_action_new(
                (*str).str_0,
                fm_side_pane_get_mode_label(i as FmSidePaneMode) as *const gchar,
                fm_side_pane_get_mode_tooltip(i as FmSidePaneMode) as *const gchar,
                ptr::null::<gchar>(),
                i as gint,
            );
            gtk_radio_action_set_group(mode_action, radio_group);
            radio_group = gtk_radio_action_get_group(mode_action);
            gtk_action_group_add_action_with_accel(
                act_grp,
                mode_action as *mut c_void as *mut GtkAction,
                &raw mut accel_str as *mut c_char,
            );
            if is_first != 0 {
                (*win).first_side_pane_mode = mode_action;
                g_signal_connect_data(
                    mode_action as gpointer,
                    b"changed\0" as *const u8 as *const gchar,
                    mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut GtkRadioAction,
                                *mut GtkRadioAction,
                                *mut FmMainWin,
                            ) -> (),
                        >,
                        GCallback,
                    >(Some(
                        on_side_pane_mode
                            as unsafe extern "C" fn(
                                *mut GtkRadioAction,
                                *mut GtkRadioAction,
                                *mut FmMainWin,
                            ) -> (),
                    )),
                    win as gpointer,
                    None,
                    G_CONNECT_DEFAULT,
                );
            }
            is_first = 0 as gboolean;
            g_object_unref(mode_action as gpointer);
            g_string_append_printf(
                xml,
                b"<menuitem action='%s'/>\0" as *const u8 as *const gchar,
                (*str).str_0,
            );
            accel_str[6 as usize] += 1;
            g_string_truncate_inline(str, 13 as gsize);
        }
        i += 1;
    }
    gtk_radio_action_set_current_value(
        (*win).first_side_pane_mode,
        ((*(fm_config as *mut FmAppConfig)).side_pane_mode as c_uint & 0x1f as c_uint) as gint,
    );
    if 0 != 0 {
        ({
            let __val: *const c_char = c"</placeholder></menu></menu></menubar>".as_ptr();
            g_string_append_len_inline(
                xml,
                __val,
                if ({
                    let mut _g_boolean_var_54: c_int = 0 as c_int;
                    if !__val.is_null() {
                        _g_boolean_var_54 = 1i32;
                    }
                    _g_boolean_var_54
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
        g_string_append_len_inline(
            xml,
            c"</placeholder></menu></menu></menubar>".as_ptr(),
            -(1i32) as gssize,
        );
    };
    if 0 != 0 {
        if 0i32 == 0 {
            g_string_free(str, (0i32 == 0) as c_int);
        } else {
            g_string_free_and_steal(str);
        };
    } else {
        g_string_free(str, (0i32 == 0) as c_int);
    };
    gtk_action_group_add_radio_actions(
        act_grp,
        &raw mut main_win_path_bar_mode_actions as *mut GtkRadioActionEntry,
        (mem::size_of::<[GtkRadioActionEntry; 2]>() as usize)
            .wrapping_div(mem::size_of::<GtkRadioActionEntry>() as usize) as guint,
        0 as gint,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkRadioAction,
                    *mut GtkRadioAction,
                    *mut FmMainWin,
                ) -> (),
            >,
            GCallback,
        >(Some(
            on_path_bar_mode
                as unsafe extern "C" fn(
                    *mut GtkRadioAction,
                    *mut GtkRadioAction,
                    *mut FmMainWin,
                ) -> (),
        )),
        win as gpointer,
    );
    accel_grp = gtk_ui_manager_get_accel_group(ui);
    gtk_window_add_accel_group(win as *mut c_void as *mut GtkWindow, accel_grp);
    gtk_ui_manager_insert_action_group(ui, act_grp, 0 as gint);
    gtk_ui_manager_add_ui_from_string(
        ui,
        &raw const main_menu_xml as *const gchar,
        -(1i32) as gssize,
        ptr::null_mut::<*mut GError>(),
    );
    gtk_ui_manager_add_ui_from_string(
        ui,
        (*xml).str_0,
        (*xml).len as gssize,
        ptr::null_mut::<*mut GError>(),
    );
    if 0 != 0 {
        if 0i32 == 0 {
            g_string_free(xml, (0i32 == 0) as c_int);
        } else {
            g_string_free_and_steal(xml);
        };
    } else {
        g_string_free(xml, (0i32 == 0) as c_int);
    };
    act = gtk_ui_manager_get_action(
        ui,
        b"/menubar/ViewMenu/SidePane/ShowSidePane\0" as *const u8 as *const gchar,
    );
    gtk_toggle_action_set_active(
        act as *mut c_void as *mut GtkToggleAction,
        ((*(fm_config as *mut FmAppConfig)).side_pane_mode as c_uint & 0x20 as c_uint
            == 0 as c_uint) as c_int,
    );
    if fm_module_is_in_use(c"vfs".as_ptr(), c"search".as_ptr()) == 0 {
        act = gtk_ui_manager_get_action(
            ui,
            b"/menubar/ToolMenu/Search\0" as *const u8 as *const gchar,
        );
        gtk_action_set_sensitive(act, 0 as gboolean);
    }
    if fm_module_is_in_use(c"vfs".as_ptr(), c"menu".as_ptr()) == 0 {
        act = gtk_ui_manager_get_action(ui, b"/menubar/GoMenu/Apps\0" as *const u8 as *const gchar);
        gtk_action_set_sensitive(act, 0 as gboolean);
    }
    menubar = gtk_ui_manager_get_widget(ui, b"/menubar\0" as *const u8 as *const gchar);
    (*win).toolbar = gtk_ui_manager_get_widget(ui, b"/toolbar\0" as *const u8 as *const gchar)
        as *mut c_void as *mut GtkToolbar;
    gtk_toolbar_set_icon_size((*win).toolbar, GTK_ICON_SIZE_SMALL_TOOLBAR);
    gtk_toolbar_set_style((*win).toolbar, GTK_TOOLBAR_ICONS);
    toolitem = fm_menu_tool_item_new();
    gtk_toolbar_insert((*win).toolbar, toolitem, 3 as gint);
    (*win).history_menu = gtk_menu_new();
    fm_menu_tool_item_set_menu(
        toolitem as *mut c_void as *mut FmMenuToolItem,
        (*win).history_menu,
    );
    gtk_tool_item_set_tooltip_text(
        toolitem,
        dcgettext(
            ptr::null::<c_char>(),
            c"Show history of visited folders".as_ptr(),
            5 as c_int,
        ),
    );
    g_signal_connect_data(
        toolitem as gpointer,
        b"show-menu\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmMenuToolItem, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_show_history_menu as unsafe extern "C" fn(*mut FmMenuToolItem, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    atk_obj = gtk_widget_get_accessible(toolitem as *mut c_void as *mut GtkWidget);
    if !atk_obj.is_null() {
        atk_object_set_name(
            atk_obj,
            dcgettext(ptr::null::<c_char>(), c"History".as_ptr(), 5 as c_int),
        );
    }
    gtk_box_pack_start(
        vbox,
        menubar,
        0 as gboolean,
        (0i32 == 0) as c_int,
        0 as guint,
    );
    gtk_box_pack_start(
        vbox,
        (*win).toolbar as *mut c_void as *mut GtkWidget,
        0 as gboolean,
        (0i32 == 0) as c_int,
        0 as guint,
    );
    load_bookmarks(win, ui);
    (*win).location = fm_path_entry_new();
    g_signal_connect_data(
        (*win).location as gpointer,
        b"activate\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkEntry, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_location_activate as unsafe extern "C" fn(*mut GtkEntry, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    if geteuid() == 0 as __uid_t {
        let mut warning: *mut GtkWidget = gtk_image_new_from_stock(
            c"gtk-dialog-warning".as_ptr() as GtkStock as *const gchar,
            GTK_ICON_SIZE_SMALL_TOOLBAR,
        );
        gtk_widget_set_tooltip_markup(
            warning,
            dcgettext(
                ptr::null::<c_char>(),
                c"You are in super user mode".as_ptr(),
                5 as c_int,
            ),
        );
        toolitem = gtk_tool_item_new();
        gtk_container_add(toolitem as *mut c_void as *mut GtkContainer, warning);
        gtk_toolbar_insert((*win).toolbar, gtk_separator_tool_item_new(), 0 as gint);
        gtk_toolbar_insert((*win).toolbar, toolitem, 0 as gint);
    }
    (*win).path_bar = fm_path_bar_new();
    g_signal_connect_data(
        (*win).path_bar as gpointer,
        b"chdir\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmPathBar, *mut FmPath, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_path_bar_chdir
                as unsafe extern "C" fn(*mut FmPathBar, *mut FmPath, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    pathbox = gtk_hbox_new(0 as gboolean, 0 as gint) as *mut GtkBox;
    gtk_box_pack_start(
        pathbox,
        (*win).location as *mut c_void as *mut GtkWidget,
        (0i32 == 0) as c_int,
        (0i32 == 0) as c_int,
        0 as guint,
    );
    gtk_box_pack_start(
        pathbox,
        (*win).path_bar as *mut c_void as *mut GtkWidget,
        (0i32 == 0) as c_int,
        (0i32 == 0) as c_int,
        0 as guint,
    );
    toolitem = gtk_tool_item_new();
    gtk_container_add(
        toolitem as *mut c_void as *mut GtkContainer,
        pathbox as *mut c_void as *mut GtkWidget,
    );
    gtk_tool_item_set_expand(toolitem, (0i32 == 0) as c_int);
    gtk_toolbar_insert(
        (*win).toolbar,
        toolitem,
        gtk_toolbar_get_n_items((*win).toolbar) - 1 as gint,
    );
    g_signal_connect_data(
        fm_config as *mut FmAppConfig as gpointer,
        b"changed::toolsbar\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_toolsbar_changed as unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*win).notebook = gtk_notebook_new() as *mut GtkNotebook;
    gtk_notebook_set_scrollable((*win).notebook, (0i32 == 0) as c_int);
    gtk_container_set_border_width(
        (*win).notebook as *mut c_void as *mut GtkContainer,
        0 as guint,
    );
    gtk_notebook_set_show_border((*win).notebook, 0 as gboolean);
    _do_settings_hack();
    g_signal_connect_data(
        (*win).notebook as gpointer,
        b"switch-page\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkNotebook, *mut gpointer, guint, *mut FmMainWin) -> (),
            >,
            GCallback,
        >(Some(
            on_notebook_switch_page
                as unsafe extern "C" fn(
                    *mut GtkNotebook,
                    *mut gpointer,
                    guint,
                    *mut FmMainWin,
                ) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_AFTER,
    );
    g_signal_connect_data(
        (*win).notebook as gpointer,
        b"page-added\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkNotebook, *mut GtkWidget, guint, *mut FmMainWin) -> (),
            >,
            GCallback,
        >(Some(
            on_notebook_page_added
                as unsafe extern "C" fn(
                    *mut GtkNotebook,
                    *mut GtkWidget,
                    guint,
                    *mut FmMainWin,
                ) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*win).notebook as gpointer,
        b"page-removed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkNotebook, *mut GtkWidget, guint, *mut FmMainWin) -> (),
            >,
            GCallback,
        >(Some(
            on_notebook_page_removed
                as unsafe extern "C" fn(
                    *mut GtkNotebook,
                    *mut GtkWidget,
                    guint,
                    *mut FmMainWin,
                ) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    gtk_box_pack_start(
        vbox,
        (*win).notebook as *mut c_void as *mut GtkWidget,
        (0i32 == 0) as c_int,
        (0i32 == 0) as c_int,
        0 as guint,
    );
    g_signal_connect_data(
        fm_config as *mut FmAppConfig as gpointer,
        b"changed::always_show_tabs\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_always_show_tabs_changed
                as unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*win).statusbar = gtk_statusbar_new() as *mut GtkStatusbar;
    gtk_widget_style_get(
        (*win).statusbar as *mut c_void as *mut GtkWidget,
        b"shadow-type\0" as *const u8 as *const gchar,
        &raw mut shadow_type,
        ptr::null_mut::<c_void>(),
    );
    (*win).vol_status = gtk_frame_new(ptr::null::<gchar>()) as *mut GtkFrame;
    gtk_frame_set_shadow_type((*win).vol_status, shadow_type);
    gtk_box_pack_start(
        (*win).statusbar as *mut c_void as *mut GtkBox,
        (*win).vol_status as *mut c_void as *mut GtkWidget,
        0 as gboolean,
        (0i32 == 0) as c_int,
        0 as guint,
    );
    gtk_container_add(
        (*win).vol_status as *mut c_void as *mut GtkContainer,
        gtk_label_new(ptr::null::<gchar>()),
    );
    gtk_box_pack_start(
        vbox,
        (*win).statusbar as *mut c_void as *mut GtkWidget,
        0 as gboolean,
        (0i32 == 0) as c_int,
        0 as guint,
    );
    (*win).statusbar_ctx =
        gtk_statusbar_get_context_id((*win).statusbar, b"status\0" as *const u8 as *const gchar);
    (*win).statusbar_ctx2 =
        gtk_statusbar_get_context_id((*win).statusbar, b"status2\0" as *const u8 as *const gchar);
    g_signal_connect_data(
        fm_config as *mut FmAppConfig as gpointer,
        b"changed::statusbar\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_statusbar_changed as unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_object_unref(act_grp as gpointer);
    (*win).ui = ui;
    atk_obj = gtk_widget_get_accessible((*win).location as *mut c_void as *mut GtkWidget);
    relation = atk_relation_new(&raw mut atk_obj, 1 as gint, ATK_RELATION_LABEL_FOR);
    atk_view = gtk_widget_get_accessible(gtk_ui_manager_get_widget(
        ui,
        b"/toolbar/Go\0" as *const u8 as *const gchar,
    ));
    atk_relation_set_add(atk_object_ref_relation_set(atk_view), relation);
    g_object_unref(relation as gpointer);
    atk_view = gtk_widget_get_accessible((*win).notebook as *mut c_void as *mut GtkWidget);
    relation = atk_relation_new(&raw mut atk_obj, 1 as gint, ATK_RELATION_CONTROLLED_BY);
    atk_relation_set_add(atk_object_ref_relation_set(atk_view), relation);
    g_object_unref(relation as gpointer);
    atk_obj = gtk_widget_get_accessible((*win).statusbar as *mut c_void as *mut GtkWidget);
    relation = atk_relation_new(&raw mut atk_obj, 1 as gint, ATK_RELATION_DESCRIBED_BY);
    atk_relation_set_add(atk_object_ref_relation_set(atk_view), relation);
    g_object_unref(relation as gpointer);
    gtk_container_add(
        win as *mut c_void as *mut GtkContainer,
        vbox as *mut c_void as *mut GtkWidget,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fm_main_win_new() -> *mut FmMainWin {
    let mut win: *mut FmMainWin =
        g_object_new(fm_main_win_get_type(), ptr::null::<gchar>()) as *mut FmMainWin;
    return win;
}
unsafe extern "C" fn fm_main_win_destroy(mut object: *mut GtkWidget) {
    let mut win: *mut FmMainWin = ptr::null_mut::<FmMainWin>();
    win = object as *mut FmMainWin;
    if !(*win).win_group.is_null() {
        g_signal_handlers_disconnect_matched(
            (*win).location as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkEntry, *mut FmMainWin) -> ()>,
                gpointer,
            >(Some(
                on_location_activate as unsafe extern "C" fn(*mut GtkEntry, *mut FmMainWin) -> (),
            )),
            win as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*win).notebook as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GtkNotebook,
                        *mut gpointer,
                        guint,
                        *mut FmMainWin,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                on_notebook_switch_page
                    as unsafe extern "C" fn(
                        *mut GtkNotebook,
                        *mut gpointer,
                        guint,
                        *mut FmMainWin,
                    ) -> (),
            )),
            win as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*win).notebook as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GtkNotebook,
                        *mut GtkWidget,
                        guint,
                        *mut FmMainWin,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                on_notebook_page_added
                    as unsafe extern "C" fn(
                        *mut GtkNotebook,
                        *mut GtkWidget,
                        guint,
                        *mut FmMainWin,
                    ) -> (),
            )),
            win as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*win).notebook as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GtkNotebook,
                        *mut GtkWidget,
                        guint,
                        *mut FmMainWin,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                on_notebook_page_removed
                    as unsafe extern "C" fn(
                        *mut GtkNotebook,
                        *mut GtkWidget,
                        guint,
                        *mut FmMainWin,
                    ) -> (),
            )),
            win as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            fm_config as *mut FmAppConfig as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> ()>,
                gpointer,
            >(Some(
                on_toolsbar_changed as unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> (),
            )),
            win as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            fm_config as *mut FmAppConfig as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> ()>,
                gpointer,
            >(Some(
                on_statusbar_changed
                    as unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> (),
            )),
            win as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            fm_config as *mut FmAppConfig as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> ()>,
                gpointer,
            >(Some(
                on_always_show_tabs_changed
                    as unsafe extern "C" fn(*mut FmAppConfig, *mut FmMainWin) -> (),
            )),
            win as gpointer,
        );
        gtk_window_group_remove_window((*win).win_group, win as *mut c_void as *mut GtkWindow);
        g_object_unref((*win).win_group as gpointer);
        (*win).win_group = ptr::null_mut::<GtkWindowGroup>();
        g_object_unref((*win).ui as gpointer);
        (*win).ui = ptr::null_mut::<GtkUIManager>();
        if !(*win).bookmarks.is_null() {
            g_signal_handlers_disconnect_matched(
                (*win).bookmarks as gpointer,
                (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
                0 as guint,
                0 as GQuark,
                ptr::null_mut::<GClosure>(),
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut FmBookmarks, *mut FmMainWin) -> ()>,
                    gpointer,
                >(Some(
                    on_bookmarks_changed
                        as unsafe extern "C" fn(*mut FmBookmarks, *mut FmMainWin) -> (),
                )),
                win as gpointer,
            );
            g_object_unref((*win).bookmarks as gpointer);
            (*win).bookmarks = ptr::null_mut::<FmBookmarks>();
        }
        if (*win).idle_handler != 0 {
            g_source_remove((*win).idle_handler);
            (*win).idle_handler = 0 as guint;
        }
        all_wins = g_slist_remove(all_wins, win as gconstpointer);
        while gtk_notebook_get_n_pages((*win).notebook) > 0i32 {
            gtk_notebook_remove_page((*win).notebook, 0 as gint);
        }
    }
    Some(
        (*(fm_main_win_parent_class as *mut GtkWidgetClass))
            .destroy
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(object);
}
unsafe extern "C" fn fm_main_win_finalize(mut object: *mut GObject) {
    if (*(fm_main_win_parent_class as *mut GObjectClass))
        .finalize
        .is_some()
    {
        Some(
            (*(fm_main_win_parent_class as *mut GObjectClass))
                .finalize
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(object);
    }
    pcmanfm_unref();
}
unsafe extern "C" fn on_unrealize(mut widget: *mut GtkWidget) {
    let mut w: c_int = 0;
    let mut h: c_int = 0;
    let mut win: *mut FmMainWin = widget as *mut c_void as *mut FmMainWin;
    gtk_window_get_size(
        widget as *mut c_void as *mut GtkWindow,
        &raw mut w,
        &raw mut h,
    );
    if (*win).fullscreen == 0
        && (*win).maximized == 0
        && (w != (*(fm_config as *mut FmAppConfig)).win_width
            || h != (*(fm_config as *mut FmAppConfig)).win_height)
    {
        (*(fm_config as *mut FmAppConfig)).win_width = w;
        (*(fm_config as *mut FmAppConfig)).win_height = h;
        pcmanfm_save_config(0 as gboolean);
    }
    if (*win).maximized != (*(fm_config as *mut FmAppConfig)).maximized {
        (*(fm_config as *mut FmAppConfig)).maximized = (*win).maximized;
        pcmanfm_save_config(0 as gboolean);
    }
    Some(
        (*(fm_main_win_parent_class as *mut GtkWidgetClass))
            .unrealize
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(widget);
}
unsafe extern "C" fn on_about_response(
    mut dlg: *mut GtkDialog,
    mut response: gint,
    mut dlgptr: *mut *mut GtkAboutDialog,
) {
    g_signal_handlers_disconnect_matched(
        dlg as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkDialog, gint, *mut *mut GtkAboutDialog) -> ()>,
            gpointer,
        >(Some(
            on_about_response
                as unsafe extern "C" fn(*mut GtkDialog, gint, *mut *mut GtkAboutDialog) -> (),
        )),
        dlgptr as gpointer,
    );
    *dlgptr = ptr::null_mut::<GtkAboutDialog>();
    pcmanfm_unref();
    gtk_widget_destroy(dlg as *mut c_void as *mut GtkWidget);
}
unsafe extern "C" fn on_about(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    if about_dlg.is_null() {
        let mut comments: *mut GString = g_string_new(dcgettext(
            ptr::null::<c_char>(),
            c"Lightweight file manager\n".as_ptr(),
            5 as c_int,
        ));
        let mut builder: *mut GtkBuilder = gtk_builder_new_from_file(
            b"/usr/share/pcmanfm/ui/about.ui\0" as *const u8 as *const gchar,
        );
        about_dlg = gtk_builder_get_object(builder, b"dlg\0" as *const u8 as *const gchar)
            as *mut c_void as *mut GtkAboutDialog;
        g_string_append_printf(
            comments,
            dcgettext(
                ptr::null::<c_char>(),
                c"using LibFM ver. %s\n".as_ptr(),
                5 as c_int,
            ),
            fm_version(),
        );
        if 0 != 0 {
            ({
                let __val: *const c_char = dcgettext(
                    ptr::null::<c_char>(),
                    c"\nDeveloped by Hon Jen Yee (PCMan)".as_ptr(),
                    5 as c_int,
                );
                g_string_append_len_inline(
                    comments,
                    __val,
                    if ({
                        let mut _g_boolean_var_55: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_55 = 1i32;
                        }
                        _g_boolean_var_55
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
            g_string_append_len_inline(
                comments,
                dcgettext(
                    ptr::null::<c_char>(),
                    c"\nDeveloped by Hon Jen Yee (PCMan)".as_ptr(),
                    5 as c_int,
                ),
                -(1i32) as gssize,
            );
        };
        gtk_about_dialog_set_comments(about_dlg, (*comments).str_0);
        if 0 != 0 {
            if 0i32 == 0 {
                g_string_free(comments, (0i32 == 0) as c_int);
            } else {
                g_string_free_and_steal(comments);
            };
        } else {
            g_string_free(comments, (0i32 == 0) as c_int);
        };
        g_object_unref(builder as gpointer);
        g_signal_connect_data(
            about_dlg as gpointer,
            b"response\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkDialog, gint, *mut *mut GtkAboutDialog) -> ()>,
                GCallback,
            >(Some(
                on_about_response
                    as unsafe extern "C" fn(*mut GtkDialog, gint, *mut *mut GtkAboutDialog) -> (),
            )),
            &raw mut about_dlg as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        pcmanfm_ref();
    }
    gtk_window_present(about_dlg as *mut c_void as *mut GtkWindow);
}
unsafe extern "C" fn on_key_nav_list_response(
    mut dlg: *mut GtkDialog,
    mut response: gint,
    mut dlgptr: *mut *mut GtkDialog,
) {
    g_signal_handlers_disconnect_matched(
        dlg as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkDialog, gint, *mut *mut GtkDialog) -> ()>,
            gpointer,
        >(Some(
            on_key_nav_list_response
                as unsafe extern "C" fn(*mut GtkDialog, gint, *mut *mut GtkDialog) -> (),
        )),
        dlgptr as gpointer,
    );
    *dlgptr = ptr::null_mut::<GtkDialog>();
    gtk_widget_destroy(dlg as *mut c_void as *mut GtkWidget);
}
unsafe extern "C" fn on_key_nav_list(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    if key_nav_list_dlg.is_null() {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_DEBUG,
            b"creating key_nav_list_dlg\0" as *const u8 as *const gchar,
        );
        key_nav_list_dlg = gtk_message_dialog_new(
            ptr::null_mut::<GtkWindow>(),
            0 as GtkDialogFlags,
            GTK_MESSAGE_INFO,
            GTK_BUTTONS_OK,
            dcgettext(
                ptr::null::<c_char>(),
                c"Keyboard Navigation".as_ptr(),
                5 as c_int,
            ),
        );
        gtk_message_dialog_format_secondary_text(
            key_nav_list_dlg as *mut c_void as *mut GtkMessageDialog,
            dcgettext(
                ptr::null::<c_char>(),
                c"Tab: cycle focus Folder View -> Side Pane -> Tools Bar\nShift+Tab: cycle focus Tools Bar -> Side Pane -> Folder View\nF6: change focus Side pane <-> Folder view\nF8: focus divider between Side pane and Folder view\nF10: activate main menu\nCtrl+L or Alt+D: jump focus to Path Bar".as_ptr(),
                5 as c_int,
            ),
        );
        gtk_window_set_title(
            key_nav_list_dlg as *mut c_void as *mut GtkWindow,
            dcgettext(
                ptr::null::<c_char>(),
                c"Keyboard Navigation".as_ptr(),
                5 as c_int,
            ),
        );
        g_signal_connect_data(
            key_nav_list_dlg as gpointer,
            b"response\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkDialog, gint, *mut *mut GtkDialog) -> ()>,
                GCallback,
            >(Some(
                on_key_nav_list_response
                    as unsafe extern "C" fn(*mut GtkDialog, gint, *mut *mut GtkDialog) -> (),
            )),
            &raw mut key_nav_list_dlg as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
    }
    gtk_window_present(key_nav_list_dlg as *mut c_void as *mut GtkWindow);
}
unsafe extern "C" fn on_open_in_terminal(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut path: *mut FmPath = ptr::null_mut::<FmPath>();
    path = fm_nav_history_get_nth_path(
        (*win).nav_history,
        fm_nav_history_get_cur_index((*win).nav_history),
    );
    if !path.is_null() {
        pcmanfm_open_folder_in_terminal(win as *mut c_void as *mut GtkWindow, path);
    }
}
unsafe extern "C" fn on_search(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut page: *mut FmTabPage = (*win).current_page;
    let mut cwd: *mut FmPath = fm_tab_page_get_cwd(page);
    let mut l: *mut GList = g_list_append(ptr::null_mut::<GList>(), cwd as gpointer);
    fm_launch_search_simple(
        win as *mut c_void as *mut GtkWindow,
        ptr::null_mut::<GAppLaunchContext>(),
        l,
        Some(
            pcmanfm_open_folder
                as unsafe extern "C" fn(
                    *mut GAppLaunchContext,
                    *mut GList,
                    gpointer,
                    *mut *mut GError,
                ) -> gboolean,
        ),
        ptr::null_mut::<c_void>(),
    );
    g_list_free(l);
}
unsafe extern "C" fn on_launch(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut cmd: *mut c_char = ptr::null_mut::<c_char>();
    let mut cwd: *mut c_char = ptr::null_mut::<c_char>();
    let mut def: *mut c_char = ptr::null_mut::<c_char>();
    cmd = fm_get_user_input(
        win as *mut c_void as *mut GtkWindow,
        dcgettext(ptr::null::<c_char>(), c"Run a command".as_ptr(), 5 as c_int),
        dcgettext(
            ptr::null::<c_char>(),
            c"Enter a command to run:".as_ptr(),
            5 as c_int,
        ),
        ptr::null::<c_char>(),
    );
    if cmd.is_null() || *cmd.offset(0 as isize) as c_int == 0 {
        return;
    }
    cwd = fm_path_to_str(fm_tab_page_get_cwd((*win).current_page));
    def = g_get_current_dir() as *mut c_char;
    if chdir(cwd) != 0i32 {
        fm_show_error(
            win as *mut c_void as *mut GtkWindow,
            dcgettext(
                ptr::null::<c_char>(),
                c"Current folder is inaccessible".as_ptr(),
                5 as c_int,
            ),
            ptr::null::<c_char>(),
        );
    } else {
        fm_launch_command_simple(
            win as *mut c_void as *mut GtkWindow,
            ptr::null_mut::<GAppLaunchContext>(),
            G_APP_INFO_CREATE_NONE,
            cmd,
            ptr::null_mut::<FmPathList>(),
        );
        if chdir(def) != 0i32 {
            g_log(
                ptr::null_mut::<gchar>(),
                G_LOG_LEVEL_WARNING,
                b"on_launch(): chdir() failed\0" as *const u8 as *const gchar,
            );
        }
    }
    g_free(cmd as gpointer);
    g_free(cwd as gpointer);
    g_free(def as gpointer);
}
unsafe extern "C" fn on_show_hidden(mut act: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut page: *mut FmTabPage = (*win).current_page;
    let mut active: gboolean = 0;
    if page.is_null() {
        return;
    }
    active = gtk_toggle_action_get_active(act);
    fm_tab_page_set_show_hidden(page, active);
}
unsafe extern "C" fn on_fullscreen(mut act: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut active: gboolean = gtk_toggle_action_get_active(act);
    if (*win).fullscreen == active {
        return;
    }
    (*win).fullscreen = active;
    if (*win).fullscreen != 0 {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        gtk_window_get_size(win as *mut c_void as *mut GtkWindow, &raw mut w, &raw mut h);
        if w != (*(fm_config as *mut FmAppConfig)).win_width
            || h != (*(fm_config as *mut FmAppConfig)).win_height
        {
            (*(fm_config as *mut FmAppConfig)).win_width = w;
            (*(fm_config as *mut FmAppConfig)).win_height = h;
            pcmanfm_save_config(0 as gboolean);
        }
        gtk_window_fullscreen(win as *mut c_void as *mut GtkWindow);
    } else {
        gtk_window_unfullscreen(win as *mut c_void as *mut GtkWindow);
    };
}
unsafe extern "C" fn on_change_mode(
    mut act: *mut GtkRadioAction,
    mut cur: *mut GtkRadioAction,
    mut win: *mut FmMainWin,
) {
    let mut mode: c_int = gtk_radio_action_get_current_value(cur) as c_int;
    if (*win).in_update != 0 {
        return;
    }
    fm_standard_view_set_mode(
        (*win).folder_view as *mut c_void as *mut FmStandardView,
        mode as FmStandardViewMode,
    );
    if (*(*win).current_page).own_config() != 0 {
        fm_app_config_save_config_for_path(
            fm_folder_view_get_cwd((*win).folder_view),
            (*(*win).current_page).sort_type,
            (*(*win).current_page).sort_by,
            mode as FmStandardViewMode,
            (*(*win).current_page).show_hidden(),
            ptr::null_mut::<*mut c_char>(),
        );
    } else {
        (*(*win).current_page).view_mode = mode as guint;
    };
}
unsafe extern "C" fn on_sort_by(
    mut act: *mut GtkRadioAction,
    mut cur: *mut GtkRadioAction,
    mut win: *mut FmMainWin,
) {
    let mut val: c_int = gtk_radio_action_get_current_value(cur) as c_int;
    let mut fv: *mut FmFolderView = (*win).folder_view;
    let mut model: *mut FmFolderModel = fm_folder_view_get_model(fv);
    if !model.is_null() {
        fm_folder_model_set_sort(model, val as FmFolderModelCol, 4294967295 as FmSortMode);
    }
    if val != (*(*win).current_page).sort_by as c_int {
        (*(*win).current_page).sort_by = val as FmFolderModelCol;
        if (*(*win).current_page).own_config() != 0 {
            fm_app_config_save_config_for_path(
                fm_folder_view_get_cwd(fv),
                (*(*win).current_page).sort_type,
                val as FmFolderModelCol,
                4294967295 as FmStandardViewMode,
                (*(*win).current_page).show_hidden(),
                ptr::null_mut::<*mut c_char>(),
            );
        } else {
            (*(fm_config as *mut FmAppConfig)).sort_by = val as FmFolderModelCol;
            pcmanfm_save_config(0 as gboolean);
        }
    }
}
#[inline]
unsafe extern "C" fn update_sort_type_for_page(
    mut page: *mut FmTabPage,
    mut fv: *mut FmFolderView,
    mut mode: FmSortMode,
) {
    if mode as c_uint != (*page).sort_type as c_uint {
        (*page).sort_type = mode;
        if (*page).own_config() != 0 {
            fm_app_config_save_config_for_path(
                fm_folder_view_get_cwd(fv),
                mode,
                (*page).sort_by,
                4294967295 as FmStandardViewMode,
                (*page).show_hidden(),
                ptr::null_mut::<*mut c_char>(),
            );
        } else {
            (*(fm_config as *mut FmAppConfig)).sort_type = mode;
            pcmanfm_save_config(0 as gboolean);
        }
    }
}
unsafe extern "C" fn on_sort_type(
    mut act: *mut GtkRadioAction,
    mut cur: *mut GtkRadioAction,
    mut win: *mut FmMainWin,
) {
    let mut val: guint = gtk_radio_action_get_current_value(cur) as guint;
    let mut fv: *mut FmFolderView = (*win).folder_view;
    let mut model: *mut FmFolderModel = fm_folder_view_get_model(fv);
    let mut mode: FmSortMode = FM_SORT_ASCENDING;
    if !model.is_null() {
        fm_folder_model_get_sort(model, ptr::null_mut::<FmFolderModelCol>(), &raw mut mode);
        mode = mem::transmute::<c_uint, FmSortMode>(
            mode as c_uint & !(FM_SORT_ORDER_MASK as c_int) as c_uint,
        );
        mode = mem::transmute::<c_uint, FmSortMode>(
            mode as c_uint
                | (if val == GTK_SORT_ASCENDING as c_int as guint {
                    FM_SORT_ASCENDING as c_int
                } else {
                    FM_SORT_DESCENDING as c_int
                }) as c_uint,
        );
        fm_folder_model_set_sort(model, 4294967295 as FmFolderModelCol, mode);
        update_sort_type_for_page((*win).current_page, fv, mode);
    }
}
unsafe extern "C" fn on_mingle_dirs(mut act: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut fv: *mut FmFolderView = (*win).folder_view;
    let mut model: *mut FmFolderModel = fm_folder_view_get_model(fv);
    let mut mode: FmSortMode = FM_SORT_ASCENDING;
    let mut active: gboolean = 0;
    if !model.is_null() {
        fm_folder_model_get_sort(model, ptr::null_mut::<FmFolderModelCol>(), &raw mut mode);
        active = gtk_toggle_action_get_active(act);
        mode = mem::transmute::<c_uint, FmSortMode>(
            mode as c_uint & !(FM_SORT_NO_FOLDER_FIRST as c_int) as c_uint,
        );
        if active != 0 {
            mode = mem::transmute::<c_uint, FmSortMode>(
                mode as c_uint | FM_SORT_NO_FOLDER_FIRST as c_int as c_uint,
            );
        }
        fm_folder_model_set_sort(model, 4294967295 as FmFolderModelCol, mode);
        update_sort_type_for_page((*win).current_page, fv, mode);
    }
}
unsafe extern "C" fn on_sort_ignore_case(mut act: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut fv: *mut FmFolderView = (*win).folder_view;
    let mut model: *mut FmFolderModel = fm_folder_view_get_model(fv);
    let mut mode: FmSortMode = FM_SORT_ASCENDING;
    let mut active: gboolean = 0;
    if !model.is_null() {
        fm_folder_model_get_sort(model, ptr::null_mut::<FmFolderModelCol>(), &raw mut mode);
        active = gtk_toggle_action_get_active(act);
        mode = mem::transmute::<c_uint, FmSortMode>(
            mode as c_uint & !(FM_SORT_CASE_SENSITIVE as c_int) as c_uint,
        );
        if active == 0 {
            mode = mem::transmute::<c_uint, FmSortMode>(
                mode as c_uint | FM_SORT_CASE_SENSITIVE as c_int as c_uint,
            );
        }
        fm_folder_model_set_sort(model, 4294967295 as FmFolderModelCol, mode);
        update_sort_type_for_page((*win).current_page, fv, mode);
    }
}
unsafe extern "C" fn on_save_per_folder(mut act: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut active: gboolean = gtk_toggle_action_get_active(act);
    let mut page: *mut FmTabPage = (*win).current_page;
    let mut fv: *mut FmFolderView = (*win).folder_view;
    if active != 0 {
        if (*page).own_config() != 0 {
            return;
        }
        (*page).set_own_config((0i32 == 0) as c_int as gboolean as gboolean);
        (*page).columns = g_strdupv((*(fm_config as *mut FmAppConfig)).columns as *mut *mut gchar)
            as *mut *mut c_char;
        fm_app_config_save_config_for_path(
            fm_folder_view_get_cwd(fv),
            (*page).sort_type,
            (*page).sort_by,
            fm_standard_view_get_mode(fv as *mut c_void as *mut FmStandardView),
            (*page).show_hidden(),
            (*page).columns,
        );
    } else if (*page).own_config() != 0 {
        (*page).set_own_config(0 as gboolean as gboolean);
        g_strfreev((*page).columns as *mut *mut gchar);
        (*page).columns = ptr::null_mut::<*mut c_char>();
        fm_app_config_clear_config_for_path(fm_folder_view_get_cwd(fv));
    }
}
unsafe extern "C" fn on_side_pane_mode(
    mut act: *mut GtkRadioAction,
    mut cur: *mut GtkRadioAction,
    mut win: *mut FmMainWin,
) {
    let mut cur_page: *mut FmTabPage = (*win).current_page;
    let mut sp: *mut FmSidePane = ptr::null_mut::<FmSidePane>();
    if cur_page.is_null() {
        return;
    }
    sp = fm_tab_page_get_side_pane(cur_page);
    let mut val: c_int = gtk_radio_action_get_current_value(cur) as c_int;
    fm_side_pane_set_mode(sp, val as FmSidePaneMode);
}
unsafe extern "C" fn main_win_on_focus_in(
    mut w: *mut GtkWidget,
    mut evt: *mut GdkEventFocus,
) -> gboolean {
    if (*all_wins).data != w as gpointer {
        all_wins = g_slist_remove(all_wins, w as gconstpointer);
        all_wins = g_slist_prepend(all_wins, w as gpointer);
    }
    return (*(fm_main_win_parent_class as *mut GtkWidgetClass))
        .focus_in_event
        .expect("non-null function pointer")(w, evt);
}
unsafe extern "C" fn on_new_win(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut path: *mut FmPath = fm_tab_page_get_cwd((*win).current_page);
    fm_main_win_add_win(win, path);
}
unsafe extern "C" fn on_new_tab(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut path: *mut FmPath = fm_tab_page_get_cwd((*win).current_page);
    fm_main_win_add_tab(win, path);
    gtk_window_set_focus(
        win as *mut c_void as *mut GtkWindow,
        (*win).location as *mut c_void as *mut GtkWidget,
    );
    if (*win).idle_handler != 0 {
        g_source_remove((*win).idle_handler);
        (*win).idle_handler = 0 as guint;
    }
}
unsafe extern "C" fn on_close_win(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    gtk_widget_destroy(win as *mut c_void as *mut GtkWidget);
}
unsafe extern "C" fn on_close_tab(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut nb: *mut GtkNotebook = (*win).notebook;
    gtk_notebook_remove_page(nb, gtk_notebook_get_current_page(nb));
}
unsafe extern "C" fn on_go(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    g_signal_emit_by_name(
        (*win).location as gpointer,
        b"activate\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn _update_hist_buttons(mut win: *mut FmMainWin) {
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    let mut nh: *mut FmNavHistory = fm_tab_page_get_history((*win).current_page);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/GoMenu/Next\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(
        act,
        (fm_nav_history_get_cur_index(nh) > 0 as guint) as c_int,
    );
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/GoMenu/Prev\0" as *const u8 as *const gchar,
    );
    gtk_action_set_sensitive(act, fm_nav_history_can_back(nh));
    update_file_menu(win, fm_tab_page_get_cwd((*win).current_page));
}
unsafe extern "C" fn on_go_back(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    fm_tab_page_back((*win).current_page);
    _update_hist_buttons(win);
}
unsafe extern "C" fn on_go_forward(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    fm_tab_page_forward((*win).current_page);
    _update_hist_buttons(win);
}
unsafe extern "C" fn on_go_up(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut parent: *mut FmPath = fm_path_get_parent(fm_tab_page_get_cwd((*win).current_page));
    if !parent.is_null() {
        fm_main_win_chdir(win, parent);
    }
}
unsafe extern "C" fn on_go_home(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    if !(*(fm_config as *mut FmAppConfig)).home_path.is_null()
        && *(*(fm_config as *mut FmAppConfig))
            .home_path
            .offset(0 as isize) as c_int
            != 0
    {
        fm_main_win_chdir_by_name(win, (*(fm_config as *mut FmAppConfig)).home_path);
    } else {
        fm_main_win_chdir(win, fm_path_get_home());
    };
}
unsafe extern "C" fn on_go_desktop(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    fm_main_win_chdir(win, fm_path_get_desktop());
}
unsafe extern "C" fn on_go_trash(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    fm_main_win_chdir(win, fm_path_get_trash());
}
unsafe extern "C" fn on_go_computer(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    fm_main_win_chdir_by_name(win, c"computer:///".as_ptr());
}
unsafe extern "C" fn on_go_network(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    fm_main_win_chdir_by_name(win, c"network:///".as_ptr());
}
unsafe extern "C" fn on_go_apps(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    fm_main_win_chdir(win, fm_path_get_apps_menu());
}
unsafe extern "C" fn on_go_connect(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    open_connect_dialog(win as *mut c_void as *mut GtkWindow);
}
#[no_mangle]
pub unsafe extern "C" fn fm_main_win_chdir_by_name(
    mut win: *mut FmMainWin,
    mut path_str: *const c_char,
) {
    let mut path: *mut FmPath = fm_path_new_for_str(path_str);
    fm_main_win_chdir(win, path);
    fm_path_unref(path);
}
#[no_mangle]
pub unsafe extern "C" fn fm_main_win_chdir(mut win: *mut FmMainWin, mut path: *mut FmPath) {
    let mut current_focus: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
    g_signal_handlers_block_matched(
        (*win).side_pane as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmSidePane, guint, *mut FmPath, *mut FmMainWin) -> ()>,
            gpointer,
        >(Some(
            on_side_pane_chdir
                as unsafe extern "C" fn(*mut FmSidePane, guint, *mut FmPath, *mut FmMainWin) -> (),
        )),
        win as gpointer,
    );
    fm_tab_page_chdir((*win).current_page, path);
    g_signal_handlers_unblock_matched(
        (*win).side_pane as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmSidePane, guint, *mut FmPath, *mut FmMainWin) -> ()>,
            gpointer,
        >(Some(
            on_side_pane_chdir
                as unsafe extern "C" fn(*mut FmSidePane, guint, *mut FmPath, *mut FmMainWin) -> (),
        )),
        win as gpointer,
    );
    _update_hist_buttons(win);
    update_view_menu(win);
    update_file_menu(win, path);
    current_focus = gtk_window_get_focus(win as *mut c_void as *mut GtkWindow);
    if current_focus == (*win).location as *mut GtkWidget {
        gtk_editable_set_position(
            (*win).location as *mut c_void as *mut GtkEditable,
            -(1 as gint),
        );
    }
}
unsafe extern "C" fn on_tab_label_button_pressed(
    mut tab_label: *mut GtkWidget,
    mut evt: *mut GdkEventButton,
    mut tab_page: *mut FmTabPage,
) -> gboolean {
    if (*evt).button == 2 as guint {
        gtk_widget_destroy(tab_page as *mut c_void as *mut GtkWidget);
        return (0i32 == 0) as c_int;
    }
    return 0 as gboolean;
}
unsafe extern "C" fn update_statusbar(mut win: *mut FmMainWin) {
    let mut page: *mut FmTabPage = (*win).current_page;
    let mut text: *const c_char = ptr::null::<c_char>();
    if (*(fm_config as *mut FmAppConfig)).show_statusbar == 0 {
        return;
    }
    gtk_statusbar_pop((*win).statusbar, (*win).statusbar_ctx);
    text = fm_tab_page_get_status_text(page, FM_STATUS_TEXT_NORMAL);
    if !text.is_null() {
        gtk_statusbar_push((*win).statusbar, (*win).statusbar_ctx, text as *const gchar);
    }
    gtk_statusbar_pop((*win).statusbar, (*win).statusbar_ctx2);
    text = fm_tab_page_get_status_text(page, FM_STATUS_TEXT_SELECTED_FILES);
    if !text.is_null() {
        gtk_statusbar_push(
            (*win).statusbar,
            (*win).statusbar_ctx2,
            text as *const gchar,
        );
    }
    text = fm_tab_page_get_status_text(page, FM_STATUS_TEXT_FS_INFO);
    if !text.is_null() {
        let mut label: *mut GtkLabel =
            gtk_bin_get_child((*win).vol_status as *mut c_void as *mut GtkBin) as *mut c_void
                as *mut GtkLabel;
        gtk_label_set_text(label, text as *const gchar);
        gtk_widget_show((*win).vol_status as *mut c_void as *mut GtkWidget);
    } else {
        gtk_widget_hide((*win).vol_status as *mut c_void as *mut GtkWidget);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fm_main_win_add_tab(
    mut win: *mut FmMainWin,
    mut path: *mut FmPath,
) -> gint {
    let mut page: *mut FmTabPage = fm_tab_page_new(path);
    let mut gpage: *mut GtkWidget = page as *mut c_void as *mut GtkWidget;
    let mut label: *mut FmTabLabel = (*page).tab_label;
    let mut folder_view: *mut FmFolderView = fm_tab_page_get_folder_view(page);
    let mut ret: gint = 0;
    gtk_paned_set_position(
        page as *mut c_void as *mut GtkPaned,
        (*(fm_config as *mut FmAppConfig)).splitter_pos as gint,
    );
    g_signal_connect_data(
        folder_view as gpointer,
        b"key-press-event\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolderView,
                    *mut GdkEventKey,
                    *mut FmMainWin,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            on_view_key_press_event
                as unsafe extern "C" fn(
                    *mut FmFolderView,
                    *mut GdkEventKey,
                    *mut FmMainWin,
                ) -> gboolean,
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*label).close_btn as gpointer,
        b"clicked\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkWidget) -> ()>, GCallback>(Some(
            gtk_widget_destroy as unsafe extern "C" fn(*mut GtkWidget) -> (),
        )),
        page as gpointer,
        None,
        G_CONNECT_SWAPPED,
    );
    g_signal_connect_data(
        label as gpointer,
        b"button-press-event\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    *mut FmTabPage,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            on_tab_label_button_pressed
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    *mut FmTabPage,
                ) -> gboolean,
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    ret = gtk_notebook_append_page(
        (*win).notebook,
        gpage,
        (*page).tab_label as *mut c_void as *mut GtkWidget,
    );
    gtk_widget_show_all(gpage);
    gtk_notebook_set_tab_reorderable((*win).notebook, gpage, (0i32 == 0) as c_int);
    gtk_notebook_set_current_page((*win).notebook, ret);
    return ret;
}
unsafe extern "C" fn on_window_state_event(
    mut widget: *mut GtkWidget,
    mut evt: *mut GdkEventWindowState,
    mut win: *mut FmMainWin,
) -> gboolean {
    if (*evt).changed_mask as c_uint & GDK_WINDOW_STATE_FULLSCREEN as c_int as c_uint != 0 {
        (*win).fullscreen = ((*evt).new_window_state as c_uint
            & GDK_WINDOW_STATE_FULLSCREEN as c_int as c_uint
            != 0 as c_uint) as c_int as gboolean;
    }
    if (*evt).changed_mask as c_uint & GDK_WINDOW_STATE_MAXIMIZED as c_int as c_uint != 0 {
        (*win).maximized = ((*evt).new_window_state as c_uint
            & GDK_WINDOW_STATE_MAXIMIZED as c_int as c_uint
            != 0 as c_uint) as c_int as gboolean;
    }
    return 0 as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn fm_main_win_add_win(
    mut win: *mut FmMainWin,
    mut path: *mut FmPath,
) -> *mut FmMainWin {
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    win = fm_main_win_new();
    gtk_window_set_default_size(
        win as *mut c_void as *mut GtkWindow,
        (*(fm_config as *mut FmAppConfig)).win_width as gint,
        (*(fm_config as *mut FmAppConfig)).win_height as gint,
    );
    if (*(fm_config as *mut FmAppConfig)).maximized != 0 {
        gtk_window_maximize(win as *mut c_void as *mut GtkWindow);
    }
    gtk_widget_show_all(win as *mut c_void as *mut GtkWidget);
    g_signal_connect_data(
        win as gpointer,
        b"window-state-event\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventWindowState,
                    *mut FmMainWin,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            on_window_state_event
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventWindowState,
                    *mut FmMainWin,
                ) -> gboolean,
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    fm_main_win_add_tab(win, path);
    gtk_window_present(win as *mut c_void as *mut GtkWindow);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Toolbar/ShowToolbar\0" as *const u8 as *const gchar,
    );
    gtk_toggle_action_set_active(
        act as *mut c_void as *mut GtkToggleAction,
        (*(fm_config as *mut FmAppConfig)).tb.visible(),
    );
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Toolbar/ToolbarNewWin\0" as *const u8 as *const gchar,
    );
    gtk_toggle_action_set_active(
        act as *mut c_void as *mut GtkToggleAction,
        (*(fm_config as *mut FmAppConfig)).tb.new_win(),
    );
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Toolbar/ToolbarNewTab\0" as *const u8 as *const gchar,
    );
    gtk_toggle_action_set_active(
        act as *mut c_void as *mut GtkToggleAction,
        (*(fm_config as *mut FmAppConfig)).tb.new_tab(),
    );
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Toolbar/ToolbarNav\0" as *const u8 as *const gchar,
    );
    gtk_toggle_action_set_active(
        act as *mut c_void as *mut GtkToggleAction,
        (*(fm_config as *mut FmAppConfig)).tb.nav(),
    );
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/Toolbar/ToolbarHome\0" as *const u8 as *const gchar,
    );
    gtk_toggle_action_set_active(
        act as *mut c_void as *mut GtkToggleAction,
        (*(fm_config as *mut FmAppConfig)).tb.home(),
    );
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/ShowStatus\0" as *const u8 as *const gchar,
    );
    gtk_toggle_action_set_active(
        act as *mut c_void as *mut GtkToggleAction,
        (*(fm_config as *mut FmAppConfig)).show_statusbar,
    );
    gtk_widget_set_visible(
        (*win).statusbar as *mut c_void as *mut GtkWidget,
        (*(fm_config as *mut FmAppConfig)).show_statusbar,
    );
    gtk_widget_hide((*win).path_bar as *mut c_void as *mut GtkWidget);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/PathMode/PathEntry\0" as *const u8 as *const gchar,
    );
    gtk_radio_action_set_current_value(
        act as *mut c_void as *mut GtkRadioAction,
        (*(fm_config as *mut FmAppConfig)).pathbar_mode_buttons as gint,
    );
    return win;
}
unsafe extern "C" fn on_open(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut files: *mut FmFileInfoList = fm_folder_view_dup_selected_files((*win).folder_view);
    if !files.is_null() {
        let mut l: *mut GList = fm_file_info_list_peek_head_link(files);
        if g_list_length(l) > 0 as guint {
            fm_launch_files_simple(
                win as *mut c_void as *mut GtkWindow,
                ptr::null_mut::<GAppLaunchContext>(),
                l,
                Some(
                    pcmanfm_open_folder
                        as unsafe extern "C" fn(
                            *mut GAppLaunchContext,
                            *mut GList,
                            gpointer,
                            *mut *mut GError,
                        ) -> gboolean,
                ),
                ptr::null_mut::<c_void>(),
            );
        }
        fm_file_info_list_unref(files);
    }
}
unsafe extern "C" fn on_link(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut files: *mut FmPathList = fm_folder_view_dup_selected_file_paths((*win).folder_view);
    if !files.is_null() {
        let mut dest: *mut FmPath =
            fm_select_folder(win as *mut c_void as *mut GtkWindow, ptr::null::<c_char>());
        if !dest.is_null() {
            fm_link_files(win as *mut c_void as *mut GtkWindow, files, dest);
            fm_path_unref(dest);
        }
        fm_path_list_unref(files);
    }
}
unsafe extern "C" fn on_copy_to(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut files: *mut FmPathList = fm_folder_view_dup_selected_file_paths((*win).folder_view);
    if !files.is_null() {
        fm_move_or_copy_files_to(win as *mut c_void as *mut GtkWindow, files, 0 as gboolean);
        fm_path_list_unref(files);
    }
}
unsafe extern "C" fn on_move_to(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut files: *mut FmPathList = fm_folder_view_dup_selected_file_paths((*win).folder_view);
    if !files.is_null() {
        fm_move_or_copy_files_to(
            win as *mut c_void as *mut GtkWindow,
            files,
            (0i32 == 0) as c_int,
        );
        fm_path_list_unref(files);
    }
}
unsafe extern "C" fn on_rename(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut files: *mut FmPathList = fm_folder_view_dup_selected_file_paths((*win).folder_view);
    if !files.is_null() {
        fm_rename_file(
            win as *mut c_void as *mut GtkWindow,
            fm_path_list_peek_head(files),
        );
        fm_path_list_unref(files);
    }
}
unsafe extern "C" fn on_trash(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut files: *mut FmPathList = fm_folder_view_dup_selected_file_paths((*win).folder_view);
    if !files.is_null() {
        fm_trash_files(win as *mut c_void as *mut GtkWindow, files);
        fm_path_list_unref(files);
    }
}
unsafe extern "C" fn on_del(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut files: *mut FmPathList = fm_folder_view_dup_selected_file_paths((*win).folder_view);
    if !files.is_null() {
        fm_delete_files(win as *mut c_void as *mut GtkWindow, files);
        fm_path_list_unref(files);
    }
}
unsafe extern "C" fn on_copy_path(mut action: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut dpy: *mut GdkDisplay = gtk_widget_get_display(win as *mut c_void as *mut GtkWidget);
    let mut clipboard: *mut GtkClipboard =
        gtk_clipboard_get_for_display(dpy, 69 as gulong as gpointer as GdkAtom);
    let mut str: *mut GString = g_string_sized_new(128 as gsize);
    let mut files: *mut FmPathList = fm_folder_view_dup_selected_file_paths((*win).folder_view);
    let mut fl: *mut GList = ptr::null_mut::<GList>();
    fl = fm_path_list_peek_head_link(files);
    while !fl.is_null() {
        let mut path: *mut c_char = fm_path_to_str((*fl).data as *mut FmPath);
        if (*str).len > 0 as gsize {
            g_string_append_c_inline(str, '\n' as i32 as gchar);
        }
        if 0 != 0 {
            ({
                let __val: *const c_char = path;
                g_string_append_len_inline(
                    str,
                    __val,
                    if ({
                        let mut _g_boolean_var_56: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_56 = 1i32;
                        }
                        _g_boolean_var_56
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
            g_string_append_len_inline(str, path, -(1i32) as gssize);
        };
        g_free(path as gpointer);
        fl = (*fl).next;
    }
    gtk_clipboard_set_text(clipboard, (*str).str_0, (*str).len as gint);
    if 0 != 0 {
        if 0i32 == 0 {
            g_string_free(str, (0i32 == 0) as c_int);
        } else {
            g_string_free_and_steal(str);
        };
    } else {
        g_string_free(str, (0i32 == 0) as c_int);
    };
}
unsafe extern "C" fn on_preference(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    fm_edit_preference(win as *mut c_void as *mut GtkWindow, 0i32);
}
unsafe extern "C" fn on_add_bookmark(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut cwd: *mut FmPath = fm_tab_page_get_cwd((*win).current_page);
    let mut disp_path: *mut c_char = fm_path_display_name(cwd, (0i32 == 0) as c_int);
    let mut msg: *mut c_char = g_strdup_printf(
        dcgettext(
            ptr::null::<c_char>(),
            c"Add following folder to bookmarks:\n'%s'\nEnter a name for the new bookmark item:"
                .as_ptr(),
            5 as c_int,
        ),
        disp_path,
    ) as *mut c_char;
    let mut disp_name: *mut c_char = fm_path_display_basename(cwd);
    let mut name: *mut c_char = ptr::null_mut::<c_char>();
    g_free(disp_path as gpointer);
    name = fm_get_user_input(
        win as *mut c_void as *mut GtkWindow,
        dcgettext(
            ptr::null::<c_char>(),
            c"Add to Bookmarks".as_ptr(),
            5 as c_int,
        ),
        msg,
        disp_name,
    );
    g_free(disp_name as gpointer);
    g_free(msg as gpointer);
    if !name.is_null() {
        fm_bookmarks_insert((*win).bookmarks, cwd, name, -(1i32));
        g_free(name as gpointer);
    }
}
unsafe extern "C" fn on_show_toolbar(mut action: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut active: gboolean = gtk_toggle_action_get_active(action);
    let ref mut fresh36 = (*(fm_config as *mut FmAppConfig)).tb;
    (*fresh36).set_visible(active as gboolean);
    fm_config_emit_changed(fm_config, c"toolsbar".as_ptr());
    pcmanfm_save_config(0 as gboolean);
}
unsafe extern "C" fn on_toolbar_new_win(mut act: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut active: gboolean = gtk_toggle_action_get_active(act);
    let ref mut fresh35 = (*(fm_config as *mut FmAppConfig)).tb;
    (*fresh35).set_new_win(active as gboolean);
    fm_config_emit_changed(fm_config, c"toolsbar".as_ptr());
    pcmanfm_save_config(0 as gboolean);
}
unsafe extern "C" fn on_toolbar_new_tab(mut act: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut active: gboolean = gtk_toggle_action_get_active(act);
    let ref mut fresh34 = (*(fm_config as *mut FmAppConfig)).tb;
    (*fresh34).set_new_tab(active as gboolean);
    fm_config_emit_changed(fm_config, c"toolsbar".as_ptr());
    pcmanfm_save_config(0 as gboolean);
}
unsafe extern "C" fn on_toolbar_nav(mut act: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut active: gboolean = gtk_toggle_action_get_active(act);
    let ref mut fresh33 = (*(fm_config as *mut FmAppConfig)).tb;
    (*fresh33).set_nav(active as gboolean);
    fm_config_emit_changed(fm_config, c"toolsbar".as_ptr());
    pcmanfm_save_config(0 as gboolean);
}
unsafe extern "C" fn on_toolbar_home(mut act: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut active: gboolean = gtk_toggle_action_get_active(act);
    let ref mut fresh32 = (*(fm_config as *mut FmAppConfig)).tb;
    (*fresh32).set_home(active as gboolean);
    fm_config_emit_changed(fm_config, c"toolsbar".as_ptr());
    pcmanfm_save_config(0 as gboolean);
}
unsafe extern "C" fn on_location(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    gtk_widget_grab_focus((*win).location as *mut c_void as *mut GtkWidget);
    if (*(fm_config as *mut FmAppConfig)).pathbar_mode_buttons != 0 {
        gtk_widget_show((*win).location as *mut c_void as *mut GtkWidget);
        gtk_widget_show(gtk_ui_manager_get_widget(
            (*win).ui,
            b"/toolbar/Go\0" as *const u8 as *const gchar,
        ));
        gtk_widget_hide((*win).path_bar as *mut c_void as *mut GtkWidget);
    }
}
unsafe extern "C" fn bounce_action(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut window: *mut GtkWindow = win as *mut c_void as *mut GtkWindow;
    let mut current_focus: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
    g_log(
        ptr::null_mut::<gchar>(),
        G_LOG_LEVEL_DEBUG,
        b"bouncing action %s to popup\0" as *const u8 as *const gchar,
        gtk_action_get_name(act),
    );
    current_focus = gtk_window_get_focus(window);
    gtk_window_set_focus(window, (*win).folder_view as *mut c_void as *mut GtkWidget);
    fm_folder_view_bounce_action(act, (*win).folder_view);
    gtk_window_set_focus(window, current_focus);
}
static mut icon_sizes: [guint; 19] = [
    8 as guint,
    10 as guint,
    12 as guint,
    16 as guint,
    20 as guint,
    24 as guint,
    32 as guint,
    40 as guint,
    48 as guint,
    64 as guint,
    80 as guint,
    96 as guint,
    128 as guint,
    160 as guint,
    192 as guint,
    256 as guint,
    320 as guint,
    400 as guint,
    512 as guint,
];
unsafe extern "C" fn on_size_decrement(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut mode: FmStandardViewMode = FM_FV_ICON_VIEW;
    let mut size: guint = 0;
    let mut i: guint = 0;
    if (*win).folder_view.is_null() {
        return;
    }
    mode = fm_standard_view_get_mode((*win).folder_view as *mut c_void as *mut FmStandardView);
    match mode as c_uint {
        0 => {
            size = (*fm_config).big_icon_size as guint;
            if size < 24 as guint {
                return;
            }
        }
        1 | 3 => {
            size = (*fm_config).small_icon_size as guint;
            if size < 10 as guint {
                return;
            }
        }
        2 => {
            size = (*fm_config).thumbnail_size as guint;
            if size < 64 as guint {
                return;
            }
        }
        _ => return,
    }
    i = (mem::size_of::<[guint; 19]>() as usize).wrapping_div(mem::size_of::<guint>() as usize)
        as guint;
    while i > 0 as guint {
        i = i.wrapping_sub(1);
        if icon_sizes[i as usize] < size {
            break;
        }
    }
    match mode as c_uint {
        0 => {
            (*fm_config).big_icon_size = icon_sizes[i as usize] as gint;
            fm_config_emit_changed(fm_config, c"big_icon_size".as_ptr());
        }
        1 | 3 => {
            (*fm_config).small_icon_size = icon_sizes[i as usize] as gint;
            fm_config_emit_changed(fm_config, c"small_icon_size".as_ptr());
        }
        2 => {
            (*fm_config).thumbnail_size = icon_sizes[i as usize] as gint;
            fm_config_emit_changed(fm_config, c"thumbnail_size".as_ptr());
        }
        _ => {}
    }
    pcmanfm_save_config(0 as gboolean);
}
unsafe extern "C" fn on_size_increment(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut mode: FmStandardViewMode = FM_FV_ICON_VIEW;
    let mut size: guint = 0;
    let mut i: guint = 0;
    if (*win).folder_view.is_null() {
        return;
    }
    mode = fm_standard_view_get_mode((*win).folder_view as *mut c_void as *mut FmStandardView);
    match mode as c_uint {
        0 => {
            size = (*fm_config).big_icon_size as guint;
            if size > 200 as guint {
                return;
            }
        }
        1 | 3 => {
            size = (*fm_config).small_icon_size as guint;
            if size > 100 as guint {
                return;
            }
        }
        2 => {
            size = (*fm_config).thumbnail_size as guint;
            if size > 400 as guint {
                return;
            }
        }
        _ => return,
    }
    i = 1 as guint;
    while (i as usize)
        < (mem::size_of::<[guint; 19]>() as usize).wrapping_div(mem::size_of::<guint>() as usize)
    {
        if icon_sizes[i as usize] > size {
            break;
        }
        i = i.wrapping_add(1);
    }
    match mode as c_uint {
        0 => {
            (*fm_config).big_icon_size = icon_sizes[i as usize] as gint;
            fm_config_emit_changed(fm_config, c"big_icon_size".as_ptr());
        }
        1 | 3 => {
            (*fm_config).small_icon_size = icon_sizes[i as usize] as gint;
            fm_config_emit_changed(fm_config, c"small_icon_size".as_ptr());
        }
        2 => {
            (*fm_config).thumbnail_size = icon_sizes[i as usize] as gint;
            fm_config_emit_changed(fm_config, c"thumbnail_size".as_ptr());
        }
        _ => {}
    }
    pcmanfm_save_config(0 as gboolean);
}
unsafe extern "C" fn on_size_default(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut mode: FmStandardViewMode = FM_FV_ICON_VIEW;
    if (*win).folder_view.is_null() {
        return;
    }
    mode = fm_standard_view_get_mode((*win).folder_view as *mut c_void as *mut FmStandardView);
    match mode as c_uint {
        0 => {
            if (*fm_config).big_icon_size == 48 as c_int {
                return;
            }
            (*fm_config).big_icon_size = 48 as gint;
            fm_config_emit_changed(fm_config, c"big_icon_size".as_ptr());
        }
        1 | 3 => {
            if (*fm_config).small_icon_size == 20i32 {
                return;
            }
            (*fm_config).small_icon_size = 20 as gint;
            fm_config_emit_changed(fm_config, c"small_icon_size".as_ptr());
        }
        2 => {
            if (*fm_config).thumbnail_size == 128 as c_int {
                return;
            }
            (*fm_config).thumbnail_size = 128 as gint;
            fm_config_emit_changed(fm_config, c"thumbnail_size".as_ptr());
        }
        _ => return,
    }
    pcmanfm_save_config(0 as gboolean);
}
unsafe extern "C" fn on_folder_view_clicked(
    mut fv: *mut FmFolderView,
    mut type_0: FmFolderViewClickType,
    mut fi: *mut FmFileInfo,
    mut win: *mut FmMainWin,
) {
    if fv != (*win).folder_view {
        return;
    }
    match type_0 as c_uint {
        2 => {
            if !fi.is_null() && fm_file_info_is_dir(fi) != 0 {
                fm_main_win_add_tab(win, fm_file_info_get_path(fi));
            }
        }
        1 | 3 | 0 | _ => {}
    };
}
unsafe extern "C" fn on_tab_page_status_text(
    mut page: *mut FmTabPage,
    mut type_0: guint,
    mut status_text: *const c_char,
    mut win: *mut FmMainWin,
) {
    if page != (*win).current_page {
        return;
    }
    if (*(fm_config as *mut FmAppConfig)).show_statusbar == 0 {
        return;
    }
    let mut current_block_14: u64;
    match type_0 {
        0 => {
            gtk_statusbar_pop((*win).statusbar, (*win).statusbar_ctx);
            if !status_text.is_null() {
                gtk_statusbar_push(
                    (*win).statusbar,
                    (*win).statusbar_ctx,
                    status_text as *const gchar,
                );
                if fm_folder_view_get_n_selected_files((*win).folder_view) == 0i32 {
                    current_block_14 = 13056961889198038528;
                } else {
                    status_text = fm_tab_page_get_status_text(page, FM_STATUS_TEXT_SELECTED_FILES);
                    current_block_14 = 7882238196683296727;
                }
            } else {
                current_block_14 = 13056961889198038528;
            }
        }
        1 => {
            current_block_14 = 7882238196683296727;
        }
        2 => {
            if !status_text.is_null() {
                let mut label: *mut GtkLabel =
                    gtk_bin_get_child((*win).vol_status as *mut c_void as *mut GtkBin)
                        as *mut c_void as *mut GtkLabel;
                gtk_label_set_text(label, status_text as *const gchar);
                gtk_widget_show((*win).vol_status as *mut c_void as *mut GtkWidget);
            } else {
                gtk_widget_hide((*win).vol_status as *mut c_void as *mut GtkWidget);
            }
            current_block_14 = 13056961889198038528;
        }
        _ => {
            current_block_14 = 13056961889198038528;
        }
    }
    match current_block_14 {
        7882238196683296727 => {
            gtk_statusbar_pop((*win).statusbar, (*win).statusbar_ctx2);
            if !status_text.is_null() {
                gtk_statusbar_push(
                    (*win).statusbar,
                    (*win).statusbar_ctx2,
                    status_text as *const gchar,
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn on_tab_page_chdir(
    mut page: *mut FmTabPage,
    mut path: *mut FmPath,
    mut win: *mut FmMainWin,
) {
    if page != (*win).current_page {
        return;
    }
    fm_path_entry_set_path((*win).location, path);
    fm_path_bar_set_path((*win).path_bar, path);
    gtk_window_set_title(
        win as *mut c_void as *mut GtkWindow,
        fm_tab_page_get_title(page) as *const gchar,
    );
}
unsafe extern "C" fn on_tab_page_got_focus(mut page: *mut FmTabPage, mut win: *mut FmMainWin) {
    let mut n: c_int =
        gtk_notebook_page_num((*win).notebook, page as *mut c_void as *mut GtkWidget) as c_int;
    if n >= 0i32 && (*win).idle_handler == 0 as guint {
        gtk_notebook_set_current_page((*win).notebook, n as gint);
    }
}
unsafe extern "C" fn on_tab_page_loaded(mut page: *mut FmTabPage, mut win: *mut FmMainWin) {
    let mut folder_view: *mut FmFolderView = fm_tab_page_get_folder_view(page);
    fm_folder_view_set_active(folder_view, 0 as gboolean);
    fm_folder_view_add_popup(folder_view, win as *mut c_void as *mut GtkWindow, None);
    fm_folder_view_set_active(folder_view, (page == (*win).current_page) as c_int);
}
unsafe extern "C" fn on_folder_view_filter_changed(
    mut fv: *mut FmFolderView,
    mut win: *mut FmMainWin,
) {
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    let mut active: gboolean = 0;
    if fv != (*win).folder_view {
        return;
    }
    active = fm_folder_view_get_show_hidden(fv);
    act = gtk_ui_manager_get_action(
        (*win).ui,
        b"/menubar/ViewMenu/ShowHidden\0" as *const u8 as *const gchar,
    );
    gtk_toggle_action_set_active(act as *mut c_void as *mut GtkToggleAction, active);
    if active != (*(*win).current_page).show_hidden() {
        (*(*win).current_page).set_show_hidden(active as gboolean);
        if (*(*win).current_page).own_config() != 0 {
            fm_app_config_save_config_for_path(
                fm_folder_view_get_cwd(fv),
                (*(*win).current_page).sort_type,
                (*(*win).current_page).sort_by,
                4294967295 as FmStandardViewMode,
                active,
                ptr::null_mut::<*mut c_char>(),
            );
        } else {
            (*(fm_config as *mut FmAppConfig)).show_hidden = active;
            pcmanfm_save_config(0 as gboolean);
        }
    }
}
#[inline]
unsafe extern "C" fn _find_tab_page(
    mut win: *mut FmMainWin,
    mut fv: *mut FmFolderView,
) -> *mut FmTabPage {
    let mut page: *mut FmTabPage = ptr::null_mut::<FmTabPage>();
    let mut n: gint = 0;
    n = gtk_notebook_get_n_pages((*win).notebook);
    while n > 0i32 {
        n -= 1;
        page = gtk_notebook_get_nth_page((*win).notebook, n) as *mut c_void as *mut FmTabPage;
        if fv == fm_tab_page_get_folder_view(page) {
            return page;
        }
    }
    return ptr::null_mut::<FmTabPage>();
}
unsafe extern "C" fn on_notebook_switch_page(
    mut nb: *mut GtkNotebook,
    mut new_page: *mut gpointer,
    mut num: guint,
    mut win: *mut FmMainWin,
) {
    let mut sw_page: *mut GtkWidget = gtk_notebook_get_nth_page(nb, num as gint);
    let mut page: *mut FmTabPage = ptr::null_mut::<FmTabPage>();
    let mut old_view: *mut FmFolderView = ptr::null_mut::<FmFolderView>();
    let mut passive_view: *mut FmFolderView = ptr::null_mut::<FmFolderView>();
    page = sw_page as *mut FmTabPage;
    if !(*win).folder_view.is_null() {
        fm_folder_view_set_active((*win).folder_view, 0 as gboolean);
        g_object_unref((*win).folder_view as gpointer);
    }
    if !(*win).current_page.is_null() {
        passive_view = fm_tab_page_get_passive_view((*win).current_page);
        old_view = fm_tab_page_get_folder_view((*win).current_page);
    }
    (*win).current_page = page;
    (*win).folder_view = fm_tab_page_get_folder_view(page);
    if !(*win).folder_view.is_null() {
        g_object_ref((*win).folder_view as gpointer);
    }
    (*win).nav_history = fm_tab_page_get_history(page);
    (*win).side_pane = fm_tab_page_get_side_pane(page);
    if (*win).enable_passive_view != 0 && !passive_view.is_null() {
        let mut psv_page: *mut FmTabPage = ptr::null_mut::<FmTabPage>();
        if (*win).folder_view == passive_view {
            (*win).passive_view_on_right = (if (*win).passive_view_on_right != 0 {
                0i32
            } else {
                (0i32 == 0) as c_int
            }) as gboolean;
            passive_view = old_view;
        }
        fm_tab_page_set_passive_view(page, passive_view, (*win).passive_view_on_right);
        psv_page = _find_tab_page(win, passive_view);
        if !psv_page.is_null() {
            gtk_widget_set_state(
                (*psv_page).tab_label as *mut c_void as *mut GtkWidget,
                GTK_STATE_SELECTED,
            );
        }
    } else {
        fm_tab_page_take_view_back(page);
    }
    fm_folder_view_set_active((*win).folder_view, (0i32 == 0) as c_int);
    g_log(
        ptr::null_mut::<gchar>(),
        G_LOG_LEVEL_DEBUG,
        b"reactivated gestures to page %u\0" as *const u8 as *const gchar,
        num,
    );
    on_folder_view_sel_changed(
        (*win).folder_view,
        fm_folder_view_get_n_selected_files((*win).folder_view),
        win,
    );
    _update_hist_buttons(win);
    on_folder_view_filter_changed((*win).folder_view, win);
    if (*(fm_config as *mut FmAppConfig)).side_pane_mode as c_uint & 0x20 as c_uint != 0 {
        gtk_widget_hide((*win).side_pane as *mut c_void as *mut GtkWidget);
    } else {
        fm_side_pane_set_mode(
            (*win).side_pane,
            ((*(fm_config as *mut FmAppConfig)).side_pane_mode as c_uint & 0x1f as c_uint)
                as FmSidePaneMode,
        );
        gtk_widget_show_all((*win).side_pane as *mut c_void as *mut GtkWidget);
    }
    fm_path_entry_set_path((*win).location, fm_tab_page_get_cwd(page));
    fm_path_bar_set_path((*win).path_bar, fm_tab_page_get_cwd(page));
    gtk_window_set_title(
        win as *mut GtkWindow,
        fm_tab_page_get_title(page) as *const gchar,
    );
    update_sort_menu(win);
    update_view_menu(win);
    update_file_menu(win, fm_tab_page_get_cwd(page));
    update_statusbar(win);
    if (*win).idle_handler == 0 as guint {
        (*win).idle_handler = gdk_threads_add_idle_full(
            300 as gint,
            Some(idle_focus_view as unsafe extern "C" fn(gpointer) -> gboolean),
            win as gpointer,
            None,
        );
    }
}
unsafe extern "C" fn on_notebook_page_added(
    mut nb: *mut GtkNotebook,
    mut page: *mut GtkWidget,
    mut num: guint,
    mut win: *mut FmMainWin,
) {
    let mut tab_page: *mut FmTabPage = page as *mut c_void as *mut FmTabPage;
    g_signal_connect_data(
        tab_page as gpointer,
        b"notify::position\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkPaned, *mut GParamSpec, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_tab_page_splitter_pos_changed
                as unsafe extern "C" fn(*mut GtkPaned, *mut GParamSpec, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        tab_page as gpointer,
        b"chdir\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmTabPage, *mut FmPath, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_tab_page_chdir
                as unsafe extern "C" fn(*mut FmTabPage, *mut FmPath, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        tab_page as gpointer,
        b"status\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut FmTabPage, guint, *const c_char, *mut FmMainWin) -> (),
            >,
            GCallback,
        >(Some(
            on_tab_page_status_text
                as unsafe extern "C" fn(*mut FmTabPage, guint, *const c_char, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        tab_page as gpointer,
        b"got-focus\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmTabPage, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_tab_page_got_focus as unsafe extern "C" fn(*mut FmTabPage, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        tab_page as gpointer,
        b"loaded\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmTabPage, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_tab_page_loaded as unsafe extern "C" fn(*mut FmTabPage, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*tab_page).folder_view as gpointer,
        b"sort-changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmFolderView, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_folder_view_sort_changed
                as unsafe extern "C" fn(*mut FmFolderView, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*tab_page).folder_view as gpointer,
        b"sel-changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmFolderView, gint, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_folder_view_sel_changed
                as unsafe extern "C" fn(*mut FmFolderView, gint, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*tab_page).folder_view as gpointer,
        b"filter-changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmFolderView, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_folder_view_filter_changed
                as unsafe extern "C" fn(*mut FmFolderView, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*tab_page).folder_view as gpointer,
        b"clicked\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolderView,
                    FmFolderViewClickType,
                    *mut FmFileInfo,
                    *mut FmMainWin,
                ) -> (),
            >,
            GCallback,
        >(Some(
            on_folder_view_clicked
                as unsafe extern "C" fn(
                    *mut FmFolderView,
                    FmFolderViewClickType,
                    *mut FmFileInfo,
                    *mut FmMainWin,
                ) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*tab_page).side_pane as gpointer,
        b"mode-changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmSidePane, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_side_pane_mode_changed
                as unsafe extern "C" fn(*mut FmSidePane, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*tab_page).side_pane as gpointer,
        b"chdir\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmSidePane, guint, *mut FmPath, *mut FmMainWin) -> ()>,
            GCallback,
        >(Some(
            on_side_pane_chdir
                as unsafe extern "C" fn(*mut FmSidePane, guint, *mut FmPath, *mut FmMainWin) -> (),
        )),
        win as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    fm_folder_view_add_popup(
        (*tab_page).folder_view,
        win as *mut c_void as *mut GtkWindow,
        None,
    );
    fm_folder_view_set_active((*tab_page).folder_view, 0 as gboolean);
    if gtk_notebook_get_n_pages(nb) > 1i32
        || (*(fm_config as *mut FmAppConfig)).always_show_tabs != 0
    {
        gtk_notebook_set_show_tabs(nb, (0i32 == 0) as c_int);
    } else {
        gtk_notebook_set_show_tabs(nb, 0 as gboolean);
    };
}
unsafe extern "C" fn on_notebook_page_removed(
    mut nb: *mut GtkNotebook,
    mut page: *mut GtkWidget,
    mut num: guint,
    mut win: *mut FmMainWin,
) {
    let mut tab_page: *mut FmTabPage = page as *mut c_void as *mut FmTabPage;
    let mut folder_view: *mut FmFolderView = fm_tab_page_get_folder_view(tab_page);
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    g_log(
        ptr::null_mut::<gchar>(),
        G_LOG_LEVEL_DEBUG,
        b"FmMainWin[%p]: removed page %u[%p](view=%p); %u pages left\0" as *const u8
            as *const gchar,
        win,
        num,
        page,
        folder_view,
        gtk_notebook_get_n_pages(nb),
    );
    g_signal_handlers_disconnect_matched(
        tab_page as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkPaned, *mut GParamSpec, *mut FmMainWin) -> ()>,
            gpointer,
        >(Some(
            on_tab_page_splitter_pos_changed
                as unsafe extern "C" fn(*mut GtkPaned, *mut GParamSpec, *mut FmMainWin) -> (),
        )),
        win as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        tab_page as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmTabPage, *mut FmPath, *mut FmMainWin) -> ()>,
            gpointer,
        >(Some(
            on_tab_page_chdir
                as unsafe extern "C" fn(*mut FmTabPage, *mut FmPath, *mut FmMainWin) -> (),
        )),
        win as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        tab_page as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut FmTabPage, guint, *const c_char, *mut FmMainWin) -> (),
            >,
            gpointer,
        >(Some(
            on_tab_page_status_text
                as unsafe extern "C" fn(*mut FmTabPage, guint, *const c_char, *mut FmMainWin) -> (),
        )),
        win as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        tab_page as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmTabPage, *mut FmMainWin) -> ()>,
            gpointer,
        >(Some(
            on_tab_page_got_focus as unsafe extern "C" fn(*mut FmTabPage, *mut FmMainWin) -> (),
        )),
        win as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        tab_page as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmTabPage, *mut FmMainWin) -> ()>,
            gpointer,
        >(Some(
            on_tab_page_loaded as unsafe extern "C" fn(*mut FmTabPage, *mut FmMainWin) -> (),
        )),
        win as gpointer,
    );
    if !folder_view.is_null() {
        g_signal_handlers_disconnect_matched(
            folder_view as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut FmFolderView,
                        *mut GdkEventKey,
                        *mut FmMainWin,
                    ) -> gboolean,
                >,
                gpointer,
            >(Some(
                on_view_key_press_event
                    as unsafe extern "C" fn(
                        *mut FmFolderView,
                        *mut GdkEventKey,
                        *mut FmMainWin,
                    ) -> gboolean,
            )),
            win as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            folder_view as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolderView, *mut FmMainWin) -> ()>,
                gpointer,
            >(Some(
                on_folder_view_sort_changed
                    as unsafe extern "C" fn(*mut FmFolderView, *mut FmMainWin) -> (),
            )),
            win as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            folder_view as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolderView, gint, *mut FmMainWin) -> ()>,
                gpointer,
            >(Some(
                on_folder_view_sel_changed
                    as unsafe extern "C" fn(*mut FmFolderView, gint, *mut FmMainWin) -> (),
            )),
            win as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            folder_view as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolderView, *mut FmMainWin) -> ()>,
                gpointer,
            >(Some(
                on_folder_view_filter_changed
                    as unsafe extern "C" fn(*mut FmFolderView, *mut FmMainWin) -> (),
            )),
            win as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            folder_view as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut FmFolderView,
                        FmFolderViewClickType,
                        *mut FmFileInfo,
                        *mut FmMainWin,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                on_folder_view_clicked
                    as unsafe extern "C" fn(
                        *mut FmFolderView,
                        FmFolderViewClickType,
                        *mut FmFileInfo,
                        *mut FmMainWin,
                    ) -> (),
            )),
            win as gpointer,
        );
        if !(*win).current_page.is_null()
            && (*win).enable_passive_view != 0
            && fm_tab_page_get_passive_view((*win).current_page) == folder_view
        {
            (*win).enable_passive_view = 0 as gboolean;
            act = gtk_ui_manager_get_action(
                (*win).ui,
                b"/menubar/ViewMenu/DualPane\0" as *const u8 as *const gchar,
            );
            gtk_toggle_action_set_active(act as *mut c_void as *mut GtkToggleAction, 0 as gboolean);
        }
    }
    g_signal_handlers_disconnect_matched(
        (*tab_page).side_pane as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmSidePane, *mut FmMainWin) -> ()>,
            gpointer,
        >(Some(
            on_side_pane_mode_changed
                as unsafe extern "C" fn(*mut FmSidePane, *mut FmMainWin) -> (),
        )),
        win as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*tab_page).side_pane as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmSidePane, guint, *mut FmPath, *mut FmMainWin) -> ()>,
            gpointer,
        >(Some(
            on_side_pane_chdir
                as unsafe extern "C" fn(*mut FmSidePane, guint, *mut FmPath, *mut FmMainWin) -> (),
        )),
        win as gpointer,
    );
    if tab_page == (*win).current_page {
        (*win).current_page = ptr::null_mut::<FmTabPage>();
        if !(*win).folder_view.is_null() {
            g_object_unref((*win).folder_view as gpointer);
        }
        (*win).folder_view = ptr::null_mut::<FmFolderView>();
        (*win).nav_history = ptr::null_mut::<FmNavHistory>();
        (*win).side_pane = ptr::null_mut::<FmSidePane>();
    }
    if gtk_notebook_get_n_pages(nb) > 1i32
        || (*(fm_config as *mut FmAppConfig)).always_show_tabs != 0
    {
        gtk_notebook_set_show_tabs(nb, (0i32 == 0) as c_int);
    } else {
        gtk_notebook_set_show_tabs(nb, 0 as gboolean);
    }
    if gtk_notebook_get_n_pages(nb) == 0i32 {
        gtk_widget_destroy(win as *mut c_void as *mut GtkWidget);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fm_main_win_get_last_active() -> *mut FmMainWin {
    return if !all_wins.is_null() {
        (*all_wins).data as *mut FmMainWin
    } else {
        ptr::null_mut::<FmMainWin>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn fm_main_win_open_in_last_active(mut path: *mut FmPath) {
    let mut win: *mut FmMainWin = fm_main_win_get_last_active();
    if win.is_null() {
        win = fm_main_win_add_win(ptr::null_mut::<FmMainWin>(), path);
    } else {
        fm_main_win_add_tab(win, path);
    }
    gtk_window_present(win as *mut c_void as *mut GtkWindow);
}
unsafe extern "C" fn switch_to_next_tab(mut win: *mut FmMainWin) {
    let mut n: c_int = gtk_notebook_get_current_page((*win).notebook) as c_int;
    if n < gtk_notebook_get_n_pages((*win).notebook) as c_int - 1i32 {
        n += 1;
    } else {
        n = 0i32;
    }
    gtk_notebook_set_current_page((*win).notebook, n as gint);
}
unsafe extern "C" fn switch_to_prev_tab(mut win: *mut FmMainWin) {
    let mut n: c_int = gtk_notebook_get_current_page((*win).notebook) as c_int;
    if n > 0i32 {
        n -= 1;
    } else {
        n = gtk_notebook_get_n_pages((*win).notebook) as c_int - 1i32;
    }
    gtk_notebook_set_current_page((*win).notebook, n as gint);
}
unsafe extern "C" fn on_key_press_event(
    mut w: *mut GtkWidget,
    mut evt: *mut GdkEventKey,
) -> gboolean {
    let mut win: *mut FmMainWin = w as *mut c_void as *mut FmMainWin;
    let mut modifier: c_int =
        ((*evt).state as c_uint & gtk_accelerator_get_default_mod_mask() as c_uint) as c_int;
    if modifier == GDK_MOD1_MASK as c_int {
        if (*evt).keyval >= '0' as i32 as guint && (*evt).keyval <= '9' as i32 as guint {
            let mut n: c_int = 0;
            if (*evt).keyval == '0' as i32 as guint {
                n = 9 as c_int;
            } else {
                n = (*evt).keyval.wrapping_sub('1' as i32 as guint) as c_int;
            }
            gtk_notebook_set_current_page((*win).notebook, n as gint);
            return (0i32 == 0) as c_int;
        }
    } else if modifier == GDK_CONTROL_MASK as c_int {
        if (*evt).keyval == 0xff09 as guint
            || (*evt).keyval == 0xfe20 as guint
            || (*evt).keyval == 0xff56 as guint
        {
            switch_to_next_tab(win);
            return (0i32 == 0) as c_int;
        } else if (*evt).keyval == 0xff55 as guint {
            switch_to_prev_tab(win);
            return (0i32 == 0) as c_int;
        }
    } else if modifier == GDK_CONTROL_MASK as c_int | GDK_SHIFT_MASK as c_int {
        if (*evt).keyval == 0xff09 as guint || (*evt).keyval == 0xfe20 as guint {
            switch_to_prev_tab(win);
            return (0i32 == 0) as c_int;
        }
    } else if (*evt).keyval == '/' as i32 as guint || (*evt).keyval == '~' as i32 as guint {
        if gtk_widget_is_focus((*win).location as *mut c_void as *mut GtkWidget) == 0 {
            gtk_widget_grab_focus((*win).location as *mut c_void as *mut GtkWidget);
            let mut path: [c_char; 2] = [(*evt).keyval as c_char, 0 as c_char];
            gtk_entry_set_text(
                (*win).location as *mut c_void as *mut GtkEntry,
                &raw mut path as *mut c_char,
            );
            gtk_editable_set_position(
                (*win).location as *mut c_void as *mut GtkEditable,
                -(1 as gint),
            );
            return (0i32 == 0) as c_int;
        }
    } else if (*evt).keyval == 0xff1b as guint {
        if gtk_widget_is_focus((*win).location as *mut c_void as *mut GtkWidget) != 0 {
            gtk_widget_grab_focus((*win).folder_view as *mut c_void as *mut GtkWidget);
            fm_path_entry_set_path((*win).location, fm_tab_page_get_cwd((*win).current_page));
            fm_path_bar_set_path((*win).path_bar, fm_tab_page_get_cwd((*win).current_page));
            return (0i32 == 0) as c_int;
        }
    }
    return (*(fm_main_win_parent_class as *mut GtkWidgetClass))
        .key_press_event
        .expect("non-null function pointer")(w, evt);
}
unsafe extern "C" fn on_button_press_event(
    mut w: *mut GtkWidget,
    mut evt: *mut GdkEventButton,
) -> gboolean {
    let mut win: *mut FmMainWin = w as *mut c_void as *mut FmMainWin;
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    if (*evt).button == 8 as guint {
        act = gtk_ui_manager_get_action((*win).ui, b"/Prev2\0" as *const u8 as *const gchar);
        gtk_action_activate(act);
    } else if (*evt).button == 9 as guint {
        act = gtk_ui_manager_get_action((*win).ui, b"/Next2\0" as *const u8 as *const gchar);
        gtk_action_activate(act);
    }
    if (*(fm_main_win_parent_class as *mut GtkWidgetClass))
        .button_press_event
        .is_some()
    {
        return (*(fm_main_win_parent_class as *mut GtkWidgetClass))
            .button_press_event
            .expect("non-null function pointer")(w, evt);
    } else {
        return 0 as gboolean;
    };
}
unsafe extern "C" fn on_scroll_event(
    mut w: *mut GtkWidget,
    mut evt: *mut GdkEventScroll,
) -> gboolean {
    let mut win: *mut FmMainWin = w as *mut c_void as *mut FmMainWin;
    let mut act: *mut GtkAction = ptr::null_mut::<GtkAction>();
    let mut modifier: c_int =
        ((*evt).state as c_uint & gtk_accelerator_get_default_mod_mask() as c_uint) as c_int;
    if !(modifier != GDK_CONTROL_MASK as c_int) {
        if (*evt).direction as c_uint == GDK_SCROLL_UP as c_int as c_uint {
            act = gtk_ui_manager_get_action(
                (*win).ui,
                b"/menubar/ViewMenu/SizeBigger\0" as *const u8 as *const gchar,
            );
            gtk_action_activate(act);
            return (0i32 == 0) as c_int;
        } else if (*evt).direction as c_uint == GDK_SCROLL_DOWN as c_int as c_uint {
            act = gtk_ui_manager_get_action(
                (*win).ui,
                b"/menubar/ViewMenu/SizeSmaller\0" as *const u8 as *const gchar,
            );
            gtk_action_activate(act);
            return (0i32 == 0) as c_int;
        }
    }
    if (*(fm_main_win_parent_class as *mut GtkWidgetClass))
        .scroll_event
        .is_some()
    {
        return (*(fm_main_win_parent_class as *mut GtkWidgetClass))
            .scroll_event
            .expect("non-null function pointer")(w, evt);
    } else {
        return 0 as gboolean;
    };
}
unsafe extern "C" fn on_reload(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut page: *mut FmTabPage = (*win).current_page;
    fm_tab_page_reload(page);
}
unsafe extern "C" fn on_filter(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut page: *mut FmTabPage = (*win).current_page;
    let mut old_filter: *const c_char = if !(*page).filter_pattern.is_null() {
        (*page).filter_pattern as *const c_char
    } else {
        c"*".as_ptr()
    };
    let mut new_filter: *mut c_char = ptr::null_mut::<c_char>();
    new_filter = fm_get_user_input(
        win as *mut c_void as *mut GtkWindow,
        dcgettext(ptr::null::<c_char>(), c"Select filter".as_ptr(), 5 as c_int),
        dcgettext(
            ptr::null::<c_char>(),
            c"Choose a new shell pattern to show files:".as_ptr(),
            5 as c_int,
        ),
        old_filter,
    );
    if new_filter.is_null() {
        return;
    }
    if strcmp(new_filter, c"*".as_ptr()) != 0 {
        fm_tab_page_set_filter_pattern(page, new_filter);
    } else {
        fm_tab_page_set_filter_pattern(page, ptr::null::<c_char>());
    }
    g_free(new_filter as gpointer);
    gtk_window_set_title(
        win as *mut c_void as *mut GtkWindow,
        fm_tab_page_get_title(page) as *const gchar,
    );
}
unsafe extern "C" fn on_show_side_pane(mut act: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut active: gboolean = 0;
    active = gtk_toggle_action_get_active(act);
    if active != 0 {
        let ref mut fresh30 = (*(fm_config as *mut FmAppConfig)).side_pane_mode;
        *fresh30 =
            mem::transmute::<c_uint, FmSidePaneMode>(*fresh30 as c_uint & !(0x20i32) as c_uint);
        gtk_widget_show_all((*win).side_pane as *mut c_void as *mut GtkWidget);
    } else {
        let ref mut fresh31 = (*(fm_config as *mut FmAppConfig)).side_pane_mode;
        *fresh31 = mem::transmute::<c_uint, FmSidePaneMode>(*fresh31 as c_uint | 0x20 as c_uint);
        gtk_widget_hide((*win).side_pane as *mut c_void as *mut GtkWidget);
    }
    pcmanfm_save_config(0 as gboolean);
}
unsafe extern "C" fn on_dual_pane(mut act: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut active: gboolean = 0;
    let mut num: c_int = 0;
    let mut fv: *mut FmFolderView = ptr::null_mut::<FmFolderView>();
    let mut page: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
    active = gtk_toggle_action_get_active(act);
    if active != 0 && (*win).enable_passive_view == 0 {
        num = gtk_notebook_get_n_pages((*win).notebook) as c_int;
        if num < 2 as c_int {
            (*win).passive_view_on_right = 0 as gboolean;
            num = fm_main_win_add_tab(win, fm_tab_page_get_cwd((*win).current_page)) as c_int;
            g_log(
                ptr::null_mut::<gchar>(),
                G_LOG_LEVEL_DEBUG,
                b"on_dual_pane: added duplicate of the single page\0" as *const u8 as *const gchar,
            );
            page = gtk_notebook_get_nth_page((*win).notebook, 0 as gint);
        } else if gtk_notebook_get_current_page((*win).notebook) < num - 1i32 {
            (*win).passive_view_on_right = (0i32 == 0) as c_int as gboolean;
            num = gtk_notebook_get_current_page((*win).notebook) as c_int + 1i32;
            g_log(
                ptr::null_mut::<gchar>(),
                G_LOG_LEVEL_DEBUG,
                b"on_dual_pane: adding passive page %d to right pane\0" as *const u8
                    as *const gchar,
                num,
            );
            page = gtk_notebook_get_nth_page((*win).notebook, num as gint);
        } else {
            (*win).passive_view_on_right = 0 as gboolean;
            g_log(
                ptr::null_mut::<gchar>(),
                G_LOG_LEVEL_DEBUG,
                b"on_dual_pane: adding passive page %d to left pane\0" as *const u8 as *const gchar,
                num - 2 as c_int,
            );
            page = gtk_notebook_get_nth_page((*win).notebook, num as gint - 2 as gint);
        }
        fv = fm_tab_page_get_folder_view(page as *mut c_void as *mut FmTabPage);
        fm_tab_page_set_passive_view((*win).current_page, fv, (*win).passive_view_on_right);
        gtk_widget_set_state(
            (*(page as *mut c_void as *mut FmTabPage)).tab_label as *mut c_void as *mut GtkWidget,
            GTK_STATE_SELECTED,
        );
        (*win).enable_passive_view = (0i32 == 0) as c_int as gboolean;
    } else if active == 0 && (*win).enable_passive_view != 0 {
        fv = fm_tab_page_get_passive_view((*win).current_page);
        if !fv.is_null() {
            let mut tp: *mut FmTabPage = _find_tab_page(win, fv);
            if !tp.is_null() {
                fm_tab_page_take_view_back(tp);
                gtk_widget_set_state(
                    (*(tp as *mut c_void as *mut FmTabPage)).tab_label as *mut c_void
                        as *mut GtkWidget,
                    GTK_STATE_ACTIVE,
                );
            }
        }
        (*win).enable_passive_view = 0 as gboolean;
    }
}
unsafe extern "C" fn on_show_status(mut action: *mut GtkToggleAction, mut win: *mut FmMainWin) {
    let mut active: gboolean = 0;
    active = gtk_toggle_action_get_active(action);
    if active != (*(fm_config as *mut FmAppConfig)).show_statusbar {
        (*(fm_config as *mut FmAppConfig)).show_statusbar = active;
        fm_config_emit_changed(fm_config, c"statusbar".as_ptr());
        pcmanfm_save_config(0 as gboolean);
    }
}
