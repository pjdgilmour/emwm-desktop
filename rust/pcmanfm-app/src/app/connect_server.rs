// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      connect-server.c
 *
 *      This file is a part of the PCManFM project.
 *
 *      Copyright 2013-2016 Andriy Grytsenko (LStranger) <andrej@rep.kiev.ua>
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

unsafe extern "C" fn on_response(
    mut dialog: *mut GtkDialog,
    mut resp: gint,
    mut dlg: *mut ConnectDlg,
) {
    let mut parent: *mut GtkWindow = gtk_window_get_transient_for((*dlg).dlg);
    let mut str: *mut GString = ptr::null_mut::<GString>();
    let mut iter: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut scheme: *mut c_char = ptr::null_mut::<c_char>();
    let mut text: *const c_char = ptr::null::<c_char>();
    let mut tmp: *const c_char = ptr::null::<c_char>();
    let mut path: *mut FmPath = ptr::null_mut::<FmPath>();
    let mut def_port: c_int = 0;
    let mut used_port: c_int = 0;
    if resp == GTK_RESPONSE_OK as c_int
        && gtk_combo_box_get_active_iter((*dlg).server_type, &raw mut iter) != 0
    {
        gtk_tree_model_get(
            gtk_combo_box_get_model((*dlg).server_type),
            &raw mut iter,
            1i32,
            &raw mut scheme,
            -(1i32),
        );
    }
    if !scheme.is_null() {
        str = g_string_new(scheme);
        if strcmp(scheme, c"sftp".as_ptr()) == 0i32 {
            def_port = 22 as c_int;
        } else if strcmp(scheme, c"ftp".as_ptr()) == 0i32 {
            def_port = 21i32;
        } else if strcmp(scheme, c"dav".as_ptr()) == 0i32 {
            def_port = 80i32;
        } else {
            def_port = -(1i32);
        }
        g_free(scheme as gpointer);
        text = gtk_entry_get_text((*dlg).server_host) as *const c_char;
        tmp = g_strstr_len(
            text as *const gchar,
            -(1i32) as gssize,
            b"://\0" as *const u8 as *const gchar,
        );
        if !tmp.is_null() {
            if def_port == 80i32 {
                if (if 0 != 0 {
                    ({
                        let __str: *const c_char = text;
                        let __prefix: *const c_char = c"https:".as_ptr();
                        let mut __result: gboolean = 0 as gboolean;
                        if ({
                            let mut _g_boolean_var_27: c_int = 0 as c_int;
                            if __str.is_null() || __prefix.is_null() {
                                _g_boolean_var_27 = 1i32;
                            }
                            _g_boolean_var_27
                        }) as c_long
                            != 0
                        {
                            __result =
                                g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                        } else {
                            let __str_len: size_t =
                                strlen(__str.offset(__str.is_null() as isize)) as size_t;
                            let __prefix_len: size_t =
                                strlen(__prefix.offset(__prefix.is_null() as isize)) as size_t;
                            if __str_len >= __prefix_len {
                                __result = (memcmp(
                                    __str.offset(__str.is_null() as isize) as *const c_void,
                                    __prefix.offset(__prefix.is_null() as isize) as *const c_void,
                                    __prefix_len,
                                ) == 0i32) as c_int
                                    as gboolean;
                            }
                        }
                        __result
                    })
                } else {
                    g_str_has_prefix(
                        text as *const gchar,
                        b"https:\0" as *const u8 as *const gchar,
                    )
                }) != 0
                    || (if 0 != 0 {
                        ({
                            let __str: *const c_char = text;
                            let __prefix: *const c_char = c"davs:".as_ptr();
                            let mut __result: gboolean = 0 as gboolean;
                            if ({
                                let mut _g_boolean_var_28: c_int = 0 as c_int;
                                if __str.is_null() || __prefix.is_null() {
                                    _g_boolean_var_28 = 1i32;
                                }
                                _g_boolean_var_28
                            }) as c_long
                                != 0
                            {
                                __result = g_str_has_prefix(
                                    __str as *const gchar,
                                    __prefix as *const gchar,
                                );
                            } else {
                                let __str_len: size_t =
                                    strlen(__str.offset(__str.is_null() as isize)) as size_t;
                                let __prefix_len: size_t =
                                    strlen(__prefix.offset(__prefix.is_null() as isize)) as size_t;
                                if __str_len >= __prefix_len {
                                    __result = (memcmp(
                                        __str.offset(__str.is_null() as isize) as *const c_void,
                                        __prefix.offset(__prefix.is_null() as isize)
                                            as *const c_void,
                                        __prefix_len,
                                    ) == 0i32)
                                        as c_int
                                        as gboolean;
                                }
                            }
                            __result
                        })
                    } else {
                        g_str_has_prefix(
                            text as *const gchar,
                            b"davs:\0" as *const u8 as *const gchar,
                        )
                    }) != 0
                {
                    if gtk_spin_button_get_value((*dlg).server_port) != 80.0f64 {
                        def_port = 443 as c_int;
                    }
                    g_string_assign(str, b"davs\0" as *const u8 as *const gchar);
                }
            }
            text = tmp.offset(3 as isize);
        }
        if 0 != 0 {
            ({
                let __val: *const c_char = c"://".as_ptr();
                g_string_append_len_inline(
                    str,
                    __val,
                    if ({
                        let mut _g_boolean_var_29: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_29 = 1i32;
                        }
                        _g_boolean_var_29
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
            g_string_append_len_inline(str, c"://".as_ptr(), -(1i32) as gssize);
        };
        if gtk_toggle_button_get_active((*dlg).user_user) != 0 {
            g_string_append_printf(
                str,
                b"%s@\0" as *const u8 as *const gchar,
                gtk_entry_get_text((*dlg).login_entry),
            );
        }
        if 0 != 0 {
            ({
                let __val: *const c_char = text;
                g_string_append_len_inline(
                    str,
                    __val,
                    if ({
                        let mut _g_boolean_var_30: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_30 = 1i32;
                        }
                        _g_boolean_var_30
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
            g_string_append_len_inline(str, text, -(1i32) as gssize);
        };
        used_port = gtk_spin_button_get_value((*dlg).server_port) as c_int;
        if def_port != used_port {
            g_string_append_printf(str, b":%d\0" as *const u8 as *const gchar, used_port);
        }
        text = gtk_entry_get_text((*dlg).server_path) as *const c_char;
        if *text.offset(0 as isize) as c_int != '/' as i32 {
            g_string_append_c_inline(str, '/' as i32 as gchar);
        }
        if 0 != 0 {
            ({
                let __val: *const c_char = text;
                g_string_append_len_inline(
                    str,
                    __val,
                    if ({
                        let mut _g_boolean_var_31: c_int = 0 as c_int;
                        if !__val.is_null() {
                            _g_boolean_var_31 = 1i32;
                        }
                        _g_boolean_var_31
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
            g_string_append_len_inline(str, text, -(1i32) as gssize);
        };
        path = fm_path_new_for_uri((*str).str_0);
        if 0 != 0 {
            if 0i32 == 0 {
                g_string_free(str, (0i32 == 0) as c_int);
            } else {
                g_string_free_and_steal(str);
            };
        } else {
            g_string_free(str, (0i32 == 0) as c_int);
        };
        if !parent.is_null() {
            fm_main_win_add_tab(parent as *mut c_void as *mut FmMainWin, path);
        } else {
            fm_main_win_add_win(ptr::null_mut::<FmMainWin>(), path);
        }
        fm_path_unref(path);
    }
    gtk_widget_destroy((*dlg).dlg as *mut c_void as *mut GtkWidget);
    g_slice_free1(mem::size_of::<ConnectDlg>() as gsize, dlg as gpointer);
    pcmanfm_unref();
}
unsafe extern "C" fn on_server_type(mut box_0: *mut GtkComboBox, mut dlg: *mut ConnectDlg) {
    let mut iter: GtkTreeIter = _GtkTreeIter {
        stamp: 0,
        user_data: ptr::null_mut::<c_void>(),
        user_data2: ptr::null_mut::<c_void>(),
        user_data3: ptr::null_mut::<c_void>(),
    };
    let mut scheme: *mut c_char = ptr::null_mut::<c_char>();
    if gtk_combo_box_get_active_iter(box_0, &raw mut iter) == 0 {
        return;
    }
    gtk_tree_model_get(
        gtk_combo_box_get_model(box_0),
        &raw mut iter,
        1i32,
        &raw mut scheme,
        -(1i32),
    );
    if scheme.is_null() {
        return;
    }
    if strcmp(scheme, c"sftp".as_ptr()) == 0i32 {
        gtk_widget_set_sensitive(
            (*dlg).user_anonymous as *mut c_void as *mut GtkWidget,
            0 as gboolean,
        );
        gtk_toggle_button_set_active((*dlg).user_user, (0i32 == 0) as c_int);
        gtk_entry_set_text((*dlg).login_entry, g_get_user_name());
        gtk_spin_button_set_value((*dlg).server_port, 22.0f64);
    } else if strcmp(scheme, c"ftp".as_ptr()) == 0i32 {
        gtk_widget_set_sensitive(
            (*dlg).user_anonymous as *mut c_void as *mut GtkWidget,
            (0i32 == 0) as c_int,
        );
        gtk_toggle_button_set_active((*dlg).user_anonymous, (0i32 == 0) as c_int);
        gtk_spin_button_set_value((*dlg).server_port, 21.0f64);
    } else if strcmp(scheme, c"dav".as_ptr()) == 0i32 {
        gtk_widget_set_sensitive(
            (*dlg).user_anonymous as *mut c_void as *mut GtkWidget,
            (0i32 == 0) as c_int,
        );
        gtk_spin_button_set_value((*dlg).server_port, 80.0f64);
    }
    g_free(scheme as gpointer);
}
unsafe extern "C" fn on_server_host(mut editable: *mut GtkEditable, mut dlg: *mut ConnectDlg) {
    let mut text: *const c_char =
        gtk_entry_get_text(editable as *mut c_void as *mut GtkEntry) as *const c_char;
    let mut ready: gboolean = 0;
    if text.is_null() || *text.offset(0 as isize) == 0 {
        ready = 0 as gboolean;
    } else if gtk_combo_box_get_active((*dlg).server_type) == -(1i32) {
        ready = 0 as gboolean;
    } else if gtk_toggle_button_get_active((*dlg).user_anonymous) != 0 {
        ready = (0i32 == 0) as c_int as gboolean;
    } else {
        text = gtk_entry_get_text((*dlg).login_entry) as *const c_char;
        ready = (!text.is_null() && *text.offset(0 as isize) as c_int != 0) as c_int as gboolean;
    }
    gtk_widget_set_sensitive((*dlg).ok, ready);
}
unsafe extern "C" fn on_user_type(
    mut user_anonymous: *mut GtkToggleButton,
    mut dlg: *mut ConnectDlg,
) {
    let mut anonymous: gboolean = gtk_toggle_button_get_active(user_anonymous);
    let mut ready: gboolean = 0;
    let mut text: *const c_char = ptr::null::<c_char>();
    if anonymous != 0 {
        gtk_widget_set_sensitive(
            (*dlg).login_entry as *mut c_void as *mut GtkWidget,
            0 as gboolean,
        );
        text = gtk_entry_get_text((*dlg).server_host) as *const c_char;
        if text.is_null() || *text.offset(0 as isize) == 0 {
            ready = 0 as gboolean;
        } else {
            ready = (gtk_combo_box_get_active((*dlg).server_type) != -(1i32)) as c_int as gboolean;
        }
    } else {
        gtk_widget_set_sensitive(
            (*dlg).login_entry as *mut c_void as *mut GtkWidget,
            (0i32 == 0) as c_int,
        );
        text = gtk_entry_get_text((*dlg).server_host) as *const c_char;
        if text.is_null() || *text.offset(0 as isize) == 0 {
            ready = 0 as gboolean;
        } else if gtk_combo_box_get_active((*dlg).server_type) == -(1i32) {
            ready = 0 as gboolean;
        } else {
            text = gtk_entry_get_text((*dlg).login_entry) as *const c_char;
            if text.is_null() || *text.offset(0 as isize) == 0 {
                ready = 0 as gboolean;
            } else {
                ready = (0i32 == 0) as c_int as gboolean;
            }
        }
    }
    gtk_widget_set_sensitive((*dlg).ok, ready);
}
unsafe extern "C" fn on_login_entry(mut editable: *mut GtkEditable, mut dlg: *mut ConnectDlg) {
    let mut text: *const c_char =
        gtk_entry_get_text(editable as *mut c_void as *mut GtkEntry) as *const c_char;
    let mut ready: gboolean = 0;
    if text.is_null() || *text.offset(0 as isize) == 0 {
        ready = 0 as gboolean;
    } else if gtk_combo_box_get_active((*dlg).server_type) == -(1i32) {
        ready = 0 as gboolean;
    } else {
        text = gtk_entry_get_text((*dlg).server_host) as *const c_char;
        ready = (!text.is_null() && *text.offset(0 as isize) as c_int != 0) as c_int as gboolean;
    }
    gtk_widget_set_sensitive((*dlg).ok, ready);
}
#[no_mangle]
pub unsafe extern "C" fn open_connect_dialog(mut parent: *mut GtkWindow) {
    let mut builder: *mut GtkBuilder = gtk_builder_new();
    let mut dlg: *mut ConnectDlg =
        g_slice_alloc(mem::size_of::<ConnectDlg>() as gsize) as *mut ConnectDlg;
    gtk_builder_add_from_file(
        builder,
        b"/usr/share/pcmanfm/ui/connect.ui\0" as *const u8 as *const gchar,
        ptr::null_mut::<*mut GError>(),
    );
    (*dlg).dlg = gtk_builder_get_object(builder, b"dlg\0" as *const u8 as *const gchar)
        as *mut c_void as *mut GtkWindow;
    g_signal_connect_data(
        (*dlg).dlg as gpointer,
        b"response\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkDialog, gint, *mut ConnectDlg) -> ()>,
            GCallback,
        >(Some(
            on_response as unsafe extern "C" fn(*mut GtkDialog, gint, *mut ConnectDlg) -> (),
        )),
        dlg as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*dlg).ok = gtk_builder_get_object(builder, b"ok\0" as *const u8 as *const gchar) as *mut c_void
        as *mut GtkWidget;
    gtk_widget_set_sensitive((*dlg).ok, 0 as gboolean);
    (*dlg).server_type =
        gtk_builder_get_object(builder, b"server_type\0" as *const u8 as *const gchar)
            as *mut c_void as *mut GtkComboBox;
    g_signal_connect_data(
        (*dlg).server_type as gpointer,
        b"changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkComboBox, *mut ConnectDlg) -> ()>,
            GCallback,
        >(Some(
            on_server_type as unsafe extern "C" fn(*mut GtkComboBox, *mut ConnectDlg) -> (),
        )),
        dlg as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*dlg).server_host =
        gtk_builder_get_object(builder, b"server_host\0" as *const u8 as *const gchar)
            as *mut c_void as *mut GtkEntry;
    g_signal_connect_data(
        (*dlg).server_host as gpointer,
        b"changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkEditable, *mut ConnectDlg) -> ()>,
            GCallback,
        >(Some(
            on_server_host as unsafe extern "C" fn(*mut GtkEditable, *mut ConnectDlg) -> (),
        )),
        dlg as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*dlg).server_port =
        gtk_builder_get_object(builder, b"server_port\0" as *const u8 as *const gchar)
            as *mut c_void as *mut GtkSpinButton;
    (*dlg).server_path =
        gtk_builder_get_object(builder, b"server_path\0" as *const u8 as *const gchar)
            as *mut c_void as *mut GtkEntry;
    (*dlg).user_anonymous =
        gtk_builder_get_object(builder, b"user_anonymous\0" as *const u8 as *const gchar)
            as *mut c_void as *mut GtkToggleButton;
    g_signal_connect_data(
        (*dlg).user_anonymous as gpointer,
        b"toggled\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkToggleButton, *mut ConnectDlg) -> ()>,
            GCallback,
        >(Some(
            on_user_type as unsafe extern "C" fn(*mut GtkToggleButton, *mut ConnectDlg) -> (),
        )),
        dlg as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*dlg).user_user = gtk_builder_get_object(builder, b"user_user\0" as *const u8 as *const gchar)
        as *mut c_void as *mut GtkToggleButton;
    (*dlg).login_entry =
        gtk_builder_get_object(builder, b"login_entry\0" as *const u8 as *const gchar)
            as *mut c_void as *mut GtkEntry;
    g_signal_connect_data(
        (*dlg).login_entry as gpointer,
        b"changed\0" as *const u8 as *const gchar,
        mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkEditable, *mut ConnectDlg) -> ()>,
            GCallback,
        >(Some(
            on_login_entry as unsafe extern "C" fn(*mut GtkEditable, *mut ConnectDlg) -> (),
        )),
        dlg as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_object_unref(builder as gpointer);
    pcmanfm_ref();
    if !parent.is_null() {
        gtk_window_set_transient_for((*dlg).dlg, parent);
    }
    gtk_window_present((*dlg).dlg);
}
