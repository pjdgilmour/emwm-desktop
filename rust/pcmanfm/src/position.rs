// SPDX-License-Identifier: GPL-2.0-or-later
//! Position calculations shared with the rust-anything comparison bridge.

pub fn anchor_x(
    x: i32,
    old_width: i32,
    was_right: i32,
    origin: i32,
    width: i32,
    cell: i32,
    right: i32,
) -> i32 {
    let x = if was_right != 0 && old_width > 0 {
        x.wrapping_add(width.wrapping_sub(old_width))
    } else {
        x
    };
    if (was_right != 0) != (right != 0) {
        origin
            .wrapping_mul(2)
            .wrapping_add(width)
            .wrapping_sub(x)
            .wrapping_sub(cell)
    } else {
        x
    }
}

pub fn snap_axis(value: i32, origin: i32, cell: i32) -> i32 {
    if cell <= 0 {
        return value;
    }
    let steps = (f64::from(value.wrapping_sub(origin)) / f64::from(cell)).round();
    let snapped = f64::from(origin) + steps * f64::from(cell);
    // The original Debian amd64 build produces INT_MIN on an out-of-range conversion.
    if snapped > f64::from(i32::MAX) || snapped < f64::from(i32::MIN) {
        i32::MIN
    } else {
        snapped as i32
    }
}

pub fn drag_axis(value: i32, origin: i32, width: i32, margin: i32, item: i32) -> i32 {
    let low = origin.wrapping_add(margin);
    let high = origin
        .wrapping_add(width)
        .wrapping_sub(margin)
        .wrapping_sub(item);
    value.min(high).max(low)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_positions_and_missing_legacy_width() {
        assert_eq!(snap_axis(-15, 0, 10), -20);
        assert_eq!(anchor_x(-200, 0, 1, -800, 800, 80, 0), -680);
        assert_eq!(drag_axis(-4000, -800, 800, 10, 100), -790);
    }
}
