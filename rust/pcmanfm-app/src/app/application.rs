// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      pcmanfm.c
 *
 *      Copyright 2009 - 2010 Hong Jen Yee (PCMan) <pcman.tw@gmail.com>
 *      Copyright 2012-2015 Andriy Grytsenko (LStranger) <andrej@rep.kiev.ua>
 *      Copyright 2020 Mamoru TASAKA <mtasaka@fedoraproject.org>
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

static mut signal_pipe: [c_int; 2] = [-(1i32), -(1i32)];
static mut daemon_mode: gboolean = 0 as gboolean;
static mut first_run: gboolean = (0i32 == 0) as c_int;
static mut save_config_idle: guint = 0 as guint;
static mut files_to_open: *mut *mut c_char = ptr::null::<*mut c_char>() as *mut *mut c_char;
static mut n_files_to_open: c_int = 0i32;
static mut profile: *mut c_char = ptr::null::<c_char>() as *mut c_char;
static mut no_desktop: gboolean = 0 as gboolean;
static mut show_desktop: gboolean = 0 as gboolean;
static mut desktop_off: gboolean = 0 as gboolean;
static mut desktop_running: gboolean = 0 as gboolean;
static mut one_screen: gboolean = 0 as gboolean;
static mut show_pref: gint = -(1 as gint);
static mut desktop_pref: gboolean = 0 as gboolean;
static mut set_wallpaper: *mut c_char = ptr::null::<c_char>() as *mut c_char;
static mut wallpaper_mode: *mut c_char = ptr::null::<c_char>() as *mut c_char;
static mut new_win: gboolean = 0 as gboolean;
static mut find_files: gboolean = 0 as gboolean;
static mut ipc_cwd: *mut c_char = ptr::null::<c_char>() as *mut c_char;
static mut window_role: *mut c_char = ptr::null::<c_char>() as *mut c_char;
static mut n_pcmanfm_ref: c_int = 0i32;
static mut opt_entries: [GOptionEntry; 15] = unsafe {
    [
        _GOptionEntry {
            long_name: b"profile\0" as *const u8 as *const gchar,
            short_name: 'p' as i32 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_STRING,
            arg_data: &raw const profile as *mut *mut c_char as gpointer,
            description: b"Name of configuration profile\0" as *const u8 as *const gchar,
            arg_description: b"PROFILE\0" as *const u8 as *const gchar,
        },
        _GOptionEntry {
            long_name: b"daemon-mode\0" as *const u8 as *const gchar,
            short_name: 'd' as i32 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_NONE,
            arg_data: &raw const daemon_mode as *mut gboolean as gpointer,
            description: b"Run PCManFM as a daemon\0" as *const u8 as *const gchar,
            arg_description: ptr::null::<gchar>(),
        },
        _GOptionEntry {
            long_name: b"no-desktop\0" as *const u8 as *const gchar,
            short_name: 0 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_NONE,
            arg_data: &raw const no_desktop as *mut gboolean as gpointer,
            description: b"No function. Just to be compatible with nautilus\0" as *const u8
                as *const gchar,
            arg_description: ptr::null::<gchar>(),
        },
        _GOptionEntry {
            long_name: b"desktop\0" as *const u8 as *const gchar,
            short_name: 0 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_NONE,
            arg_data: &raw const show_desktop as *mut gboolean as gpointer,
            description: b"Launch desktop manager\0" as *const u8 as *const gchar,
            arg_description: ptr::null::<gchar>(),
        },
        _GOptionEntry {
            long_name: b"desktop-off\0" as *const u8 as *const gchar,
            short_name: 0 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_NONE,
            arg_data: &raw const desktop_off as *mut gboolean as gpointer,
            description: b"Turn off desktop manager if it's running\0" as *const u8 as *const gchar,
            arg_description: ptr::null::<gchar>(),
        },
        _GOptionEntry {
            long_name: b"desktop-pref\0" as *const u8 as *const gchar,
            short_name: 0 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_NONE,
            arg_data: &raw const desktop_pref as *mut gboolean as gpointer,
            description: b"Open desktop preference dialog\0" as *const u8 as *const gchar,
            arg_description: ptr::null::<gchar>(),
        },
        _GOptionEntry {
            long_name: b"one-screen\0" as *const u8 as *const gchar,
            short_name: 0 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_NONE,
            arg_data: &raw const one_screen as *mut gboolean as gpointer,
            description: b"Use --desktop option only for one screen\0" as *const u8 as *const gchar,
            arg_description: ptr::null::<gchar>(),
        },
        _GOptionEntry {
            long_name: b"set-wallpaper\0" as *const u8 as *const gchar,
            short_name: 'w' as i32 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_FILENAME,
            arg_data: &raw const set_wallpaper as *mut *mut c_char as gpointer,
            description: b"Set desktop wallpaper from image FILE\0" as *const u8 as *const gchar,
            arg_description: b"FILE\0" as *const u8 as *const gchar,
        },
        _GOptionEntry {
            long_name: b"wallpaper-mode\0" as *const u8 as *const gchar,
            short_name: 0 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_STRING,
            arg_data: &raw const wallpaper_mode as *mut *mut c_char as gpointer,
            description:
                b"Set mode of desktop wallpaper. MODE=(color|stretch|fit|crop|center|tile|screen)\0"
                    as *const u8 as *const gchar,
            arg_description: b"MODE\0" as *const u8 as *const gchar,
        },
        _GOptionEntry {
            long_name: b"show-pref\0" as *const u8 as *const gchar,
            short_name: 0 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_INT,
            arg_data: &raw const show_pref as *mut gint as gpointer,
            description: b"Open Preferences dialog on the page N\0" as *const u8 as *const gchar,
            arg_description: b"N\0" as *const u8 as *const gchar,
        },
        _GOptionEntry {
            long_name: b"new-win\0" as *const u8 as *const gchar,
            short_name: 'n' as i32 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_NONE,
            arg_data: &raw const new_win as *mut gboolean as gpointer,
            description: b"Open new window\0" as *const u8 as *const gchar,
            arg_description: ptr::null::<gchar>(),
        },
        _GOptionEntry {
            long_name: b"find-files\0" as *const u8 as *const gchar,
            short_name: 'f' as i32 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_NONE,
            arg_data: &raw const find_files as *mut gboolean as gpointer,
            description: b"Open a Find Files window\0" as *const u8 as *const gchar,
            arg_description: ptr::null::<gchar>(),
        },
        _GOptionEntry {
            long_name: b"role\0" as *const u8 as *const gchar,
            short_name: 0 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_STRING,
            arg_data: &raw const window_role as *mut *mut c_char as gpointer,
            description: b"Window role for usage by window manager\0" as *const u8 as *const gchar,
            arg_description: b"ROLE\0" as *const u8 as *const gchar,
        },
        _GOptionEntry {
            long_name: b"\0" as *const u8 as *const gchar,
            short_name: 0 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_FILENAME_ARRAY,
            arg_data: &raw const files_to_open as *mut *mut *mut c_char as gpointer,
            description: ptr::null::<gchar>(),
            arg_description: b"[FILE1, FILE2,...]\0" as *const u8 as *const gchar,
        },
        _GOptionEntry {
            long_name: ptr::null::<gchar>(),
            short_name: 0,
            flags: 0,
            arg: G_OPTION_ARG_NONE,
            arg_data: ptr::null::<c_void>() as *mut c_void,
            description: ptr::null::<gchar>(),
            arg_description: ptr::null::<gchar>(),
        },
    ]
};
unsafe extern "C" fn unix_signal_handler(mut sig_num: c_int) {
    if write(
        signal_pipe[1 as usize],
        &raw mut sig_num as *const c_void,
        mem::size_of::<c_int>() as size_t,
    ) as usize
        != mem::size_of::<c_int>() as usize
    {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_CRITICAL,
            b"cannot bounce the signal, stop\0" as *const u8 as *const gchar,
        );
        exit(2 as c_int);
    }
}
unsafe extern "C" fn on_unix_signal(
    mut ch: *mut GIOChannel,
    mut cond: GIOCondition,
    mut user_data: gpointer,
) -> gboolean {
    let mut sig_num: c_int = 0;
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    let mut got: gsize = 0;
    status = g_io_channel_read_chars(
        ch,
        &raw mut sig_num as *mut gchar,
        mem::size_of::<c_int>() as gsize,
        &raw mut got,
        ptr::null_mut::<*mut GError>(),
    );
    if status as c_uint == G_IO_STATUS_AGAIN as c_int as c_uint {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_DEBUG,
            b"got G_IO_STATUS_AGAIN\0" as *const u8 as *const gchar,
        );
        return (0i32 == 0) as c_int;
    }
    if status as c_uint != G_IO_STATUS_NORMAL as c_int as c_uint
        || got as usize != mem::size_of::<c_int>() as usize
    {
        g_log(
            ptr::null_mut::<gchar>(),
            G_LOG_LEVEL_DEBUG,
            b"signal pipe is broken\0" as *const u8 as *const gchar,
        );
        gtk_main_quit();
        return 0 as gboolean;
    }
    g_log(
        ptr::null_mut::<gchar>(),
        G_LOG_LEVEL_DEBUG,
        b"got signal %d from pipe\0" as *const u8 as *const gchar,
        sig_num,
    );
    match sig_num {
        15 | _ => {}
    }
    gtk_main_quit();
    return 0 as gboolean;
}
unsafe extern "C" fn single_inst_cb(mut cwd: *const c_char, mut screen_num: c_int) {
    g_free(ipc_cwd as gpointer);
    ipc_cwd = g_strdup_inline(cwd);
    if !files_to_open.is_null() {
        let mut i: c_int = 0;
        n_files_to_open = g_strv_length(files_to_open as *mut *mut gchar) as c_int;
        i = 0i32;
        while i < n_files_to_open {
            let mut file: *mut c_char = *files_to_open.offset(i as isize);
            let mut scheme: *mut c_char = g_uri_parse_scheme(file);
            g_log(
                ptr::null_mut::<gchar>(),
                G_LOG_LEVEL_DEBUG,
                b"file: %s\0" as *const u8 as *const gchar,
                file,
            );
            if !scheme.is_null() {
                g_free(scheme as gpointer);
            } else {
                let ref mut fresh38 = *files_to_open.offset(i as isize);
                *fresh38 = fm_canonicalize_filename(file, cwd);
                g_free(file as gpointer);
            }
            i += 1;
        }
    }
    pcmanfm_run(screen_num as gint);
}
#[inline]
unsafe extern "C" fn fm_module_register_tab_page_status() {
    fm_module_register_type(
        c"tab_page_status".as_ptr(),
        1i32,
        1i32,
        Some(
            fm_module_callback_tab_page_status
                as unsafe extern "C" fn(*const c_char, gpointer, c_int) -> gboolean,
        ),
    );
}
#[no_mangle]
pub static mut _tab_page_modules: *mut GList = ptr::null::<GList>() as *mut GList;
unsafe extern "C" fn fm_module_callback_tab_page_status(
    mut name: *const c_char,
    mut init: gpointer,
    mut ver: c_int,
) -> gboolean {
    if (*(init as *mut FmTabPageStatusInit)).sel_message.is_none() {
        return 0 as gboolean;
    }
    if (*(init as *mut FmTabPageStatusInit)).init.is_some()
        && (*(init as *mut FmTabPageStatusInit))
            .init
            .expect("non-null function pointer")()
            == 0
    {
        return 0 as gboolean;
    }
    _tab_page_modules = g_list_append(_tab_page_modules, init);
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn on_config_changed(mut cfg: *mut FmAppConfig, mut _unused: gpointer) {
    pcmanfm_save_config(0 as gboolean);
}
unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut config: *mut FmConfig = ptr::null_mut::<FmConfig>();
    let mut err: *mut GError = ptr::null_mut::<GError>();
    let mut inst: SingleInstData = SingleInstData {
        prog_name: ptr::null::<c_char>(),
        cb: None,
        opt_entries: ptr::null::<GOptionEntry>(),
        screen_num: 0,
        io_channel: ptr::null_mut::<GIOChannel>(),
        sock: 0,
        io_watch: 0,
    };
    let mut l: *mut GList = ptr::null_mut::<GList>();
    bindtextdomain(c"pcmanfm".as_ptr(), c"/usr/share/locale".as_ptr());
    bind_textdomain_codeset(c"pcmanfm".as_ptr(), c"UTF-8".as_ptr());
    textdomain(c"pcmanfm".as_ptr());
    if ({
        let mut _g_boolean_var_57: c_int = 0i32;
        if gtk_init_with_args(
            &raw mut argc,
            &raw mut argv,
            b" \0" as *const u8 as *const gchar,
            &raw mut opt_entries as *mut GOptionEntry,
            b"pcmanfm\0" as *const u8 as *const gchar,
            &raw mut err,
        ) == 0
        {
            _g_boolean_var_57 = 1i32;
        }
        _g_boolean_var_57
    }) as c_long
        != 0
    {
        g_printf(b"%s\n\0" as *const u8 as *const gchar, (*err).message);
        g_error_free(err);
        return 1i32;
    }
    inst.prog_name = c"pcmanfm".as_ptr();
    inst.cb = Some(single_inst_cb as unsafe extern "C" fn(*const c_char, c_int) -> ())
        as SingleInstCallback;
    inst.opt_entries = (&raw mut opt_entries as *mut GOptionEntry).offset(3 as isize);
    inst.screen_num = gdk_x11_get_default_screen() as c_int;
    match single_inst_init(&raw mut inst) as c_uint {
        1 => {
            single_inst_finalize(&raw mut inst);
            gdk_notify_startup_complete();
            return 0i32;
        }
        2 => {
            single_inst_finalize(&raw mut inst);
            return 1i32;
        }
        0 | _ => {}
    }
    if pipe(&raw mut signal_pipe as *mut c_int) == 0i32 {
        let mut ch: *mut GIOChannel = g_io_channel_unix_new(signal_pipe[0 as usize]);
        g_io_add_watch(
            ch,
            (G_IO_IN as c_int | G_IO_PRI as c_int) as GIOCondition,
            mem::transmute::<
                Option<unsafe extern "C" fn(*mut GIOChannel, GIOCondition, gpointer) -> gboolean>,
                GIOFunc,
            >(Some(
                on_unix_signal
                    as unsafe extern "C" fn(*mut GIOChannel, GIOCondition, gpointer) -> gboolean,
            )),
            ptr::null_mut::<c_void>(),
        );
        g_io_channel_set_encoding(ch, ptr::null::<gchar>(), ptr::null_mut::<*mut GError>());
        g_io_channel_unref(ch);
        signal(
            1i32,
            Some(unix_signal_handler as unsafe extern "C" fn(c_int) -> ()),
        );
        signal(
            15 as c_int,
            Some(unix_signal_handler as unsafe extern "C" fn(c_int) -> ()),
        );
        signal(
            2 as c_int,
            Some(unix_signal_handler as unsafe extern "C" fn(c_int) -> ()),
        );
    }
    config = fm_app_config_new();
    fm_gtk_init(config);
    fm_modules_add_directory(c"/usr/lib/pcmanfm".as_ptr());
    fm_module_register_tab_page_status();
    fm_app_config_load_from_profile(config as *mut c_void as *mut FmAppConfig, profile);
    g_signal_connect_data(
        config as gpointer,
        b"changed::saved_search\0" as *const u8 as *const gchar,
        mem::transmute::<Option<unsafe extern "C" fn(*mut FmAppConfig, gpointer) -> ()>, GCallback>(
            Some(on_config_changed as unsafe extern "C" fn(*mut FmAppConfig, gpointer) -> ()),
        ),
        ptr::null_mut::<c_void>(),
        None,
        G_CONNECT_DEFAULT,
    );
    if pcmanfm_run(gdk_screen_get_number(gdk_screen_get_default())) != 0 {
        first_run = 0 as gboolean;
        fm_volume_manager_init();
        gtk_main();
        if save_config_idle != 0 {
            pcmanfm_save_config((0i32 == 0) as c_int);
            g_source_remove(save_config_idle);
            save_config_idle = 0 as guint;
        }
        fm_volume_manager_finalize();
    }
    l = _tab_page_modules;
    while !l.is_null() {
        if (*((*l).data as *mut FmTabPageStatusInit))
            .finalize
            .is_some()
        {
            (*((*l).data as *mut FmTabPageStatusInit))
                .finalize
                .expect("non-null function pointer")();
        }
        l = (*l).next;
    }
    fm_module_unregister_type(c"tab_page_status".as_ptr());
    g_list_free(_tab_page_modules);
    _tab_page_modules = ptr::null_mut::<GList>();
    single_inst_finalize(&raw mut inst);
    fm_gtk_finalize();
    g_object_unref(config as gpointer);
    return 0i32;
}
unsafe extern "C" fn reset_options() -> gboolean {
    show_desktop = 0 as gboolean;
    desktop_off = 0 as gboolean;
    desktop_pref = 0 as gboolean;
    one_screen = 0 as gboolean;
    g_free(set_wallpaper as gpointer);
    set_wallpaper = ptr::null_mut::<c_char>();
    g_free(wallpaper_mode as gpointer);
    wallpaper_mode = ptr::null_mut::<c_char>();
    show_pref = -(1i32) as gint;
    new_win = 0 as gboolean;
    find_files = 0 as gboolean;
    g_free(window_role as gpointer);
    window_role = ptr::null_mut::<c_char>();
    g_strfreev(files_to_open as *mut *mut gchar);
    files_to_open = ptr::null_mut::<*mut c_char>();
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn pcmanfm_run(mut screen_num: gint) -> gboolean {
    let mut win: *mut FmMainWin = ptr::null_mut::<FmMainWin>();
    let mut ret: gboolean = (0i32 == 0) as c_int;
    if files_to_open.is_null() {
        let mut desktop: *mut FmDesktop = fm_desktop_get(screen_num, 0 as gint);
        if show_desktop != 0 {
            if desktop_running == 0 {
                fm_desktop_manager_init(if one_screen != 0 {
                    screen_num
                } else {
                    -(1 as gint)
                });
                desktop_running = (0i32 == 0) as c_int as gboolean;
            }
            return reset_options();
        } else if desktop_off != 0 {
            if desktop_running != 0 {
                desktop_running = 0 as gboolean;
                fm_desktop_manager_finalize();
            }
            reset_options();
            return 0 as gboolean;
        } else if show_pref > 0i32 {
            fm_edit_preference(
                desktop as *mut c_void as *mut GtkWindow,
                show_pref as c_int - 1i32,
            );
            return reset_options();
        } else if desktop.is_null() {
            if desktop_pref != 0 || !wallpaper_mode.is_null() || !set_wallpaper.is_null() {
                fm_show_error(
                    ptr::null_mut::<GtkWindow>(),
                    ptr::null::<c_char>(),
                    dcgettext(
                        ptr::null::<c_char>(),
                        c"Desktop manager is not active.".as_ptr(),
                        5 as c_int,
                    ),
                );
                reset_options();
                return 0 as gboolean;
            }
        } else if desktop_pref != 0 {
            fm_desktop_preference(ptr::null_mut::<GtkAction>(), desktop);
            return reset_options();
        } else if !wallpaper_mode.is_null() || !set_wallpaper.is_null() {
            let mut wallpaper_changed: gboolean = 0 as gboolean;
            if !set_wallpaper.is_null() {
                if !gdk_pixbuf_get_file_info(
                    set_wallpaper,
                    ptr::null_mut::<gint>(),
                    ptr::null_mut::<gint>(),
                )
                .is_null()
                {
                    if !(*desktop).conf.wallpaper.is_null() {
                        g_free((*desktop).conf.wallpaper as gpointer);
                    }
                    (*desktop).conf.wallpaper = set_wallpaper;
                    if wallpaper_mode.is_null() {
                        if (*desktop).conf.wallpaper_mode as c_uint
                            == FM_WP_COLOR as c_int as c_uint
                        {
                            (*desktop).conf.wallpaper_mode = FM_WP_FIT;
                        }
                    }
                    wallpaper_changed = (0i32 == 0) as c_int as gboolean;
                    set_wallpaper = ptr::null_mut::<c_char>();
                }
            }
            if !wallpaper_mode.is_null() {
                let mut mode: FmWallpaperMode = fm_app_wallpaper_get_mode_by_name(wallpaper_mode);
                if mode as c_uint != 4294967295 as FmWallpaperMode as c_uint
                    && mode as c_uint != (*desktop).conf.wallpaper_mode as c_uint
                {
                    (*desktop).conf.wallpaper_mode = mode;
                    wallpaper_changed = (0i32 == 0) as c_int as gboolean;
                }
            }
            if wallpaper_changed != 0 {
                fm_desktop_wallpaper_changed(desktop);
            }
            reset_options();
            return 0 as gboolean;
        }
    }
    if !files_to_open.is_null() {
        let mut filename: *mut *mut c_char = ptr::null_mut::<*mut c_char>();
        let mut cwd: *mut FmPath = ptr::null_mut::<FmPath>();
        let mut paths: *mut GList = ptr::null_mut::<GList>();
        let mut current_block_70: u64;
        filename = files_to_open;
        while !(*filename).is_null() {
            let mut path: *mut FmPath = ptr::null_mut::<FmPath>();
            if **filename as c_int == '/' as i32 {
                path = fm_path_new_for_path(*filename);
                current_block_70 = 16203797167131938757;
            } else if !strstr(*filename, c":/".as_ptr()).is_null() {
                path = fm_path_new_for_uri(*filename);
                current_block_70 = 16203797167131938757;
            } else if strcmp(*filename, c"~".as_ptr()) == 0i32 {
                path = fm_path_get_home();
                win = fm_main_win_add_win(ptr::null_mut::<FmMainWin>(), path);
                if new_win != 0 && !window_role.is_null() {
                    gtk_window_set_role(win as *mut c_void as *mut GtkWindow, window_role);
                }
                current_block_70 = 10150597327160359210;
            } else {
                if ({
                    let mut _g_boolean_var_58: c_int = 0 as c_int;
                    if cwd.is_null() {
                        _g_boolean_var_58 = 1i32;
                    }
                    _g_boolean_var_58
                }) as c_long
                    != 0
                {
                    let mut cwd_str: *mut c_char = if !ipc_cwd.is_null() {
                        ipc_cwd
                    } else {
                        g_get_current_dir() as *mut c_char
                    };
                    cwd = fm_path_new_for_str(cwd_str);
                    g_free(cwd_str as gpointer);
                }
                path = fm_path_new_relative(cwd, *filename);
                current_block_70 = 16203797167131938757;
            }
            match current_block_70 {
                16203797167131938757 => {
                    paths = g_list_append(paths, path as gpointer);
                }
                _ => {}
            }
            filename = filename.offset(1);
        }
        if !cwd.is_null() {
            fm_path_unref(cwd);
        }
        fm_launch_paths_simple(
            ptr::null_mut::<GtkWindow>(),
            ptr::null_mut::<GAppLaunchContext>(),
            paths,
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
        g_list_foreach(
            paths,
            mem::transmute::<Option<unsafe extern "C" fn(*mut FmPath) -> ()>, GFunc>(Some(
                fm_path_unref as unsafe extern "C" fn(*mut FmPath) -> (),
            )),
            ptr::null_mut::<c_void>(),
        );
        g_list_free(paths);
        ret = (n_pcmanfm_ref >= 1i32) as c_int as gboolean;
    } else if first_run != 0 && daemon_mode != 0 {
        pcmanfm_ref();
    } else if ({
        let mut _g_boolean_var_59: c_int = 0i32;
        if find_files == 0 || n_pcmanfm_ref < 1i32 {
            _g_boolean_var_59 = 1i32;
        }
        _g_boolean_var_59
    }) as c_long
        != 0
    {
        let mut path_0: *mut FmPath = ptr::null_mut::<FmPath>();
        let mut cwd_0: *mut c_char = if !ipc_cwd.is_null() {
            ipc_cwd
        } else {
            g_get_current_dir() as *mut c_char
        };
        path_0 = fm_path_new_for_path(cwd_0);
        win = fm_main_win_add_win(ptr::null_mut::<FmMainWin>(), path_0);
        if new_win != 0 && !window_role.is_null() {
            gtk_window_set_role(win as *mut c_void as *mut GtkWindow, window_role);
        }
        fm_path_unref(path_0);
        g_free(cwd_0 as gpointer);
        ipc_cwd = ptr::null_mut::<c_char>();
    }
    if ret != 0 && find_files != 0 {
        fm_launch_search_simple(
            win as *mut c_void as *mut GtkWindow,
            ptr::null_mut::<GAppLaunchContext>(),
            ptr::null::<GList>(),
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
    reset_options();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn pcmanfm_ref() {
    n_pcmanfm_ref += 1;
}
#[no_mangle]
pub unsafe extern "C" fn pcmanfm_unref() {
    n_pcmanfm_ref -= 1;
    if 0i32 == n_pcmanfm_ref {
        gtk_main_quit();
    }
}
unsafe extern "C" fn move_window_to_desktop(mut win: *mut FmMainWin, mut desktop: *mut FmDesktop) {
    let mut screen: *mut GdkScreen =
        gtk_widget_get_screen(desktop as *mut c_void as *mut GtkWidget);
    let mut atom: Atom = 0;
    let mut atom_name: *mut c_char = c"_NET_WM_DESKTOP".as_ptr() as *mut c_char;
    let mut xev: XClientMessageEvent = XClientMessageEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: ptr::null_mut::<Display>(),
        window: 0,
        message_type: 0,
        format: 0,
        data: C2RustUnnamed_6 { b: [0; 20] },
    };
    gtk_window_set_screen(win as *mut c_void as *mut GtkWindow, screen);
    if XInternAtoms(
        gdk_x11_get_default_xdisplay(),
        &raw mut atom_name,
        1i32,
        0i32,
        &raw mut atom,
    ) == 0
    {
        return;
    }
    xev.type_0 = 33 as c_int;
    xev.window =
        gdk_x11_window_get_xid(gtk_widget_get_window(win as *mut c_void as *mut GtkWidget));
    xev.message_type = atom;
    xev.format = 32 as c_int;
    xev.data.l[0 as usize] = (*desktop).cur_desktop as c_long;
    xev.data.l[1 as usize] = 0 as c_long;
    xev.data.l[2 as usize] = 0 as c_long;
    xev.data.l[3 as usize] = 0 as c_long;
    xev.data.l[4 as usize] = 0 as c_long;
    XSendEvent(
        gdk_x11_get_default_xdisplay(),
        gdk_x11_get_default_root_xwindow(),
        0i32,
        (1 as c_long) << 19 as c_int | (1 as c_long) << 20i32,
        &raw mut xev as *mut XEvent,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pcmanfm_open_folder(
    mut ctx: *mut GAppLaunchContext,
    mut folder_infos: *mut GList,
    mut user_data: gpointer,
    mut err: *mut *mut GError,
) -> gboolean {
    let mut l: *mut GList = folder_infos;
    let mut use_new_win: gboolean = new_win;
    if use_new_win == 0
        && !user_data.is_null()
        && ({
            let mut __inst: *mut GTypeInstance = user_data as *mut GTypeInstance;
            let mut __t: GType = fm_desktop_get_type();
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
        use_new_win = (*(fm_config as *mut FmAppConfig)).desktop_folder_new_win;
    }
    if use_new_win != 0 {
        let mut win: *mut FmMainWin = fm_main_win_add_win(
            ptr::null_mut::<FmMainWin>(),
            fm_file_info_get_path((*l).data as *mut FmFileInfo),
        );
        if !window_role.is_null() {
            gtk_window_set_role(win as *mut c_void as *mut GtkWindow, window_role);
        }
        new_win = 0 as gboolean;
        l = (*l).next;
    }
    while !l.is_null() {
        let mut fi: *mut FmFileInfo = (*l).data as *mut FmFileInfo;
        fm_main_win_open_in_last_active(fm_file_info_get_path(fi));
        l = (*l).next;
    }
    if !user_data.is_null()
        && ({
            let mut __inst: *mut GTypeInstance = user_data as *mut GTypeInstance;
            let mut __t: GType = fm_desktop_get_type();
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
        move_window_to_desktop(fm_main_win_get_last_active(), user_data as *mut FmDesktop);
    }
    return (0i32 == 0) as c_int;
}
unsafe extern "C" fn on_save_config_idle(mut user_data: gpointer) -> gboolean {
    pcmanfm_save_config((0i32 == 0) as c_int);
    save_config_idle = 0 as guint;
    return 0 as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn pcmanfm_save_config(mut immediate: gboolean) {
    if immediate != 0 {
        fm_config_save(fm_config, ptr::null::<c_char>());
        fm_app_config_save_profile(fm_config as *mut FmAppConfig, profile);
    } else if 0 as guint == save_config_idle {
        save_config_idle = gdk_threads_add_idle_full(
            300 as gint,
            mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gboolean>, GSourceFunc>(
                Some(on_save_config_idle as unsafe extern "C" fn(gpointer) -> gboolean),
            ),
            ptr::null_mut::<c_void>(),
            None,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn pcmanfm_can_open_path_in_terminal(mut dir: *mut FmPath) -> gboolean {
    let mut gf: *mut GFile = ptr::null_mut::<GFile>();
    let mut wd: *mut c_char = ptr::null_mut::<c_char>();
    if fm_path_get_flags(dir) as c_uint & FM_PATH_IS_NATIVE as c_int as c_uint != 0 as c_uint {
        return (0i32 == 0) as c_int;
    }
    gf = fm_path_to_gfile(dir);
    wd = g_file_get_path(gf);
    g_object_unref(gf as gpointer);
    g_free(wd as gpointer);
    return (wd != ptr::null_mut::<c_void>() as *mut c_char) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pcmanfm_open_folder_in_terminal(
    mut parent: *mut GtkWindow,
    mut dir: *mut FmPath,
) {
    if (*fm_config).terminal.is_null() {
        fm_show_error(
            parent,
            ptr::null::<c_char>(),
            dcgettext(
                ptr::null::<c_char>(),
                c"Terminal emulator is not set.".as_ptr(),
                5 as c_int,
            ),
        );
        fm_edit_preference(parent, PREF_ADVANCED as c_int);
        return;
    } else {
        let mut err: *mut GError = ptr::null_mut::<GError>();
        let mut cwd_str: *mut c_char = ptr::null_mut::<c_char>();
        if fm_path_get_flags(dir) as c_uint & FM_PATH_IS_NATIVE as c_int as c_uint != 0 as c_uint {
            cwd_str = fm_path_to_str(dir);
        } else {
            let mut gf: *mut GFile = fm_path_to_gfile(dir);
            cwd_str = g_file_get_path(gf);
            g_object_unref(gf as gpointer);
        }
        if fm_terminal_launch(cwd_str, &raw mut err) == 0 {
            fm_show_error(parent, ptr::null::<c_char>(), (*err).message);
            g_error_free(err);
        }
        g_free(cwd_str as gpointer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pcmanfm_get_profile_dir(mut create: gboolean) -> *mut c_char {
    let mut dir: *mut c_char = g_build_filename(
        g_get_user_config_dir(),
        c"pcmanfm".as_ptr(),
        if !profile.is_null() {
            profile as *const c_char
        } else {
            c"default".as_ptr()
        },
        ptr::null_mut::<c_void>(),
    ) as *mut c_char;
    if create != 0 {
        g_mkdir_with_parents(dir, 0o700 as gint);
    }
    return dir;
}
static mut volume_manager_vol_mon: *mut GVolumeMonitor =
    ptr::null::<GVolumeMonitor>() as *mut GVolumeMonitor;
