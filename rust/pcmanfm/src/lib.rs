// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright 2010–2012 Hong Jen Yee, 2012–2017 Andriy Grytsenko, 2024 Ingo Brückl.
//! Layout of PCManFM's X11 desktop, using measurements from GTK and Pango.
//! The C declarations in the PCManFM patch mirror these fixed-width records.

mod position;
mod profiles;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    fn intersects(self, other: Rect) -> bool {
        self.x.max(other.x) < (self.x + self.width).min(other.x + other.width)
            && self.y.max(other.y) < (self.y + self.height).min(other.y + other.height)
    }

    fn contains(self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Geometry {
    area: Rect,
    icon: Rect,
    text: Rect,
}

impl Geometry {
    fn move_to(&mut self, x: i32, y: i32) {
        let dx = x - self.area.x;
        let dy = y - self.area.y;
        self.area.x = x;
        self.area.y = y;
        self.icon.translate(dx, dy);
        self.text.translate(dx, dy);
    }

    fn bounds(self) -> Rect {
        let x = self.icon.x.min(self.text.x);
        let y = self.icon.y.min(self.text.y);
        Rect {
            x,
            y,
            width: (self.icon.x + self.icon.width).max(self.text.x + self.text.width) - x,
            height: (self.icon.y + self.icon.height).max(self.text.y + self.text.height) - y,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Grid {
    work: Rect,
    x_margin: i32,
    y_margin: i32,
    cell_width: i32,
    cell_height: i32,
    right: i32,
}

#[repr(C)]
pub struct Item {
    geometry: Geometry,
    old_bounds: Rect,
    fixed: i32,
}

impl Grid {
    fn first_x(self) -> i32 {
        if self.right != 0 {
            self.work.x + self.work.width - self.x_margin - self.cell_width
        } else {
            self.work.x + self.x_margin
        }
    }
    fn arrange(self, items: &mut [Item]) {
        let mut fixed: Vec<_> = items
            .iter()
            .filter(|i| i.fixed != 0)
            .map(|i| i.old_bounds)
            .collect();
        let mut next_fixed = 0;
        let mut x = self.first_x();
        let top = self.work.y + self.y_margin;
        // PCManFM checks absolute icon Y against this monitor-relative bottom.
        let bottom = self.work.height - self.y_margin;
        let mut y = top;
        for item in items {
            if item.fixed != 0 {
                fixed[next_fixed] = item.geometry.bounds();
                next_fixed += 1;
                continue;
            }
            loop {
                let g = &mut item.geometry;
                g.move_to(x, y);
                if g.area.y + g.area.height > bottom && y > top {
                    x += if self.right != 0 {
                        -self.cell_width
                    } else {
                        self.cell_width
                    };
                    y = top;
                    continue;
                }
                // Pango can produce multi-row labels. Skip every occupied row.
                y += g.area.height.div_euclid(self.cell_height) * self.cell_height;
                if g.area.height.rem_euclid(self.cell_height) != 0 {
                    y += self.cell_height;
                }
                if !fixed
                    .iter()
                    .any(|r| r.intersects(g.icon) || r.intersects(g.text))
                {
                    break;
                }
            }
        }
    }
}

/// # Safety
/// For a nonzero count, `items` points to that many initialized, uniquely borrowed C records.
#[no_mangle]
pub unsafe extern "C" fn emwm_layout_icons(grid: Grid, items: *mut Item, count: usize) {
    if count == 0 || grid.cell_width <= 0 || grid.cell_height <= 0 {
        return;
    }
    // SAFETY: the GTK caller owns the contiguous GArray for this synchronous call.
    let items = unsafe { std::slice::from_raw_parts_mut(items, count) };
    grid.arrange(items);
}

#[repr(C)]
pub struct Metrics {
    cell_width: i32,
    y_pad: i32,
    icon_size: i32,
    spacing: i32,
    pix_width: i32,
    pix_height: i32,
    label_width: i32,
    label_height: i32,
    label_y: i32,
}

#[no_mangle]
pub extern "C" fn emwm_icon_geometry(x: i32, y: i32, m: Metrics) -> Geometry {
    let icon = Rect {
        x: x + (m.cell_width - m.pix_width) / 2,
        y: y + m.y_pad + (m.icon_size - m.pix_height) / 2,
        width: m.pix_width,
        height: m.pix_height + m.spacing,
    };
    let text = Rect {
        x: x + (m.cell_width - m.label_width - 4) / 2,
        y: y + m.y_pad + m.icon_size + m.spacing,
        width: m.label_width + 4,
        height: m.label_y + m.label_height + 4,
    };
    Geometry {
        area: Rect {
            x,
            y,
            width: (m.cell_width + icon.width.max(text.width)) / 2,
            height: text.y + text.height - y,
        },
        icon,
        text,
    }
}

#[no_mangle]
pub extern "C" fn emwm_anchor_x(
    x: i32,
    old_width: i32,
    was_right: i32,
    work: Rect,
    cell: i32,
    right: i32,
) -> i32 {
    position::anchor_x(x, old_width, was_right, work.x, work.width, cell, right)
}

#[repr(C)]
pub struct Point {
    x: i32,
    y: i32,
}

#[no_mangle]
pub extern "C" fn emwm_snap_icon(grid: Grid, x: i32, y: i32) -> Point {
    Point {
        x: position::snap_axis(x, grid.first_x(), grid.cell_width),
        y: position::snap_axis(y, grid.work.y + grid.y_margin, grid.cell_height),
    }
}

#[no_mangle]
pub extern "C" fn emwm_move_icon(
    mut geometry: Geometry,
    work: Rect,
    x_margin: i32,
    y_margin: i32,
    x: i32,
    y: i32,
) -> Geometry {
    let x = position::drag_axis(x, work.x, work.width, x_margin, geometry.area.width);
    let y = position::drag_axis(y, work.y, work.height, y_margin, geometry.area.height);
    geometry.move_to(x, y);
    geometry
}

#[no_mangle]
pub extern "C" fn emwm_selection_rect(start_x: i32, start_y: i32, x: i32, y: i32) -> Rect {
    Rect {
        x: x.min(start_x),
        y: y.min(start_y),
        width: x.abs_diff(start_x) as i32,
        height: y.abs_diff(start_y) as i32,
    }
}

#[no_mangle]
pub extern "C" fn emwm_icon_hit(mut geometry: Geometry, x: i32, y: i32) -> i32 {
    // PCManFM treats the gap above the label as part of the icon.
    geometry.icon.height = geometry.text.y - geometry.icon.y;
    i32::from(geometry.icon.contains(x, y) || geometry.text.contains(x, y))
}

#[no_mangle]
pub extern "C" fn emwm_wallpaper_rect(
    mode: i32,
    src_w: i32,
    src_h: i32,
    dest_w: i32,
    dest_h: i32,
    offset_x: i32,
    offset_y: i32,
) -> Rect {
    // FmWallpaperMode from app-config.h; the C header checks its discriminants.
    let mut rect = Rect {
        x: 0,
        y: 0,
        width: src_w,
        height: src_h,
    };
    match mode {
        1 | 6 => Rect {
            x: offset_x,
            y: offset_y,
            width: dest_w,
            height: dest_h,
        },
        2 | 3 | 5 => {
            if mode != 3 && src_w > 0 && src_h > 0 {
                // The upstream path rounds the ratios to float before multiplying in double.
                let xr = f64::from(dest_w as f32 / src_w as f32);
                let yr = f64::from(dest_h as f32 / src_h as f32);
                let ratio = if mode == 2 { xr.min(yr) } else { xr.max(yr) };
                rect.width = (f64::from(src_w) * ratio) as i32;
                rect.height = (f64::from(src_h) * ratio) as i32;
            }
            rect.x = (dest_w - rect.width) / 2;
            rect.y = (dest_h - rect.height) / 2;
            rect
        }
        _ => rect,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: i32, y: i32, width: i32, height: i32) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    fn grid(right: bool) -> Grid {
        Grid {
            work: rect(6, 4, 88, 36),
            x_margin: 2,
            y_margin: 2,
            cell_width: 20,
            cell_height: 10,
            right: i32::from(right),
        }
    }

    #[test]
    fn uses_saved_bounds_until_fixed_icon_is_measured_then_wraps_columns() {
        for right in [false, true] {
            let grid = grid(right);
            let first = grid.first_x();
            let geometry = Geometry {
                area: rect(0, 0, 10, 10),
                icon: rect(0, 0, 10, 7),
                text: rect(0, 7, 10, 3),
            };
            let normal = || Item {
                geometry,
                old_bounds: Rect::default(),
                fixed: 0,
            };
            let mut fixed = normal();
            fixed.geometry.move_to(first, 6);
            fixed.fixed = -1; // GTK stores this as a signed bitfield.
            fixed.old_bounds = rect(first, 6, 10, 10);
            let mut items = [normal(), fixed, normal(), normal()];
            grid.arrange(&mut items);
            let next = first + if right { -20 } else { 20 };
            let positions: Vec<_> = items
                .iter()
                .map(|i| (i.geometry.area.x, i.geometry.area.y))
                .collect();
            assert_eq!(positions, [(first, 16), (first, 6), (next, 6), (next, 16)]);
        }
    }

    #[test]
    fn recovers_anchor_after_resize_and_rounds_half_cells_away_from_origin() {
        let grid = grid(true);
        assert_eq!(emwm_anchor_x(25, 78, 1, grid.work, 20, 1), 35);
        assert_eq!(emwm_anchor_x(25, 78, 1, grid.work, 20, 0), 45);
        assert_eq!(emwm_snap_icon(grid, 62, 11).x, 52);
        assert_eq!(emwm_snap_icon(grid, 82, 11).x, 92);
        assert_eq!(emwm_snap_icon(grid, 62, 11).y, 16);
    }

    #[test]
    fn fits_and_crops_with_original_pixel_rounding_and_screen_offset() {
        assert_eq!(
            emwm_wallpaper_rect(2, 750, 350, 320, 200, 0, 0),
            rect(0, 25, 320, 149)
        );
        assert_eq!(
            emwm_wallpaper_rect(5, 750, 350, 320, 200, 0, 0),
            rect(-54, 0, 428, 200)
        );
        assert_eq!(
            emwm_wallpaper_rect(6, 100, 40, 800, 600, -800, 50),
            rect(-800, 50, 800, 600)
        );
        assert_eq!(
            emwm_wallpaper_rect(3, 101, 31, 50, 32, 0, 0),
            rect(-25, 0, 101, 31)
        );
    }

    #[test]
    fn selection_and_drag_include_label_gap_and_clamp_oversized_icons() {
        let geometry = Geometry {
            area: rect(10, 10, 100, 60),
            icon: rect(12, 12, 8, 8),
            text: rect(9, 35, 25, 10),
        };
        assert_eq!(emwm_icon_hit(geometry, 19, 34), 1);
        assert_eq!(emwm_icon_hit(geometry, 20, 34), 0);
        assert_eq!(emwm_selection_rect(55, 10, 2, -15), rect(2, -15, 53, 25));
        let moved = emwm_move_icon(geometry, rect(4, 5, 30, 40), 2, 3, 90, -10);
        assert_eq!(moved.area, rect(6, 8, 100, 60));
        assert_eq!(moved.icon, rect(8, 10, 8, 8));
    }
}
