// SPDX-License-Identifier: MIT
use super::*;
use std::mem::zeroed;
const MAX_ICONS: usize = 128;

#[derive(Clone, Copy)]
struct Icon {
    window: Window,
    socket: Window,
    cmap: Colormap,
    mapped: bool,
}

pub(crate) struct Host {
    pub ui: Ui,
    pub d: *mut Display,
    root: Window,
    owner: Window,
    focused: Window,
    selection: Atom,
    opcode: Atom,
    embed: Atom,
    info: Atom,
    timestamp: Time,
    background: XColor,
    icons: Vec<Icon>,
}

impl Host {
    pub fn new(ui: Ui) -> Self {
        // SAFETY: shell has been initialized; an all-zero XColor is valid.
        unsafe {
            Self {
                d: xt::XtDisplay(ui.shell),
                ui,
                root: 0,
                owner: 0,
                focused: 0,
                selection: 0,
                opcode: 0,
                embed: 0,
                info: 0,
                timestamp: 0,
                background: zeroed(),
                icons: Vec::new(),
            }
        }
    }
    pub fn atom(&self, name: &CStr) -> Atom {
        // SAFETY: d is live and name is terminated.
        unsafe { XInternAtom(self.d, name.as_ptr(), 0) }
    }
    pub fn property(&self, w: Window, key: Atom, kind: Atom, value: c_ulong) {
        // SAFETY: format 32 expects an array of native unsigned longs in Xlib.
        unsafe {
            XChangeProperty(
                self.d,
                w,
                key,
                kind,
                32,
                PropModeReplace,
                (&value as *const c_ulong).cast(),
                1,
            )
        };
    }
    fn message(&self, window: Window, kind: Atom, mask: c_long, values: [c_long; 5]) {
        // SAFETY: only the ClientMessage variant is used. XSendEvent consumes
        // the message during the call.
        unsafe {
            let mut event: XEvent = zeroed();
            event.client_message.type_ = ClientMessage;
            event.client_message.window = window;
            event.client_message.message_type = kind;
            event.client_message.format = 32;
            event
                .client_message
                .data
                .as_longs_mut()
                .copy_from_slice(&values);
            XSendEvent(self.d, window, 0, mask, &mut event);
        }
    }
    fn notify(&self, w: Window, time: Time, op: i32, detail: Window) {
        self.message(w, self.embed, 0, [time as _, op as _, 0, detail as _, 0]);
    }
    fn lookup(&self, w: Window) -> Option<usize> {
        self.icons
            .iter()
            .position(|i| i.window == w || i.socket == w)
    }
    fn mapped(&self, window: Window) -> bool {
        let (mut kind, mut format, mut count, mut left, mut data) = (0, 0, 0, 0, null_mut());
        // SAFETY: Xlib allocates the result; check its type, format and count
        // before reading, and release it on every path.
        unsafe {
            let status = XGetWindowProperty(
                self.d,
                window,
                self.info,
                0,
                2,
                0,
                self.info,
                &mut kind,
                &mut format,
                &mut count,
                &mut left,
                &mut data,
            );
            let mapped = status != 0
                || kind != self.info
                || format != 32
                || count != 2
                || data.is_null()
                || *data.cast::<c_ulong>().add(1) & 1 != 0;
            if !data.is_null() {
                XFree(data.cast());
            }
            mapped
        }
    }
    fn configure(&self, icon: Icon) {
        let size = self.ui.settings.size;
        // SAFETY: icon belongs to the host. Xlib handles clients that disappear
        // asynchronously through our error handler.
        unsafe {
            XMoveResizeWindow(self.d, icon.window, 0, 0, size as _, size as _);
            let mut event: XEvent = zeroed();
            event.configure.type_ = ConfigureNotify;
            event.configure.event = icon.window;
            event.configure.window = icon.window;
            event.configure.width = size;
            event.configure.height = size;
            XSendEvent(self.d, icon.window, 0, StructureNotifyMask, &mut event);
        }
    }
    fn grid(&self) -> Grid {
        Grid::new(
            self.ui.embedded,
            self.icons.iter().filter(|i| i.mapped).count() as i32,
            self.ui.settings.size,
            self.ui.settings.spacing,
        )
    }
    pub fn position(&self) {
        let grid = self.grid();
        let width = self.ui.width();
        let mut slot = 0;
        for &icon in &self.icons {
            // SAFETY: XIDs are from the current host and Xt thread.
            unsafe {
                if !icon.mapped {
                    XUnmapWindow(self.d, icon.window);
                    XUnmapWindow(self.d, icon.socket);
                    continue;
                }
                let (x, y) = grid.position(width, slot);
                XMoveResizeWindow(self.d, icon.socket, x, y, grid.size as _, grid.size as _);
                self.configure(icon);
                XMapWindow(self.d, icon.window);
                XMapWindow(self.d, icon.socket);
            }
            slot += 1;
        }
    }
    pub fn layout(&self) {
        let grid = self.grid();
        let (width, height) = grid.dimensions();
        self.ui.set_grid_size(
            width.max(self.ui.settings.minimum_width),
            height,
            grid.visible != 0,
        );
        self.position();
    }
    fn remove(&mut self, index: usize, detach: bool) {
        let icon = self.icons.remove(index);
        // SAFETY: only registered drawables are removed. Save-set cleanup is
        // guarded because the foreign client may already have disconnected.
        unsafe {
            xt::XtUnregisterDrawable(self.d, icon.window);
            xt::XtUnregisterDrawable(self.d, icon.socket);
            trap(self.d, || {
                if detach {
                    XUnmapWindow(self.d, icon.window);
                    XReparentWindow(self.d, icon.window, self.root, 0, 0);
                }
                XRemoveFromSaveSet(self.d, icon.window);
                XDestroyWindow(self.d, icon.socket);
            });
            if icon.cmap != 0 {
                XFreeColormap(self.d, icon.cmap);
            }
        }
        if self.focused == icon.window {
            self.focused = 0;
        }
        self.layout();
    }
    fn dock(&mut self, w: Window) {
        if self.icons.len() >= MAX_ICONS
            || w == 0
            || w == self.root
            || w == self.owner
            || self.lookup(w).is_some()
        {
            return;
        }
        // SAFETY: Xt/Xlib are called on the toolkit thread. The client owns w.
        // Check its window attributes before using the visual/depth and unwind
        // every server resource acquired if it disappears while docking.
        unsafe {
            if !xt::XtWindowToWidget(self.d, w).is_null() {
                return;
            }
            let (icon, ok) = trap(self.d, || {
                let mut attrs: XWindowAttributes = zeroed();
                if XGetWindowAttributes(self.d, w, &mut attrs) == 0
                    || attrs.class != InputOutput
                    || attrs.root != self.root
                {
                    return None;
                }
                let mapped = self.mapped(w);
                let cmap = XCreateColormap(self.d, self.root, attrs.visual, AllocNone);
                let mut color = self.background;
                XAllocColor(self.d, cmap, &mut color);
                let format = xrender::XRenderFindVisualFormat(self.d, attrs.visual);
                if let Some(f) = format.as_ref() {
                    if f.type_ == xrender::PictTypeDirect && f.direct.alphaMask != 0 {
                        color.pixel |= (f.direct.alphaMask as c_ulong) << f.direct.alpha;
                    }
                }
                let mut wa: XSetWindowAttributes = zeroed();
                wa.colormap = cmap;
                wa.background_pixel = color.pixel;
                wa.event_mask = SubstructureRedirectMask | SubstructureNotifyMask;
                let pad = self.grid().pad;
                let size = self.ui.settings.size as u32;
                let socket = XCreateWindow(
                    self.d,
                    xt::XtWindow(self.ui.area),
                    pad,
                    pad,
                    size,
                    size,
                    0,
                    attrs.depth,
                    InputOutput as _,
                    attrs.visual,
                    CWColormap | CWBackPixel | CWBorderPixel | CWEventMask,
                    &mut wa,
                );
                XSelectInput(self.d, w, PropertyChangeMask | StructureNotifyMask);
                XAddToSaveSet(self.d, w);
                XSetWindowBorderWidth(self.d, w, 0);
                XReparentWindow(self.d, w, socket, 0, 0);
                Some(Icon {
                    window: w,
                    socket,
                    cmap,
                    mapped,
                })
            });
            let Some(icon) = icon else { return };
            if !ok {
                trap(self.d, || {
                    XReparentWindow(self.d, w, self.root, 0, 0);
                    XRemoveFromSaveSet(self.d, w);
                    if icon.socket != 0 {
                        XDestroyWindow(self.d, icon.socket);
                    }
                    if icon.cmap != 0 {
                        XFreeColormap(self.d, icon.cmap);
                    }
                });
                return;
            }
            self.icons.push(icon);
            xt::XtRegisterDrawable(self.d, w, self.ui.area);
            xt::XtRegisterDrawable(self.d, icon.socket, self.ui.area);
            self.layout();
            self.notify(w, self.timestamp, 0, icon.socket);
        }
    }
    fn focus(&mut self, index: usize, time: Time) {
        let icon = self.icons[index];
        if !icon.mapped {
            return;
        }
        if self.focused != 0 && self.focused != icon.window {
            self.notify(self.focused, time, 5, 0);
        }
        self.focused = icon.window;
        // SAFETY: a tracked mapped client is a valid focus target.
        unsafe { XSetInputFocus(self.d, self.focused, RevertToParent, time) };
        self.notify(self.focused, time, 1, 0);
        self.notify(self.focused, time, 4, 0);
    }
    fn client_message(&mut self, m: XClientMessageEvent) {
        if m.format != 32 {
            return;
        }
        let data = m.data.as_longs();
        let op = data[1];
        if m.window == self.owner && m.message_type == self.opcode && op == 0 {
            self.dock(data[2] as Window);
            return;
        }
        if m.message_type != self.embed {
            return;
        }
        let Some(i) = self.lookup(m.window) else {
            return;
        };
        let time = data[0] as Time;
        match op {
            3 => self.focus(i, time),
            6 | 7 => {
                let n = self.icons.len();
                for j in 1..=n {
                    let next = if op == 6 {
                        (i + j) % n
                    } else {
                        (i + n - j) % n
                    };
                    if self.icons[next].mapped {
                        self.focus(next, time);
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    #[expect(
        non_upper_case_globals,
        reason = "match the Xlib event names as published"
    )]
    pub fn event(&mut self, e: &XEvent) {
        // SAFETY: the event type determines which XEvent union member is valid.
        unsafe {
            match e.get_type() {
                SelectionClear if e.selection_clear.selection == self.selection => {
                    if self.ui.embedded {
                        self.release();
                        xt::XtUnmanageChild(self.ui.frame);
                    } else {
                        xt::XtAppSetExitFlag(xt::XtWidgetToApplicationContext(self.ui.shell));
                    }
                }
                ClientMessage => self.client_message(e.client_message),
                PropertyNotify if e.property.atom == self.info => {
                    if let Some(i) = self.lookup(e.property.window) {
                        self.icons[i].mapped = self.mapped(self.icons[i].window);
                        self.layout();
                    }
                }
                ConfigureRequest => {
                    if let Some(i) = self.lookup(e.configure_request.window) {
                        self.configure(self.icons[i])
                    }
                }
                MapRequest if self.lookup(e.map_request.window).is_some() => self.layout(),
                DestroyNotify => {
                    if let Some(i) = self.lookup(e.destroy_window.window) {
                        self.remove(i, false);
                    }
                }
                ReparentNotify => {
                    if let Some(i) = self.lookup(e.reparent.window) {
                        let icon = self.icons[i];
                        if icon.window == e.reparent.window && icon.socket != e.reparent.parent {
                            self.remove(i, false);
                        }
                    }
                }
                FocusIn | FocusOut
                    if e.focus_change.window == xt::XtWindow(self.ui.shell)
                        && e.focus_change.detail != NotifyInferior =>
                {
                    for i in &self.icons {
                        self.notify(
                            i.window,
                            CurrentTime,
                            if e.get_type() == FocusIn { 1 } else { 2 },
                            0,
                        );
                    }
                }
                _ => {}
            }
        }
    }
    pub fn claim(&mut self) -> bool {
        // SAFETY: shell and drawing area were realized before claiming. Use the
        // X server timestamp from the owner window to claim its screen selection.
        unsafe {
            let screen = XScreenNumberOfScreen(xt::XtScreen(self.ui.shell));
            self.root = XRootWindow(self.d, screen);
            PREVIOUS.set(XSetErrorHandler(Some(x_error)));
            self.background.pixel = self.ui.background();
            XQueryColor(
                self.d,
                XDefaultColormap(self.d, screen),
                &mut self.background,
            );
            self.selection = self.atom(&ui::selection_name(screen));
            self.opcode = self.atom(c"_NET_SYSTEM_TRAY_OPCODE");
            self.embed = self.atom(c"_XEMBED");
            self.info = self.atom(c"_XEMBED_INFO");
            self.owner = XCreateSimpleWindow(self.d, self.root, 0, 0, 1, 1, 0, 0, 0);
            XSelectInput(self.d, self.owner, PropertyChangeMask);
            self.property(self.owner, self.atom(c"_XMTRAY_TIMESTAMP"), XA_CARDINAL, 0);
            let mut ev: XEvent = zeroed();
            XWindowEvent(self.d, self.owner, PropertyChangeMask, &mut ev);
            self.timestamp = ev.property.time;
            self.property(
                self.owner,
                self.atom(c"_NET_SYSTEM_TRAY_ORIENTATION"),
                XA_CARDINAL,
                0,
            );
            self.property(
                self.owner,
                self.atom(c"_NET_SYSTEM_TRAY_VISUAL"),
                self.atom(c"VISUALID"),
                XVisualIDFromVisual(XDefaultVisual(self.d, screen)),
            );
            self.property(
                self.owner,
                self.atom(c"_NET_SYSTEM_TRAY_ICON_SIZE"),
                XA_CARDINAL,
                self.ui.settings.size as _,
            );
            XGrabServer(self.d);
            let occupied = XGetSelectionOwner(self.d, self.selection) != 0;
            if !occupied {
                XSetSelectionOwner(self.d, self.selection, self.owner, self.timestamp);
            }
            XUngrabServer(self.d);
            if occupied || XGetSelectionOwner(self.d, self.selection) != self.owner {
                XDestroyWindow(self.d, self.owner);
                self.owner = 0;
                XSetErrorHandler(PREVIOUS.replace(None));
                if occupied {
                    eprintln!("xmtray: another tray already owns this screen");
                }
                return false;
            }
            xt::XtRegisterDrawable(self.d, self.owner, self.ui.area);
            self.message(
                self.root,
                self.atom(c"MANAGER"),
                StructureNotifyMask,
                [
                    self.timestamp as _,
                    self.selection as _,
                    self.owner as _,
                    0,
                    0,
                ],
            );
            XFlush(self.d);
            true
        }
    }
    pub fn release(&mut self) {
        if self.owner == 0 {
            return;
        }
        while !self.icons.is_empty() {
            self.remove(self.icons.len() - 1, true);
        }
        // SAFETY: this host registered and created the owner drawable.
        unsafe {
            xt::XtUnregisterDrawable(self.d, self.owner);
            XDestroyWindow(self.d, self.owner);
            XSetErrorHandler(PREVIOUS.replace(None));
        }
        self.owner = 0;
    }
}

struct Grid {
    visible: i32,
    columns: i32,
    pad: i32,
    size: i32,
    spacing: i32,
}
impl Grid {
    fn new(embedded: bool, visible: i32, size: i32, spacing: i32) -> Self {
        Self {
            visible,
            columns: if embedded { 4 } else { visible.max(1) },
            pad: if embedded { 3 } else { 6 },
            size,
            spacing,
        }
    }
    fn span(&self, slots: i32) -> i32 {
        slots * self.size + (slots - 1) * self.spacing
    }
    fn dimensions(&self) -> (i32, i32) {
        let used = self.visible.min(self.columns).max(1);
        let rows = ((self.visible + self.columns - 1) / self.columns).max(1);
        (
            2 * self.pad + self.span(used),
            2 * self.pad + self.span(rows),
        )
    }
    fn position(&self, width: i32, slot: i32) -> (i32, i32) {
        let row = slot / self.columns;
        let occupied = (self.visible - row * self.columns).min(self.columns);
        let left = ((width - self.span(occupied)) / 2).max(self.pad);
        (
            left + (slot % self.columns) * (self.size + self.spacing),
            self.pad + row * (self.size + self.spacing),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn incomplete_row_is_centered() {
        let grid = Grid::new(true, 5, 24, 4);
        assert_eq!(grid.dimensions(), (114, 58));
        assert_eq!(grid.position(114, 0), (3, 3));
        assert_eq!(grid.position(114, 4), (45, 31));
        assert_eq!(Grid::new(false, 0, 24, 4).dimensions(), (36, 36));
    }
}
