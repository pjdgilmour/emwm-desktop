// SPDX-License-Identifier: GPL-3.0-only
use crate::screen::Screen;
use x11::{keysym::*, xinput2::*, xlib::*};

/// Observe a lone Super without grabbing it: the WM must still see Super+arrow
/// and applications must receive their own modifier chords.
#[derive(Default)]
pub struct Shortcuts {
    opcode: i32,
    super_keys: [u8; 2],
    pub super_mask: u32,
    tab: u8,
    modifiers: Vec<u32>,
    tab_down: bool,
    tap: Option<i32>,
}

impl Shortcuts {
    pub unsafe fn start(x: &Screen) -> Self {
        let mut keys = Self::default();
        let (mut event, mut error, mut major, mut minor) = (0, 0, 2, 1);
        if XQueryExtension(
            x.d,
            c"XInputExtension".as_ptr(),
            &mut keys.opcode,
            &mut event,
            &mut error,
        ) != 0
            && XIQueryVersion(x.d, &mut major, &mut minor) == 0
        {
            let mut bits = [0; 2];
            for kind in [XI_RawKeyPress, XI_RawKeyRelease, XI_RawButtonPress] {
                XISetMask(&mut bits, kind);
            }
            let mut mask = XIEventMask {
                deviceid: XIAllMasterDevices,
                mask_len: bits.len() as _,
                mask: bits.as_mut_ptr(),
            };
            XISelectEvents(x.d, x.root, &mut mask, 1);
        }
        let mut supported = 0;
        XkbSetDetectableAutoRepeat(x.d, True, &mut supported);
        keys.remap(x);
        keys
    }

    pub unsafe fn remap(&mut self, x: &Screen) {
        for mask in self.modifiers.drain(..) {
            XUngrabKey(x.d, self.tab.into(), mask, x.root);
        }
        self.tab_down = false;
        self.tap = None;
        self.super_keys = [XK_Super_L, XK_Super_R].map(|sym| XKeysymToKeycode(x.d, sym.into()));
        self.tab = XKeysymToKeycode(x.d, XK_Tab.into());
        self.super_mask = 0;
        let locks = LockMask
            | XkbKeysymToModifiers(x.d, XK_Num_Lock.into())
            | XkbKeysymToModifiers(x.d, XK_Scroll_Lock.into());
        for sym in [XK_Super_L, XK_Super_R] {
            let modifier = XkbKeysymToModifiers(x.d, sym.into());
            self.super_mask |= modifier;
            if modifier == 0 || self.tab == 0 {
                continue;
            }
            for mask in 0..256 {
                if mask & !locks == modifier && !self.modifiers.contains(&mask) {
                    XGrabKey(
                        x.d,
                        self.tab.into(),
                        mask,
                        x.root,
                        False,
                        GrabModeAsync,
                        GrabModeAsync,
                    );
                    self.modifiers.push(mask);
                }
            }
        }
    }

    pub fn first_tab_press(&mut self) -> bool {
        self.tap = None;
        !std::mem::replace(&mut self.tab_down, true)
    }
    pub fn released(&mut self, code: u32) {
        if code == u32::from(self.tab) {
            self.tab_down = false;
        }
    }

    #[allow(non_upper_case_globals)] // XInput event names
    pub unsafe fn tapped(&mut self, x: &Screen, cookie: &mut XGenericEventCookie) -> bool {
        if cookie.extension != self.opcode
            || !matches!(
                cookie.evtype,
                XI_RawKeyPress | XI_RawKeyRelease | XI_RawButtonPress
            )
            || XGetEventData(x.d, cookie) == 0
        {
            return false;
        }
        let raw = &*cookie.data.cast::<XIRawEvent>();
        let key = raw.detail;
        let mut tapped = false;
        match cookie.evtype {
            XI_RawKeyPress if raw.flags & XIKeyRepeat == 0 => {
                self.tap = None;
                if (1..=255).contains(&key) && self.super_keys.contains(&(key as u8)) {
                    // Raw releases can be consumed during a passive core grab. Read
                    // the actual keyboard state instead of retaining pressed keys.
                    let mut down = [0; 32];
                    XQueryKeymap(x.d, down.as_mut_ptr());
                    down[key as usize / 8] &= !(1 << (key % 8));
                    if down.iter().all(|&byte| byte == 0) {
                        self.tap = Some(key);
                    }
                }
            }
            XI_RawKeyRelease => {
                self.released(key as u32);
                tapped = self.tap.take() == Some(key);
            }
            XI_RawButtonPress => self.tap = None,
            _ => {}
        }
        XFreeEventData(x.d, cookie);
        tapped
    }
}
