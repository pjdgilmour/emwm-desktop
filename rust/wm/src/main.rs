#![cfg_attr(not(test), no_main)]
// The legacy Xt callback ABI uses upstream symbol names and concrete C layouts.
#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused,
    clashing_extern_declarations
)]
// The Motif private records and the imported Xlib bitfield layout come from
// the GNU/Linux amd64 ABI validated by this port.
#[cfg(not(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu")))]
compile_error!("The EMWM Motif bindings require GNU/Linux x86_64.");
#[macro_use]
extern crate c2rust_bitfields;
extern crate emwm_monitor as _;
#[link(name = "Xm")]
#[link(name = "Xt")]
#[link(name = "X11")]
#[link(name = "Xrender")]
#[link(name = "Xext")]
#[link(name = "Xrandr")]
#[link(name = "Xinerama")]
#[link(name = "m")]
extern "C" {}
mod WmArgb;
mod WmBackdrop;
mod WmCDInfo;
mod WmCDecor;
mod WmCEvent;
mod WmCPlace;
mod WmColormap;
mod WmError;
mod WmEvent;
mod WmEwmh;
mod WmFeedback;
mod WmFunction;
mod WmGraphics;
mod WmIDecor;
mod WmIPlace;
mod WmIconBox;
mod WmImage;
mod WmInitWs;
mod WmKeyFocus;
mod WmMain;
mod WmManage;
mod WmMenu;
mod WmPresence;
mod WmProperty;
mod WmProtocol;
mod WmRequest;
mod WmResCvt;
mod WmResParse;
mod WmResource;
mod WmSignal;
mod WmTile;
mod WmWinConf;
mod WmWinInfo;
mod WmWinList;
mod WmWinState;
mod WmWrkspace;
mod WmXSMP;
mod WmXinerama;
mod abi;
