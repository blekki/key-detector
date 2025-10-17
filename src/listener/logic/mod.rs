use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use rdev::{EventType, Key};

use crate::listener::logic::hotkey::HotKey;
mod hotkey;

pub struct Logic;

impl Logic {
// ##### PUBLIC AREA #####
    // key printer
    pub fn print_key_in_console(key: Option<String>) {
        match key {
            Some(key) => println!("{}", key),
            None => (),
        };
    }

    // return a signal after process the event
    pub fn process_event(event: EventType, signal_ptr: Arc<AtomicU8>) {

        // lamda func: check does the key is a hotkey component
        let is_hotkey_component = move |key: Key| -> HotKey {
            let is_component: HotKey;
            match key {
                Key::Escape      => is_component = HotKey::Escape,
                Key::ControlLeft => is_component = HotKey::ControlLeft,
                Key::ShiftLeft   => is_component = HotKey::ShiftLeft,
                Key::KeyQ        => is_component = HotKey::KeyQ,
                Key::KeyC        => is_component = HotKey::KeyC,
                // default
                _ => is_component = HotKey::NoKey,
            }
            return is_component;
        };

        // find event type
        match event {
            EventType::KeyPress(key) => {
                let comp: HotKey = is_hotkey_component(key);
                if comp != HotKey::NoKey {
                    comp.press_key();
                    
                    // !!!!!
                    // Msg: here can be troubles.
                    // When key pressed it send to much the same signals

                    let signal_as_num = HotKey::get_hotkey_signal().as_num();
                    signal_ptr.store(signal_as_num, Ordering::Relaxed);
                }
            },
            EventType::KeyRelease(key) => {
                let comp: HotKey = is_hotkey_component(key);
                if comp != HotKey::NoKey {
                    comp.release_key();
                }
            },
            _ => ()
        }
    }
}