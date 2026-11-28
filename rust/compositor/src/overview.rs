// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    effects::{ease, Rect},
    renderer::{Image, Renderer},
    screen::Screen,
    surface::Surface,
};
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    rc::Rc,
};
use x11::{keysym::*, xlib::*, xrandr::*};

const CAPTION_HEIGHT: f64 = 28.;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Workspaces,
    Windows,
}

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Workspace(Atom),
    Window(Window),
    Empty,
}

struct Card {
    target: Target,
    bounds: Rect,
    visible_bounds: Rect,
    title: String,
    label: Option<Rc<Image>>,
    background: u32,
    layers: Vec<Window>,
}

impl Card {
    fn new(target: Target, title: String, background: u32, layers: Vec<Window>) -> Self {
        Self {
            target,
            title,
            background,
            layers,
            bounds: Rect::default(),
            visible_bounds: Rect::default(),
            label: None,
        }
    }
}

pub struct Overview {
    pub mode: Mode,
    pub stale: bool,
    viewport: Rect,
    cards: Vec<Card>,
    workspaces: Vec<Atom>,
    current: usize,
    selected: usize,
    cols: usize,
    opened: f64,
    previous_focus: Window,
    revert: i32,
    pending: Option<usize>,
}

fn fit(area: Rect, width: f64, height: f64) -> Rect {
    let scale = (area.w / width.max(1.))
        .min(area.h / height.max(1.))
        .min(1.);
    let (w, h) = (width * scale, height * scale);
    Rect::new(area.x + (area.w - w) / 2., area.y + (area.h - h) / 2., w, h)
}

fn contains(r: Rect, x: i32, y: i32) -> bool {
    let (x, y) = (x as f64, y as f64);
    r.x <= x && x < r.x + r.w && r.y <= y && y < r.y + r.h
}

impl Overview {
    pub unsafe fn open(x: &Screen, mode: Mode, t: f64) -> Option<Self> {
        if x.property(x.root, x.atoms.wm_info, x.atoms.wm_info).len() < 2 {
            return None;
        }
        if XGrabKeyboard(
            x.d,
            x.root,
            False,
            GrabModeAsync,
            GrabModeAsync,
            CurrentTime,
        ) != GrabSuccess
        {
            return None;
        }
        if XGrabPointer(
            x.d,
            x.root,
            False,
            (ButtonPressMask | ButtonReleaseMask | PointerMotionMask) as _,
            GrabModeAsync,
            GrabModeAsync,
            0,
            0,
            CurrentTime,
        ) != GrabSuccess
        {
            XUngrabKeyboard(x.d, CurrentTime);
            return None;
        }
        // Check for another modal owner before raising the input window. Release
        // both probes immediately, including when the overview stays open.
        let (mut previous_focus, mut revert) = (0, 0);
        XGetInputFocus(x.d, &mut previous_focus, &mut revert);
        let (w, h) = x.dimensions();
        XResizeWindow(x.d, x.input, w as _, h as _);
        XMapRaised(x.d, x.input);
        XSetInputFocus(x.d, x.input, RevertToPointerRoot, CurrentTime);
        XUngrabPointer(x.d, CurrentTime);
        XUngrabKeyboard(x.d, CurrentTime);
        Some(Self {
            previous_focus,
            revert,
            mode,
            viewport: monitor_at_pointer(x),
            cards: vec![],
            workspaces: vec![],
            current: 0,
            selected: 0,
            cols: 1,
            stale: true,
            opened: t,
            pending: None,
        })
    }

    pub unsafe fn close(&self, x: &Screen) {
        let (mut focus, mut revert) = (0, 0);
        XGetInputFocus(x.d, &mut focus, &mut revert);
        if focus == x.input {
            let previous = self.previous_focus;
            let visible = previous <= PointerRoot as Window
                || x.attrs(previous).is_some_and(|a| a.map_state == IsViewable);
            XSetInputFocus(
                x.d,
                if visible {
                    previous
                } else {
                    PointerRoot as Window
                },
                self.revert,
                CurrentTime,
            );
        }
        XUnmapWindow(x.d, x.input);
    }

    pub unsafe fn refocus(&mut self, x: &Screen) {
        let (mut focus, mut revert) = (0, 0);
        XGetInputFocus(x.d, &mut focus, &mut revert);
        if focus != x.input {
            self.previous_focus = focus;
            self.revert = revert;
            XRaiseWindow(x.d, x.input);
            XSetInputFocus(x.d, x.input, RevertToPointerRoot, CurrentTime);
        }
    }

    pub unsafe fn resize(&mut self, x: &Screen) {
        let (w, h) = x.dimensions();
        XResizeWindow(x.d, x.input, w as _, h as _);
        self.viewport = monitor_at_pointer(x);
        self.stale = true;
    }

    pub unsafe fn sync(
        &mut self,
        x: &Screen,
        order: &[Window],
        surfaces: &HashMap<Window, Surface>,
    ) {
        let a = &x.atoms;
        let wm = x.property(x.root, a.wm_info, a.wm_info);
        self.workspaces = wm
            .get(1)
            .map(|&w| x.property(w, a.workspace_list, XA_ATOM))
            .unwrap_or_default();
        let previous = self.cards.get(self.selected).map(|c| c.target);
        let current = x
            .property(x.root, a.current_desktop, XA_CARDINAL)
            .first()
            .copied()
            .unwrap_or(0) as usize;
        let switched = self.current != current;
        self.current = current;
        if self.pending == Some(current) || self.pending.is_some_and(|i| i >= self.workspaces.len())
        {
            self.pending = None;
        }
        let mut cards = Vec::new();
        let names = x.strings(x.root, a.desktop_names);
        match self.mode {
            Mode::Workspaces => {
                for (i, &id) in self.workspaces.iter().enumerate() {
                    let (color, backdrop) = backdrop(x, wm[1], id);
                    let mut layers = Vec::new();
                    if backdrop != 0 {
                        layers.push(backdrop);
                    }
                    for &w in order {
                        let s = &surfaces[&w];
                        if s.desktop && s.mapped && i == current
                            || s.application
                                && !s.minimized
                                && (s.workspaces.contains(&id)
                                    || s.workspaces.contains(&a.all_workspaces)
                                    || s.workspaces.is_empty() && s.mapped && i == current)
                        {
                            layers.push(w);
                        }
                    }
                    let title = names
                        .get(i)
                        .filter(|n| !n.is_empty())
                        .cloned()
                        .unwrap_or_else(|| format!("Espaço {}", i + 1));
                    cards.push(Card::new(Target::Workspace(id), title, color, layers));
                }
            }
            Mode::Windows => {
                for &w in order.iter().rev() {
                    let s = &surfaces[&w];
                    if !s.application {
                        continue;
                    }
                    let title = [a.wm_name, XA_WM_NAME]
                        .into_iter()
                        .find_map(|prop| {
                            x.strings(s.client, prop)
                                .into_iter()
                                .find(|s| !s.is_empty())
                        })
                        .unwrap_or_else(|| "Janela sem título".into());
                    cards.push(Card::new(
                        Target::Window(s.client),
                        title,
                        0x354352,
                        vec![w],
                    ));
                }
                if cards.is_empty() {
                    cards.push(Card::new(
                        Target::Empty,
                        "Nenhuma janela aberta".into(),
                        0x354352,
                        vec![],
                    ));
                }
            }
        }
        let (width, height) = x.dimensions();
        let (w, h) = (width as f64, height as f64);
        self.cols = (cards.len() as f64).sqrt().ceil().max(1.) as usize;
        let rows = cards.len().div_ceil(self.cols).max(1);
        let area = self.viewport;
        let (cw, ch) = (area.w / self.cols as f64, area.h / rows as f64);
        let gap = (cw.min(ch) * 0.075).min(36.);
        let count = cards.len();
        for (i, card) in cards.iter_mut().enumerate() {
            let row = i / self.cols;
            let in_row = (count - row * self.cols).min(self.cols);
            let cell = Rect::new(
                area.x + (area.w - cw * in_row as f64) / 2. + cw * (i % self.cols) as f64 + gap,
                area.y + ch * row as f64 + gap,
                (cw - 2. * gap).max(1.),
                (ch - 2. * gap - CAPTION_HEIGHT).max(1.),
            );
            let (aspect_w, aspect_h) = match card.target {
                Target::Window(_) => {
                    let g = surfaces[&card.layers[0]].geometry();
                    (g.w, g.h)
                }
                Target::Empty => (300., 50.),
                _ => (w, h),
            };
            let preview = fit(cell, aspect_w, aspect_h);
            card.bounds = Rect::new(preview.x, preview.y, preview.w, preview.h + CAPTION_HEIGHT);
            card.visible_bounds = card.bounds;
        }
        let active = x
            .property(x.root, a.active_window, XA_WINDOW)
            .first()
            .copied()
            .unwrap_or(0);
        self.selected = match self.mode {
            Mode::Workspaces if previous.is_none() || switched => self.pending.unwrap_or(current),
            _ => cards
                .iter()
                .position(|c| Some(c.target) == previous)
                .or_else(|| {
                    cards
                        .iter()
                        .position(|c| c.target == Target::Window(active))
                })
                .unwrap_or(0),
        }
        .min(cards.len().saturating_sub(1));
        self.cards = cards;
        self.stale = false;
    }

    pub unsafe fn paint(
        &mut self,
        x: &Screen,
        r: &mut Renderer,
        surfaces: &mut HashMap<Window, Surface>,
        t: f64,
        duration: f64,
    ) -> Result<bool, String> {
        let (width, height) = x.dimensions();
        let screen = Rect::new(0., 0., width as f64, height as f64);
        let p = if duration > 0. {
            ease((t - self.opened) / duration)
        } else {
            1.
        };
        r.fill(screen, 0x161c23);
        let active = self
            .cards
            .get(self.current)
            .map(|c| c.bounds)
            .unwrap_or(screen);
        for (i, card) in self.cards.iter_mut().enumerate() {
            let b = card.bounds;
            let end = Rect::new(b.x, b.y, b.w, b.h - CAPTION_HEIGHT);
            let start = match card.target {
                Target::Workspace(_) => {
                    let scale = screen.w / active.w.max(1.);
                    Rect::new(
                        (end.x - active.x) * scale,
                        (end.y - active.y) * scale,
                        end.w * scale,
                        end.h * scale,
                    )
                }
                Target::Window(_) => surfaces
                    .get(&card.layers[0])
                    .filter(|s| s.mapped)
                    .map_or(end, Surface::geometry),
                Target::Empty => end,
            };
            let rect = start.interpolate(end, p);
            card.visible_bounds = Rect::new(rect.x, rect.y, rect.w, rect.h + CAPTION_HEIGHT * p);
            if rect.x + rect.w <= 0.
                || rect.y + rect.h <= 0.
                || rect.x >= screen.w
                || rect.y >= screen.h
            {
                continue;
            }
            if i == self.selected {
                r.fill(
                    Rect::new(
                        rect.x - 3.,
                        rect.y - 3.,
                        rect.w + 6.,
                        rect.h + 6. + CAPTION_HEIGHT * p,
                    ),
                    0x92bcde,
                );
            }
            r.fill(rect, card.background);
            for id in &card.layers {
                let Some(s) = surfaces.get_mut(id) else {
                    continue;
                };
                if s.mapped {
                    s.capture(x, r)?;
                }
                let g = s.geometry();
                let dst = if self.mode == Mode::Workspaces {
                    Rect::new(
                        rect.x + g.x * rect.w / screen.w,
                        rect.y + g.y * rect.h / screen.h,
                        g.w * rect.w / screen.w,
                        g.h * rect.h / screen.h,
                    )
                } else {
                    rect
                };
                // A window may straddle the root edge. Clip its source pixels to this workspace.
                let clip = XRectangle {
                    x: (-g.x).max(0.) as _,
                    y: (-g.y).max(0.) as _,
                    width: ((g.x + g.w).min(screen.w) - g.x.max(0.)).max(0.) as _,
                    height: ((g.y + g.h).min(screen.h) - g.y.max(0.)).max(0.) as _,
                };
                if let Some(image) = &s.image {
                    let shape =
                        (self.mode == Mode::Workspaces).then_some(std::slice::from_ref(&clip));
                    r.draw(image, dst, 1., shape);
                }
            }
            if p > 0. {
                if card.label.is_none() {
                    card.label = Some(r.caption(&card.title, end.w.round() as i32)?);
                }
                r.draw(
                    card.label.as_ref().unwrap(),
                    Rect::new(rect.x, rect.y + rect.h, rect.w, CAPTION_HEIGHT * p),
                    p,
                    None,
                );
            }
        }
        Ok(p < 1.)
    }

    pub fn hit(&self, x: i32, y: i32) -> Option<usize> {
        self.cards
            .iter()
            .rposition(|c| contains(c.visible_bounds, x, y))
    }
    pub fn hover(&mut self, x: i32, y: i32) {
        if let Some(i) = self.hit(x, y) {
            self.selected = i;
        }
    }
    #[allow(non_upper_case_globals)] // X11 keysym names
    pub fn navigate(&mut self, key: u32) {
        let count = self.cards.len() as isize;
        if count == 0 {
            return;
        }
        let step = match key {
            XK_Left => -1,
            XK_Right | XK_Tab => 1,
            XK_Up => -(self.cols as isize),
            XK_Down => self.cols as isize,
            _ => return,
        };
        self.selected = (self.selected as isize + step).rem_euclid(count) as usize;
    }
    pub unsafe fn scroll(&mut self, x: &Screen, step: isize, time: Time) {
        let n = self.workspaces.len();
        if n == 0 {
            return;
        }
        let i =
            (self.pending.unwrap_or(self.current) as isize + step).rem_euclid(n as isize) as usize;
        self.pending = Some(i);
        if self.mode == Mode::Workspaces {
            self.selected = i;
        }
        x.request(x.root, x.atoms.current_desktop, i, time);
    }
    pub unsafe fn activate(&self, x: &Screen, surfaces: &HashMap<Window, Surface>, time: Time) {
        let Some(card) = self.cards.get(self.selected) else {
            return;
        };
        match card.target {
            Target::Workspace(_) => x.request(x.root, x.atoms.current_desktop, self.selected, time),
            Target::Window(client) => {
                let Some(s) = surfaces.get(&card.layers[0]) else {
                    return;
                };
                if !s.workspaces.contains(&x.atoms.all_workspaces)
                    && !self
                        .workspaces
                        .get(self.current)
                        .is_some_and(|ws| s.workspaces.contains(ws))
                {
                    if let Some(i) = self
                        .workspaces
                        .iter()
                        .position(|ws| s.workspaces.contains(ws))
                    {
                        x.request(x.root, x.atoms.current_desktop, i, time);
                    }
                }
                x.request(client, x.atoms.active_window, 2, time);
            }
            Target::Empty => {}
        }
    }
}

unsafe fn backdrop(x: &Screen, wm: Window, id: Atom) -> (u32, Window) {
    let name = XGetAtomName(x.d, id);
    if name.is_null() {
        return (0x354352, 0);
    }
    let key =
        CString::new([b"_MWM_WORKSPACE_INFO_", CStr::from_ptr(name).to_bytes()].concat()).unwrap();
    XFree(name.cast());
    let prop = XInternAtom(x.d, key.as_ptr(), True);
    if prop == 0 {
        return (0x354352, 0);
    }
    let fields = x.strings(wm, prop);
    let number = |i: usize| {
        fields
            .get(i)
            .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
    };
    (
        number(2).unwrap_or(0x354352) as _,
        number(6).unwrap_or(0) as _,
    )
}

// Keep all choices on a visible output even when the virtual desktop has gaps.
unsafe fn monitor_at_pointer(x: &Screen) -> Rect {
    let (w, h) = x.dimensions();
    let root = Rect::new(0., 0., w as f64, h as f64);
    let (mut major, mut minor) = (0, 0);
    if x.randr_event.is_none()
        || XRRQueryVersion(x.d, &mut major, &mut minor) == 0
        || (major, minor) < (1, 5)
    {
        return root;
    }
    let (mut rr, mut child, mut px, mut py, mut wx, mut wy, mut mask) = (0, 0, 0, 0, 0, 0, 0);
    XQueryPointer(
        x.d, x.root, &mut rr, &mut child, &mut px, &mut py, &mut wx, &mut wy, &mut mask,
    );
    let mut count = 0;
    let monitors = XRRGetMonitors(x.d, x.root, True, &mut count);
    if monitors.is_null() {
        return root;
    }
    let screens = std::slice::from_raw_parts(monitors, count.max(0) as usize);
    let rect =
        |m: &XRRMonitorInfo| Rect::new(m.x as f64, m.y as f64, m.width as f64, m.height as f64);
    let area = screens
        .iter()
        .map(rect)
        .find(|r| contains(*r, px, py))
        .or_else(|| screens.first().map(rect))
        .unwrap_or(root);
    XRRFreeMonitors(monitors);
    area
}
