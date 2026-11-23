// SPDX-License-Identifier: LGPL-2.0-or-later
use crate::TileRect;

/// Score in X11 root coordinates, doubling the centers to retain half-pixels.
pub(crate) fn tile_score(client: TileRect, candidate: TileRect) -> (i64, i64) {
    let axis = |a: i32, size_a: i32, b: i32, size_b: i32| {
        // Keep the original C int edge arithmetic even for synthetic coordinates
        // outside the bounds of the X11 protocol.
        let end = a.wrapping_add(size_a).min(b.wrapping_add(size_b));
        let overlap = end.wrapping_sub(a.max(b)).max(0);
        let center = |origin, size| 2 * i64::from(origin) + i64::from(size);
        (i64::from(overlap), center(a, size_a) - center(b, size_b))
    };
    let (w, dx) = axis(client.x, client.width, candidate.x, candidate.width);
    let (h, dy) = axis(client.y, client.height, candidate.y, candidate.height);
    (w * h, dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)))
}

pub(crate) fn select_tile_monitor(
    client: TileRect,
    candidate: TileRect,
    best_area: i64,
    best_distance: i64,
) -> Option<(i64, i64)> {
    if candidate.width <= 0 || candidate.height <= 0 {
        return None;
    }
    let (area, distance) = tile_score(client, candidate);
    (area > best_area || (area == best_area && distance < best_distance))
        .then_some((area, distance))
}
