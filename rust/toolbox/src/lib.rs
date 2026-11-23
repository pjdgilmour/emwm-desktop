// SPDX-License-Identifier: MIT
mod applications;
mod xdg;

mod command_line;
mod commands;
mod tbmain;
mod wswitch;
extern crate emwm_toolbox_config as _;
extern crate emwm_tray as _;
#[link(name = "Xrandr")]
extern "C" {}
pub use tbmain::main as run;
