// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    effects::{ease, Effect, Rect},
    native::*,
    renderer::{Image, Renderer},
    screen::Screen,
    settings::Settings,
};
use std::rc::Rc;
use x11::xlib::*;

pub struct Animation {
    pub image: Option<Rc<Image>>,
    pub from: Rect,
    pub to: Rect,
    pub start: f64,
    pub kind: i32,
    pub pending: bool,
}
pub struct Surface {
    d: *mut Display,
    pub id: Window,
    pub client: Window,
    pub above: Option<Window>,
    pub attr: XWindowAttributes,
    damage: XID,
    pub image: Option<Rc<Image>>,
    pub animation: Option<Animation>,
    pub mapped: bool,
    pub desktop: bool,
    pub managed: bool,
    shadow: bool,
    pub dirty: bool,
    shaped: bool,
    pub metadata_dirty: bool,
    pub shape_dirty: bool,
    opacity: f64,
    pub appeared: Option<f64>,
    shape: Vec<XRectangle>,
}
impl Surface {
    pub unsafe fn add(x: &Screen, id: Window) -> Option<Self> {
        let attr = x.attrs(id)?;
        if attr.class == InputOnly {
            return None;
        }
        XSelectInput(x.d, id, PropertyChangeMask);
        XShapeSelectInput(x.d, id, 1);
        let mut s = Self {
            d: x.d,
            id,
            client: id,
            above: None,
            attr,
            damage: XDamageCreate(x.d, id, 3),
            image: None,
            animation: None,
            mapped: attr.map_state == IsViewable,
            desktop: false,
            managed: false,
            shadow: false,
            dirty: true,
            shaped: false,
            metadata_dirty: false,
            shape_dirty: false,
            opacity: 1.,
            appeared: None,
            shape: Vec::new(),
        };
        s.update_shape(x);
        s.identify(x);
        Some(s)
    }
    pub fn geometry(&self) -> Rect {
        let a = &self.attr;
        Rect::new(
            a.x as f64,
            a.y as f64,
            (a.width + 2 * a.border_width) as f64,
            (a.height + 2 * a.border_width) as f64,
        )
    }
    pub unsafe fn identify(&mut self, x: &Screen) {
        let a = &x.atoms;
        let client = x.property(self.id, a.client, XA_WINDOW);
        self.client = client.first().copied().unwrap_or(self.id);
        self.managed = !client.is_empty();
        if self.client != self.id {
            XSelectInput(x.d, self.client, PropertyChangeMask);
        }
        let types = x.property(self.client, a.window_type, XA_ATOM);
        self.desktop = types.contains(&a.desktop);
        let dock = types.contains(&a.dock);
        let mut opacity = x.property(self.client, a.opacity, XA_CARDINAL);
        if opacity.is_empty() {
            opacity = x.property(self.id, a.opacity, XA_CARDINAL);
        }
        self.opacity = opacity
            .first()
            .map_or(1., |&v| v as u32 as f64 / u32::MAX as f64);
        let (w, h) = x.dimensions();
        self.shadow = !self.desktop
            && !dock
            && !self.shaped
            && (self.managed
                || (self.attr.override_redirect != 0
                    && self.attr.depth != 32
                    && (self.attr.width as f64) < (w as f64) * 0.9
                    && (self.attr.height as f64) < (h as f64) * 0.9))
            && !x
                .property(self.client, a.state, XA_ATOM)
                .contains(&a.fullscreen);
        self.metadata_dirty = false;
    }
    pub unsafe fn update_shape(&mut self, x: &Screen) {
        let (mut count, mut ordering) = (0, 0);
        let rects = XShapeGetRectangles(x.d, self.id, 0, &mut count, &mut ordering);
        self.shape.clear();
        self.shaped = false;
        if !rects.is_null() {
            self.shape
                .extend_from_slice(std::slice::from_raw_parts(rects, count as usize));
            XFree(rects.cast());
            self.shaped = match self.shape.as_slice() {
                [r] => {
                    r.x != 0
                        || r.y != 0
                        || i32::from(r.width) != self.attr.width
                        || i32::from(r.height) != self.attr.height
                }
                _ => true,
            };
        }
        self.shape_dirty = false;
    }
    unsafe fn capture(&mut self, x: &Screen, r: &mut Renderer) -> Result<(), String> {
        if self.dirty {
            XDamageSubtract(x.d, self.damage, 0, 0);
        }
        if self.image.is_none() {
            let (image, ok) = x.trap(|| r.capture(self.id, &self.attr));
            let image = image?;
            if !ok {
                x.trap(|| drop(image));
                return Ok(());
            }
            self.image = Some(image);
            self.dirty = false;
        }
        if self.dirty {
            if let Some(im) = &self.image {
                r.refresh(im);
            }
            self.dirty = false;
        }
        Ok(())
    }
    pub unsafe fn paint(
        &mut self,
        x: &Screen,
        r: &mut Renderer,
        settings: &Settings,
        t: f64,
    ) -> Result<bool, String> {
        if self.animation.as_ref().is_some_and(|a| {
            let elapsed = (t - a.start) * 1000.;
            elapsed > f64::from(settings.duration + 100)
                || (!a.pending && elapsed >= f64::from(settings.duration))
        }) {
            self.animation = None;
        }
        if !self.mapped && self.animation.is_none() {
            return Ok(false);
        }
        if self.shape_dirty {
            self.update_shape(x);
        }
        if self.metadata_dirty {
            self.identify(x);
        }
        if self.mapped {
            self.capture(x, r)?;
        }
        let (mut rect, mut alpha, mut image, mut progress, mut active) = (
            self.geometry(),
            self.opacity,
            self.image.as_deref(),
            1.,
            false,
        );
        if let Some(anim) = self.animation.as_ref().filter(|a| !a.pending) {
            progress = ((t - anim.start) * 1000. / f64::from(settings.duration)).clamp(0., 1.);

            active = true;
            rect = anim.from.interpolate(anim.to, ease(progress));
            if anim.kind == 1 {
                image = anim.image.as_deref();
                alpha *= 1. - 0.7 * ease(progress);
            } else if image.is_none() {
                image = anim.image.as_deref();
            }
        }
        if let Some(appeared) = self.appeared.filter(|_| settings.fade != 0) {
            let p = (t - appeared) * 1000. / f64::from(settings.fade);
            if p < 1. {
                alpha *= ease(p);
                active = true;
            } else {
                self.appeared = None;
            }
        }
        let Some(image) = image else {
            return Ok(active);
        };
        let anim = self.animation.as_ref().filter(|a| !a.pending);
        let transformed = anim.is_some() && matches!(settings.effect, Effect::Twist | Effect::Flip);
        if settings.shadows && self.shadow && !transformed {
            r.shadow_at(
                rect,
                settings.shadow_opacity * alpha,
                settings.radius,
                settings.offset,
            );
        }
        let shape = self.shaped.then_some(self.shape.as_slice());
        if transformed {
            r.transformed(image, rect.outline(settings.effect, progress), alpha, shape);
        } else {
            r.draw(image, rect, alpha, shape);
        }
        if let Some(anim) = anim {
            let count = if settings.effect == Effect::Zoom {
                3
            } else {
                1
            };
            for i in 0..count {
                let i = i as f64;
                let p = (progress - i * 0.045).clamp(0., 1.);
                r.line(
                    anim.from
                        .interpolate(anim.to, ease(p))
                        .outline(settings.effect, p),
                    0.85 - i * 0.2,
                );
            }
        }
        Ok(active)
    }
    pub unsafe fn transition(
        &mut self,
        x: &Screen,
        msg: &XClientMessageEvent,
        settings: &Settings,
        t: f64,
    ) {
        let kind = msg.data.get_long(1) as i32;
        if self.metadata_dirty {
            self.identify(x);
        }
        if !self.managed || self.desktop || !(1..=4).contains(&kind) {
            return;
        }
        let rect = self.geometry();
        let icon = msg.data.get_long(2) as Window;
        let geometry = (icon != 0)
            .then(|| x.attrs(icon))
            .flatten()
            .and_then(|a| {
                let (mut px, mut py, mut child) = (0, 0, 0);
                (XTranslateCoordinates(x.d, icon, x.root, 0, 0, &mut px, &mut py, &mut child) != 0)
                    .then(|| Rect::new(px as f64, py as f64, a.width as f64, a.height as f64))
            })
            .unwrap_or_else(|| {
                Rect::new(
                    rect.x + rect.w / 2. - 24.,
                    f64::from(x.dimensions().1 - 56),
                    48.,
                    48.,
                )
            });
        match msg.data.get_long(3) {
            0 => {
                let from = if kind == 2 {
                    geometry
                } else {
                    self.animation
                        .as_ref()
                        .filter(|a| !a.pending)
                        .map_or(rect, |a| {
                            a.from.interpolate(
                                a.to,
                                ease((t - a.start) * 1000. / settings.duration as f64),
                            )
                        })
                };
                self.animation = Some(Animation {
                    image: self.image.clone(),
                    from,
                    to: rect,
                    start: t,
                    kind,
                    pending: true,
                });
            }
            1 if self.animation.as_ref().is_some_and(|a| a.kind == kind) => {
                if let Some(a) = x.attrs(self.id) {
                    self.attr = a;
                }
                let to = if kind == 1 { geometry } else { self.geometry() };
                if let Some(anim) = &mut self.animation {
                    anim.to = to;
                    anim.start = t;
                    anim.pending = false;
                }
                self.appeared = None;
                let values = [kind as libc::c_ulong, self.id];
                XChangeProperty(
                    x.d,
                    x.owner,
                    x.atoms.last_animation,
                    XA_CARDINAL,
                    32,
                    PropModeReplace,
                    values.as_ptr().cast(),
                    2,
                );
            }
            _ => {}
        }
    }
}
impl Drop for Surface {
    fn drop(&mut self) {
        // SAFETY: same live connection that created the damage object.
        unsafe {
            XDamageDestroy(self.d, self.damage);
        }
    }
}
