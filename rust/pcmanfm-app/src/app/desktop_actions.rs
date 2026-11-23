// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      desktop-ui.c
 *
 *      Copyright 2010 PCMan <pcman.tw@gmail.com>
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

static mut desktop_menu_xml: [c_char; 95] = unsafe {
    mem::transmute::<
        [u8; 95],
        [c_char; 95],
    >(
        *b"<popup><placeholder name='CustomCommonOps'><menuitem action='DeskPref'/></placeholder></popup>\0",
    )
};
static mut desktop_actions: [GtkActionEntry; 1] = unsafe {
    [_GtkActionEntry {
        name: b"DeskPref\0" as *const u8 as *const gchar,
        stock_id: c"gtk-properties".as_ptr() as GtkStock as *const gchar,
        label: b"Desktop Preferences\0" as *const u8 as *const gchar,
        accelerator: b"\0" as *const u8 as *const gchar,
        tooltip: ptr::null::<gchar>(),
        callback: mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkAction, *mut FmDesktop) -> ()>,
            GCallback,
        >(Some(
            fm_desktop_preference as unsafe extern "C" fn(*mut GtkAction, *mut FmDesktop) -> (),
        )),
    }]
};
static mut folder_menu_xml: [c_char; 133] = unsafe {
    mem::transmute::<
        [u8; 133],
        [c_char; 133],
    >(
        *b"<popup><placeholder name='ph1'><menuitem action='NewTab'/><menuitem action='NewWin'/><menuitem action='Term'/></placeholder></popup>\0",
    )
};
static mut folder_menu_actions: [GtkActionEntry; 4] = unsafe {
    [
        _GtkActionEntry {
            name: b"NewTab\0" as *const u8 as *const gchar,
            stock_id: b"tab-new\0" as *const u8 as *const gchar,
            label: b"Open in New Ta_b\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, gpointer) -> ()>,
                GCallback,
            >(Some(
                on_open_in_new_tab as unsafe extern "C" fn(*mut GtkAction, gpointer) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"NewWin\0" as *const u8 as *const gchar,
            stock_id: c"gtk-new".as_ptr() as GtkStock as *const gchar,
            label: b"Open in New Win_dow\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, gpointer) -> ()>,
                GCallback,
            >(Some(
                on_open_in_new_win as unsafe extern "C" fn(*mut GtkAction, gpointer) -> (),
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
                Option<unsafe extern "C" fn(*mut GtkAction, gpointer) -> ()>,
                GCallback,
            >(Some(
                on_open_folder_in_terminal as unsafe extern "C" fn(*mut GtkAction, gpointer) -> (),
            )),
        },
    ]
};
static mut extra_item_menu_xml: [c_char; 94] = unsafe {
    mem::transmute::<
        [u8; 94],
        [c_char; 94],
    >(
        *b"<popup><placeholder name='ph2'><separator/><menuitem action='Disable'/></placeholder></popup>\0",
    )
};
static mut extra_item_menu_actions: [GtkActionEntry; 1] = unsafe {
    [_GtkActionEntry {
        name: b"Disable\0" as *const u8 as *const gchar,
        stock_id: ptr::null::<gchar>(),
        label: b"_Remove from Desktop\0" as *const u8 as *const gchar,
        accelerator: ptr::null::<gchar>(),
        tooltip: ptr::null::<gchar>(),
        callback: mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkAction, gpointer) -> ()>,
            GCallback,
        >(Some(
            on_disable as unsafe extern "C" fn(*mut GtkAction, gpointer) -> (),
        )),
    }]
};
static mut desktop_icon_menu_xml: [c_char; 115] = unsafe {
    mem::transmute::<
        [u8; 115],
        [c_char; 115],
    >(
        *b"<popup><placeholder name='ph2'><separator/><menuitem action='Fix'/><menuitem action='Snap'/></placeholder></popup>\0",
    )
};
static mut desktop_icon_toggle_actions: [GtkToggleActionEntry; 1] = unsafe {
    [_GtkToggleActionEntry {
        name: b"Fix\0" as *const u8 as *const gchar,
        stock_id: ptr::null::<gchar>(),
        label: b"Stic_k to Current Position\0" as *const u8 as *const gchar,
        accelerator: ptr::null::<gchar>(),
        tooltip: ptr::null::<gchar>(),
        callback: mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkToggleAction, gpointer) -> ()>,
            GCallback,
        >(Some(
            on_fix_pos as unsafe extern "C" fn(*mut GtkToggleAction, gpointer) -> (),
        )),
        is_active: 0 as gboolean,
    }]
};
static mut desktop_icon_actions: [GtkActionEntry; 1] = unsafe {
    [_GtkActionEntry {
        name: b"Snap\0" as *const u8 as *const gchar,
        stock_id: ptr::null::<gchar>(),
        label: b"Snap to _Grid\0" as *const u8 as *const gchar,
        accelerator: ptr::null::<gchar>(),
        tooltip: ptr::null::<gchar>(),
        callback: mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkAction, gpointer) -> ()>,
            GCallback,
        >(Some(
            on_snap_to_grid as unsafe extern "C" fn(*mut GtkAction, gpointer) -> (),
        )),
    }]
};
