// SPDX-License-Identifier: GPL-2.0-or-later
/* Additional entries for FmFileMenu popup */

static mut signals: [guint; 5] = [0; 5];
static mut popup_qdata: GQuark = 0;
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_get_type() -> GType {
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
        let mut g_define_type_id: GType = fm_tab_page_get_type_once();
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
unsafe extern "C" fn fm_tab_page_class_intern_init(mut klass: gpointer) {
    fm_tab_page_parent_class = g_type_class_peek_parent(klass);
    if FmTabPage_private_offset != 0i32 {
        g_type_class_adjust_private_offset(klass, &raw mut FmTabPage_private_offset);
    }
    fm_tab_page_class_init(klass as *mut FmTabPageClass);
}
static mut FmTabPage_private_offset: gint = 0;
static mut fm_tab_page_parent_class: gpointer = ptr::null::<c_void>() as *mut c_void;
#[inline(never)]
unsafe extern "C" fn fm_tab_page_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        gtk_hpaned_get_type(),
        g_intern_static_string(b"FmTabPage\0" as *const u8 as *const gchar),
        mem::size_of::<FmTabPageClass>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(mem::transmute::<
            Option<unsafe extern "C" fn(gpointer) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            fm_tab_page_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
        ))),
        mem::size_of::<FmTabPage>() as guint,
        mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmTabPage) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                fm_tab_page_init as unsafe extern "C" fn(*mut FmTabPage) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fm_tab_page_class_init(mut klass: *mut FmTabPageClass) {
    let mut g_object_class: *mut GObjectClass = klass as *mut c_void as *mut GObjectClass;
    let mut widget_class: *mut GtkWidgetClass = klass as *mut c_void as *mut GtkWidgetClass;
    (*widget_class).destroy =
        Some(fm_tab_page_destroy as unsafe extern "C" fn(*mut GtkWidget) -> ())
            as Option<unsafe extern "C" fn(*mut GtkWidget) -> ()>;
    (*g_object_class).finalize =
        Some(fm_tab_page_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*widget_class).realize =
        Some(fm_tab_page_realize as unsafe extern "C" fn(*mut GtkWidget) -> ())
            as Option<unsafe extern "C" fn(*mut GtkWidget) -> ()>;
    (*widget_class).unrealize =
        Some(fm_tab_page_unrealize as unsafe extern "C" fn(*mut GtkWidget) -> ())
            as Option<unsafe extern "C" fn(*mut GtkWidget) -> ()>;
    signals[CHDIR as c_int as usize] = g_signal_new(
        b"chdir\0" as *const u8 as *const gchar,
        (*(klass as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_FIRST,
        1056 as c_ulong as glong as guint,
        None,
        ptr::null_mut::<c_void>(),
        Some(
            g_cclosure_marshal_VOID__POINTER
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        ((1i32) << 2 as c_int) as GType,
        1 as guint,
        ((17 as c_int) << 2 as c_int) as GType,
    );
    signals[STATUS as c_int as usize] = g_signal_new(
        b"status\0" as *const u8 as *const gchar,
        (*(klass as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_FIRST,
        1064 as c_ulong as glong as guint,
        None,
        ptr::null_mut::<c_void>(),
        Some(
            g_cclosure_marshal_VOID__UINT_POINTER
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        ((1i32) << 2 as c_int) as GType,
        2 as guint,
        ((7 as c_int) << 2 as c_int) as GType,
        ((17 as c_int) << 2 as c_int) as GType,
    );
    signals[GOT_FOCUS as c_int as usize] = g_signal_new(
        b"got-focus\0" as *const u8 as *const gchar,
        (*(klass as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_FIRST,
        1072 as c_ulong as glong as guint,
        None,
        ptr::null_mut::<c_void>(),
        Some(
            g_cclosure_marshal_VOID__VOID
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        ((1i32) << 2 as c_int) as GType,
        0 as guint,
    );
    signals[LOADED as c_int as usize] = g_signal_new(
        b"loaded\0" as *const u8 as *const gchar,
        (*(klass as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_FIRST,
        1080 as c_ulong as glong as guint,
        None,
        ptr::null_mut::<c_void>(),
        Some(
            g_cclosure_marshal_VOID__VOID
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        ((1i32) << 2 as c_int) as GType,
        0 as guint,
    );
    popup_qdata =
        g_quark_from_static_string(b"tab-page::popup-filelist\0" as *const u8 as *const gchar);
}
unsafe extern "C" fn fm_tab_page_finalize(mut object: *mut GObject) {
    let mut page: *mut FmTabPage = ptr::null_mut::<FmTabPage>();
    let mut i: c_int = 0;
    page = object as *mut FmTabPage;
    i = 0i32;
    while i < FM_STATUS_TEXT_NUM as c_int {
        g_free((*page).status_text[i as usize] as gpointer);
        i += 1;
    }
    g_free((*page).filter_pattern as gpointer);
    (*(fm_tab_page_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn free_folder(mut page: *mut FmTabPage) {
    if (*page).update_scroll_id != 0 {
        g_source_remove((*page).update_scroll_id);
        (*page).update_scroll_id = 0 as guint;
    }
    if !(*page).folder.is_null() {
        g_signal_handlers_disconnect_matched(
            (*page).folder as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
                gpointer,
            >(Some(
                tab_page_on_folder_start_loading
                    as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
            )),
            page as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*page).folder as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
                gpointer,
            >(Some(
                tab_page_on_folder_finish_loading
                    as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
            )),
            page as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*page).folder as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
                gpointer,
            >(Some(
                on_folder_fs_info as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
            )),
            page as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*page).folder as gpointer,
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
                        *mut FmTabPage,
                    ) -> FmJobErrorAction,
                >,
                gpointer,
            >(Some(
                tab_page_on_folder_error
                    as unsafe extern "C" fn(
                        *mut FmFolder,
                        *mut GError,
                        FmJobErrorSeverity,
                        *mut FmTabPage,
                    ) -> FmJobErrorAction,
            )),
            page as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*page).folder as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
                gpointer,
            >(Some(
                on_folder_content_changed
                    as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
            )),
            page as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*page).folder as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
                gpointer,
            >(Some(
                on_folder_removed as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
            )),
            page as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*page).folder as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
                gpointer,
            >(Some(
                on_folder_unmount as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
            )),
            page as gpointer,
        );
        g_object_unref((*page).folder as gpointer);
        (*page).folder = ptr::null_mut::<FmFolder>();
        if !(*page).want_focus.is_null() {
            fm_path_unref((*page).want_focus);
        }
        (*page).want_focus = ptr::null_mut::<FmPath>();
    }
}
unsafe extern "C" fn on_folder_view_add(
    mut container: *mut GtkContainer,
    mut child: *mut GtkWidget,
    mut page: *mut FmTabPage,
) {
    g_signal_connect_data(
        child as gpointer,
        b"focus-in-event\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut FmTabPage) -> gboolean>,
            GCallback,
        >(Some(
            on_folder_view_focus_in
                as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut FmTabPage) -> gboolean,
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
}
unsafe extern "C" fn on_folder_view_remove(
    mut container: *mut GtkContainer,
    mut child: *mut GtkWidget,
    mut page: *mut FmTabPage,
) {
    g_signal_handlers_disconnect_matched(
        child as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut FmTabPage) -> gboolean>,
            gpointer,
        >(Some(
            on_folder_view_focus_in
                as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut FmTabPage) -> gboolean,
        )),
        page as gpointer,
    );
}
unsafe extern "C" fn _connect_focus_in(
    mut folder_view: *mut FmFolderView,
    mut page: *mut FmTabPage,
) {
    let mut children: *mut GList = ptr::null_mut::<GList>();
    let mut l: *mut GList = ptr::null_mut::<GList>();
    g_signal_connect_data(
        folder_view as gpointer,
        b"focus-in-event\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut FmTabPage) -> gboolean>,
            GCallback,
        >(Some(
            on_folder_view_focus_in
                as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut FmTabPage) -> gboolean,
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        folder_view as gpointer,
        b"add\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut FmTabPage) -> ()>,
            GCallback,
        >(Some(
            on_folder_view_add
                as unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut FmTabPage) -> (),
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        folder_view as gpointer,
        b"remove\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut FmTabPage) -> ()>,
            GCallback,
        >(Some(
            on_folder_view_remove
                as unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut FmTabPage) -> (),
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    children = gtk_container_get_children(folder_view as *mut c_void as *mut GtkContainer);
    l = children;
    while !l.is_null() {
        g_signal_connect_data(
            (*l).data,
            b"focus-in-event\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut FmTabPage) -> gboolean,
                >,
                GCallback,
            >(Some(
                on_folder_view_focus_in
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEvent,
                        *mut FmTabPage,
                    ) -> gboolean,
            )),
            page as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        l = (*l).next;
    }
    g_list_free(children);
}
unsafe extern "C" fn _disconnect_focus_in(
    mut folder_view: *mut FmFolderView,
    mut page: *mut FmTabPage,
) {
    let mut children: *mut GList = ptr::null_mut::<GList>();
    let mut l: *mut GList = ptr::null_mut::<GList>();
    g_signal_handlers_disconnect_matched(
        folder_view as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut FmTabPage) -> gboolean>,
            gpointer,
        >(Some(
            on_folder_view_focus_in
                as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut FmTabPage) -> gboolean,
        )),
        page as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        folder_view as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut FmTabPage) -> ()>,
            gpointer,
        >(Some(
            on_folder_view_add
                as unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut FmTabPage) -> (),
        )),
        page as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        folder_view as gpointer,
        (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ptr::null_mut::<GClosure>(),
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut FmTabPage) -> ()>,
            gpointer,
        >(Some(
            on_folder_view_remove
                as unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut FmTabPage) -> (),
        )),
        page as gpointer,
    );
    children = gtk_container_get_children(folder_view as *mut c_void as *mut GtkContainer);
    l = children;
    while !l.is_null() {
        g_signal_handlers_disconnect_matched(
            (*l).data,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut FmTabPage) -> gboolean,
                >,
                gpointer,
            >(Some(
                on_folder_view_focus_in
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEvent,
                        *mut FmTabPage,
                    ) -> gboolean,
            )),
            page as gpointer,
        );
        l = (*l).next;
    }
    g_list_free(children);
}
unsafe extern "C" fn tab_page_on_home_path_changed(
    mut cfg: *mut FmAppConfig,
    mut sp: *mut FmSidePane,
) {
    if !(*cfg).home_path.is_null() && *(*cfg).home_path.offset(0 as isize) as c_int != 0 {
        fm_side_pane_set_home_dir(sp, (*cfg).home_path);
    } else {
        fm_side_pane_set_home_dir(sp, fm_get_home_dir());
    };
}
unsafe extern "C" fn fm_tab_page_destroy(mut object: *mut GtkWidget) {
    let mut page: *mut FmTabPage = object as *mut c_void as *mut FmTabPage;
    g_log(
        ptr::null_mut::<gchar>(),
        G_LOG_LEVEL_DEBUG,
        b"fm_tab_page_destroy, folder: %s\0" as *const u8 as *const gchar,
        if !(*page).folder.is_null() {
            fm_path_get_basename(fm_folder_get_path((*page).folder))
        } else {
            c"(none)".as_ptr()
        },
    );
    free_folder(page);
    if !(*page).nav_history.is_null() {
        g_object_unref((*page).nav_history as gpointer);
        (*page).nav_history = ptr::null_mut::<FmNavHistory>();
    }
    if !(*page).folder_view.is_null() {
        let mut fv: *mut GtkWidget = (*page).folder_view as *mut c_void as *mut GtkWidget;
        let mut parent: *mut GtkWidget = gtk_widget_get_parent(fv);
        let mut panes: *mut GList = ptr::null_mut::<GList>();
        let mut l: *mut GList = ptr::null_mut::<GList>();
        if !parent.is_null() {
            panes = gtk_container_get_children((*page).views as *mut c_void as *mut GtkContainer);
            l = panes;
            while !l.is_null() {
                if (*l).data as *mut GtkWidget == fv {
                    break;
                }
                l = (*l).next;
            }
            if l.is_null() {
                gtk_container_remove(parent as *mut c_void as *mut GtkContainer, fv);
            }
            g_list_free(panes);
        }
        g_signal_handlers_disconnect_matched(
            (*page).folder_view as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolderView, gint, *mut FmTabPage) -> ()>,
                gpointer,
            >(Some(
                tab_page_on_folder_view_sel_changed
                    as unsafe extern "C" fn(*mut FmFolderView, gint, *mut FmTabPage) -> (),
            )),
            page as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            (*page).folder_view as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolderView, *mut FmTabPage) -> ()>,
                gpointer,
            >(Some(
                on_folder_view_columns_changed
                    as unsafe extern "C" fn(*mut FmFolderView, *mut FmTabPage) -> (),
            )),
            page as gpointer,
        );
        g_signal_handlers_disconnect_matched(
            fm_config as *mut FmAppConfig as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmAppConfig, *mut FmSidePane) -> ()>,
                gpointer,
            >(Some(
                tab_page_on_home_path_changed
                    as unsafe extern "C" fn(*mut FmAppConfig, *mut FmSidePane) -> (),
            )),
            (*page).side_pane as gpointer,
        );
        _disconnect_focus_in((*page).folder_view, page);
        g_object_unref((*page).folder_view as gpointer);
        (*page).folder_view = ptr::null_mut::<FmFolderView>();
    }
    g_strfreev((*page).columns as *mut *mut gchar);
    (*page).columns = ptr::null_mut::<*mut c_char>();
    if (*page).update_scroll_id != 0 {
        g_source_remove((*page).update_scroll_id);
        (*page).update_scroll_id = 0 as guint;
    }
    fm_side_pane_set_popup_updater((*page).side_pane, None, ptr::null_mut::<c_void>());
    if !(*page).dd.is_null() {
        g_object_unref((*page).dd as gpointer);
        (*page).dd = ptr::null_mut::<FmDndDest>();
    }
    if (*(fm_tab_page_parent_class as *mut GtkWidgetClass))
        .destroy
        .is_some()
    {
        Some(
            (*(fm_tab_page_parent_class as *mut GtkWidgetClass))
                .destroy
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn on_folder_content_changed(
    mut folder: *mut FmFolder,
    mut page: *mut FmTabPage,
) {
    g_free((*page).status_text[FM_STATUS_TEXT_NORMAL as c_int as usize] as gpointer);
    (*page).status_text[FM_STATUS_TEXT_NORMAL as c_int as usize] = format_status_text(page);
    g_signal_emit(
        page as gpointer,
        signals[STATUS as c_int as usize],
        0 as GQuark,
        FM_STATUS_TEXT_NORMAL as c_int as guint,
        (*page).status_text[FM_STATUS_TEXT_NORMAL as c_int as usize],
    );
}
unsafe extern "C" fn tab_page_on_folder_view_sel_changed(
    mut fv: *mut FmFolderView,
    mut n_sel: gint,
    mut page: *mut FmTabPage,
) {
    let mut msg: *mut c_char = (*page).status_text[FM_STATUS_TEXT_SELECTED_FILES as c_int as usize];
    let mut str: *mut GString = ptr::null_mut::<GString>();
    g_free(msg as gpointer);
    if n_sel > 0i32 {
        str = g_string_sized_new(64 as gsize);
        if n_sel == 1i32 {
            let mut files: *mut FmFileInfoList = fm_folder_view_dup_selected_files(fv);
            let mut fi: *mut FmFileInfo = fm_file_info_list_peek_head(files);
            let mut size_str: *const c_char = fm_file_info_get_disp_size(fi);
            let mut l: *mut GList = ptr::null_mut::<GList>();
            if !size_str.is_null() {
                g_string_printf(
                    str,
                    dcgettext(
                        ptr::null::<c_char>(),
                        c"\"%s\" (%s) %s".as_ptr(),
                        5 as c_int,
                    ),
                    fm_file_info_get_disp_name(fi),
                    if !size_str.is_null() {
                        size_str
                    } else {
                        c"".as_ptr()
                    },
                    fm_file_info_get_desc(fi),
                );
            } else {
                g_string_printf(
                    str,
                    dcgettext(ptr::null::<c_char>(), c"\"%s\" %s".as_ptr(), 5 as c_int),
                    fm_file_info_get_disp_name(fi),
                    fm_file_info_get_desc(fi),
                );
            }
            if ({
                let mut _g_boolean_var_60: c_int = 0i32;
                if fm_modules_loaded == 0 {
                    _g_boolean_var_60 = 1i32;
                }
                _g_boolean_var_60
            }) as c_long
                != 0
            {
                fm_modules_load();
            }
            l = _tab_page_modules;
            while !l.is_null() {
                let mut module: *mut FmTabPageStatusInit = (*l).data as *mut FmTabPageStatusInit;
                let mut message: *mut c_char =
                    (*module).sel_message.expect("non-null function pointer")(files, n_sel);
                if !message.is_null() && *message.offset(0 as isize) as c_int != 0 {
                    g_string_append_c_inline(str, ' ' as i32 as gchar);
                    if 0 != 0 {
                        ({
                            let __val: *const c_char = message;
                            g_string_append_len_inline(
                                str,
                                __val,
                                if ({
                                    let mut _g_boolean_var_61: c_int = 0 as c_int;
                                    if !__val.is_null() {
                                        _g_boolean_var_61 = 1i32;
                                    }
                                    _g_boolean_var_61
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
                        g_string_append_len_inline(str, message, -(1i32) as gssize);
                    };
                }
                g_free(message as gpointer);
                l = (*l).next;
            }
            fm_file_info_list_unref(files);
        } else {
            let mut files_0: *mut FmFileInfoList = ptr::null_mut::<FmFileInfoList>();
            let mut sum: goffset = 0;
            let mut l_0: *mut GList = ptr::null_mut::<GList>();
            let mut size_str_0: [c_char; 128] = [0; 128];
            g_string_printf(
                str,
                dcngettext(
                    ptr::null::<c_char>(),
                    c"%d item selected".as_ptr(),
                    c"%d items selected".as_ptr(),
                    n_sel as c_ulong,
                    5 as c_int,
                ),
                n_sel,
            );
            if n_sel < 1000i32 {
                sum = 0 as goffset;
                files_0 = fm_folder_view_dup_selected_files(fv);
                l_0 = fm_file_info_list_peek_head_link(files_0);
                while !l_0.is_null() {
                    if fm_file_info_is_dir((*l_0).data as *mut FmFileInfo) != 0 {
                        sum = -(1i32) as goffset;
                        break;
                    } else {
                        sum += fm_file_info_get_size((*l_0).data as *mut FmFileInfo);
                        l_0 = (*l_0).next;
                    }
                }
                if sum >= 0 as goffset {
                    fm_file_size_to_str(
                        &raw mut size_str_0 as *mut c_char,
                        mem::size_of::<[c_char; 128]>() as size_t,
                        sum,
                        (*fm_config).si_unit,
                    );
                    g_string_append_printf(
                        str,
                        b" (%s)\0" as *const u8 as *const gchar,
                        &raw mut size_str_0 as *mut c_char,
                    );
                }
                if ({
                    let mut _g_boolean_var_62: c_int = 0 as c_int;
                    if fm_modules_loaded == 0 {
                        _g_boolean_var_62 = 1i32;
                    }
                    _g_boolean_var_62
                }) as c_long
                    != 0
                {
                    fm_modules_load();
                }
                l_0 = _tab_page_modules;
                while !l_0.is_null() {
                    let mut module_0: *mut FmTabPageStatusInit =
                        (*l_0).data as *mut FmTabPageStatusInit;
                    let mut message_0: *mut c_char =
                        (*module_0).sel_message.expect("non-null function pointer")(files_0, n_sel);
                    if !message_0.is_null() && *message_0.offset(0 as isize) as c_int != 0 {
                        g_string_append_c_inline(str, ' ' as i32 as gchar);
                        if 0 != 0 {
                            ({
                                let __val: *const c_char = message_0;
                                g_string_append_len_inline(
                                    str,
                                    __val,
                                    if ({
                                        let mut _g_boolean_var_63: c_int = 0 as c_int;
                                        if !__val.is_null() {
                                            _g_boolean_var_63 = 1i32;
                                        }
                                        _g_boolean_var_63
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
                            g_string_append_len_inline(str, message_0, -(1i32) as gssize);
                        };
                    }
                    g_free(message_0 as gpointer);
                    l_0 = (*l_0).next;
                }
                fm_file_info_list_unref(files_0);
            }
        }
        msg = (if 0 != 0 {
            if 0i32 != 0 {
                g_string_free(str, 0 as gboolean)
            } else {
                g_string_free_and_steal(str)
            }
        } else {
            g_string_free(str, 0 as gboolean)
        }) as *mut c_char;
    } else {
        msg = ptr::null_mut::<c_char>();
    }
    (*page).status_text[FM_STATUS_TEXT_SELECTED_FILES as c_int as usize] = msg;
    g_signal_emit(
        page as gpointer,
        signals[STATUS as c_int as usize],
        0 as GQuark,
        FM_STATUS_TEXT_SELECTED_FILES as c_int as guint,
        msg,
    );
}
unsafe extern "C" fn on_folder_view_columns_changed(
    mut fv: *mut FmFolderView,
    mut page: *mut FmTabPage,
) {
    let mut columns: *mut GSList = fm_folder_view_get_columns(fv);
    let mut l: *mut GSList = ptr::null_mut::<GSList>();
    let mut cols: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
    let mut i: guint = 0;
    if columns.is_null() {
        return;
    }
    i = g_slist_length(columns);
    cols = ({
        let mut __n: gsize = i.wrapping_add(1 as guint) as gsize;
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
    i = 0 as guint;
    l = columns;
    while !l.is_null() {
        let mut info: *mut FmFolderViewColumnInfo = (*l).data as *mut FmFolderViewColumnInfo;
        if (*info).width > 0i32 {
            let ref mut fresh24 = *cols.offset(i as isize);
            *fresh24 = g_strdup_printf(
                b"%s:%d\0" as *const u8 as *const gchar,
                fm_folder_model_col_get_name((*info).col_id),
                (*info).width,
            ) as *mut c_char;
        } else {
            let ref mut fresh25 = *cols.offset(i as isize);
            *fresh25 = g_strdup_inline(fm_folder_model_col_get_name((*info).col_id));
        }
        i = i.wrapping_add(1);
        l = (*l).next;
    }
    g_slist_free(columns);
    let ref mut fresh26 = *cols.offset(i as isize);
    *fresh26 = ptr::null_mut::<c_char>();
    if (*page).own_config() != 0 {
        g_strfreev((*page).columns as *mut *mut gchar);
        (*page).columns = cols;
        fm_app_config_save_config_for_path(
            fm_folder_view_get_cwd(fv),
            (*page).sort_type,
            (*page).sort_by,
            4294967295 as FmStandardViewMode,
            (*page).show_hidden(),
            cols,
        );
    } else {
        g_strfreev((*(fm_config as *mut FmAppConfig)).columns as *mut *mut gchar);
        let ref mut fresh27 = (*(fm_config as *mut FmAppConfig)).columns;
        *fresh27 = cols;
        pcmanfm_save_config(0 as gboolean);
    };
}
unsafe extern "C" fn on_folder_view_focus_in(
    mut widget: *mut GtkWidget,
    mut event: *mut GdkEvent,
    mut page: *mut FmTabPage,
) -> gboolean {
    g_signal_emit(
        page as gpointer,
        signals[GOT_FOCUS as c_int as usize],
        0 as GQuark,
    );
    return 0 as gboolean;
}
unsafe extern "C" fn tab_page_on_folder_error(
    mut folder: *mut FmFolder,
    mut err: *mut GError,
    mut severity: FmJobErrorSeverity,
    mut page: *mut FmTabPage,
) -> FmJobErrorAction {
    let mut win: *mut GtkWindow = gtk_widget_get_toplevel(page as *mut c_void as *mut GtkWidget)
        as *mut c_void as *mut FmMainWin as *mut c_void
        as *mut GtkWindow;
    if (*err).domain == g_io_error_quark() {
        if (*err).code == G_IO_ERROR_NOT_MOUNTED as c_int
            && (severity as c_uint) < FM_JOB_ERROR_CRITICAL as c_int as c_uint
        {
            let mut path: *mut FmPath = fm_folder_get_path(folder);
            if fm_mount_path(win, path, (0i32 == 0) as c_int) != 0 {
                return FM_JOB_RETRY;
            }
        }
    }
    if severity as c_uint >= FM_JOB_ERROR_MODERATE as c_int as c_uint {
        fm_show_error(win, ptr::null::<c_char>(), (*err).message);
    }
    return FM_JOB_CONTINUE;
}
unsafe extern "C" fn fm_tab_page_realize(mut page: *mut GtkWidget) {
    (*(fm_tab_page_parent_class as *mut GtkWidgetClass))
        .realize
        .expect("non-null function pointer")(page);
    if (*(page as *mut c_void as *mut FmTabPage)).busy() != 0 {
        fm_set_busy_cursor(page);
    }
}
unsafe extern "C" fn fm_tab_page_unrealize(mut page: *mut GtkWidget) {
    if (*(page as *mut c_void as *mut FmTabPage)).busy() != 0 {
        fm_unset_busy_cursor(page);
    }
    (*(fm_tab_page_parent_class as *mut GtkWidgetClass))
        .unrealize
        .expect("non-null function pointer")(page);
}
unsafe extern "C" fn _tab_set_busy_cursor(mut page: *mut FmTabPage) {
    (*page).set_busy((0i32 == 0) as c_int as gboolean as gboolean);
    if gtk_widget_get_realized(page as *mut c_void as *mut GtkWidget) != 0 {
        fm_set_busy_cursor(page as *mut c_void as *mut GtkWidget);
    }
}
unsafe extern "C" fn _tab_unset_busy_cursor(mut page: *mut FmTabPage) {
    (*page).set_busy(0 as gboolean as gboolean);
    if gtk_widget_get_realized(page as *mut c_void as *mut GtkWidget) != 0 {
        fm_unset_busy_cursor(page as *mut c_void as *mut GtkWidget);
    }
}
unsafe extern "C" fn fm_tab_page_path_filter(
    mut file: *mut FmFileInfo,
    mut user_data: gpointer,
) -> gboolean {
    let mut page: *mut FmTabPage = ptr::null_mut::<FmTabPage>();
    let mut disp_name: *const c_char = ptr::null::<c_char>();
    let mut casefold: *mut c_char = ptr::null_mut::<c_char>();
    let mut key: *mut c_char = ptr::null_mut::<c_char>();
    let mut result: gboolean = 0;
    page = user_data as *mut FmTabPage;
    if (*page).filter_pattern.is_null() {
        return (0i32 == 0) as c_int;
    }
    disp_name = fm_file_info_get_disp_name(file);
    casefold = g_utf8_casefold(disp_name as *const gchar, -(1i32) as gssize) as *mut c_char;
    key = g_utf8_normalize(casefold, -(1i32) as gssize, G_NORMALIZE_ALL) as *mut c_char;
    g_free(casefold as gpointer);
    result = (fnmatch((*page).filter_pattern, key, 0i32) == 0i32) as c_int as gboolean;
    g_free(key as gpointer);
    return result;
}
unsafe extern "C" fn tab_page_on_folder_start_loading(
    mut folder: *mut FmFolder,
    mut page: *mut FmTabPage,
) {
    let mut fv: *mut FmFolderView = (*page).folder_view;
    _tab_set_busy_cursor(page);
    if fm_folder_is_incremental(folder) != 0 {
        let mut model: *mut FmFolderModel = fm_folder_model_new(folder, (*page).show_hidden());
        if !(*page).filter_pattern.is_null() {
            fm_folder_model_add_filter(
                model,
                Some(
                    fm_tab_page_path_filter
                        as unsafe extern "C" fn(*mut FmFileInfo, gpointer) -> gboolean,
                ),
                page as gpointer,
            );
            fm_folder_model_apply_filters(model);
        }
        fm_folder_view_set_model(fv, model);
        fm_folder_model_set_sort(model, (*page).sort_by, (*page).sort_type);
        g_object_unref(model as gpointer);
    } else {
        fm_folder_view_set_model(fv, ptr::null_mut::<FmFolderModel>());
    };
}
unsafe extern "C" fn update_scroll(mut data: gpointer) -> gboolean {
    let mut page: *mut FmTabPage = data as *mut FmTabPage;
    let mut scroll: *mut GtkScrolledWindow = ptr::null_mut::<GtkScrolledWindow>();
    if g_source_is_destroyed(g_main_current_source()) != 0 {
        return 0 as gboolean;
    }
    scroll = (*page).folder_view as *mut c_void as *mut GtkScrolledWindow;
    gtk_adjustment_set_value(
        gtk_scrolled_window_get_vadjustment(scroll),
        fm_nav_history_get_scroll_pos((*page).nav_history) as gdouble,
    );
    if !(*page).want_focus.is_null() {
        fm_folder_view_select_file_path((*page).folder_view, (*page).want_focus);
        fm_folder_view_scroll_to_path(
            (*page).folder_view,
            (*page).want_focus,
            (0i32 == 0) as c_int,
        );
        fm_path_unref((*page).want_focus);
        (*page).want_focus = ptr::null_mut::<FmPath>();
    }
    (*page).update_scroll_id = 0 as guint;
    return 0 as gboolean;
}
unsafe extern "C" fn tab_page_on_folder_finish_loading(
    mut folder: *mut FmFolder,
    mut page: *mut FmTabPage,
) {
    let mut fv: *mut FmFolderView = (*page).folder_view;
    if fm_folder_view_get_model(fv).is_null() {
        let mut model: *mut FmFolderModel = fm_folder_model_new(folder, (*page).show_hidden());
        fm_folder_view_set_model(fv, model);
        if !(*page).filter_pattern.is_null() {
            fm_folder_model_add_filter(
                model,
                Some(
                    fm_tab_page_path_filter
                        as unsafe extern "C" fn(*mut FmFileInfo, gpointer) -> gboolean,
                ),
                page as gpointer,
            );
            fm_folder_model_apply_filters(model);
        }
        fm_folder_model_set_sort(model, (*page).sort_by, (*page).sort_type);
        g_object_unref(model as gpointer);
    }
    fm_folder_query_filesystem_info(folder);
    if (*page).update_scroll_id != 0 {
        g_source_remove((*page).update_scroll_id);
    }
    (*page).update_scroll_id = gdk_threads_add_timeout(
        50 as guint,
        Some(update_scroll as unsafe extern "C" fn(gpointer) -> gboolean),
        page as gpointer,
    );
    g_free((*page).status_text[FM_STATUS_TEXT_NORMAL as c_int as usize] as gpointer);
    (*page).status_text[FM_STATUS_TEXT_NORMAL as c_int as usize] = format_status_text(page);
    g_signal_emit(
        page as gpointer,
        signals[STATUS as c_int as usize],
        0 as GQuark,
        FM_STATUS_TEXT_NORMAL as c_int as guint,
        (*page).status_text[FM_STATUS_TEXT_NORMAL as c_int as usize],
    );
    _tab_unset_busy_cursor(page);
    g_signal_emit(
        page as gpointer,
        signals[LOADED as c_int as usize],
        0 as GQuark,
    );
}
unsafe extern "C" fn on_folder_unmount(mut folder: *mut FmFolder, mut page: *mut FmTabPage) {
    if (*(fm_config as *mut FmAppConfig)).close_on_unmount != 0 {
        gtk_widget_destroy(page as *mut c_void as *mut GtkWidget);
    } else if !(*(fm_config as *mut FmAppConfig)).home_path.is_null()
        && *(*(fm_config as *mut FmAppConfig))
            .home_path
            .offset(0 as isize) as c_int
            != 0
    {
        let mut path: *mut FmPath =
            fm_path_new_for_str((*(fm_config as *mut FmAppConfig)).home_path);
        fm_tab_page_chdir(page, path);
        fm_path_unref(path);
    } else {
        fm_tab_page_chdir(page, fm_path_get_home());
    };
}
unsafe extern "C" fn on_folder_removed(mut folder: *mut FmFolder, mut page: *mut FmTabPage) {
    if (*(fm_config as *mut FmAppConfig)).close_on_unmount != 0 {
        gtk_widget_destroy(page as *mut c_void as *mut GtkWidget);
    } else if !(*(fm_config as *mut FmAppConfig)).home_path.is_null()
        && *(*(fm_config as *mut FmAppConfig))
            .home_path
            .offset(0 as isize) as c_int
            != 0
    {
        let mut path: *mut FmPath =
            fm_path_new_for_str((*(fm_config as *mut FmAppConfig)).home_path);
        fm_tab_page_chdir(page, path);
        fm_path_unref(path);
    } else {
        fm_tab_page_chdir(page, fm_path_get_home());
    };
}
unsafe extern "C" fn on_folder_fs_info(mut folder: *mut FmFolder, mut page: *mut FmTabPage) {
    let mut free: guint64 = 0;
    let mut total: guint64 = 0;
    let mut msg: *mut c_char = (*page).status_text[FM_STATUS_TEXT_FS_INFO as c_int as usize];
    g_free(msg as gpointer);
    if fm_folder_get_filesystem_info(folder, &raw mut total, &raw mut free) != 0 {
        let mut total_str: [c_char; 64] = [0; 64];
        let mut free_str: [c_char; 64] = [0; 64];
        fm_file_size_to_str(
            &raw mut free_str as *mut c_char,
            mem::size_of::<[c_char; 64]>() as size_t,
            free as goffset,
            (*fm_config).si_unit,
        );
        fm_file_size_to_str(
            &raw mut total_str as *mut c_char,
            mem::size_of::<[c_char; 64]>() as size_t,
            total as goffset,
            (*fm_config).si_unit,
        );
        msg = g_strdup_printf(
            dcgettext(
                ptr::null::<c_char>(),
                c"Free space: %s (Total: %s)".as_ptr(),
                5 as c_int,
            ),
            &raw mut free_str as *mut c_char,
            &raw mut total_str as *mut c_char,
        ) as *mut c_char;
    } else {
        msg = ptr::null_mut::<c_char>();
    }
    (*page).status_text[FM_STATUS_TEXT_FS_INFO as c_int as usize] = msg;
    g_signal_emit(
        page as gpointer,
        signals[STATUS as c_int as usize],
        0 as GQuark,
        FM_STATUS_TEXT_FS_INFO as c_int as guint,
        msg,
    );
}
unsafe extern "C" fn format_status_text(mut page: *mut FmTabPage) -> *mut c_char {
    let mut model: *mut FmFolderModel = fm_folder_view_get_model((*page).folder_view);
    let mut folder: *mut FmFolder = fm_folder_view_get_folder((*page).folder_view);
    if !model.is_null() && !folder.is_null() {
        let mut files: *mut FmFileInfoList = fm_folder_get_files(folder);
        let mut msg: *mut GString = g_string_sized_new(128 as gsize);
        let mut total_files: c_int = fm_file_info_list_get_length(files) as c_int;
        let mut shown_files: c_int = gtk_tree_model_iter_n_children(
            model as *mut c_void as *mut GtkTreeModel,
            ptr::null_mut::<GtkTreeIter>(),
        ) as c_int;
        let mut hidden_files: c_int = total_files - shown_files;
        let mut visible_fmt: *const c_char = dcngettext(
            ptr::null::<c_char>(),
            c"%d item".as_ptr(),
            c"%d items".as_ptr(),
            shown_files as c_ulong,
            5 as c_int,
        );
        let mut hidden_fmt: *const c_char = dcngettext(
            ptr::null::<c_char>(),
            c" (%d hidden)".as_ptr(),
            c" (%d hidden)".as_ptr(),
            hidden_files as c_ulong,
            5 as c_int,
        );
        g_string_append_printf(msg, visible_fmt as *const gchar, shown_files);
        if hidden_files > 0i32 {
            g_string_append_printf(msg, hidden_fmt as *const gchar, hidden_files);
        }
        return if 0 != 0 {
            if 0i32 != 0 {
                g_string_free(msg, 0 as gboolean) as *mut c_char
            } else {
                g_string_free_and_steal(msg) as *mut c_char
            }
        } else {
            g_string_free(msg, 0 as gboolean) as *mut c_char
        };
    }
    return ptr::null_mut::<c_char>();
}
unsafe extern "C" fn tab_page_on_open_in_new_tab(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut act_grp: *mut GObject = ptr::null_mut::<GObject>();
    let mut sels: *mut FmFileInfoList = ptr::null_mut::<FmFileInfoList>();
    let mut l: *mut GList = ptr::null_mut::<GList>();
    g_object_get(
        act as gpointer,
        b"action-group\0" as *const u8 as *const gchar,
        &raw mut act_grp,
        ptr::null_mut::<c_void>(),
    );
    sels = g_object_get_qdata(act_grp, popup_qdata) as *mut FmFileInfoList;
    g_object_unref(act_grp as gpointer);
    l = fm_file_info_list_peek_head_link(sels);
    while !l.is_null() {
        let mut fi: *mut FmFileInfo = (*l).data as *mut FmFileInfo;
        fm_main_win_add_tab(win, fm_file_info_get_path(fi));
        l = (*l).next;
    }
}
unsafe extern "C" fn tab_page_on_open_in_new_win(mut act: *mut GtkAction, mut win: *mut FmMainWin) {
    let mut act_grp: *mut GObject = ptr::null_mut::<GObject>();
    let mut sels: *mut FmFileInfoList = ptr::null_mut::<FmFileInfoList>();
    let mut l: *mut GList = ptr::null_mut::<GList>();
    g_object_get(
        act as gpointer,
        b"action-group\0" as *const u8 as *const gchar,
        &raw mut act_grp,
        ptr::null_mut::<c_void>(),
    );
    sels = g_object_get_qdata(act_grp, popup_qdata) as *mut FmFileInfoList;
    g_object_unref(act_grp as gpointer);
    l = fm_file_info_list_peek_head_link(sels);
    while !l.is_null() {
        let mut fi: *mut FmFileInfo = (*l).data as *mut FmFileInfo;
        fm_main_win_add_win(win, fm_file_info_get_path(fi));
        l = (*l).next;
    }
}
unsafe extern "C" fn tab_page_on_open_folder_in_terminal(
    mut act: *mut GtkAction,
    mut win: *mut FmMainWin,
) {
    let mut act_grp: *mut GObject = ptr::null_mut::<GObject>();
    let mut files: *mut FmFileInfoList = ptr::null_mut::<FmFileInfoList>();
    let mut l: *mut GList = ptr::null_mut::<GList>();
    g_object_get(
        act as gpointer,
        b"action-group\0" as *const u8 as *const gchar,
        &raw mut act_grp,
        ptr::null_mut::<c_void>(),
    );
    files = g_object_get_qdata(act_grp, popup_qdata) as *mut FmFileInfoList;
    g_object_unref(act_grp as gpointer);
    l = fm_file_info_list_peek_head_link(files);
    while !l.is_null() {
        let mut fi: *mut FmFileInfo = (*l).data as *mut FmFileInfo;
        pcmanfm_open_folder_in_terminal(
            win as *mut c_void as *mut GtkWindow,
            fm_file_info_get_path(fi),
        );
        l = (*l).next;
    }
}
unsafe extern "C" fn update_files_popup(
    mut fv: *mut FmFolderView,
    mut win: *mut GtkWindow,
    mut ui: *mut GtkUIManager,
    mut act_grp: *mut GtkActionGroup,
    mut files: *mut FmFileInfoList,
) {
    let mut l: *mut GList = ptr::null_mut::<GList>();
    let mut all_native: gboolean = (0i32 == 0) as c_int;
    l = fm_file_info_list_peek_head_link(files);
    while !l.is_null() {
        if fm_file_info_is_dir((*l).data as *mut FmFileInfo) == 0 {
            return;
        } else if pcmanfm_can_open_path_in_terminal(fm_file_info_get_path(
            (*l).data as *mut FmFileInfo,
        )) == 0
        {
            all_native = 0 as gboolean;
        }
        l = (*l).next;
    }
    g_object_set_qdata_full(
        act_grp as *mut c_void as *mut GObject,
        popup_qdata,
        fm_file_info_list_ref(files) as gpointer,
        mem::transmute::<Option<unsafe extern "C" fn(*mut FmFileInfoList) -> ()>, GDestroyNotify>(
            Some(fm_file_info_list_unref as unsafe extern "C" fn(*mut FmFileInfoList) -> ()),
        ),
    );
    gtk_action_group_set_translation_domain(act_grp, ptr::null::<gchar>());
    gtk_action_group_add_actions(
        act_grp,
        &raw mut tab_page_folder_menu_actions as *mut GtkActionEntry,
        (mem::size_of::<[GtkActionEntry; 4]>() as usize)
            .wrapping_div(mem::size_of::<GtkActionEntry>() as usize) as guint,
        win as gpointer,
    );
    gtk_ui_manager_add_ui_from_string(
        ui,
        &raw const tab_page_folder_menu_xml as *const gchar,
        -(1i32) as gssize,
        ptr::null_mut::<*mut GError>(),
    );
    if all_native == 0 {
        gtk_action_set_visible(
            gtk_action_group_get_action(act_grp, b"Term\0" as *const u8 as *const gchar),
            0 as gboolean,
        );
    }
}
unsafe extern "C" fn open_folder_func(
    mut ctx: *mut GAppLaunchContext,
    mut folder_infos: *mut GList,
    mut user_data: gpointer,
    mut err: *mut *mut GError,
) -> gboolean {
    let mut win: *mut FmMainWin = user_data as *mut FmMainWin;
    let mut l: *mut GList = folder_infos;
    let mut fi: *mut FmFileInfo = (*l).data as *mut FmFileInfo;
    fm_main_win_chdir(win, fm_file_info_get_path(fi));
    l = (*l).next;
    while !l.is_null() {
        let mut fi_0: *mut FmFileInfo = (*l).data as *mut FmFileInfo;
        fm_main_win_add_tab(win, fm_file_info_get_path(fi_0));
        l = (*l).next;
    }
    return (0i32 == 0) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _update_sidepane_popup(
    mut sp: *mut FmSidePane,
    mut ui: *mut GtkUIManager,
    mut act_grp: *mut GtkActionGroup,
    mut file: *mut FmFileInfo,
    mut user_data: gpointer,
) {
    let mut win: *mut FmMainWin =
        gtk_widget_get_toplevel(user_data as *mut GtkWidget) as *mut c_void as *mut FmMainWin;
    let mut files: *mut FmFileInfoList = ptr::null_mut::<FmFileInfoList>();
    if ({
        let mut _g_boolean_var_64: c_int = 0i32;
        if file.is_null() || fm_file_info_is_dir(file) == 0 {
            _g_boolean_var_64 = 1i32;
        }
        _g_boolean_var_64
    }) as c_long
        != 0
    {
        return;
    }
    if ({
        let mut _g_boolean_var_65: c_int = 0i32;
        if ({
            let mut __inst: *mut GTypeInstance = win as *mut GTypeInstance;
            let mut __t: GType = fm_main_win_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0i32 == 0) as c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) == 0
        {
            _g_boolean_var_65 = 1i32;
        }
        _g_boolean_var_65
    }) as c_long
        != 0
    {
        return;
    }
    files = fm_file_info_list_new();
    fm_file_info_list_push_tail(files, file);
    g_object_set_qdata_full(
        act_grp as *mut c_void as *mut GObject,
        popup_qdata,
        files as gpointer,
        mem::transmute::<Option<unsafe extern "C" fn(*mut FmFileInfoList) -> ()>, GDestroyNotify>(
            Some(fm_file_info_list_unref as unsafe extern "C" fn(*mut FmFileInfoList) -> ()),
        ),
    );
    gtk_action_group_set_translation_domain(act_grp, ptr::null::<gchar>());
    gtk_action_group_add_actions(
        act_grp,
        &raw mut tab_page_folder_menu_actions as *mut GtkActionEntry,
        (mem::size_of::<[GtkActionEntry; 4]>() as usize)
            .wrapping_div(mem::size_of::<GtkActionEntry>() as usize) as guint,
        win as gpointer,
    );
    gtk_ui_manager_add_ui_from_string(
        ui,
        &raw const tab_page_folder_menu_xml as *const gchar,
        -(1i32) as gssize,
        ptr::null_mut::<*mut GError>(),
    );
    if pcmanfm_can_open_path_in_terminal(fm_file_info_get_path(file)) == 0 {
        gtk_action_set_visible(
            gtk_action_group_get_action(act_grp, b"Term\0" as *const u8 as *const gchar),
            0 as gboolean,
        );
    }
}
unsafe extern "C" fn tab_page_on_drag_motion(
    mut label: *mut FmTabLabel,
    mut drag_context: *mut GdkDragContext,
    mut x: gint,
    mut y: gint,
    mut time: guint,
    mut page: *mut FmTabPage,
) -> gboolean {
    let mut target: GdkAtom = ptr::null_mut::<_GdkAtom>();
    let mut action: GdkDragAction = 0 as GdkDragAction;
    let mut file_info: *mut FmFileInfo = ptr::null_mut::<FmFileInfo>();
    if (*(fm_config as *mut FmAppConfig)).change_tab_on_drop == 0 && !(*page).folder_view.is_null()
    {
        file_info = fm_folder_view_get_cwd_info((*page).folder_view);
    }
    fm_dnd_dest_set_dest_file((*page).dd, file_info);
    if file_info.is_null() {
        return 0 as gboolean;
    }
    target = fm_dnd_dest_find_target((*page).dd, drag_context);
    if !target.is_null() && fm_dnd_dest_is_target_supported((*page).dd, target) != 0 {
        action = fm_dnd_dest_get_default_action((*page).dd, drag_context, target);
    }
    if action as c_uint == 0 as c_uint {
        return 0 as gboolean;
    }
    gdk_drag_status(drag_context, action, time as guint32);
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn fm_tab_page_init(mut page: *mut FmTabPage) {
    let mut paned: *mut GtkPaned = page as *mut c_void as *mut GtkPaned;
    let mut tab_label: *mut FmTabLabel = ptr::null_mut::<FmTabLabel>();
    let mut folder_view: *mut FmFolderView = ptr::null_mut::<FmFolderView>();
    let mut focus_chain: *mut GList = ptr::null_mut::<GList>();
    let mut atk_widget: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut atk_label: *mut AtkObject = ptr::null_mut::<AtkObject>();
    let mut relation: *mut AtkRelation = ptr::null_mut::<AtkRelation>();
    let mut mode: FmSidePaneMode = (*(fm_config as *mut FmAppConfig)).side_pane_mode;
    (*page).side_pane = fm_side_pane_new();
    fm_side_pane_set_mode(
        (*page).side_pane,
        (mode as c_uint & 0x1f as c_uint) as FmSidePaneMode,
    );
    fm_side_pane_set_popup_updater(
        (*page).side_pane,
        Some(
            _update_sidepane_popup
                as unsafe extern "C" fn(
                    *mut FmSidePane,
                    *mut GtkUIManager,
                    *mut GtkActionGroup,
                    *mut FmFileInfo,
                    gpointer,
                ) -> (),
        ),
        page as gpointer,
    );
    if !(*(fm_config as *mut FmAppConfig)).home_path.is_null()
        && *(*(fm_config as *mut FmAppConfig))
            .home_path
            .offset(0 as isize) as c_int
            != 0
    {
        fm_side_pane_set_home_dir(
            (*page).side_pane,
            (*(fm_config as *mut FmAppConfig)).home_path,
        );
    }
    g_signal_connect_data(
        fm_config as *mut FmAppConfig as gpointer,
        b"changed::home_path\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmAppConfig, *mut FmSidePane) -> ()>,
            GCallback,
        >(Some(
            tab_page_on_home_path_changed
                as unsafe extern "C" fn(*mut FmAppConfig, *mut FmSidePane) -> (),
        )),
        (*page).side_pane as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    gtk_paned_add1(paned, (*page).side_pane as *mut c_void as *mut GtkWidget);
    focus_chain = g_list_prepend(focus_chain, (*page).side_pane as gpointer);
    (*page).view_mode = (*(fm_config as *mut FmAppConfig)).view_mode;
    folder_view = fm_standard_view_new(
        (*(fm_config as *mut FmAppConfig)).view_mode as FmStandardViewMode,
        Some(
            update_files_popup
                as unsafe extern "C" fn(
                    *mut FmFolderView,
                    *mut GtkWindow,
                    *mut GtkUIManager,
                    *mut GtkActionGroup,
                    *mut FmFileInfoList,
                ) -> (),
        ),
        Some(
            open_folder_func
                as unsafe extern "C" fn(
                    *mut GAppLaunchContext,
                    *mut GList,
                    gpointer,
                    *mut *mut GError,
                ) -> gboolean,
        ),
    ) as *mut FmFolderView;
    (*page).folder_view =
        g_object_ref_sink(folder_view as gpointer) as *mut FmFolderView as *mut FmFolderView;
    fm_folder_view_set_selection_mode(folder_view, GTK_SELECTION_MULTIPLE);
    (*page).nav_history = fm_nav_history_new();
    (*page).views = gtk_hbox_new((0i32 == 0) as c_int, 4 as gint) as *mut c_void as *mut GtkBox;
    gtk_box_pack_start(
        (*page).views,
        folder_view as *mut c_void as *mut GtkWidget,
        (0i32 == 0) as c_int,
        (0i32 == 0) as c_int,
        0 as guint,
    );
    gtk_paned_add2(paned, (*page).views as *mut c_void as *mut GtkWidget);
    focus_chain = g_list_prepend(focus_chain, (*page).views as gpointer);
    gtk_container_set_focus_chain(page as *mut c_void as *mut GtkContainer, focus_chain);
    g_list_free(focus_chain);
    tab_label = fm_tab_label_new(c"".as_ptr());
    gtk_label_set_max_width_chars(
        (*tab_label).label,
        (*(fm_config as *mut FmAppConfig)).max_tab_chars as gint,
    );
    (*page).tab_label = tab_label;
    atk_widget = gtk_widget_get_accessible(folder_view as *mut c_void as *mut GtkWidget);
    atk_label = gtk_widget_get_accessible(tab_label as *mut c_void as *mut GtkWidget);
    relation = atk_relation_new(&raw mut atk_widget, 1 as gint, ATK_RELATION_LABEL_FOR);
    atk_relation_set_add(atk_object_ref_relation_set(atk_label), relation);
    g_object_unref(relation as gpointer);
    g_signal_connect_data(
        folder_view as gpointer,
        b"sel-changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmFolderView, gint, *mut FmTabPage) -> ()>,
            GCallback,
        >(Some(
            tab_page_on_folder_view_sel_changed
                as unsafe extern "C" fn(*mut FmFolderView, gint, *mut FmTabPage) -> (),
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        folder_view as gpointer,
        b"columns-changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmFolderView, *mut FmTabPage) -> ()>,
            GCallback,
        >(Some(
            on_folder_view_columns_changed
                as unsafe extern "C" fn(*mut FmFolderView, *mut FmTabPage) -> (),
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    _connect_focus_in(folder_view, page);
    (*page).dd = fm_dnd_dest_new_with_handlers(tab_label as *mut c_void as *mut GtkWidget);
    g_signal_connect_data(
        tab_label as gpointer,
        b"drag-motion\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmTabLabel,
                    *mut GdkDragContext,
                    gint,
                    gint,
                    guint,
                    *mut FmTabPage,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            tab_page_on_drag_motion
                as unsafe extern "C" fn(
                    *mut FmTabLabel,
                    *mut GdkDragContext,
                    gint,
                    gint,
                    guint,
                    *mut FmTabPage,
                ) -> gboolean,
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*page).set_busy(0 as gboolean as gboolean);
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_new(mut path: *mut FmPath) -> *mut FmTabPage {
    let mut page: *mut FmTabPage =
        g_object_new(fm_tab_page_get_type(), ptr::null::<gchar>()) as *mut FmTabPage;
    fm_tab_page_chdir(page, path);
    return page;
}
unsafe extern "C" fn fm_tab_page_chdir_without_history(
    mut page: *mut FmTabPage,
    mut path: *mut FmPath,
) {
    let mut disp_name: *mut c_char = fm_path_display_basename(path);
    let mut disp_path: *mut c_char = ptr::null_mut::<c_char>();
    let mut view_mode: FmStandardViewMode = FM_FV_ICON_VIEW;
    let mut show_hidden: gboolean = 0;
    let mut columns: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
    let mut prev_path: *mut FmPath = ptr::null_mut::<FmPath>();
    if !(*page).filter_pattern.is_null() && *(*page).filter_pattern.offset(0 as isize) as c_int != 0
    {
        let mut text: *mut c_char = g_strdup_printf(
            b"%s [%s]\0" as *const u8 as *const gchar,
            disp_name,
            (*page).filter_pattern,
        ) as *mut c_char;
        g_free(disp_name as gpointer);
        disp_name = text;
    }
    fm_tab_label_set_text((*page).tab_label, disp_name);
    g_free(disp_name as gpointer);
    if (*(fm_config as *mut FmAppConfig)).focus_previous != 0 && !(*page).folder.is_null() {
        prev_path = fm_folder_get_path((*page).folder);
        if fm_path_equal(fm_path_get_parent(prev_path), path) != 0 {
            fm_path_ref(prev_path);
        } else {
            prev_path = ptr::null_mut::<FmPath>();
        }
    }
    disp_path = fm_path_display_name(path, 0 as gboolean);
    fm_tab_label_set_tooltip_text(
        (*page).tab_label as *mut c_void as *mut FmTabLabel,
        disp_path,
    );
    g_free(disp_path as gpointer);
    free_folder(page);
    (*page).folder = fm_folder_from_path(path);
    g_signal_connect_data(
        (*page).folder as gpointer,
        b"start-loading\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
            GCallback,
        >(Some(
            tab_page_on_folder_start_loading
                as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*page).folder as gpointer,
        b"finish-loading\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
            GCallback,
        >(Some(
            tab_page_on_folder_finish_loading
                as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*page).folder as gpointer,
        b"error\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut FmFolder,
                    *mut GError,
                    FmJobErrorSeverity,
                    *mut FmTabPage,
                ) -> FmJobErrorAction,
            >,
            GCallback,
        >(Some(
            tab_page_on_folder_error
                as unsafe extern "C" fn(
                    *mut FmFolder,
                    *mut GError,
                    FmJobErrorSeverity,
                    *mut FmTabPage,
                ) -> FmJobErrorAction,
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*page).folder as gpointer,
        b"fs-info\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
            GCallback,
        >(Some(
            on_folder_fs_info as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*page).folder as gpointer,
        b"removed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
            GCallback,
        >(Some(
            on_folder_removed as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*page).folder as gpointer,
        b"unmount\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
            GCallback,
        >(Some(
            on_folder_unmount as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*page).folder as gpointer,
        b"content-changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> ()>,
            GCallback,
        >(Some(
            on_folder_content_changed as unsafe extern "C" fn(*mut FmFolder, *mut FmTabPage) -> (),
        )),
        page as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*page).want_focus = prev_path;
    (*page).set_own_config(fm_app_config_get_config_for_path(
        path,
        &raw mut (*page).sort_type,
        &raw mut (*page).sort_by,
        &raw mut view_mode,
        &raw mut show_hidden,
        &raw mut columns,
    ) as gboolean);
    if (*page).own_config() == 0 {
        view_mode = (*page).view_mode as FmStandardViewMode;
    }
    (*page).set_show_hidden(show_hidden as gboolean);
    g_signal_handlers_block_matched(
        (*page).folder_view as gpointer,
        G_SIGNAL_MATCH_DETAIL,
        0 as guint,
        g_quark_try_string(b"filter-changed\0" as *const u8 as *const gchar),
        ptr::null_mut::<GClosure>(),
        ptr::null_mut::<c_void>(),
        ptr::null_mut::<c_void>(),
    );
    tab_page_on_folder_start_loading((*page).folder, page);
    fm_folder_view_set_show_hidden((*page).folder_view, show_hidden);
    fm_side_pane_set_show_hidden((*page).side_pane, show_hidden);
    g_signal_handlers_unblock_matched(
        (*page).folder_view as gpointer,
        G_SIGNAL_MATCH_DETAIL,
        0 as guint,
        g_quark_try_string(b"filter-changed\0" as *const u8 as *const gchar),
        ptr::null_mut::<GClosure>(),
        ptr::null_mut::<c_void>(),
        ptr::null_mut::<c_void>(),
    );
    if fm_folder_is_loaded((*page).folder) != 0 {
        tab_page_on_folder_finish_loading((*page).folder, page);
        on_folder_fs_info((*page).folder, page);
    }
    fm_standard_view_set_mode(
        (*page).folder_view as *mut c_void as *mut FmStandardView,
        view_mode,
    );
    if !columns.is_null() {
        let mut i: guint = 0;
        let mut n: guint = g_strv_length(columns as *mut *mut gchar);
        let mut infos: *mut FmFolderViewColumnInfo = ({
            let mut __n: gsize = n as gsize;
            let mut __s: gsize = mem::size_of::<FmFolderViewColumnInfo>() as gsize;
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
        }) as *mut FmFolderViewColumnInfo;
        let mut infos_list: *mut GSList = ptr::null_mut::<GSList>();
        i = 0 as guint;
        while i < n {
            let mut name: *mut c_char = g_strdup_inline(*columns.offset(i as isize));
            let mut delim: *mut c_char = ptr::null_mut::<c_char>();
            (*infos.offset(i as isize)).width = 0 as gint;
            delim = strchr(name, ':' as i32);
            if !delim.is_null() {
                let fresh23 = delim;
                delim = delim.offset(1);
                *fresh23 = 0 as c_char;
                (*infos.offset(i as isize)).width = atoi(delim) as gint;
            }
            (*infos.offset(i as isize)).col_id = fm_folder_model_get_col_by_name(name);
            g_free(name as gpointer);
            infos_list = g_slist_append(
                infos_list,
                infos.offset(i as isize) as *mut FmFolderViewColumnInfo as gpointer,
            );
            i = i.wrapping_add(1);
        }
        g_signal_handlers_block_matched(
            (*page).folder_view as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolderView, *mut FmTabPage) -> ()>,
                gpointer,
            >(Some(
                on_folder_view_columns_changed
                    as unsafe extern "C" fn(*mut FmFolderView, *mut FmTabPage) -> (),
            )),
            page as gpointer,
        );
        fm_folder_view_set_columns((*page).folder_view, infos_list);
        g_signal_handlers_unblock_matched(
            (*page).folder_view as gpointer,
            (G_SIGNAL_MATCH_FUNC as c_int | G_SIGNAL_MATCH_DATA as c_int) as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ptr::null_mut::<GClosure>(),
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut FmFolderView, *mut FmTabPage) -> ()>,
                gpointer,
            >(Some(
                on_folder_view_columns_changed
                    as unsafe extern "C" fn(*mut FmFolderView, *mut FmTabPage) -> (),
            )),
            page as gpointer,
        );
        g_slist_free(infos_list);
        g_free(infos as gpointer);
        if (*page).own_config() != 0 {
            (*page).columns = g_strdupv(columns as *mut *mut gchar) as *mut *mut c_char;
        }
    }
    fm_side_pane_chdir((*page).side_pane, path);
    g_signal_emit(
        page as gpointer,
        signals[CHDIR as c_int as usize],
        0 as GQuark,
        path,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_chdir(mut page: *mut FmTabPage, mut path: *mut FmPath) {
    let mut cwd: *mut FmPath = fm_tab_page_get_cwd(page);
    let mut scroll_pos: c_int = 0;
    if !cwd.is_null() && !path.is_null() && fm_path_equal(cwd, path) != 0 {
        return;
    }
    scroll_pos = gtk_adjustment_get_value(gtk_scrolled_window_get_vadjustment(
        (*page).folder_view as *mut c_void as *mut GtkScrolledWindow,
    )) as c_int;
    fm_nav_history_chdir((*page).nav_history, path, scroll_pos as gint);
    fm_tab_page_chdir_without_history(page, path);
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_set_show_hidden(
    mut page: *mut FmTabPage,
    mut show_hidden: gboolean,
) {
    fm_folder_view_set_show_hidden((*page).folder_view, show_hidden);
    fm_side_pane_set_show_hidden((*page).side_pane, show_hidden);
    g_free((*page).status_text[FM_STATUS_TEXT_NORMAL as c_int as usize] as gpointer);
    (*page).status_text[FM_STATUS_TEXT_NORMAL as c_int as usize] = format_status_text(page);
    g_signal_emit(
        page as gpointer,
        signals[STATUS as c_int as usize],
        0 as GQuark,
        FM_STATUS_TEXT_NORMAL as c_int as guint,
        (*page).status_text[FM_STATUS_TEXT_NORMAL as c_int as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_get_cwd(mut page: *mut FmTabPage) -> *mut FmPath {
    return if !(*page).folder.is_null() {
        fm_folder_get_path((*page).folder)
    } else {
        ptr::null_mut::<FmPath>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_get_side_pane(mut page: *mut FmTabPage) -> *mut FmSidePane {
    return (*page).side_pane;
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_get_folder_view(
    mut page: *mut FmTabPage,
) -> *mut FmFolderView {
    return (*page).folder_view;
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_get_folder(mut page: *mut FmTabPage) -> *mut FmFolder {
    return fm_folder_view_get_folder((*page).folder_view);
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_get_history(mut page: *mut FmTabPage) -> *mut FmNavHistory {
    return (*page).nav_history;
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_forward(mut page: *mut FmTabPage) {
    let mut index: guint = fm_nav_history_get_cur_index((*page).nav_history);
    if index > 0 as guint {
        let mut vadjustment: *mut GtkAdjustment = gtk_scrolled_window_get_vadjustment(
            (*page).folder_view as *mut c_void as *mut GtkScrolledWindow,
        );
        let mut scroll_pos: c_int = gtk_adjustment_get_value(vadjustment) as c_int;
        let mut path: *mut FmPath = fm_nav_history_go_to(
            (*page).nav_history,
            index.wrapping_sub(1 as guint),
            scroll_pos as gint,
        );
        fm_tab_page_chdir_without_history(page, path);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_back(mut page: *mut FmTabPage) {
    if fm_nav_history_can_back((*page).nav_history) != 0 {
        let mut vadjustment: *mut GtkAdjustment = gtk_scrolled_window_get_vadjustment(
            (*page).folder_view as *mut c_void as *mut GtkScrolledWindow,
        );
        let mut scroll_pos: c_int = gtk_adjustment_get_value(vadjustment) as c_int;
        let mut index: guint = fm_nav_history_get_cur_index((*page).nav_history);
        let mut path: *mut FmPath = fm_nav_history_go_to(
            (*page).nav_history,
            index.wrapping_add(1 as guint),
            scroll_pos as gint,
        );
        fm_tab_page_chdir_without_history(page, path);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_history(mut page: *mut FmTabPage, mut history_item: guint) {
    let mut vadjustment: *mut GtkAdjustment = gtk_scrolled_window_get_vadjustment(
        (*page).folder_view as *mut c_void as *mut GtkScrolledWindow,
    );
    let mut scroll_pos: c_int = gtk_adjustment_get_value(vadjustment) as c_int;
    let mut path: *mut FmPath =
        fm_nav_history_go_to((*page).nav_history, history_item, scroll_pos as gint);
    fm_tab_page_chdir_without_history(page, path);
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_get_title(mut page: *mut FmTabPage) -> *const c_char {
    let mut label: *mut FmTabLabel = (*page).tab_label;
    return gtk_label_get_text((*label).label) as *const c_char;
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_get_status_text(
    mut page: *mut FmTabPage,
    mut type_0: FmStatusTextType,
) -> *const c_char {
    return if (type_0 as c_uint) < FM_STATUS_TEXT_NUM as c_int as c_uint {
        (*page).status_text[type_0 as usize]
    } else {
        ptr::null_mut::<c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_reload(mut page: *mut FmTabPage) {
    let mut folder: *mut FmFolder = fm_folder_view_get_folder((*page).folder_view);
    if !folder.is_null() {
        let mut vadjustment: *mut GtkAdjustment = gtk_scrolled_window_get_vadjustment(
            (*page).folder_view as *mut c_void as *mut GtkScrolledWindow,
        );
        let mut scroll_pos: c_int = gtk_adjustment_get_value(vadjustment) as c_int;
        let mut idx: c_int = fm_nav_history_get_cur_index((*page).nav_history) as c_int;
        fm_nav_history_go_to((*page).nav_history, idx as guint, scroll_pos as gint);
        fm_folder_reload(folder);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_take_view_back(mut page: *mut FmTabPage) -> gboolean {
    let mut panes: *mut GList = ptr::null_mut::<GList>();
    let mut l: *mut GList = ptr::null_mut::<GList>();
    let mut folder_view: *mut GtkWidget = (*page).folder_view as *mut c_void as *mut GtkWidget;
    gtk_widget_set_state(folder_view, GTK_STATE_NORMAL);
    panes = gtk_container_get_children((*page).views as *mut c_void as *mut GtkContainer);
    l = panes;
    while !l.is_null() {
        if (*l).data as *mut GtkWidget == folder_view {
            break;
        }
        l = (*l).next;
    }
    g_list_free(panes);
    if !l.is_null() {
        return 0 as gboolean;
    }
    gtk_container_remove(
        gtk_widget_get_parent(folder_view) as *mut c_void as *mut GtkContainer,
        folder_view,
    );
    gtk_box_pack_start(
        (*page).views,
        folder_view,
        (0i32 == 0) as c_int,
        (0i32 == 0) as c_int,
        0 as guint,
    );
    return (0i32 == 0) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_set_passive_view(
    mut page: *mut FmTabPage,
    mut view: *mut FmFolderView,
    mut on_right: gboolean,
) -> gboolean {
    let mut pane: *mut GtkWidget = ptr::null_mut::<GtkWidget>();
    fm_tab_page_take_view_back(page);
    pane = view as *mut c_void as *mut GtkWidget;
    gtk_widget_set_state(pane, GTK_STATE_ACTIVE);
    g_object_ref(view as gpointer);
    if !gtk_widget_get_parent(pane).is_null() {
        gtk_container_remove(
            gtk_widget_get_parent(pane) as *mut c_void as *mut GtkContainer,
            pane,
        );
    }
    gtk_box_pack_start(
        (*page).views,
        pane,
        (0i32 == 0) as c_int,
        (0i32 == 0) as c_int,
        0 as guint,
    );
    g_object_unref(view as gpointer);
    if on_right == 0 {
        gtk_box_reorder_child((*page).views, pane, 0 as gint);
    }
    return (0i32 == 0) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_get_passive_view(
    mut page: *mut FmTabPage,
) -> *mut FmFolderView {
    let mut panes: *mut GList = ptr::null_mut::<GList>();
    let mut l: *mut GList = ptr::null_mut::<GList>();
    let mut view: *mut FmFolderView = ptr::null_mut::<FmFolderView>();
    panes = gtk_container_get_children((*page).views as *mut c_void as *mut GtkContainer);
    l = panes;
    while !l.is_null() {
        if (*l).data as *mut FmFolderView != (*page).folder_view {
            break;
        }
        l = (*l).next;
    }
    view = (if !l.is_null() {
        (*l).data
    } else {
        ptr::null_mut::<c_void>()
    }) as *mut FmFolderView;
    g_list_free(panes);
    return view;
}
#[no_mangle]
pub unsafe extern "C" fn fm_tab_page_set_filter_pattern(
    mut page: *mut FmTabPage,
    mut pattern: *const c_char,
) {
    let mut model: *mut FmFolderModel = ptr::null_mut::<FmFolderModel>();
    let mut disp_name: *mut c_char = ptr::null_mut::<c_char>();
    if !pattern.is_null() && *pattern.offset(0 as isize) as c_int == 0 {
        pattern = ptr::null::<c_char>();
    }
    if !(*page).folder_view.is_null() {
        model = fm_folder_view_get_model((*page).folder_view);
    }
    if (*page).filter_pattern.is_null() && pattern.is_null() {
        return;
    }
    if !model.is_null() {
        if (*page).filter_pattern.is_null() && !pattern.is_null() {
            fm_folder_model_add_filter(
                model,
                Some(
                    fm_tab_page_path_filter
                        as unsafe extern "C" fn(*mut FmFileInfo, gpointer) -> gboolean,
                ),
                page as gpointer,
            );
        } else if !(*page).filter_pattern.is_null() && pattern.is_null() {
            fm_folder_model_remove_filter(
                model,
                Some(
                    fm_tab_page_path_filter
                        as unsafe extern "C" fn(*mut FmFileInfo, gpointer) -> gboolean,
                ),
                page as gpointer,
            );
        }
    }
    g_free((*page).filter_pattern as gpointer);
    if !pattern.is_null() {
        let mut casefold: *mut c_char =
            g_utf8_casefold(pattern as *const gchar, -(1i32) as gssize) as *mut c_char;
        (*page).filter_pattern =
            g_utf8_normalize(casefold, -(1i32) as gssize, G_NORMALIZE_ALL) as *mut c_char;
        g_free(casefold as gpointer);
    } else {
        (*page).filter_pattern = ptr::null_mut::<c_char>();
    }
    if !model.is_null() {
        fm_folder_model_apply_filters(model);
    }
    disp_name = fm_path_display_basename(fm_folder_view_get_cwd((*page).folder_view));
    if !(*page).filter_pattern.is_null() {
        let mut text: *mut c_char = g_strdup_printf(
            b"%s [%s]\0" as *const u8 as *const gchar,
            disp_name,
            (*page).filter_pattern,
        ) as *mut c_char;
        g_free(disp_name as gpointer);
        disp_name = text;
    }
    fm_tab_label_set_text((*page).tab_label, disp_name);
    g_free(disp_name as gpointer);
}
