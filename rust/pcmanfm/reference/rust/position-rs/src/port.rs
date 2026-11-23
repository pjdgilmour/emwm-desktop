// SPDX-License-Identifier: GPL-2.0-or-later
#![forbid(unsafe_code)]

#[path = "../../../../src/position.rs"]
mod position;

pub use position::{anchor_x, drag_axis, snap_axis};
