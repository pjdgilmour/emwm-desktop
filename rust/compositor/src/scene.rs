// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    effects::Effect,
    overview::{Mode, Overview},
    renderer::Renderer,
    screen::{Screen, RELOAD, STOP},
    settings::Settings,
    shortcuts::Shortcuts,
    surface::Surface,
};
use std::{
    collections::{hash_map::Entry, HashMap},
    fs, mem,
    path::PathBuf,
    sync::atomic::Ordering,
    time::Instant,
};
use x11::{
    keysym::*,
    xlib::{self, *},
    xrandr::XRRUpdateConfiguration,
};

pub struct Scene {
    // Drop order matters: all images before the GL context and the X connection.
    view: Option<Overview>,
    keys: Shortcuts,
    surfaces: HashMap<Window, Surface>,
    renderer: Option<Renderer>,
    x: Screen,
    order: Vec<Window>,
    settings: Settings,
    config: PathBuf,
    backend: String,
    dirty: bool,
    stack_dirty: bool,
}
impl Scene {
    pub fn start(config: PathBuf, backend: String) -> Result<(), String> {
        let x = Screen::open()?;
        let settings = Settings::read(&config, &backend);
        let mut scene = Self {
            view: None,
            keys: unsafe { Shortcuts::start(&x) },
            surfaces: HashMap::new(),
            renderer: None,
            x,
            order: Vec::new(),
            settings,
            config,
            backend,
            dirty: true,
            stack_dirty: true,
        };
        // SAFETY: scene owns the only thread using x, and tears down the display last.
        unsafe {
            scene.configure(false)?;
            scene.stack();
            let result = scene.run();
            scene.close_view();
            result
        }
    }
    unsafe fn configure(&mut self, fallback: bool) -> Result<(), String> {
        self.cancel();
        for s in self.surfaces.values_mut() {
            s.image = None;
            s.dirty = true;
        }
        self.close_view();
        self.renderer.take();
        let x = &self.x;
        let backend = if fallback {
            "xrender"
        } else {
            &self.settings.backend
        };
        let renderer = match Renderer::open(x.d, x.screen, x.overlay, backend, self.settings.vsync)
        {
            Ok(r) => r,
            Err(e) if backend != "xrender" => {
                if e != "requested backend failed" {
                    eprintln!("emwm-compositor: initialization failed: {e}");
                }
                eprintln!("emwm-compositor: requested backend failed; restoring XRender");
                Renderer::open(x.d, x.screen, x.overlay, "xrender", false)?
            }
            Err(e) => return Err(e),
        };
        x.backend(renderer.accelerated());
        self.renderer = Some(renderer);
        self.stack_dirty = true;
        self.dirty = true;
        Ok(())
    }
    unsafe fn reload(&mut self) -> Result<(), String> {
        let settings = Settings::read(&self.config, &self.backend);
        let rebuild =
            settings.backend != self.settings.backend || settings.vsync != self.settings.vsync;
        self.settings = settings;
        if rebuild {
            self.configure(false)?;
        }
        if self.settings.effect == Effect::Disabled {
            self.cancel();
        }
        self.dirty = true;
        Ok(())
    }
    fn cancel(&mut self) {
        for s in self.surfaces.values_mut() {
            s.animation = None;
            s.appeared = None;
        }
    }
    unsafe fn add(&mut self, w: Window) {
        if w == self.x.owner
            || w == self.x.input
            || w == self.x.overlay
            || self.renderer.as_ref().is_some_and(|r| w == r.output())
        {
            return;
        }
        if let Entry::Vacant(entry) = self.surfaces.entry(w) {
            if let Some(s) = Surface::add(&self.x, w) {
                entry.insert(s);
                self.stack_dirty = true;
                self.dirty = true;
            }
        }
    }
    unsafe fn stack(&mut self) {
        self.order.clear();
        for w in self.x.children() {
            self.add(w);
            if self.surfaces.contains_key(&w) {
                self.order.push(w);
            }
        }
        self.stack_dirty = false;
        if let Some(view) = &mut self.view {
            view.stale = true;
        }
    }
    unsafe fn close_view(&mut self) {
        if let Some(view) = self.view.take() {
            view.close(&self.x);
            self.dirty = true;
        }
    }

    unsafe fn choose(&mut self, time: Time) {
        if let Some(view) = self.view.take() {
            view.close(&self.x);
            view.activate(&self.x, &self.surfaces, time);
            self.dirty = true;
        }
    }

    unsafe fn open_view(&mut self, mode: Mode, t: f64) {
        self.view = Overview::open(&self.x, mode, t);
        self.sync_view();
    }

    unsafe fn sync_view(&mut self) {
        if self.stack_dirty {
            self.stack();
        }
        if let Some(view) = self.view.as_mut().filter(|v| v.stale) {
            for s in self.surfaces.values_mut().filter(|s| s.metadata_dirty) {
                s.identify(&self.x);
            }
            view.sync(&self.x, &self.order, &self.surfaces);
        }
    }

    #[allow(non_upper_case_globals)] // X11 event types and keysyms
    unsafe fn input(&mut self, e: &mut XEvent, t: f64) -> bool {
        match e.get_type() {
            GenericEvent => {
                if self.keys.tapped(&self.x, &mut e.generic_event_cookie) {
                    if self.view.is_some() {
                        self.close_view();
                    } else {
                        self.open_view(Mode::Workspaces, t);
                    }
                    self.dirty = true;
                }
            }
            MappingNotify => {
                XRefreshKeyboardMapping(&mut e.mapping);
                self.keys.remap(&self.x);
            }
            KeyPress => {
                let key = XLookupKeysym(&mut e.key, 0) as u32;
                if key == XK_Tab && e.key.state & self.keys.super_mask != 0 {
                    if !self.keys.first_tab_press() {
                        return true;
                    }
                    let already_open = self.view.as_ref().is_some_and(|v| v.mode == Mode::Windows);
                    self.close_view();
                    if !already_open {
                        self.open_view(Mode::Windows, t);
                    }
                } else if let Some(view) = &mut self.view {
                    match key {
                        XK_Return | XK_KP_Enter => {
                            self.choose(e.key.time);
                        }
                        XK_Escape => self.close_view(),
                        _ => view.navigate(key),
                    }
                }
                self.dirty = true;
            }
            FocusOut if e.focus_change.window == self.x.input => {
                if e.focus_change.mode == NotifyGrab {
                    // A locker, menu or modal dialog has taken the keyboard.
                    self.close_view();
                } else if e.focus_change.mode == NotifyNormal {
                    if let Some(view) = &mut self.view {
                        view.refocus(&self.x);
                    }
                }
            }
            KeyRelease => {
                self.keys.released(e.key.keycode);
            }
            ButtonPress | ButtonRelease => {
                if let Some(view) = &mut self.view {
                    let b = e.button;
                    match b.button {
                        1 if b.type_ == ButtonRelease => {
                            if view.hit(b.x_root, b.y_root).is_some() {
                                view.hover(b.x_root, b.y_root);
                                self.choose(b.time);
                            } else {
                                self.close_view();
                            }
                        }
                        4 | 5
                            if b.type_ == ButtonPress
                                && (b.state & self.keys.super_mask != 0
                                    || view.mode == Mode::Workspaces) =>
                        {
                            view.scroll(&self.x, if b.button == 4 { -1 } else { 1 }, b.time);
                        }
                        _ => {}
                    }
                    self.dirty = true;
                }
            }
            MotionNotify => {
                if let Some(view) = &mut self.view {
                    view.hover(e.motion.x_root, e.motion.y_root);
                    self.dirty = true;
                }
            }
            _ => return false,
        }
        true
    }
    unsafe fn events(&mut self, e: &mut XEvent, t: f64) {
        if self.input(e, t) {
            return;
        }
        let ty = e.get_type();
        if ty == self.x.damage_event || ty == self.x.shape_event {
            // Both notifications have XAnyEvent's common header, followed by drawable/window.
            if let Some(s) = self.surfaces.get_mut(&e.any.window) {
                if ty == self.x.damage_event {
                    s.dirty = true;
                } else {
                    s.shape_dirty = true;
                    s.metadata_dirty = true;
                }
                self.dirty = true;
            }
            return;
        }
        if self.x.randr_event == Some(ty) {
            XRRUpdateConfiguration(e);
            let (w, h) = self.x.dimensions();
            if let Some(view) = &mut self.view {
                view.resize(&self.x);
            }
            if let Some(r) = &mut self.renderer {
                r.resize(w, h);
            }
            self.cancel();
            self.dirty = true;
            return;
        }
        match ty {
            xlib::CreateNotify => {
                if e.create_window.parent == self.x.root {
                    self.add(e.create_window.window);
                }
            }
            xlib::ReparentNotify => {
                if e.reparent.parent == self.x.root {
                    self.add(e.reparent.window);
                } else {
                    self.surfaces.remove(&e.reparent.window);
                }
                self.stack_dirty = true;
                self.dirty = true;
            }
            xlib::MapNotify => {
                let w = e.map.window;
                self.add(w);
                if let Some(s) = self.surfaces.get_mut(&w) {
                    s.mapped = true;
                    s.image = None;
                    s.dirty = true;
                    if let Some(attr) = self.x.attrs(w) {
                        s.attr = attr;
                    }
                    s.update_shape(&self.x);
                    s.identify(&self.x);
                    if s.managed && !s.desktop && s.animation.is_none() {
                        s.appeared = Some(t);
                    }
                    self.stack_dirty = true;
                    self.dirty = true;
                }
            }
            xlib::UnmapNotify => {
                if let Some(s) = self.surfaces.get_mut(&e.unmap.window) {
                    s.mapped = false;
                    // The named pixmap remains valid after unmap. Keep the last
                    // frame for windows on hidden workspaces and minimized apps.
                    let (w, h) = self.x.dimensions();
                    let backdrop =
                        s.attr.override_redirect != 0 && s.attr.width == w && s.attr.height == h;
                    if !s.application && !backdrop {
                        s.image = None;
                    }
                    if let Some(view) = &mut self.view {
                        view.stale = true;
                    }
                    self.dirty = true;
                }
            }
            xlib::DestroyNotify => {
                if self.surfaces.remove(&e.destroy_window.window).is_some() {
                    self.dirty = true;
                    self.stack_dirty = true;
                }
            }
            xlib::ConfigureNotify => {
                let c = e.configure;
                if let Some(view) = &mut self.view {
                    view.stale = true;
                }
                if c.window == self.x.root {
                    if let Some(r) = &mut self.renderer {
                        r.resize(c.width, c.height);
                    }
                    self.cancel();
                    self.dirty = true;
                } else if let Some(s) = self.surfaces.get_mut(&c.window) {
                    if (s.attr.width, s.attr.height, s.attr.border_width)
                        != (c.width, c.height, c.border_width)
                    {
                        s.image = None;
                        s.dirty = true;
                        s.shape_dirty = true;
                        s.metadata_dirty = true;
                    }
                    (
                        s.attr.x,
                        s.attr.y,
                        s.attr.width,
                        s.attr.height,
                        s.attr.border_width,
                    ) = (c.x, c.y, c.width, c.height, c.border_width);
                    if s.above != Some(c.above) {
                        self.stack_dirty = true;
                    }
                    s.above = Some(c.above);
                    self.dirty = true;
                }
            }
            xlib::CirculateNotify => {
                self.stack_dirty = true;
                self.dirty = true;
            }
            xlib::PropertyNotify => {
                let p = e.property;
                let a = &self.x.atoms;
                if [
                    a.current_desktop,
                    a.desktop_names,
                    a.workspace_list,
                    a.presence,
                    a.client,
                    a.window_type,
                    a.wm_state,
                    a.state,
                    a.wm_name,
                    XA_WM_NAME,
                    a.wm_info,
                ]
                .contains(&p.atom)
                {
                    if let Some(view) = &mut self.view {
                        view.stale = true;
                        self.dirty = true;
                    }
                }
                if p.window == self.x.root {
                    if p.atom == a.current_desktop {
                        self.cancel();
                        self.dirty = true;
                    }
                } else if [
                    a.client,
                    a.window_type,
                    a.opacity,
                    a.state,
                    a.wm_state,
                    a.presence,
                ]
                .contains(&p.atom)
                {
                    for s in self
                        .surfaces
                        .values_mut()
                        .filter(|s| s.id == p.window || s.client == p.window)
                    {
                        s.metadata_dirty = true;
                        self.dirty = true;
                    }
                }
            }
            xlib::ClientMessage => {
                let msg = &e.client_message;
                if msg.message_type == self.x.atoms.animation
                    && msg.format == 32
                    && self.settings.effect != Effect::Disabled
                {
                    if let Some(s) = self.surfaces.get_mut(&(msg.data.get_long(0) as Window)) {
                        s.transition(&self.x, msg, &self.settings, t);
                        self.dirty = true;
                    }
                }
            }
            xlib::SelectionClear => {
                if e.selection_clear.selection == self.x.selection {
                    STOP.store(true, Ordering::Relaxed);
                }
            }
            xlib::Expose => self.dirty = true,
            _ => {}
        }
    }
    unsafe fn paint(&mut self, t: f64) -> Result<bool, String> {
        self.sync_view();
        let r = self.renderer.as_mut().ok_or("Cannot initialize renderer")?;
        r.begin();
        if let Some(view) = &mut self.view {
            let duration = if self.settings.effect == Effect::Disabled {
                0.
            } else {
                self.settings.duration as f64 / 1000.
            };
            let active = view.paint(&self.x, r, &mut self.surfaces, t, duration)?;
            r.present();
            self.dirty = false;
            return Ok(active);
        }
        let mut active = false;
        for w in &self.order {
            if let Some(s) = self.surfaces.get_mut(w) {
                active |= s.paint(&self.x, r, &self.settings, t)?;
            }
        }
        r.present();
        self.dirty = false;
        Ok(active)
    }
    unsafe fn run(&mut self) -> Result<(), String> {
        let start = Instant::now();
        let (mut last_config, mut last_paint) = (-1., -1.);
        let mut stamp = None;
        let mut active = false;
        let d = self.x.d;
        while !STOP.load(Ordering::Relaxed) {
            for _ in 0..2048 {
                if XPending(d) == 0 {
                    break;
                }
                let mut e = mem::zeroed();
                XNextEvent(d, &mut e);
                self.events(&mut e, start.elapsed().as_secs_f64());
            }
            let t = start.elapsed().as_secs_f64();
            let requested = RELOAD.swap(false, Ordering::Relaxed);
            if requested || t - last_config >= 1. {
                let modified = fs::metadata(&self.config).and_then(|m| m.modified()).ok();
                if requested || (modified.is_some() && modified != stamp) {
                    self.reload()?;
                    stamp = modified;
                }
                last_config = t;
            }
            let interval = 1. / f64::from(self.settings.fps);
            if (self.dirty || active) && t - last_paint >= interval {
                match self.paint(t) {
                    Ok(a) => active = a,
                    Err(e) if self.renderer.as_ref().is_some_and(Renderer::accelerated) => {
                        eprintln!("emwm-compositor: {e}; switching to XRender");
                        self.configure(true)?;
                        active = false;
                    }
                    Err(e) => return Err(e),
                }
                last_paint = t;
            }
            if XPending(d) == 0 {
                let timeout = if self.dirty || active {
                    (((interval - (start.elapsed().as_secs_f64() - last_paint)) * 1000.) as i32)
                        .max(1)
                } else {
                    500
                };
                let mut poll = libc::pollfd {
                    fd: XConnectionNumber(d),
                    events: libc::POLLIN,
                    revents: 0,
                };
                libc::poll(&mut poll, 1, timeout);
            }
        }
        Ok(())
    }
}
