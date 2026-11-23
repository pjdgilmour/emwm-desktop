// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      main-win-ui.c
 *
 *      Copyright 2009 - 2012 Hong Jen Yee (PCMan) <pcman.tw@gmail.com>
 *      Copyright 2012-2014 Andriy Grytsenko (LStranger) <andrej@rep.kiev.ua>
 *      Copyright 2020 Ingo Brückl
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

static mut main_menu_xml: [c_char; 2957] = unsafe {
    mem::transmute::<
        [u8; 2957],
        [c_char; 2957],
    >(
        *b"<menubar><menu action='FileMenu'><menuitem action='New'/><menuitem action='NewTab'/><separator/><menu action='CreateNew'><menuitem action='NewFolder'/><menuitem action='NewBlank'/></menu><separator/><menuitem action='Prop'/><separator/><menuitem action='CloseTab'/><menuitem action='Close'/></menu><menu action='EditMenu'><menuitem action='Open'/><separator/><menuitem action='Cut'/><menuitem action='Copy'/><menuitem action='Paste'/><menuitem action='ToTrash'/><menuitem action='Del'/><menuitem action='CopyPath'/><separator/><menuitem action='FileProp'/><separator/><menuitem action='Rename'/><menuitem action='Link'/><menuitem action='MoveTo'/><menuitem action='CopyTo'/><separator/><menuitem action='SelAll'/><menuitem action='InvSel'/><separator/><menuitem action='Pref'/></menu><menu action='ViewMenu'><menuitem action='Reload'/><separator/><menuitem action='ShowHidden'/><menu action='Sort'><menuitem action='Asc'/><menuitem action='Desc'/><separator/><menuitem action='ByName'/><menuitem action='ByMTime'/><menuitem action='BySize'/><menuitem action='ByType'/><menuitem action='ByExt'/><separator/><menuitem action='MingleDirs'/><menuitem action='SortIgnoreCase'/></menu><menu action='FolderView'><placeholder name='ViewModes'/></menu><menuitem action='SavePerFolder'/><separator/><menu action='Toolbar'><menuitem action='ShowToolbar'/><separator/><menuitem action='ToolbarNewWin'/><menuitem action='ToolbarNewTab'/><menuitem action='ToolbarNav'/><menuitem action='ToolbarHome'/></menu><menu action='PathMode'><menuitem action='PathEntry'/><menuitem action='PathBar'/></menu><menu action='SidePane'><menuitem action='ShowSidePane' /><separator/><placeholder name='SidePaneModes'/></menu><menuitem action='ShowStatus'/><separator/><menuitem action='DualPane'/><menuitem action='Fullscreen' /><separator/><menuitem action='SizeBigger'/><menuitem action='SizeSmaller'/><menuitem action='SizeDefault'/><separator/><menuitem action='Filter'/></menu><menu action='BookmarksMenu'><menuitem action='AddBookmark'/></menu><menu action='GoMenu'><menuitem action='Prev'/><menuitem action='Next'/><menuitem action='Up'/><separator/><menuitem action='Home'/><menuitem action='Desktop'/><menuitem action='Trash'/><menuitem action='Apps'/><menuitem action='Computer'/><menuitem action='Network'/><separator/><menuitem action='Location'/><menuitem action='Connect'/></menu><menu action='ToolMenu'><menuitem action='Term'/><menuitem action='Search'/><menuitem action='Launch'/></menu><menu action='HelpMenu'><menuitem action='About'/><menuitem action='KeyNavList'/></menu></menubar><toolbar><toolitem action='New'/><toolitem action='NewTab'/><toolitem action='Prev'/><toolitem action='Next'/><toolitem action='Up'/><toolitem action='Home'/><toolitem action='Go'/></toolbar><accelerator action='Location2'/><accelerator action='Prev2'/><accelerator action='Next2'/><accelerator action='Reload2'/><accelerator action='SizeBigger2'/><accelerator action='SizeSmaller2'/>\0",
    )
};
static mut main_win_actions: [GtkActionEntry; 66] = unsafe {
    [
        _GtkActionEntry {
            name: b"FileMenu\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_File\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"New\0" as *const u8 as *const gchar,
            stock_id: c"gtk-new".as_ptr() as GtkStock as *const gchar,
            label: b"_New Window\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>N\0" as *const u8 as *const gchar,
            tooltip: b"Open new file manager window\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_new_win as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"NewTab\0" as *const u8 as *const gchar,
            stock_id: b"tab-new\0" as *const u8 as *const gchar,
            label: b"New T_ab\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>T\0" as *const u8 as *const gchar,
            tooltip: b"Create new tab for this folder\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_new_tab as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"CreateNew\0" as *const u8 as *const gchar,
            stock_id: c"gtk-add".as_ptr() as GtkStock as *const gchar,
            label: b"C_reate New...\0" as *const u8 as *const gchar,
            accelerator: b"\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"NewFolder\0" as *const u8 as *const gchar,
            stock_id: b"folder\0" as *const u8 as *const gchar,
            label: b"Folder\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl><Shift>N\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                bounce_action as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"NewBlank\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Empty File\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl><Alt>N\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                bounce_action as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Prop\0" as *const u8 as *const gchar,
            stock_id: c"gtk-properties".as_ptr() as GtkStock as *const gchar,
            label: b"Folder Propertie_s\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                bounce_action as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"CloseTab\0" as *const u8 as *const gchar,
            stock_id: c"gtk-close".as_ptr() as GtkStock as *const gchar,
            label: b"_Close Tab\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>W\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_close_tab as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Close\0" as *const u8 as *const gchar,
            stock_id: c"gtk-quit".as_ptr() as GtkStock as *const gchar,
            label: b"Close _Window\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>Q\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_close_win as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"EditMenu\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_Edit\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"Open\0" as *const u8 as *const gchar,
            stock_id: c"gtk-open".as_ptr() as GtkStock as *const gchar,
            label: ptr::null::<gchar>(),
            accelerator: b"\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_open as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Cut\0" as *const u8 as *const gchar,
            stock_id: c"gtk-cut".as_ptr() as GtkStock as *const gchar,
            label: b"C_ut\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                bounce_action as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Copy\0" as *const u8 as *const gchar,
            stock_id: c"gtk-copy".as_ptr() as GtkStock as *const gchar,
            label: ptr::null::<gchar>(),
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                bounce_action as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Paste\0" as *const u8 as *const gchar,
            stock_id: c"gtk-paste".as_ptr() as GtkStock as *const gchar,
            label: ptr::null::<gchar>(),
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                bounce_action as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"ToTrash\0" as *const u8 as *const gchar,
            stock_id: c"gtk-delete".as_ptr() as GtkStock as *const gchar,
            label: b"Move to _Trash\0" as *const u8 as *const gchar,
            accelerator: b"\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_trash as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Del\0" as *const u8 as *const gchar,
            stock_id: c"gtk-remove".as_ptr() as GtkStock as *const gchar,
            label: ptr::null::<gchar>(),
            accelerator: b"\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_del as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"CopyPath\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Copy Pat_h(s)\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_copy_path as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Rename\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"R_ename...\0" as *const u8 as *const gchar,
            accelerator: b"F2\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_rename as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Duplicate\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"D_uplicate...\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>U\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"Link\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Create Lin_k...\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_link as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"MoveTo\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_Move to...\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_move_to as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"CopyTo\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Copy to_...\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_copy_to as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"FileProp\0" as *const u8 as *const gchar,
            stock_id: c"gtk-properties".as_ptr() as GtkStock as *const gchar,
            label: b"Propertie_s\0" as *const u8 as *const gchar,
            accelerator: b"<Alt>Return\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                bounce_action as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"SelAll\0" as *const u8 as *const gchar,
            stock_id: c"gtk-select-all".as_ptr() as GtkStock as *const gchar,
            label: ptr::null::<gchar>(),
            accelerator: b"<Ctrl>A\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                bounce_action as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"InvSel\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_Invert Selection\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>I\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                bounce_action as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Pref\0" as *const u8 as *const gchar,
            stock_id: c"gtk-preferences".as_ptr() as GtkStock as *const gchar,
            label: b"Prefere_nces\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_preference as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"ViewMenu\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_View\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"Reload\0" as *const u8 as *const gchar,
            stock_id: c"gtk-refresh".as_ptr() as GtkStock as *const gchar,
            label: b"_Reload Folder\0" as *const u8 as *const gchar,
            accelerator: b"F5\0" as *const u8 as *const gchar,
            tooltip: b"Reload current folder\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_reload as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Toolbar\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Tool_bar\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"PathMode\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Pat_h Bar\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"SidePane\0" as *const u8 as *const gchar,
            stock_id: b"view-sidetree\0" as *const u8 as *const gchar,
            label: b"Side _Pane\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"FolderView\0" as *const u8 as *const gchar,
            stock_id: b"view-choose\0" as *const u8 as *const gchar,
            label: b"Fo_lder View Mode\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"Sort\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"S_ort Files\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"SizeBigger\0" as *const u8 as *const gchar,
            stock_id: c"gtk-zoom-in".as_ptr() as GtkStock as *const gchar,
            label: b"Zoom _In\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>KP_Add\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_size_increment as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"SizeSmaller\0" as *const u8 as *const gchar,
            stock_id: c"gtk-zoom-out".as_ptr() as GtkStock as *const gchar,
            label: b"Zoom O_ut\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>KP_Subtract\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_size_decrement as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"SizeDefault\0" as *const u8 as *const gchar,
            stock_id: c"gtk-zoom-100".as_ptr() as GtkStock as *const gchar,
            label: ptr::null::<gchar>(),
            accelerator: b"<Ctrl>0\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_size_default as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Filter\0" as *const u8 as *const gchar,
            stock_id: b"view-filter\0" as *const u8 as *const gchar,
            label: b"Fil_ter...\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>E\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_filter as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"HelpMenu\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_Help\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"About\0" as *const u8 as *const gchar,
            stock_id: c"gtk-about".as_ptr() as GtkStock as *const gchar,
            label: ptr::null::<gchar>(),
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_about as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"KeyNavList\0" as *const u8 as *const gchar,
            stock_id: c"gtk-info".as_ptr() as GtkStock as *const gchar,
            label: b"_Keyboard Navigation\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_key_nav_list as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"GoMenu\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_Go\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"Prev\0" as *const u8 as *const gchar,
            stock_id: c"gtk-go-back".as_ptr() as GtkStock as *const gchar,
            label: b"_Previous Folder\0" as *const u8 as *const gchar,
            accelerator: b"<Alt>Left\0" as *const u8 as *const gchar,
            tooltip: b"Return to previous folder in history\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_back as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Next\0" as *const u8 as *const gchar,
            stock_id: c"gtk-go-forward".as_ptr() as GtkStock as *const gchar,
            label: b"_Next Folder\0" as *const u8 as *const gchar,
            accelerator: b"<Alt>Right\0" as *const u8 as *const gchar,
            tooltip: b"Go forward to next folder in history\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_forward as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Up\0" as *const u8 as *const gchar,
            stock_id: c"gtk-go-up".as_ptr() as GtkStock as *const gchar,
            label: b"Pa_rent Folder\0" as *const u8 as *const gchar,
            accelerator: b"<Alt>Up\0" as *const u8 as *const gchar,
            tooltip: b"Go to parent Folder\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_up as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Home\0" as *const u8 as *const gchar,
            stock_id: b"user-home\0" as *const u8 as *const gchar,
            label: b"H_ome Folder\0" as *const u8 as *const gchar,
            accelerator: b"<Alt>Home\0" as *const u8 as *const gchar,
            tooltip: b"Go to home folder\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_home as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Desktop\0" as *const u8 as *const gchar,
            stock_id: b"user-desktop\0" as *const u8 as *const gchar,
            label: b"_Desktop\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: b"Go to desktop folder\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_desktop as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Trash\0" as *const u8 as *const gchar,
            stock_id: b"user-trash\0" as *const u8 as *const gchar,
            label: b"_Trash Can\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: b"Open trash can\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_trash as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Root\0" as *const u8 as *const gchar,
            stock_id: b"drive-harddisk\0" as *const u8 as *const gchar,
            label: b"Filesyste_m Root\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: b"Go fo filesystem root\0" as *const u8 as *const gchar,
            callback: None,
        },
        _GtkActionEntry {
            name: b"Apps\0" as *const u8 as *const gchar,
            stock_id: b"system-software-install\0" as *const u8 as *const gchar,
            label: b"_Applications\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: b"Go to root of applications menu folder\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_apps as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Computer\0" as *const u8 as *const gchar,
            stock_id: b"computer\0" as *const u8 as *const gchar,
            label: b"Dev_ices\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: b"Go to list of devices connected to the computer\0" as *const u8
                as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_computer as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Network\0" as *const u8 as *const gchar,
            stock_id: c"gtk-network".as_ptr() as GtkStock as *const gchar,
            label: b"Net_work\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: b"Go to list of places on the network\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_network as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Location\0" as *const u8 as *const gchar,
            stock_id: c"gtk-jump-to".as_ptr() as GtkStock as *const gchar,
            label: b"_Go to Location...\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>L\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_location as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Connect\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_Connect to Server...\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: b"Open a window to choose remote folder location\0" as *const u8
                as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_connect as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Go\0" as *const u8 as *const gchar,
            stock_id: c"gtk-jump-to".as_ptr() as GtkStock as *const gchar,
            label: ptr::null::<gchar>(),
            accelerator: ptr::null::<gchar>(),
            tooltip: b"Go to the path in the location bar\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"BookmarksMenu\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_Bookmarks\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"AddBookmark\0" as *const u8 as *const gchar,
            stock_id: c"gtk-add".as_ptr() as GtkStock as *const gchar,
            label: b"_Add to Bookmarks...\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>D\0" as *const u8 as *const gchar,
            tooltip: b"Add current folder to bookmarks list\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_add_bookmark as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"ToolMenu\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Too_ls\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: None,
        },
        _GtkActionEntry {
            name: b"Term\0" as *const u8 as *const gchar,
            stock_id: b"utilities-terminal\0" as *const u8 as *const gchar,
            label: b"Open Current Folder in _Terminal\0" as *const u8 as *const gchar,
            accelerator: b"F4\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_open_in_terminal as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Search\0" as *const u8 as *const gchar,
            stock_id: c"gtk-find".as_ptr() as GtkStock as *const gchar,
            label: b"Fin_d Files...\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl><Shift>F\0" as *const u8 as *const gchar,
            tooltip: b"Open search dialog\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_search as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Launch\0" as *const u8 as *const gchar,
            stock_id: c"gtk-execute".as_ptr() as GtkStock as *const gchar,
            label: b"_Run a Command in Current Folder...\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_launch as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Location2\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: ptr::null::<gchar>(),
            accelerator: b"<Alt>d\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_location as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Prev2\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: ptr::null::<gchar>(),
            accelerator: b"XF86Back\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_back as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Next2\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: ptr::null::<gchar>(),
            accelerator: b"XF86Forward\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_go_forward as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"Reload2\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: ptr::null::<gchar>(),
            accelerator: b"<Ctrl>R\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_reload as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"SizeBigger2\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: ptr::null::<gchar>(),
            accelerator: b"<Ctrl>equal\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_size_increment as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
        _GtkActionEntry {
            name: b"SizeSmaller2\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: ptr::null::<gchar>(),
            accelerator: b"<Ctrl>minus\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_size_decrement as unsafe extern "C" fn(*mut GtkAction, *mut FmMainWin) -> (),
            )),
        },
    ]
};
static mut main_win_toggle_actions: [GtkToggleActionEntry; 13] = unsafe {
    [
        _GtkToggleActionEntry {
            name: b"MingleDirs\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Mingle _Files and Folders\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_mingle_dirs as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: 0 as gboolean,
        },
        _GtkToggleActionEntry {
            name: b"SortIgnoreCase\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_Ignore Name Case\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_sort_ignore_case
                    as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: (0i32 == 0) as c_int,
        },
        _GtkToggleActionEntry {
            name: b"ShowHidden\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Sho_w Hidden\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>H\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_show_hidden as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: 0 as gboolean,
        },
        _GtkToggleActionEntry {
            name: b"SavePerFolder\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Preserve This Folder's Settings\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: b"Check to remember view and sort as folder setting rather than global one\0"
                as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_save_per_folder
                    as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: 0 as gboolean,
        },
        _GtkToggleActionEntry {
            name: b"ShowToolbar\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_Show Toolbar\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_show_toolbar as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: (0i32 == 0) as c_int,
        },
        _GtkToggleActionEntry {
            name: b"ToolbarNewWin\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Show 'New _Window' Button\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_toolbar_new_win
                    as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: (0i32 == 0) as c_int,
        },
        _GtkToggleActionEntry {
            name: b"ToolbarNewTab\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Show 'New _Tab' Button\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_toolbar_new_tab
                    as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: (0i32 == 0) as c_int,
        },
        _GtkToggleActionEntry {
            name: b"ToolbarNav\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Show _Navigation Buttons\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_toolbar_nav as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: (0i32 == 0) as c_int,
        },
        _GtkToggleActionEntry {
            name: b"ToolbarHome\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Show '_Home' Button\0" as *const u8 as *const gchar,
            accelerator: ptr::null::<gchar>(),
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_toolbar_home as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: (0i32 == 0) as c_int,
        },
        _GtkToggleActionEntry {
            name: b"ShowSidePane\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Sho_w Side Pane\0" as *const u8 as *const gchar,
            accelerator: b"F9\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_show_side_pane
                    as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: (0i32 == 0) as c_int,
        },
        _GtkToggleActionEntry {
            name: b"ShowStatus\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Show Status B_ar\0" as *const u8 as *const gchar,
            accelerator: b"<Ctrl>B\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_show_status as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: (0i32 == 0) as c_int,
        },
        _GtkToggleActionEntry {
            name: b"DualPane\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"_Dual Pane Mode\0" as *const u8 as *const gchar,
            accelerator: b"F3\0" as *const u8 as *const gchar,
            tooltip: b"Show two panels with folder views\0" as *const u8 as *const gchar,
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_dual_pane as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: 0 as gboolean,
        },
        _GtkToggleActionEntry {
            name: b"Fullscreen\0" as *const u8 as *const gchar,
            stock_id: ptr::null::<gchar>(),
            label: b"Fullscreen _Mode\0" as *const u8 as *const gchar,
            accelerator: b"F11\0" as *const u8 as *const gchar,
            tooltip: ptr::null::<gchar>(),
            callback: mem::transmute::<
                Option<unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> ()>,
                GCallback,
            >(Some(
                on_fullscreen as unsafe extern "C" fn(*mut GtkToggleAction, *mut FmMainWin) -> (),
            )),
            is_active: 0 as gboolean,
        },
    ]
};
static mut main_win_sort_type_actions: [GtkRadioActionEntry; 2] = [
    _GtkRadioActionEntry {
        name: b"Asc\0" as *const u8 as *const gchar,
        stock_id: c"gtk-sort-ascending".as_ptr() as GtkStock as *const gchar,
        label: ptr::null::<gchar>(),
        accelerator: ptr::null::<gchar>(),
        tooltip: ptr::null::<gchar>(),
        value: GTK_SORT_ASCENDING as c_int as gint,
    },
    _GtkRadioActionEntry {
        name: b"Desc\0" as *const u8 as *const gchar,
        stock_id: c"gtk-sort-descending".as_ptr() as GtkStock as *const gchar,
        label: ptr::null::<gchar>(),
        accelerator: ptr::null::<gchar>(),
        tooltip: ptr::null::<gchar>(),
        value: GTK_SORT_DESCENDING as c_int as gint,
    },
];
static mut main_win_sort_by_actions: [GtkRadioActionEntry; 5] = [
    _GtkRadioActionEntry {
        name: b"ByName\0" as *const u8 as *const gchar,
        stock_id: ptr::null::<gchar>(),
        label: b"By _Name\0" as *const u8 as *const gchar,
        accelerator: b"<Alt><Ctrl>1\0" as *const u8 as *const gchar,
        tooltip: ptr::null::<gchar>(),
        value: FM_FOLDER_MODEL_COL_NAME as c_int as gint,
    },
    _GtkRadioActionEntry {
        name: b"ByMTime\0" as *const u8 as *const gchar,
        stock_id: ptr::null::<gchar>(),
        label: b"By _Modification Time\0" as *const u8 as *const gchar,
        accelerator: b"<Alt><Ctrl>2\0" as *const u8 as *const gchar,
        tooltip: ptr::null::<gchar>(),
        value: FM_FOLDER_MODEL_COL_MTIME as c_int as gint,
    },
    _GtkRadioActionEntry {
        name: b"BySize\0" as *const u8 as *const gchar,
        stock_id: ptr::null::<gchar>(),
        label: b"By _Size\0" as *const u8 as *const gchar,
        accelerator: b"<Alt><Ctrl>3\0" as *const u8 as *const gchar,
        tooltip: ptr::null::<gchar>(),
        value: FM_FOLDER_MODEL_COL_SIZE as c_int as gint,
    },
    _GtkRadioActionEntry {
        name: b"ByType\0" as *const u8 as *const gchar,
        stock_id: ptr::null::<gchar>(),
        label: b"By File _Type\0" as *const u8 as *const gchar,
        accelerator: b"<Alt><Ctrl>4\0" as *const u8 as *const gchar,
        tooltip: ptr::null::<gchar>(),
        value: FM_FOLDER_MODEL_COL_DESC as c_int as gint,
    },
    _GtkRadioActionEntry {
        name: b"ByExt\0" as *const u8 as *const gchar,
        stock_id: ptr::null::<gchar>(),
        label: b"By _Extension\0" as *const u8 as *const gchar,
        accelerator: b"<Alt><Ctrl>5\0" as *const u8 as *const gchar,
        tooltip: ptr::null::<gchar>(),
        value: FM_FOLDER_MODEL_COL_EXT as c_int as gint,
    },
];
static mut main_win_path_bar_mode_actions: [GtkRadioActionEntry; 2] = [
    _GtkRadioActionEntry {
        name: b"PathEntry\0" as *const u8 as *const gchar,
        stock_id: ptr::null::<gchar>(),
        label: b"_Location\0" as *const u8 as *const gchar,
        accelerator: ptr::null::<gchar>(),
        tooltip: ptr::null::<gchar>(),
        value: 0 as gint,
    },
    _GtkRadioActionEntry {
        name: b"PathBar\0" as *const u8 as *const gchar,
        stock_id: ptr::null::<gchar>(),
        label: b"_Buttons\0" as *const u8 as *const gchar,
        accelerator: ptr::null::<gchar>(),
        tooltip: ptr::null::<gchar>(),
        value: 1 as gint,
    },
];
