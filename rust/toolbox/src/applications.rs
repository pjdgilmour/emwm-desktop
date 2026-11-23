// SPDX-License-Identifier: MIT
//! Motif owns widgets; each application gadget retains its own GIO reference
//! until Xt destroys it. Detached submenu panes live in shared MenuShells.
use crate::tbmain::{
    Arg, Widget, WidgetClass, XmCreateCascadeButtonGadget, XmCreatePulldownMenu,
    XmCreatePushButtonGadget, XmCreateSeparatorGadget, XmMessageBoxGetChild, XmString,
    XmStringCreateLocalized, XmStringFree, XtAddCallback, XtDestroyWidget, XtManageChild,
    XtSetValues, XtUnmanageChild,
};
use crate::xdg::{tb_xdg_free, tb_xdg_launch, tb_xdg_load, Entry, Kind};
use std::ffi::{c_char, c_long, c_void, CStr, CString};
use std::ptr::{null, null_mut};

fn arg(name: &CStr, value: c_long) -> Arg {
    Arg {
        name: name.as_ptr().cast_mut(),
        value,
    }
}

#[link(name = "Xm")]
extern "C" {
    static xmCascadeButtonGadgetClass: WidgetClass;
    fn XmCreateErrorDialog(
        parent: Widget,
        name: *const c_char,
        args: *mut Arg,
        count: u32,
    ) -> Widget;
}

extern "C" {
    fn XtParent(w: Widget) -> Widget;
    fn XtGetValues(w: Widget, args: *mut Arg, count: u32);
    fn XtIsSubclass(w: Widget, class: WidgetClass) -> libc::c_char;
}

struct Label(XmString);
impl Label {
    fn new(utf8: *const c_char) -> Self {
        // SAFETY: the Garcon label is either null or NUL-terminated and valid
        // until Motif has copied it into an XmString. Free the GLib conversion.
        unsafe {
            let src = if utf8.is_null() { c"".as_ptr() } else { utf8 };
            let local = glib::ffi::g_locale_from_utf8(src, -1, null_mut(), null_mut(), null_mut());
            let result = Self(XmStringCreateLocalized(if local.is_null() {
                src.cast_mut()
            } else {
                local.cast()
            }));
            glib::ffi::g_free(local.cast());
            result
        }
    }
}
impl Drop for Label {
    fn drop(&mut self) {
        // SAFETY: XmStringCreateLocalized transfers an independent XmString.
        unsafe { XmStringFree(self.0) };
    }
}

struct Menu {
    menubar: Widget,
    pane: Widget,
    filename: Option<CString>,
}
unsafe extern "C" fn destroy_menu(_: Widget, data: *mut c_void, _: *mut c_void) {
    // SAFETY: single Xt destroy notification for the Box installed on this gadget.
    drop(Box::from_raw(data.cast::<Menu>()));
}
unsafe extern "C" fn destroy_app(_: Widget, data: *mut c_void, _: *mut c_void) {
    // SAFETY: balances the widget's reference, installed by populate().
    glib::gobject_ffi::g_object_unref(data.cast());
}
unsafe extern "C" fn dismiss(w: Widget, _: *mut c_void, _: *mut c_void) {
    // SAFETY: Xt invokes this before destroying the dialog.
    XtDestroyWidget(w);
}
unsafe extern "C" fn launch(w: Widget, data: *mut c_void, _: *mut c_void) {
    // SAFETY: Xt invokes this callback with the current live widget and its retained GIO object.
    let mut err = null_mut();
    let display = x11::xlib::XDisplayString(crate::wswitch::XtDisplayOfObject(w.cast()).cast());
    if tb_xdg_launch(data.cast(), display, &mut err) != 0 || err.is_null() {
        return;
    }
    let message = Label::new((*err).message);
    let mut shell = w;
    while !XtParent(shell).is_null() {
        shell = XtParent(shell);
    }
    let mut args = [arg(c"messageString", message.0 as _)];
    let dlg = XmCreateErrorDialog(shell, c"applicationError".as_ptr(), args.as_mut_ptr(), 1);
    const HELP: u8 = 7;
    const CANCEL: u8 = 2;
    for button in [HELP, CANCEL] {
        XtUnmanageChild(XmMessageBoxGetChild(dlg, button));
    }
    XtAddCallback(dlg, c"okCallback".as_ptr(), Some(dismiss), null_mut());
    XtManageChild(dlg);
    glib::ffi::g_error_free(err);
}

unsafe fn make_cascade(parent: Widget, text: *const c_char, pane: Widget) -> Widget {
    // SAFETY: Xt copies resources into the new gadget before this label is freed.
    let label = Label::new(text);
    let mut args = [
        arg(c"labelString", label.0 as _),
        arg(c"subMenuId", pane as _),
    ];
    let w = XmCreateCascadeButtonGadget(
        parent,
        c"cascadeButton".as_ptr().cast_mut(),
        args.as_mut_ptr(),
        2,
    );
    XtManageChild(w);
    w
}
unsafe fn button(parent: Widget, text: *const c_char, sensitive: bool) -> Widget {
    // SAFETY: parent is a live menu pane, and arguments are copied by Motif.
    let label = Label::new(text);
    let mut args = [arg(c"labelString", label.0 as _), arg(c"sensitive", 0)];
    let w = XmCreatePushButtonGadget(
        parent,
        c"menuButton".as_ptr().cast_mut(),
        args.as_mut_ptr(),
        if sensitive { 1 } else { 2 },
    );
    XtManageChild(w);
    w
}
unsafe fn populate(menu: Widget, tree: &Entry) {
    // SAFETY: children belong to tree until this recursive traversal completes.
    for &item in tree.items() {
        let entry = &*item;
        match entry.kind {
            Kind::Separator => XtManageChild(XmCreateSeparatorGadget(
                menu,
                c"separator".as_ptr().cast_mut(),
                null_mut(),
                0,
            )),
            Kind::Menu => {
                let pane = XmCreatePulldownMenu(
                    menu,
                    c"commandPulldown".as_ptr().cast_mut(),
                    null_mut(),
                    0,
                );
                populate(pane, entry);
                make_cascade(menu, entry.name, pane);
            }
            Kind::Application => {
                let w = button(menu, entry.name, true);
                let app = glib::gobject_ffi::g_object_ref(entry.app.cast());
                XtAddCallback(w, c"activateCallback".as_ptr(), Some(launch), app.cast());
                XtAddCallback(
                    w,
                    c"destroyCallback".as_ptr(),
                    Some(destroy_app),
                    app.cast(),
                );
            }
        }
    }
}
unsafe fn destroy_pane(pane: Widget) {
    // SAFETY: Xt returns the pane's children array. Copy submenu widget IDs first,
    // because destroying panes can re-enter Xt and mutate other widget lists.
    let mut children: *mut Widget = null_mut();
    let mut count = 0u32;
    let mut args = [
        arg(c"children", &mut children as *mut _ as _),
        arg(c"numChildren", &mut count as *mut _ as _),
    ];
    XtGetValues(pane, args.as_mut_ptr().cast(), 2);
    if count > 0 && !children.is_null() {
        let submenus: Vec<_> = std::slice::from_raw_parts(children, count as usize)
            .iter()
            .filter_map(|&w| {
                if XtIsSubclass(w, xmCascadeButtonGadgetClass) == 0 {
                    return None;
                }
                let mut child: Widget = null_mut();
                let mut arg = arg(c"subMenuId", &mut child as *mut _ as _);
                XtGetValues(w, (&mut arg as *mut Arg).cast(), 1);
                (!child.is_null()).then_some(child)
            })
            .collect();
        for child in submenus {
            destroy_pane(child);
        }
    }
    XtDestroyWidget(pane);
}
unsafe extern "C" fn refresh(cascade: Widget, data: *mut c_void, _: *mut c_void) {
    // SAFETY: cascading precedes posting and runs on the Xt thread. Swap the pane
    // before destroying the old one so the invoking gadget stays usable.
    let state = &mut *data.cast::<Menu>();
    let mut error = null_mut();
    let tree = tb_xdg_load(
        state.filename.as_deref().map_or(null(), CStr::as_ptr),
        &mut error,
    );
    let pane = XmCreatePulldownMenu(
        state.menubar,
        c"applicationsPulldown".as_ptr().cast_mut(),
        null_mut(),
        0,
    );
    if let Some(root) = tree.as_ref() {
        populate(pane, root);
        if root.items().is_empty() {
            button(pane, c"No applications found".as_ptr(), false);
        }
        tb_xdg_free(tree);
    } else {
        button(pane, c"Cannot load applications menu".as_ptr(), false);
        if !error.is_null() {
            eprintln!(
                "xmtoolbox: {}",
                CStr::from_ptr((*error).message).to_string_lossy()
            );
            glib::ffi::g_error_free(error);
        }
    }
    let old = std::mem::replace(&mut state.pane, pane);
    let mut arg = arg(c"subMenuId", pane as _);
    XtSetValues(cascade, (&mut arg as *mut Arg).cast(), 1);
    destroy_pane(old);
}
pub(super) unsafe fn tb_xdg_menu_create(
    menubar: Widget,
    title: *const c_char,
    filename: *const c_char,
) {
    // SAFETY: tbmain provides its live menubar and resource strings. Each gadget
    // has a separate state pointer released by its own destroy callback.
    let pane = XmCreatePulldownMenu(
        menubar,
        c"applicationsPulldown".as_ptr().cast_mut(),
        null_mut(),
        0,
    );
    button(pane, c"No applications found".as_ptr(), false);
    let cascade = make_cascade(menubar, title, pane);
    let state = Box::into_raw(Box::new(Menu {
        menubar,
        pane,
        filename: if filename.is_null() {
            None
        } else {
            Some(CStr::from_ptr(filename).to_owned())
        },
    }))
    .cast();
    XtAddCallback(cascade, c"cascadingCallback".as_ptr(), Some(refresh), state);
    XtAddCallback(
        cascade,
        c"destroyCallback".as_ptr(),
        Some(destroy_menu),
        state,
    );
}
