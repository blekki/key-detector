use std::sync::atomic::{AtomicU16, Ordering};
use super::super::signals::{Signals, Signals::*};

// pressed keys combination
static COMBINATION: AtomicU16 = AtomicU16::new(0x0);

// hotkeys signature (example: 0x00100110)
const _STOP_LISTENER: u16 = HotKey::Escape as u16;
const _HELLO_WORLD: u16 = (HotKey::KeyQ as u16) | (HotKey::KeyC as u16);
// todo: make special set list


#[derive(Copy, Clone)]  // auto copy/clone
#[derive(PartialEq)]    // != operator realization
pub enum HotKey {
    NoComponent     = 0b0,
    Escape          = 0b1,
    ControlLeft     = 0b10,
    ShiftLeft       = 0b100,
    KeyQ            = 0b1000,
    KeyC            = 0b10000,
}

impl HotKey {
// ##### PUBLIC AREA #####
    pub fn get_hotkey_signal() -> Signals {
        let copy = COMBINATION.load(Ordering::Acquire);

        // checking
        let signal: Signals = match copy {
            _STOP_LISTENER  => StopListener,
            _HELLO_WORLD    => HelloWorld,
            _ => NoSignal
        };

        return signal;
    }

    pub fn press_key(&self) {
        // add pressed key to the COMBINATION
        COMBINATION.fetch_or(*self as u16, Ordering::Release);
    }

    pub fn release_key(&self) {
        // remove released key from the COMBINATION
        COMBINATION.fetch_xor(*self as u16, Ordering::Release);
    }
}