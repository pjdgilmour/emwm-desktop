// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      pref.c
 *
 *      Copyright 2009 PCMan <pcman.tw@gmail.com>
 *      Copyright 2012-2014 Andriy Grytsenko (LStranger) <andrej@rep.kiev.ua>
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

static mut pref_dlg: *mut GtkWindow = ptr::null::<GtkWindow>() as *mut GtkWindow;
static mut notebook: *mut GtkNotebook = ptr::null::<GtkNotebook>() as *mut GtkNotebook;
static mut b_objects: [*mut GObject; 2] = [ptr::null::<GObject>() as *mut GObject; 2];
unsafe extern "C" fn pref_on_response(
    mut dlg: *mut GtkDialog,
    mut res: c_int,
    mut pdlg: *mut *mut GtkWindow,
) {
    *pdlg = ptr::null_mut::<GtkWindow>();
    pcmanfm_save_config((0i32 == 0) as c_int);
    gtk_widget_destroy(dlg as *mut c_void as *mut GtkWidget);
}
unsafe extern "C" fn on_icon_size_changed(mut combo: *mut GtkComboBox, mut _off: gpointer) {
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if gtk_combo_box_get_active_iter(combo, &raw mut it) != 0 {
        let mut off: gsize = _off as gsize;
        let mut val: *mut c_int =
            (fm_config as *mut guint8).offset(off as glong as isize) as gpointer as *mut c_int;
        let mut size: c_int = 0;
        let mut model: *mut GtkTreeModel = gtk_combo_box_get_model(combo);
        gtk_tree_model_get(model, &raw mut it, 1i32, &raw mut size, -(1i32));
        if size != *val {
            let mut name: *const c_char =
                gtk_buildable_get_name(combo as *mut GtkBuildable) as *const c_char;
            *val = size;
            fm_config_emit_changed(fm_config, name);
        }
    }
}
unsafe extern "C" fn init_icon_sizes(
    mut builder: *mut GtkBuilder,
    mut name: *const c_char,
    mut off: gsize,
) {
    let mut combo: *mut GtkComboBox =
        gtk_builder_get_object(builder, name as *const gchar) as *mut GtkComboBox;
    let mut model: *mut GtkTreeModel = gtk_combo_box_get_model(combo);
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut val: *mut c_int =
        (fm_config as *mut guint8).offset(off as glong as isize) as gpointer as *mut c_int;
    gtk_tree_model_get_iter_first(model, &raw mut it);
    gtk_combo_box_set_active_iter(combo, &raw mut it);
    loop {
        let mut size: c_int = 0;
        gtk_tree_model_get(model, &raw mut it, 1i32, &raw mut size, -(1i32));
        if size == *val {
            gtk_combo_box_set_active_iter(combo, &raw mut it);
            break;
        } else if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
            break;
        }
    }
    g_signal_connect_data(
        combo as gpointer,
        b"changed\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkComboBox, gpointer) -> ()>, GCallback>(
            Some(on_icon_size_changed as unsafe extern "C" fn(*mut GtkComboBox, gpointer) -> ()),
        ),
        off as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
}
unsafe extern "C" fn on_combo_changed(mut combo: *mut GtkComboBox, mut _off: gpointer) {
    let mut off: gsize = _off as gsize;
    let mut val: *mut c_int =
        (fm_config as *mut guint8).offset(off as glong as isize) as gpointer as *mut c_int;
    let mut sel: c_int = gtk_combo_box_get_active(combo) as c_int;
    if sel != *val {
        let mut name: *const c_char = g_object_get_data(
            combo as *mut GObject,
            b"changed\0" as *const u8 as *const gchar,
        ) as *const c_char;
        if name.is_null() {
            name = gtk_buildable_get_name(combo as *mut GtkBuildable) as *const c_char;
        }
        *val = sel;
        fm_config_emit_changed(fm_config, name);
    }
}
unsafe extern "C" fn init_combo(
    mut builder: *mut GtkBuilder,
    mut name: *const c_char,
    mut off: gsize,
    mut changed_notify: *const c_char,
) {
    let mut combo: *mut GtkComboBox =
        gtk_builder_get_object(builder, name as *const gchar) as *mut GtkComboBox;
    let mut val: *mut c_int =
        (fm_config as *mut guint8).offset(off as glong as isize) as gpointer as *mut c_int;
    if !changed_notify.is_null() {
        g_object_set_data_full(
            combo as *mut c_void as *mut GObject,
            b"changed\0" as *const u8 as *const gchar,
            g_strdup_inline(changed_notify) as gpointer,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    gtk_combo_box_set_active(combo, *val);
    g_signal_connect_data(
        combo as gpointer,
        b"changed\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkComboBox, gpointer) -> ()>, GCallback>(
            Some(on_combo_changed as unsafe extern "C" fn(*mut GtkComboBox, gpointer) -> ()),
        ),
        off as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
}
unsafe extern "C" fn on_archiver_combo_changed(
    mut combo: *mut GtkComboBox,
    mut user_data: gpointer,
) {
    let mut model: *mut GtkTreeModel = gtk_combo_box_get_model(combo);
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if gtk_combo_box_get_active_iter(combo, &raw mut it) != 0 {
        let mut archiver: *mut FmArchiver = ptr::null_mut::<FmArchiver>();
        gtk_tree_model_get(model, &raw mut it, 1i32, &raw mut archiver, -(1i32));
        if !archiver.is_null() {
            g_free((*fm_config).archiver as gpointer);
            (*fm_config).archiver = g_strdup_inline((*archiver).program);
            fm_archiver_set_default(archiver);
            fm_config_emit_changed(fm_config, c"archiver".as_ptr());
        }
    }
}
unsafe extern "C" fn init_archiver_combo(mut builder: *mut GtkBuilder) {
    let mut model: *mut GtkListStore = gtk_list_store_new(
        2 as gint,
        ((16 as c_int) << 2 as c_int) as GType,
        ((17 as c_int) << 2 as c_int) as GType,
    );
    let mut combo: *mut GtkComboBox =
        gtk_builder_get_object(builder, b"archiver\0" as *const u8 as *const gchar)
            as *mut GtkComboBox;
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut archivers: *const GList = fm_archiver_get_all();
    let mut default_archiver: *mut FmArchiver = fm_archiver_get_default();
    let mut l: *const GList = ptr::null::<GList>();
    gtk_combo_box_set_model(combo, model as *mut c_void as *mut GtkTreeModel);
    l = archivers;
    while !l.is_null() {
        let mut archiver: *mut FmArchiver = (*l).data as *mut FmArchiver;
        gtk_list_store_insert_with_values(
            model,
            &raw mut it,
            -(1 as gint),
            0i32,
            (*archiver).program,
            1i32,
            archiver,
            -(1i32),
        );
        if archiver == default_archiver {
            gtk_combo_box_set_active_iter(combo, &raw mut it);
        }
        l = (*l).next;
    }
    g_object_unref(model as gpointer);
    g_signal_connect_data(
        combo as gpointer,
        b"changed\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkComboBox, gpointer) -> ()>, GCallback>(
            Some(
                on_archiver_combo_changed as unsafe extern "C" fn(*mut GtkComboBox, gpointer) -> (),
            ),
        ),
        ptr::null_mut::<c_void>(),
        None,
        G_CONNECT_DEFAULT,
    );
}
unsafe extern "C" fn on_auto_sel_changed(mut scale: *mut GtkRange, mut unused: gpointer) {
    let mut new_val: gint = (gtk_range_get_value(scale) * 1000 as gdouble) as gint;
    if new_val != (*fm_config).auto_selection_delay {
        (*fm_config).auto_selection_delay = new_val;
        fm_config_emit_changed(fm_config, c"auto_selection_delay".as_ptr());
    }
}
unsafe extern "C" fn init_auto_selection_delay_scale(mut builder: *mut GtkBuilder) {
    let mut scale: *mut GtkScale = ptr::null_mut::<GtkScale>();
    let mut val: gdouble = (*fm_config).auto_selection_delay as gdouble * 0.001f64;
    scale = gtk_builder_get_object(
        builder,
        b"auto_selection_delay\0" as *const u8 as *const gchar,
    ) as *mut c_void as *mut GtkScale;
    gtk_range_set_value(scale as *mut c_void as *mut GtkRange, val);
    g_signal_connect_data(
        scale as gpointer,
        b"value-changed\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkRange, gpointer) -> ()>, GCallback>(
            Some(on_auto_sel_changed as unsafe extern "C" fn(*mut GtkRange, gpointer) -> ()),
        ),
        ptr::null_mut::<c_void>(),
        None,
        G_CONNECT_DEFAULT,
    );
    gtk_widget_show(
        gtk_builder_get_object(builder, b"auto_sel_box\0" as *const u8 as *const gchar)
            as *mut c_void as *mut GtkWidget,
    );
}
unsafe extern "C" fn on_drop_default_action_changed(
    mut combo: *mut GtkComboBox,
    mut smart_desktop_autodrop: gpointer,
) {
    let mut sel: c_int = gtk_combo_box_get_active(combo) as c_int;
    match sel {
        0 => {
            sel = FM_DND_DEST_DROP_AUTO as c_int;
        }
        1 => {
            sel = FM_DND_DEST_DROP_COPY as c_int;
        }
        2 => {
            sel = FM_DND_DEST_DROP_MOVE as c_int;
        }
        _ => {
            sel = FM_DND_DEST_DROP_ASK as c_int;
        }
    }
    if sel != (*fm_config).drop_default_action {
        (*fm_config).drop_default_action = sel as gint;
        fm_config_emit_changed(fm_config, c"drop_default_action".as_ptr());
        gtk_widget_set_sensitive(
            smart_desktop_autodrop as *mut GtkWidget,
            (sel == FM_DND_DEST_DROP_AUTO as c_int) as c_int,
        );
    }
}
unsafe extern "C" fn init_drop_default_action_combo(mut builder: *mut GtkBuilder) {
    let mut combo: *mut GObject = gtk_builder_get_object(
        builder,
        b"drop_default_action\0" as *const u8 as *const gchar,
    );
    let mut smart_desktop_autodrop: *mut GObject = gtk_builder_get_object(
        builder,
        b"smart_desktop_autodrop\0" as *const u8 as *const gchar,
    );
    let mut var: gint = 0;
    match (*fm_config).drop_default_action {
        1 => {
            var = 1 as gint;
        }
        2 => {
            var = 2 as gint;
        }
        3 => {
            var = 3 as gint;
        }
        _ => {
            var = 0 as gint;
        }
    }
    gtk_combo_box_set_active(combo as *mut GtkComboBox, var);
    gtk_widget_set_sensitive(
        smart_desktop_autodrop as *mut c_void as *mut GtkWidget,
        ((*fm_config).drop_default_action == FM_DND_DEST_DROP_AUTO as c_int) as c_int,
    );
    g_signal_connect_data(
        combo as gpointer,
        b"changed\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkComboBox, gpointer) -> ()>, GCallback>(
            Some(
                on_drop_default_action_changed
                    as unsafe extern "C" fn(*mut GtkComboBox, gpointer) -> (),
            ),
        ),
        smart_desktop_autodrop as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
}
unsafe extern "C" fn on_toggled(mut btn: *mut GtkToggleButton, mut _off: gpointer) {
    let mut off: gsize = _off as gsize;
    let mut val: *mut gboolean =
        (fm_config as *mut guint8).offset(off as glong as isize) as gpointer as *mut gboolean;
    let mut new_val: gboolean = gtk_toggle_button_get_active(btn);
    if *val != new_val {
        let mut name: *const c_char = g_object_get_data(
            btn as *mut GObject,
            b"changed\0" as *const u8 as *const gchar,
        ) as *const c_char;
        if name.is_null() {
            name = gtk_buildable_get_name(btn as *mut GtkBuildable) as *const c_char;
        }
        *val = new_val;
        fm_config_emit_changed(fm_config, name);
    }
}
unsafe extern "C" fn init_bool(
    mut b: *mut GtkBuilder,
    mut name: *const c_char,
    mut off: gsize,
    mut changed_notify: *const c_char,
    mut force_show: gboolean,
) {
    let mut btn: *mut GtkToggleButton =
        gtk_builder_get_object(b, name as *const gchar) as *mut c_void as *mut GtkToggleButton;
    let mut val: *mut gboolean =
        (fm_config as *mut guint8).offset(off as glong as isize) as gpointer as *mut gboolean;
    if !changed_notify.is_null() {
        g_object_set_data_full(
            btn as *mut c_void as *mut GObject,
            b"changed\0" as *const u8 as *const gchar,
            g_strdup_inline(changed_notify) as gpointer,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    if force_show != 0 {
        gtk_widget_show(btn as *mut c_void as *mut GtkWidget);
    }
    gtk_toggle_button_set_active(btn, *val);
    g_signal_connect_data(
        btn as gpointer,
        b"toggled\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkToggleButton, gpointer) -> ()>,
            GCallback,
        >(Some(
            on_toggled as unsafe extern "C" fn(*mut GtkToggleButton, gpointer) -> (),
        )),
        off as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
}
unsafe extern "C" fn on_single_click_toggled(
    mut btn: *mut GtkToggleButton,
    mut object: *mut *mut GObject,
) {
    let mut new_val: gboolean = gtk_toggle_button_get_active(btn);
    if new_val != (*fm_config).single_click {
        (*fm_config).single_click = new_val;
        fm_config_emit_changed(fm_config, c"single_click".as_ptr());
    }
    if !(*object.offset(0 as isize)).is_null() {
        gtk_widget_set_sensitive(
            *object.offset(0 as isize) as *mut c_void as *mut GtkWidget,
            new_val,
        );
    }
    if new_val != 0 && !(*object.offset(1 as isize)).is_null() {
        gtk_toggle_button_set_active(
            *object.offset(1 as isize) as *mut c_void as *mut GtkToggleButton,
            0 as gboolean,
        );
    }
}
unsafe extern "C" fn on_middle_click_toggled(
    mut btn: *mut GtkToggleButton,
    mut single_click: gpointer,
) {
    let mut new_val: gboolean = gtk_toggle_button_get_active(btn);
    if new_val != (*fm_config).c2rust_unnamed_0.middle_click {
        (*fm_config).c2rust_unnamed_0.middle_click = new_val;
        fm_config_emit_changed(fm_config, c"middle_click".as_ptr());
    }
    if new_val != 0 {
        gtk_toggle_button_set_active(single_click as *mut GtkToggleButton, 0 as gboolean);
    }
}
unsafe extern "C" fn on_use_trash_toggled(mut btn: *mut GtkToggleButton, mut vbox_trash: gpointer) {
    let mut new_val: gboolean = gtk_toggle_button_get_active(btn);
    if new_val != (*fm_config).use_trash {
        (*fm_config).use_trash = new_val;
        fm_config_emit_changed(fm_config, c"use_trash".as_ptr());
    }
    if !vbox_trash.is_null() {
        gtk_widget_set_sensitive(vbox_trash as *mut GtkWidget, new_val);
    }
}
unsafe extern "C" fn on_close_on_unmount_toggled(
    mut btn: *mut GtkToggleButton,
    mut cfg: *mut FmAppConfig,
) {
    let mut new_val: gboolean = gtk_toggle_button_get_active(btn);
    if new_val != (*cfg).close_on_unmount {
        (*cfg).close_on_unmount = new_val;
        fm_config_emit_changed(fm_config, c"close_on_unmount".as_ptr());
    }
}
unsafe extern "C" fn on_spin_changed(mut btn: *mut GtkSpinButton, mut _off: gpointer) {
    let mut off: gsize = _off as gsize;
    let mut val: *mut guint =
        (fm_config as *mut guint8).offset(off as glong as isize) as gpointer as *mut guint;
    let mut new_val: guint = gtk_spin_button_get_value(btn) as guint;
    if *val != new_val {
        let mut name: *const c_char = g_object_get_data(
            btn as *mut GObject,
            b"changed\0" as *const u8 as *const gchar,
        ) as *const c_char;
        if name.is_null() {
            name = gtk_buildable_get_name(btn as *mut GtkBuildable) as *const c_char;
        }
        *val = new_val;
        fm_config_emit_changed(fm_config, name);
    }
}
unsafe extern "C" fn init_spin(
    mut b: *mut GtkBuilder,
    mut name: *const c_char,
    mut off: gsize,
    mut changed_notify: *const c_char,
) {
    let mut btn: *mut GtkSpinButton =
        gtk_builder_get_object(b, name as *const gchar) as *mut c_void as *mut GtkSpinButton;
    let mut val: *mut guint =
        (fm_config as *mut guint8).offset(off as glong as isize) as gpointer as *mut guint;
    if !changed_notify.is_null() {
        g_object_set_data_full(
            btn as *mut c_void as *mut GObject,
            b"changed\0" as *const u8 as *const gchar,
            g_strdup_inline(changed_notify) as gpointer,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    gtk_spin_button_set_value(btn, *val as gdouble);
    g_signal_connect_data(
        btn as gpointer,
        b"value-changed\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkSpinButton, gpointer) -> ()>, GCallback>(
            Some(on_spin_changed as unsafe extern "C" fn(*mut GtkSpinButton, gpointer) -> ()),
        ),
        off as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
}
unsafe extern "C" fn on_entry_changed(mut entry: *mut GtkEntry, mut _off: gpointer) {
    let mut off: gsize = _off as gsize;
    let mut val: *mut *mut gchar =
        (fm_config as *mut guint8).offset(off as glong as isize) as gpointer as *mut *mut gchar;
    let mut new_val: *const c_char = gtk_entry_get_text(entry) as *const c_char;
    if g_strcmp0(*val, new_val) != 0 {
        let mut name: *const c_char = g_object_get_data(
            entry as *mut GObject,
            b"changed\0" as *const u8 as *const gchar,
        ) as *const c_char;
        if name.is_null() {
            name = gtk_buildable_get_name(entry as *mut GtkBuildable) as *const c_char;
        }
        g_free(*val as gpointer);
        *val = (if *new_val as c_int != 0 {
            g_strdup_inline(new_val)
        } else {
            ptr::null_mut::<c_char>()
        }) as *mut gchar;
        fm_config_emit_changed(fm_config, name);
    }
}
unsafe extern "C" fn init_entry(
    mut b: *mut GtkBuilder,
    mut name: *const c_char,
    mut off: gsize,
    mut changed_notify: *const c_char,
) {
    let mut btn: *mut GtkEntry =
        gtk_builder_get_object(b, name as *const gchar) as *mut c_void as *mut GtkEntry;
    let mut val: *mut *mut gchar =
        (fm_config as *mut guint8).offset(off as glong as isize) as gpointer as *mut *mut gchar;
    if !changed_notify.is_null() {
        g_object_set_data_full(
            btn as *mut c_void as *mut GObject,
            b"changed\0" as *const u8 as *const gchar,
            g_strdup_inline(changed_notify) as gpointer,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    if !(*val).is_null() {
        gtk_entry_set_text(btn, *val);
    }
    g_signal_connect_data(
        btn as gpointer,
        b"changed\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut GtkEntry, gpointer) -> ()>, GCallback>(
            Some(on_entry_changed as unsafe extern "C" fn(*mut GtkEntry, gpointer) -> ()),
        ),
        off as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
}
unsafe extern "C" fn on_tab_label_list_sel_changed(
    mut tree_sel: *mut GtkTreeSelection,
    mut user_data: gpointer,
) {
    let mut tp: *mut GtkTreePath = ptr::null_mut::<GtkTreePath>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut page: c_int = 0;
    if gtk_tree_selection_get_selected(tree_sel, &raw mut model, &raw mut it) == 0 {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_WARNING,
            b"pref: on_tab_label_list_sel_changed() got no selection\0" as *const u8
                as *const gchar,
        );
        return;
    }
    tp = gtk_tree_model_get_path(model, &raw mut it);
    page = *gtk_tree_path_get_indices(tp).offset(0 as isize) as c_int;
    if gtk_notebook_get_current_page(notebook) != page {
        gtk_notebook_set_current_page(notebook, page as gint);
    }
    gtk_tree_path_free(tp);
}
unsafe extern "C" fn on_notebook_page_changed(
    mut notebook_0: *mut GtkNotebook,
    mut page: gpointer,
    mut n: guint,
    mut view: *mut GtkTreeView,
) {
    let mut tp: *mut GtkTreePath = ptr::null_mut::<GtkTreePath>();
    gtk_tree_view_get_cursor(view, &raw mut tp, ptr::null_mut::<*mut GtkTreeViewColumn>());
    if tp.is_null() || *gtk_tree_path_get_indices(tp).offset(0 as isize) != n as c_int {
        if !tp.is_null() {
            gtk_tree_path_free(tp);
        }
        tp = gtk_tree_path_new_from_indices(n as gint, -(1i32));
        gtk_tree_view_set_cursor(
            view,
            tp,
            ptr::null_mut::<GtkTreeViewColumn>(),
            0 as gboolean,
        );
    }
    gtk_tree_path_free(tp);
}
unsafe extern "C" fn pref_on_key_press(
    mut w: *mut GtkWidget,
    mut evt: *mut GdkEventKey,
    mut notebook_0: *mut GtkNotebook,
) -> gboolean {
    let mut modifier: c_int =
        ((*evt).state as c_uint & gtk_accelerator_get_default_mod_mask() as c_uint) as c_int;
    if modifier == GDK_MOD1_MASK as c_int {
        if (*evt).keyval >= '1' as i32 as guint && (*evt).keyval <= '9' as i32 as guint {
            gtk_notebook_set_current_page(
                notebook_0,
                (*evt).keyval.wrapping_sub('1' as i32 as guint) as gint,
            );
            return (0i32 == 0) as c_int;
        }
    }
    return 0 as gboolean;
}
unsafe extern "C" fn on_autorun_toggled(
    mut btn: *mut GtkToggleButton,
    mut autorun_choices_area: *mut GtkWidget,
) {
    let mut new_val: gboolean = gtk_toggle_button_get_active(btn);
    if new_val != (*(fm_config as *mut FmAppConfig)).autorun {
        (*(fm_config as *mut FmAppConfig)).autorun = new_val;
        fm_config_emit_changed(fm_config, c"autorun".as_ptr());
    }
    if !autorun_choices_area.is_null() {
        gtk_widget_set_sensitive(autorun_choices_area, new_val);
    }
}
unsafe extern "C" fn on_remove_autorun_choice_clicked(
    mut button: *mut GtkButton,
    mut view: *mut GtkTreeView,
) {
    let mut tree_sel: *mut GtkTreeSelection = gtk_tree_view_get_selection(view);
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut rows: *mut GList = gtk_tree_selection_get_selected_rows(tree_sel, &raw mut model);
    let mut l: *mut GList = ptr::null_mut::<GList>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    l = rows;
    while !l.is_null() {
        let mut tp: *mut GtkTreePath = (*l).data as *mut GtkTreePath;
        (*l).data = gtk_tree_row_reference_new(model, tp) as gpointer;
        gtk_tree_path_free(tp);
        l = (*l).next;
    }
    l = rows;
    while !l.is_null() {
        if gtk_tree_model_get_iter(
            model,
            &raw mut it,
            gtk_tree_row_reference_get_path((*l).data as *mut GtkTreeRowReference),
        ) != 0
        {
            let mut type_0: *mut c_char = ptr::null_mut::<c_char>();
            gtk_tree_model_get(model, &raw mut it, 2 as c_int, &raw mut type_0, -(1i32));
            if !type_0.is_null() {
                g_hash_table_remove(
                    (*(fm_config as *mut FmAppConfig)).autorun_choices,
                    type_0 as gconstpointer,
                );
            }
            g_free(type_0 as gpointer);
            gtk_list_store_remove(model as *mut c_void as *mut GtkListStore, &raw mut it);
        } else {
            g_log(
                ptr::null_mut::<gchar>(),
                G_LOG_LEVEL_CRITICAL,
                b"autorun_choice not found in model\0" as *const u8 as *const gchar,
            );
        }
        gtk_tree_row_reference_free((*l).data as *mut GtkTreeRowReference);
        l = (*l).next;
    }
    g_list_free(rows);
}
unsafe extern "C" fn on_choices_sel_changed(
    mut selection: *mut GtkTreeSelection,
    mut btn: *mut GtkWidget,
) {
    gtk_widget_set_sensitive(
        btn,
        (gtk_tree_selection_count_selected_rows(selection) > 0i32) as c_int,
    );
}
unsafe extern "C" fn on_use_home_path_toggled(
    mut btn: *mut GtkToggleButton,
    mut home_path_custom: *mut GtkWidget,
) {
    let mut active: gboolean = gtk_toggle_button_get_active(btn);
    gtk_widget_set_sensitive(home_path_custom, active);
}
unsafe extern "C" fn on_use_home_path_toggled2(
    mut btn: *mut GtkToggleButton,
    mut home_path: *mut GtkEntry,
) {
    if gtk_toggle_button_get_active(btn) == 0 {
        gtk_entry_set_text(home_path, fm_get_home_dir() as *const gchar);
    }
}
unsafe extern "C" fn on_home_path_changed(mut home_path: *mut GtkEntry, mut cfg: *mut FmAppConfig) {
    let mut path: *const c_char = gtk_entry_get_text(home_path) as *const c_char;
    if *path.offset(0 as isize) as c_int != 0 && strcmp(path, fm_get_home_dir()) != 0i32 {
        if g_strcmp0(path, (*cfg).home_path) == 0i32 {
            return;
        }
        g_free((*cfg).home_path as gpointer);
        (*cfg).home_path = g_strdup_inline(path);
    } else {
        g_free((*cfg).home_path as gpointer);
        (*cfg).home_path = ptr::null_mut::<c_char>();
    }
    fm_config_emit_changed(cfg as *mut c_void as *mut FmConfig, c"home_path".as_ptr());
}
unsafe extern "C" fn on_home_path_current_clicked(
    mut button: *mut GtkButton,
    mut home_path: *mut GtkEntry,
) {
    let mut win: *mut FmMainWin = fm_main_win_get_last_active();
    let mut cwd: *mut FmPath = ptr::null_mut::<FmPath>();
    let mut path: *mut c_char = ptr::null_mut::<c_char>();
    if win.is_null() || (*win).folder_view.is_null() {
        return;
    }
    cwd = fm_folder_view_get_cwd((*win).folder_view);
    path = fm_path_to_str(cwd);
    gtk_entry_set_text(home_path, path);
    g_free(path as gpointer);
}
unsafe extern "C" fn on_add_to_blacklist_clicked(
    mut button: *mut GtkButton,
    mut model: *mut GtkListStore,
) {
    let mut item: *mut c_char = fm_get_user_input(
        ptr::null_mut::<GtkWindow>(),
        dcgettext(
            ptr::null::<c_char>(),
            c"Add to Modules Blacklist".as_ptr(),
            5 as c_int,
        ),
        dcgettext(
            ptr::null::<c_char>(),
            c"Enter a blacklisted module mask:".as_ptr(),
            5 as c_int,
        ),
        ptr::null::<c_char>(),
    );
    let mut i: c_int = 0;
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if item.is_null() || *item.offset(0 as isize) as c_int == 0 {
        return;
    }
    gtk_list_store_append(model, &raw mut it);
    gtk_list_store_set(
        model,
        &raw mut it,
        0i32,
        item,
        1i32,
        (0i32 == 0) as c_int,
        -(1i32),
    );
    i = (if !(*fm_config).modules_blacklist.is_null() {
        g_strv_length((*fm_config).modules_blacklist)
    } else {
        0 as guint
    }) as c_int;
    (*fm_config).modules_blacklist = ({
        let mut __n: gsize = (i + 2 as c_int) as gsize;
        let mut __s: gsize = mem::size_of::<*mut c_char>() as gsize;
        let mut __p: gpointer = (*fm_config).modules_blacklist as gpointer;
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
    }) as *mut *mut c_char as *mut *mut gchar;
    let ref mut fresh21 = *(*fm_config).modules_blacklist.offset((i + 1i32) as isize);
    *fresh21 = ptr::null_mut::<gchar>();
    let ref mut fresh22 = *(*fm_config).modules_blacklist.offset(i as isize);
    *fresh22 = item as *mut gchar;
}
unsafe extern "C" fn on_remove_from_blacklist_clicked(
    mut button: *mut GtkButton,
    mut view: *mut GtkTreeView,
) {
    let mut tree_sel: *mut GtkTreeSelection = gtk_tree_view_get_selection(view);
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut rows: *mut GList = gtk_tree_selection_get_selected_rows(tree_sel, &raw mut model);
    let mut l: *mut GList = ptr::null_mut::<GList>();
    let mut item: *mut c_char = ptr::null_mut::<c_char>();
    let mut bl: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
    let mut i: c_int = 0;
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    l = rows;
    while !l.is_null() {
        let mut tp: *mut GtkTreePath = (*l).data as *mut GtkTreePath;
        (*l).data = gtk_tree_row_reference_new(model, tp) as gpointer;
        gtk_tree_path_free(tp);
        l = (*l).next;
    }
    l = rows;
    while !l.is_null() {
        if gtk_tree_model_get_iter(
            model,
            &raw mut it,
            gtk_tree_row_reference_get_path((*l).data as *mut GtkTreeRowReference),
        ) != 0
        {
            gtk_list_store_remove(model as *mut c_void as *mut GtkListStore, &raw mut it);
        } else {
            g_log(
                ptr::null_mut::<gchar>(),
                G_LOG_LEVEL_CRITICAL,
                b"the item not found in model\0" as *const u8 as *const gchar,
            );
        }
        gtk_tree_row_reference_free((*l).data as *mut GtkTreeRowReference);
        l = (*l).next;
    }
    g_list_free(rows);
    if gtk_tree_model_get_iter_first(model, &raw mut it) != 0 {
        bl = ({
            let mut __n: gsize =
                (gtk_tree_model_iter_n_children(model, ptr::null_mut::<GtkTreeIter>()) as c_int
                    + 1i32) as gsize;
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
        loop {
            gtk_tree_model_get(model, &raw mut it, 0i32, &raw mut item, -(1i32));
            let fresh18 = i;
            i = i + 1;
            let ref mut fresh19 = *bl.offset(fresh18 as isize);
            *fresh19 = item;
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        }
        let ref mut fresh20 = *bl.offset(i as isize);
        *fresh20 = ptr::null_mut::<c_char>();
    } else {
        bl = ptr::null_mut::<*mut c_char>();
    }
    g_strfreev((*fm_config).modules_blacklist);
    (*fm_config).modules_blacklist = bl as *mut *mut gchar;
}
unsafe extern "C" fn on_blacklist_sel_changed(
    mut selection: *mut GtkTreeSelection,
    mut btn: *mut GtkWidget,
) {
    let mut rows: *mut GList = ptr::null_mut::<GList>();
    let mut l: *mut GList = ptr::null_mut::<GList>();
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut tp: *mut GtkTreePath = ptr::null_mut::<GtkTreePath>();
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut can_del: gboolean = 0;
    if gtk_tree_selection_count_selected_rows(selection) == 0i32 {
        gtk_widget_set_sensitive(btn, 0 as gboolean);
        return;
    }
    rows = gtk_tree_selection_get_selected_rows(selection, &raw mut model);
    can_del = (0i32 == 0) as c_int as gboolean;
    l = rows;
    while !l.is_null() {
        tp = (*l).data as *mut GtkTreePath;
        if can_del != 0 && gtk_tree_model_get_iter(model, &raw mut it, tp) != 0 {
            gtk_tree_model_get(model, &raw mut it, 1i32, &raw mut can_del, -(1i32));
        } else {
            can_del = 0 as gboolean;
        }
        gtk_tree_path_free(tp);
        l = (*l).next;
    }
    g_list_free(rows);
    gtk_widget_set_sensitive(btn, can_del);
}
unsafe extern "C" fn on_add_to_whitelist_clicked(
    mut button: *mut GtkButton,
    mut model: *mut GtkListStore,
) {
    let mut item: *mut c_char = fm_get_user_input(
        ptr::null_mut::<GtkWindow>(),
        dcgettext(
            ptr::null::<c_char>(),
            c"Add to Modules Whitelist".as_ptr(),
            5 as c_int,
        ),
        dcgettext(
            ptr::null::<c_char>(),
            c"Enter a whitelisted module mask:".as_ptr(),
            5 as c_int,
        ),
        ptr::null::<c_char>(),
    );
    let mut i: c_int = 0;
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    if item.is_null() || *item.offset(0 as isize) as c_int == 0 {
        return;
    }
    gtk_list_store_append(model, &raw mut it);
    gtk_list_store_set(model, &raw mut it, 0i32, item, -(1i32));
    i = (if !(*fm_config).modules_whitelist.is_null() {
        g_strv_length((*fm_config).modules_whitelist)
    } else {
        0 as guint
    }) as c_int;
    (*fm_config).modules_whitelist = ({
        let mut __n: gsize = (i + 2 as c_int) as gsize;
        let mut __s: gsize = mem::size_of::<*mut c_char>() as gsize;
        let mut __p: gpointer = (*fm_config).modules_whitelist as gpointer;
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
    }) as *mut *mut c_char as *mut *mut gchar;
    let ref mut fresh16 = *(*fm_config).modules_whitelist.offset((i + 1i32) as isize);
    *fresh16 = ptr::null_mut::<gchar>();
    let ref mut fresh17 = *(*fm_config).modules_whitelist.offset(i as isize);
    *fresh17 = item as *mut gchar;
}
unsafe extern "C" fn on_remove_from_whitelist_clicked(
    mut button: *mut GtkButton,
    mut view: *mut GtkTreeView,
) {
    let mut tree_sel: *mut GtkTreeSelection = gtk_tree_view_get_selection(view);
    let mut model: *mut GtkTreeModel = ptr::null_mut::<GtkTreeModel>();
    let mut rows: *mut GList = gtk_tree_selection_get_selected_rows(tree_sel, &raw mut model);
    let mut l: *mut GList = ptr::null_mut::<GList>();
    let mut item: *mut c_char = ptr::null_mut::<c_char>();
    let mut wl: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
    let mut i: c_int = 0;
    let mut it: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    l = rows;
    while !l.is_null() {
        let mut tp: *mut GtkTreePath = (*l).data as *mut GtkTreePath;
        (*l).data = gtk_tree_row_reference_new(model, tp) as gpointer;
        gtk_tree_path_free(tp);
        l = (*l).next;
    }
    l = rows;
    while !l.is_null() {
        if gtk_tree_model_get_iter(
            model,
            &raw mut it,
            gtk_tree_row_reference_get_path((*l).data as *mut GtkTreeRowReference),
        ) != 0
        {
            gtk_list_store_remove(model as *mut c_void as *mut GtkListStore, &raw mut it);
        } else {
            g_log(
                ptr::null_mut::<gchar>(),
                G_LOG_LEVEL_CRITICAL,
                b"the item not found in model\0" as *const u8 as *const gchar,
            );
        }
        gtk_tree_row_reference_free((*l).data as *mut GtkTreeRowReference);
        l = (*l).next;
    }
    g_list_free(rows);
    if gtk_tree_model_get_iter_first(model, &raw mut it) != 0 {
        wl = ({
            let mut __n: gsize =
                (gtk_tree_model_iter_n_children(model, ptr::null_mut::<GtkTreeIter>()) as c_int
                    + 1i32) as gsize;
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
        loop {
            gtk_tree_model_get(model, &raw mut it, 0i32, &raw mut item, -(1i32));
            let fresh13 = i;
            i = i + 1;
            let ref mut fresh14 = *wl.offset(fresh13 as isize);
            *fresh14 = item;
            if !(gtk_tree_model_iter_next(model, &raw mut it) != 0) {
                break;
            }
        }
        let ref mut fresh15 = *wl.offset(i as isize);
        *fresh15 = ptr::null_mut::<c_char>();
    } else {
        wl = ptr::null_mut::<*mut c_char>();
    }
    g_strfreev((*fm_config).modules_whitelist);
    (*fm_config).modules_whitelist = wl as *mut *mut gchar;
}
unsafe extern "C" fn on_whitelist_sel_changed(
    mut selection: *mut GtkTreeSelection,
    mut btn: *mut GtkWidget,
) {
    gtk_widget_set_sensitive(
        btn,
        (gtk_tree_selection_count_selected_rows(selection) > 0i32) as c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fm_edit_preference(mut parent: *mut GtkWindow, mut page: c_int) {
    if pref_dlg.is_null() {
        let mut builder: *mut GtkBuilder = gtk_builder_new();
        let mut tab_label_list: *mut GtkTreeView = ptr::null_mut::<GtkTreeView>();
        let mut tree_sel: *mut GtkTreeSelection = ptr::null_mut::<GtkTreeSelection>();
        let mut tab_label_model: *mut GtkListStore = ptr::null_mut::<GtkListStore>();
        let mut obj: *mut GObject = ptr::null_mut::<GObject>();
        let mut it: GtkTreeIter = _GtkTreeIter {
            stamp: 0,
            user_data: ptr::null_mut::<c_void>(),
            user_data2: ptr::null_mut::<c_void>(),
            user_data3: ptr::null_mut::<c_void>(),
        };
        let mut i: c_int = 0;
        let mut n: c_int = 0;
        gtk_builder_add_from_file(
            builder,
            b"/usr/share/pcmanfm/ui/pref.ui\0" as *const u8 as *const gchar,
            ptr::null_mut::<*mut GError>(),
        );
        pref_dlg = gtk_builder_get_object(builder, b"dlg\0" as *const u8 as *const gchar)
            as *mut c_void as *mut GtkWindow;
        notebook = gtk_builder_get_object(builder, b"notebook\0" as *const u8 as *const gchar)
            as *mut c_void as *mut GtkNotebook;
        b_objects[0 as usize] =
            gtk_builder_get_object(builder, b"auto_sel_box\0" as *const u8 as *const gchar);
        b_objects[1 as usize] =
            gtk_builder_get_object(builder, b"middle_click\0" as *const u8 as *const gchar);
        g_signal_connect_data(
            gtk_builder_get_object(builder, b"single_click\0" as *const u8 as *const gchar)
                as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut *mut GObject) -> ()>,
                GCallback,
            >(Some(
                on_single_click_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, *mut *mut GObject) -> (),
            )),
            &raw mut b_objects as *mut *mut GObject as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        gtk_toggle_button_set_active(
            gtk_builder_get_object(builder, b"single_click\0" as *const u8 as *const gchar)
                as *mut c_void as *mut GtkToggleButton,
            (*fm_config).single_click,
        );
        g_signal_connect_data(
            gtk_builder_get_object(builder, b"middle_click\0" as *const u8 as *const gchar)
                as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, gpointer) -> ()>,
                GCallback,
            >(Some(
                on_middle_click_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, gpointer) -> (),
            )),
            gtk_builder_get_object(builder, b"single_click\0" as *const u8 as *const gchar)
                as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        gtk_toggle_button_set_active(
            gtk_builder_get_object(builder, b"middle_click\0" as *const u8 as *const gchar)
                as *mut c_void as *mut GtkToggleButton,
            (*fm_config).c2rust_unnamed_0.middle_click,
        );
        gtk_widget_show(gtk_builder_get_object(
            builder,
            b"middle_click\0" as *const u8 as *const gchar,
        ) as *mut c_void as *mut GtkWidget);
        init_auto_selection_delay_scale(builder);
        init_bool(
            builder,
            c"confirm_del".as_ptr(),
            84 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        g_signal_connect_data(
            gtk_builder_get_object(builder, b"use_trash\0" as *const u8 as *const gchar)
                as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, gpointer) -> ()>,
                GCallback,
            >(Some(
                on_use_trash_toggled as unsafe extern "C" fn(*mut GtkToggleButton, gpointer) -> (),
            )),
            gtk_builder_get_object(builder, b"vbox_trash\0" as *const u8 as *const gchar)
                as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        gtk_toggle_button_set_active(
            gtk_builder_get_object(builder, b"use_trash\0" as *const u8 as *const gchar)
                as *mut c_void as *mut GtkToggleButton,
            (*fm_config).use_trash,
        );
        init_bool(
            builder,
            c"no_usb_trash".as_ptr(),
            120 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            (0i32 == 0) as c_int,
        );
        init_bool(
            builder,
            c"confirm_trash".as_ptr(),
            88 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            (0i32 == 0) as c_int,
        );
        init_bool(
            builder,
            c"quick_exec".as_ptr(),
            184 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            (0i32 == 0) as c_int,
        );
        init_combo(
            builder,
            c"bm_open_method".as_ptr(),
            312 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
        );
        init_drop_default_action_combo(builder);
        gtk_widget_show(gtk_builder_get_object(
            builder,
            b"drop_default_action\0" as *const u8 as *const gchar,
        ) as *mut c_void as *mut GtkWidget);
        gtk_widget_show(gtk_builder_get_object(
            builder,
            b"drop_default_action_label\0" as *const u8 as *const gchar,
        ) as *mut c_void as *mut GtkWidget);
        init_bool(
            builder,
            c"smart_desktop_autodrop".as_ptr(),
            232 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            (0i32 == 0) as c_int,
        );
        init_bool(
            builder,
            c"focus_previous".as_ptr(),
            376 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            (0i32 == 0) as c_int,
        );
        init_bool(
            builder,
            c"change_tab_on_drop".as_ptr(),
            368 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            (0i32 == 0) as c_int,
        );
        gtk_widget_show(gtk_builder_get_object(
            builder,
            b"on_unmount_vbox\0" as *const u8 as *const gchar,
        ) as *mut c_void as *mut GtkWidget);
        if (*(fm_config as *mut FmAppConfig)).close_on_unmount != 0 {
            gtk_toggle_button_set_active(
                gtk_builder_get_object(builder, b"close_on_unmount\0" as *const u8 as *const gchar)
                    as *mut c_void as *mut GtkToggleButton,
                (0i32 == 0) as c_int,
            );
        } else {
            gtk_toggle_button_set_active(
                gtk_builder_get_object(
                    builder,
                    b"go_home_on_unmount\0" as *const u8 as *const gchar,
                ) as *mut c_void as *mut GtkToggleButton,
                (0i32 == 0) as c_int,
            );
        }
        g_signal_connect_data(
            gtk_builder_get_object(builder, b"close_on_unmount\0" as *const u8 as *const gchar)
                as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut FmAppConfig) -> ()>,
                GCallback,
            >(Some(
                on_close_on_unmount_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, *mut FmAppConfig) -> (),
            )),
            fm_config as *mut FmAppConfig as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        init_combo(
            builder,
            c"view_mode".as_ptr(),
            392 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
        );
        init_icon_sizes(
            builder,
            c"big_icon_size".as_ptr(),
            48 as c_ulong as glong as gsize,
        );
        init_icon_sizes(
            builder,
            c"small_icon_size".as_ptr(),
            52 as c_ulong as glong as gsize,
        );
        init_icon_sizes(
            builder,
            c"thumbnail_size".as_ptr(),
            60 as c_ulong as glong as gsize,
        );
        init_icon_sizes(
            builder,
            c"pane_icon_size".as_ptr(),
            56 as c_ulong as glong as gsize,
        );
        init_bool(
            builder,
            c"show_thumbnail".as_ptr(),
            92 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"thumbnail_local".as_ptr(),
            96 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_spin(
            builder,
            c"thumbnail_max".as_ptr(),
            64 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
        );
        init_bool(
            builder,
            c"si_unit".as_ptr(),
            104 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"date_iso_8601".as_ptr(),
            248 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            (0i32 == 0) as c_int,
        );
        init_bool(
            builder,
            c"backup_as_hidden".as_ptr(),
            116 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"show_full_names".as_ptr(),
            128 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            (0i32 == 0) as c_int,
        );
        init_bool(
            builder,
            c"shadow_hidden".as_ptr(),
            132 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            (0i32 == 0) as c_int,
        );
        init_bool(
            builder,
            c"hide_close_btn".as_ptr(),
            340 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"always_show_tabs".as_ptr(),
            336 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_spin(
            builder,
            c"max_tab_chars".as_ptr(),
            344 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
        );
        init_bool(
            builder,
            c"no_child_non_expandable".as_ptr(),
            124 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        gtk_widget_show(gtk_builder_get_object(
            builder,
            b"vbox_dir_tree\0" as *const u8 as *const gchar,
        ) as *mut c_void as *mut GtkWidget);
        init_bool(
            builder,
            c"places_home".as_ptr(),
            136 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"places_desktop".as_ptr(),
            140 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"places_applications".as_ptr(),
            144 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"places_trash".as_ptr(),
            148 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"places_root".as_ptr(),
            152 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"places_computer".as_ptr(),
            156 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"places_network".as_ptr(),
            160 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        gtk_widget_show(gtk_builder_get_object(
            builder,
            b"vbox_places\0" as *const u8 as *const gchar,
        ) as *mut c_void as *mut GtkWidget);
        init_bool(
            builder,
            c"mount_on_startup".as_ptr(),
            316 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"mount_removable".as_ptr(),
            320 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        obj = gtk_builder_get_object(builder, b"autorun\0" as *const u8 as *const gchar);
        gtk_toggle_button_set_active(
            obj as *mut c_void as *mut GtkToggleButton,
            (*(fm_config as *mut FmAppConfig)).autorun,
        );
        g_signal_connect_data(
            obj as gpointer,
            b"toggled\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut GtkWidget) -> ()>,
                GCallback,
            >(Some(
                on_autorun_toggled
                    as unsafe extern "C" fn(*mut GtkToggleButton, *mut GtkWidget) -> (),
            )),
            gtk_builder_get_object(
                builder,
                b"autorun_choices_area\0" as *const u8 as *const gchar,
            ) as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        init_bool(
            builder,
            c"media_in_new_tab".as_ptr(),
            360 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        gtk_widget_show(gtk_builder_get_object(
            builder,
            b"media_in_new_tab\0" as *const u8 as *const gchar,
        ) as *mut c_void as *mut GtkWidget);
        obj = gtk_builder_get_object(
            builder,
            b"autorun_choices_area\0" as *const u8 as *const gchar,
        );
        if !obj.is_null() {
            let mut view: *mut GtkTreeView =
                gtk_builder_get_object(builder, b"autorun_choices\0" as *const u8 as *const gchar)
                    as *mut c_void as *mut GtkTreeView;
            let mut model: *mut GtkListStore = gtk_list_store_new(
                3 as gint,
                g_icon_get_type(),
                ((16 as c_int) << 2 as c_int) as GType,
                ((16 as c_int) << 2 as c_int) as GType,
            );
            let mut tree_sel_0: *mut GtkTreeSelection = ptr::null_mut::<GtkTreeSelection>();
            let mut col: *mut GtkTreeViewColumn = ptr::null_mut::<GtkTreeViewColumn>();
            let mut render: *mut GtkCellRenderer = ptr::null_mut::<GtkCellRenderer>();
            let mut icon: *mut GIcon = ptr::null_mut::<GIcon>();
            let mut desc: *mut c_char = ptr::null_mut::<c_char>();
            let mut keys: *mut GList = ptr::null_mut::<GList>();
            let mut l: *mut GList = ptr::null_mut::<GList>();
            let mut it_0: GtkTreeIter = _GtkTreeIter {
                stamp: 0,
                user_data: ptr::null_mut::<c_void>(),
                user_data2: ptr::null_mut::<c_void>(),
                user_data3: ptr::null_mut::<c_void>(),
            };
            gtk_widget_set_sensitive(
                obj as *mut c_void as *mut GtkWidget,
                (*(fm_config as *mut FmAppConfig)).autorun,
            );
            gtk_widget_show(obj as *mut c_void as *mut GtkWidget);
            tree_sel_0 = gtk_tree_view_get_selection(view);
            gtk_tree_selection_set_mode(tree_sel_0, GTK_SELECTION_MULTIPLE);
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
            gtk_tree_view_append_column(view, col);
            keys = g_hash_table_get_keys((*(fm_config as *mut FmAppConfig)).autorun_choices);
            l = keys;
            while !l.is_null() {
                gtk_list_store_append(model, &raw mut it_0);
                desc = g_content_type_get_description((*l).data as *const gchar) as *mut c_char;
                icon = g_content_type_get_icon((*l).data as *const gchar);
                gtk_list_store_set(
                    model,
                    &raw mut it_0,
                    0i32,
                    icon,
                    1i32,
                    desc,
                    2 as c_int,
                    (*l).data,
                    -(1i32),
                );
                if !icon.is_null() {
                    g_object_unref(icon as gpointer);
                }
                g_free(desc as gpointer);
                l = (*l).next;
            }
            g_list_free(keys);
            gtk_tree_view_set_model(view, model as *mut c_void as *mut GtkTreeModel);
            obj = gtk_builder_get_object(
                builder,
                b"remove_autorun_choice\0" as *const u8 as *const gchar,
            );
            g_signal_connect_data(
                obj as gpointer,
                b"clicked\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkButton, *mut GtkTreeView) -> ()>,
                    GCallback,
                >(Some(
                    on_remove_autorun_choice_clicked
                        as unsafe extern "C" fn(*mut GtkButton, *mut GtkTreeView) -> (),
                )),
                view as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
            g_signal_connect_data(
                tree_sel_0 as gpointer,
                b"changed\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkTreeSelection, *mut GtkWidget) -> ()>,
                    GCallback,
                >(Some(
                    on_choices_sel_changed
                        as unsafe extern "C" fn(*mut GtkTreeSelection, *mut GtkWidget) -> (),
                )),
                obj as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
        }
        init_entry(
            builder,
            c"terminal".as_ptr(),
            32 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
        );
        init_entry(
            builder,
            c"format_cmd".as_ptr(),
            224 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
        );
        gtk_widget_show(gtk_builder_get_object(
            builder,
            b"hbox_format\0" as *const u8 as *const gchar,
        ) as *mut c_void as *mut GtkWidget);
        init_archiver_combo(builder);
        init_bool(
            builder,
            c"only_user_templates".as_ptr(),
            168 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"template_type_once".as_ptr(),
            176 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        init_bool(
            builder,
            c"template_run_app".as_ptr(),
            172 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        gtk_widget_show(gtk_builder_get_object(
            builder,
            b"vbox_templates\0" as *const u8 as *const gchar,
        ) as *mut c_void as *mut GtkWidget);
        init_bool(
            builder,
            c"defer_content_test".as_ptr(),
            180 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            (0i32 == 0) as c_int,
        );
        init_bool(
            builder,
            c"force_startup_notify".as_ptr(),
            112 as c_ulong as glong as gsize,
            ptr::null::<c_char>(),
            0 as gboolean,
        );
        tab_label_list =
            gtk_builder_get_object(builder, b"tab_label_list\0" as *const u8 as *const gchar)
                as *mut c_void as *mut GtkTreeView;
        tab_label_model = gtk_list_store_new(1 as gint, ((16 as c_int) << 2 as c_int) as GType);
        n = gtk_notebook_get_n_pages(notebook) as c_int;
        i = 0i32;
        while i < n {
            let mut page_0: *mut GtkWidget = gtk_notebook_get_nth_page(notebook, i as gint);
            let mut title: *const c_char =
                gtk_notebook_get_tab_label_text(notebook, page_0) as *const c_char;
            gtk_list_store_insert_with_values(
                tab_label_model,
                ptr::null_mut::<GtkTreeIter>(),
                i as gint,
                0i32,
                title,
                -(1i32),
            );
            i += 1;
        }
        gtk_tree_view_set_model(
            tab_label_list,
            tab_label_model as *mut c_void as *mut GtkTreeModel,
        );
        gtk_tree_view_set_enable_search(tab_label_list, 0 as gboolean);
        gtk_tree_model_get_iter_first(
            tab_label_model as *mut c_void as *mut GtkTreeModel,
            &raw mut it,
        );
        tree_sel = gtk_tree_view_get_selection(tab_label_list);
        gtk_tree_selection_set_mode(tree_sel, GTK_SELECTION_BROWSE);
        gtk_tree_selection_select_iter(tree_sel, &raw mut it);
        g_object_unref(tab_label_model as gpointer);
        obj = gtk_builder_get_object(builder, b"home_path_vbox\0" as *const u8 as *const gchar);
        if !obj.is_null() {
            let mut is_set: gboolean = 0;
            gtk_widget_show(obj as *mut c_void as *mut GtkWidget);
            obj = gtk_builder_get_object(builder, b"use_home_path\0" as *const u8 as *const gchar);
            g_signal_connect_data(
                obj as gpointer,
                b"toggled\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut GtkWidget) -> ()>,
                    GCallback,
                >(Some(
                    on_use_home_path_toggled
                        as unsafe extern "C" fn(*mut GtkToggleButton, *mut GtkWidget) -> (),
                )),
                gtk_builder_get_object(builder, b"home_path_custom\0" as *const u8 as *const gchar)
                    as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
            g_signal_connect_data(
                obj as gpointer,
                b"toggled\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut GtkEntry) -> ()>,
                    GCallback,
                >(Some(
                    on_use_home_path_toggled2
                        as unsafe extern "C" fn(*mut GtkToggleButton, *mut GtkEntry) -> (),
                )),
                gtk_builder_get_object(builder, b"home_path\0" as *const u8 as *const gchar)
                    as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
            is_set = (!(*(fm_config as *mut FmAppConfig)).home_path.is_null()
                && strcmp(
                    (*(fm_config as *mut FmAppConfig)).home_path,
                    fm_get_home_dir(),
                ) != 0i32) as c_int as gboolean;
            if is_set != 0 {
                gtk_toggle_button_set_active(
                    obj as *mut c_void as *mut GtkToggleButton,
                    (0i32 == 0) as c_int,
                );
            } else {
                gtk_toggle_button_set_active(
                    gtk_builder_get_object(
                        builder,
                        b"use_home_default\0" as *const u8 as *const gchar,
                    ) as *mut c_void as *mut GtkToggleButton,
                    (0i32 == 0) as c_int,
                );
            }
            obj = gtk_builder_get_object(builder, b"home_path\0" as *const u8 as *const gchar);
            if is_set != 0 {
                gtk_entry_set_text(
                    obj as *mut c_void as *mut GtkEntry,
                    (*(fm_config as *mut FmAppConfig)).home_path,
                );
            } else {
                gtk_entry_set_text(
                    obj as *mut c_void as *mut GtkEntry,
                    fm_get_home_dir() as *const gchar,
                );
            }
            g_signal_connect_data(
                obj as gpointer,
                b"changed\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkEntry, *mut FmAppConfig) -> ()>,
                    GCallback,
                >(Some(
                    on_home_path_changed
                        as unsafe extern "C" fn(*mut GtkEntry, *mut FmAppConfig) -> (),
                )),
                fm_config as *mut FmAppConfig as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
            obj = gtk_builder_get_object(
                builder,
                b"home_path_current\0" as *const u8 as *const gchar,
            );
            if !parent.is_null()
                && ({
                    let mut __inst: *mut GTypeInstance = parent as *mut GTypeInstance;
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
                }) != 0
            {
                g_signal_connect_data(
                    obj as gpointer,
                    b"clicked\0" as *const u8 as *const gchar,
                    mem::transmute::<
                        Option<unsafe extern "C" fn(*mut GtkButton, *mut GtkEntry) -> ()>,
                        GCallback,
                    >(Some(
                        on_home_path_current_clicked
                            as unsafe extern "C" fn(*mut GtkButton, *mut GtkEntry) -> (),
                    )),
                    gtk_builder_get_object(builder, b"home_path\0" as *const u8 as *const gchar)
                        as gpointer,
                    None,
                    G_CONNECT_DEFAULT,
                );
            } else {
                gtk_widget_set_sensitive(obj as *mut c_void as *mut GtkWidget, 0 as gboolean);
            }
        }
        obj = gtk_builder_get_object(builder, b"modules_tab\0" as *const u8 as *const gchar);
        if !obj.is_null() {
            let mut view_0: *mut GtkTreeView = gtk_builder_get_object(
                builder,
                b"modules_blacklist\0" as *const u8 as *const gchar,
            ) as *mut c_void as *mut GtkTreeView;
            let mut model_0: *mut GtkListStore = gtk_list_store_new(
                2 as gint,
                ((16 as c_int) << 2 as c_int) as GType,
                ((5 as c_int) << 2 as c_int) as GType,
            );
            let mut tree_sel_1: *mut GtkTreeSelection = ptr::null_mut::<GtkTreeSelection>();
            let mut col_0: *mut GtkTreeViewColumn = ptr::null_mut::<GtkTreeViewColumn>();
            let mut render_0: *mut GtkCellRenderer = ptr::null_mut::<GtkCellRenderer>();
            let mut item: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
            let mut it_1: GtkTreeIter = _GtkTreeIter {
                stamp: 0,
                user_data: ptr::null_mut::<c_void>(),
                user_data2: ptr::null_mut::<c_void>(),
                user_data3: ptr::null_mut::<c_void>(),
            };
            gtk_widget_show(obj as *mut c_void as *mut GtkWidget);
            tree_sel_1 = gtk_tree_view_get_selection(view_0);
            gtk_tree_selection_set_mode(tree_sel_1, GTK_SELECTION_MULTIPLE);
            col_0 = gtk_tree_view_column_new();
            render_0 = gtk_cell_renderer_text_new();
            gtk_tree_view_column_pack_start(col_0, render_0, 0 as gboolean);
            gtk_tree_view_column_set_attributes(
                col_0,
                render_0,
                c"text".as_ptr(),
                0i32,
                ptr::null_mut::<c_void>(),
            );
            gtk_tree_view_append_column(view_0, col_0);
            item = (*fm_config).system_modules_blacklist as *mut *mut c_char;
            while !item.is_null() && !(*item).is_null() {
                gtk_list_store_append(model_0, &raw mut it_1);
                gtk_list_store_set(model_0, &raw mut it_1, 0i32, *item, 1i32, 0i32, -(1i32));
                item = item.offset(1);
            }
            item = (*fm_config).modules_blacklist as *mut *mut c_char;
            while !item.is_null() && !(*item).is_null() {
                gtk_list_store_append(model_0, &raw mut it_1);
                gtk_list_store_set(
                    model_0,
                    &raw mut it_1,
                    0i32,
                    *item,
                    1i32,
                    (0i32 == 0) as c_int,
                    -(1i32),
                );
                item = item.offset(1);
            }
            gtk_tree_view_set_model(view_0, model_0 as *mut c_void as *mut GtkTreeModel);
            obj =
                gtk_builder_get_object(builder, b"add_to_blacklist\0" as *const u8 as *const gchar);
            g_signal_connect_data(
                obj as gpointer,
                b"clicked\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkButton, *mut GtkListStore) -> ()>,
                    GCallback,
                >(Some(
                    on_add_to_blacklist_clicked
                        as unsafe extern "C" fn(*mut GtkButton, *mut GtkListStore) -> (),
                )),
                model_0 as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
            obj = gtk_builder_get_object(
                builder,
                b"remove_from_blacklist\0" as *const u8 as *const gchar,
            );
            g_signal_connect_data(
                obj as gpointer,
                b"clicked\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkButton, *mut GtkTreeView) -> ()>,
                    GCallback,
                >(Some(
                    on_remove_from_blacklist_clicked
                        as unsafe extern "C" fn(*mut GtkButton, *mut GtkTreeView) -> (),
                )),
                view_0 as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
            g_signal_connect_data(
                tree_sel_1 as gpointer,
                b"changed\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkTreeSelection, *mut GtkWidget) -> ()>,
                    GCallback,
                >(Some(
                    on_blacklist_sel_changed
                        as unsafe extern "C" fn(*mut GtkTreeSelection, *mut GtkWidget) -> (),
                )),
                obj as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
            view_0 = gtk_builder_get_object(
                builder,
                b"modules_whitelist\0" as *const u8 as *const gchar,
            ) as *mut c_void as *mut GtkTreeView;
            model_0 = gtk_list_store_new(1 as gint, ((16 as c_int) << 2 as c_int) as GType);
            tree_sel_1 = gtk_tree_view_get_selection(view_0);
            gtk_tree_selection_set_mode(tree_sel_1, GTK_SELECTION_MULTIPLE);
            col_0 = gtk_tree_view_column_new();
            render_0 = gtk_cell_renderer_text_new();
            gtk_tree_view_column_pack_start(col_0, render_0, 0 as gboolean);
            gtk_tree_view_column_set_attributes(
                col_0,
                render_0,
                c"text".as_ptr(),
                0i32,
                ptr::null_mut::<c_void>(),
            );
            gtk_tree_view_append_column(view_0, col_0);
            item = (*fm_config).modules_whitelist as *mut *mut c_char;
            while !item.is_null() && !(*item).is_null() {
                gtk_list_store_append(model_0, &raw mut it_1);
                gtk_list_store_set(model_0, &raw mut it_1, 0i32, *item, -(1i32));
                item = item.offset(1);
            }
            gtk_tree_view_set_model(view_0, model_0 as *mut c_void as *mut GtkTreeModel);
            obj =
                gtk_builder_get_object(builder, b"add_to_whitelist\0" as *const u8 as *const gchar);
            g_signal_connect_data(
                obj as gpointer,
                b"clicked\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkButton, *mut GtkListStore) -> ()>,
                    GCallback,
                >(Some(
                    on_add_to_whitelist_clicked
                        as unsafe extern "C" fn(*mut GtkButton, *mut GtkListStore) -> (),
                )),
                model_0 as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
            obj = gtk_builder_get_object(
                builder,
                b"remove_from_whitelist\0" as *const u8 as *const gchar,
            );
            g_signal_connect_data(
                obj as gpointer,
                b"clicked\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkButton, *mut GtkTreeView) -> ()>,
                    GCallback,
                >(Some(
                    on_remove_from_whitelist_clicked
                        as unsafe extern "C" fn(*mut GtkButton, *mut GtkTreeView) -> (),
                )),
                view_0 as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
            g_signal_connect_data(
                tree_sel_1 as gpointer,
                b"changed\0" as *const u8 as *const gchar,
                mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GtkTreeSelection, *mut GtkWidget) -> ()>,
                    GCallback,
                >(Some(
                    on_whitelist_sel_changed
                        as unsafe extern "C" fn(*mut GtkTreeSelection, *mut GtkWidget) -> (),
                )),
                obj as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
        }
        g_signal_connect_data(
            tree_sel as gpointer,
            b"changed\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkTreeSelection, gpointer) -> ()>,
                GCallback,
            >(Some(
                on_tab_label_list_sel_changed
                    as unsafe extern "C" fn(*mut GtkTreeSelection, gpointer) -> (),
            )),
            notebook as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        g_signal_connect_data(
            notebook as gpointer,
            b"switch-page\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut GtkNotebook, gpointer, guint, *mut GtkTreeView) -> (),
                >,
                GCallback,
            >(Some(
                on_notebook_page_changed
                    as unsafe extern "C" fn(
                        *mut GtkNotebook,
                        gpointer,
                        guint,
                        *mut GtkTreeView,
                    ) -> (),
            )),
            tab_label_list as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        gtk_notebook_set_show_tabs(notebook, 0 as gboolean);
        g_signal_connect_data(
            pref_dlg as gpointer,
            b"response\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkDialog, c_int, *mut *mut GtkWindow) -> ()>,
                GCallback,
            >(Some(
                pref_on_response
                    as unsafe extern "C" fn(*mut GtkDialog, c_int, *mut *mut GtkWindow) -> (),
            )),
            &raw mut pref_dlg as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        g_signal_connect_data(
            pref_dlg as gpointer,
            b"key-press-event\0" as *const u8 as *const gchar,
            mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventKey,
                        *mut GtkNotebook,
                    ) -> gboolean,
                >,
                GCallback,
            >(Some(
                pref_on_key_press
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventKey,
                        *mut GtkNotebook,
                    ) -> gboolean,
            )),
            notebook as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        g_object_unref(builder as gpointer);
        pcmanfm_ref();
        g_signal_connect_data(
            pref_dlg as gpointer,
            b"destroy\0" as *const u8 as *const gchar,
            mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GCallback>(Some(pcmanfm_unref)),
            ptr::null_mut::<c_void>(),
            None,
            G_CONNECT_DEFAULT,
        );
        if !parent.is_null() {
            gtk_window_set_transient_for(pref_dlg, parent);
        }
    }
    if page < 0i32 || page >= gtk_notebook_get_n_pages(notebook) {
        page = 0i32;
    }
    gtk_notebook_set_current_page(notebook, page as gint);
    gtk_window_present(pref_dlg);
}
