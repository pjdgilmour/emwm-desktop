// SPDX-License-Identifier: GPL-2.0-or-later
// Geometry adapted from Window Maker src/actions.c, copyright Alfredo K. Kojima
// (1997–2003), Dan Pascu (1998–2003), Window Maker Team (2014–2026).
// See apps/emwm-compositor/COPYING.WindowMaker.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Effect {
    Disabled,
    Zoom,
    Twist,
    Flip,
}
#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
#[derive(Clone, Copy, Default, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
pub type Quad = [Point; 4];

pub fn ease(t: f64) -> f64 {
    let t = t.clamp(0., 1.);
    t * t * (3. - 2. * t)
}
impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
    pub fn interpolate(self, other: Self, t: f64) -> Self {
        let mix = |a, b| a + (b - a) * t;
        Self::new(
            mix(self.x, other.x),
            mix(self.y, other.y),
            mix(self.w, other.w),
            mix(self.h, other.h),
        )
    }
    pub fn quad(self) -> Quad {
        let Self { x, y, w, h } = self;
        [
            Point { x, y },
            Point { x: x + w, y },
            Point { x: x + w, y: y + h },
            Point { x, y: y + h },
        ]
    }
    pub fn outline(self, effect: Effect, t: f64) -> Quad {
        let (sin, cos) = (std::f64::consts::TAU * t).sin_cos();
        let Self { x, y, w, h } = self;
        let cy = y + h / 2.;
        match effect {
            Effect::Twist => self.quad().map(|p| {
                let cx = x + w / 2.;
                let (dx, dy) = (p.x - cx, p.y - cy);
                Point {
                    x: cx + dx * cos - dy * sin,
                    y: cy + dx * sin + dy * cos,
                }
            }),
            Effect::Flip => {
                let shift = w * 0.1 * sin;
                let half = h * 0.5 * cos;
                [
                    Point {
                        x: x + shift,
                        y: cy - half,
                    },
                    Point {
                        x: x + w + shift,
                        y: cy - half,
                    },
                    Point {
                        x: x + w - shift,
                        y: cy + half,
                    },
                    Point {
                        x: x - shift,
                        y: cy + half,
                    },
                ]
            }
            _ => self.quad(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn endpoints() {
        let a = Rect::new(10., 20., 800., 600.);
        let b = Rect::new(300., 700., 48., 48.);
        assert_eq!(ease(-1.), 0.);
        assert_eq!(ease(2.), 1.);
        for e in [Effect::Zoom, Effect::Twist, Effect::Flip] {
            for i in 0..=100 {
                let t = i as f64 / 100.;
                for p in a.interpolate(b, ease(t)).outline(e, t) {
                    assert!(p.x.is_finite() && p.y.is_finite());
                }
            }
            for (p, q) in b.outline(e, 1.).into_iter().zip(b.quad()) {
                assert!((p.x - q.x).abs() < 0.001 && (p.y - q.y).abs() < 0.001);
            }
        }
    }
}
