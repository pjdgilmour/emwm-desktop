// SPDX-License-Identifier: GPL-3.0-only
#[path = "../../shared/text.rs"]
pub mod text;

pub mod files;
pub mod images;
pub mod lightdm;
pub mod runtime;
pub mod settings;

#[path = "../../../gradient-rs/src/model.rs"]
pub mod gradient;
mod resample;
