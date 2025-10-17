use super::super::signals::{Signals, Signals::*};

static mut COMBINATION: u16 = 0x0;  // pressed keys combination

#[derive(Copy, Clone)]  // auto copy/clone
#[derive(PartialEq)]    // != operator realization
pub enum HotKey {
    NoKey = 0b0,
    Escape = 0b01,
    ControlLeft = 0b010,
    ShiftLeft = 0b0100,
    KeyQ    = 0b1000,
    KeyC    = 0b10000,
}

impl HotKey {
// ##### PRIVATE AREA #####

// ##### PUBLIC AREA #####
    pub fn get_hotkey_signal() -> Signals {
        let signal: Signals;
        unsafe {
            let copy = COMBINATION;
            
            // todo: make special set list
            // ps: hk = hotkeys
            const _SHOULD_STOP: u16 = HotKey::Escape as u16;
            const _HELLO_WORLD: u16 = (HotKey::KeyQ as u16) | (HotKey::KeyC as u16);

            // checking
            match copy {
                _SHOULD_STOP=> signal = ShouldStop,
                _HELLO_WORLD=> signal = HelloWorld,
                _ => signal = NoSignal
            }
        }
        return signal;
    }

    pub fn press_key(&self) {
        unsafe {
            COMBINATION |= *self as u16;
        };
    }

    pub fn release_key(&self) {
        unsafe {
            COMBINATION ^= *self as u16;
        };
    }

    // todo: remove or use as log and debug
    // pub fn print() {
    //     unsafe {
    //         let copy = COMBINATION; // use for higher safety
    //         println!("combination {:08b}", copy);
    //     };
    // }

}