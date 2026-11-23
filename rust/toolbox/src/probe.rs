// SPDX-License-Identifier: MIT
//! Allow the C/XTest fixtures to exercise the Rust widgets on their private display.
use super::*;
use std::ffi::{c_char, c_int};
use std::ptr::{null, null_mut};

#[no_mangle]
unsafe extern "C" fn toolbox_probe_init(mut argc: c_int, argv: *mut *mut c_char, footer: c_int) {
    // SAFETY: tests call once per process with a live C argv, after setting DISPLAY.
    XtSetLanguageProc(null_mut(), None, null_mut());
    wshell = XtVaAppInitialize(
        &raw mut app_context,
        c"XmToolbox".as_ptr(),
        (&raw mut xrdb_options).cast(),
        8,
        &mut argc,
        argv,
        (&raw mut fallback_res).cast(),
        c"allowShellResize".as_ptr(),
        1 as c_int,
        c"title".as_ptr(),
        c"Toolchest XDG integration test".as_ptr(),
        null::<c_char>(),
    );
    XtGetApplicationResources(
        wshell,
        (&raw mut app_res).cast(),
        (&raw mut xrdb_resources).cast(),
        13,
        null_mut(),
        0,
    );
    if footer != 0 {
        wmain = XmVaCreateManagedRowColumn(
            wshell,
            c"main".as_ptr().cast_mut(),
            c"orientation".as_ptr(),
            if app_res.horizontal != 0 {
                2 as c_int
            } else {
                1 as c_int
            },
            null::<c_char>(),
        );
        create_utility_widgets(wmain);
        XtRealizeWidget(wshell);
        if app_res.system_tray != 0 {
            TrayStart();
        }
    } else {
        wmain = XmVaCreateManagedRowColumn(wshell, c"main".as_ptr().cast_mut(), null::<c_char>());
    }
}
#[no_mangle]
unsafe extern "C" fn toolbox_probe_shell() -> Widget {
    wshell
}
#[no_mangle]
unsafe extern "C" fn toolbox_probe_context() -> XtAppContext {
    app_context
}
#[no_mangle]
unsafe extern "C" fn toolbox_probe_menu() -> Widget {
    wmenu
}
#[no_mangle]
unsafe extern "C" fn toolbox_probe_clock() -> Widget {
    wdtframe
}
#[no_mangle]
unsafe extern "C" fn toolbox_probe_tray_enabled() -> Boolean {
    app_res.system_tray
}
#[no_mangle]
unsafe extern "C" fn toolbox_probe_clock_enabled() -> Boolean {
    app_res.show_date_time
}
#[no_mangle]
unsafe extern "C" fn toolbox_probe_set_config(path: *mut c_char) {
    rc_file_path = path;
}
#[no_mangle]
unsafe extern "C" fn toolbox_probe_applications(show: Boolean) {
    app_res.applications_menu = show;
}
#[no_mangle]
unsafe extern "C" fn toolbox_probe_construct_menu() -> Boolean {
    construct_menu()
}
