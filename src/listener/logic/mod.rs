use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::io::Error;

use rdev::{Event, EventType, Key};

use super::signals::{Signals::*};
use crate::listener::logic::hotkey::HotKey;
use logger::Logger;

mod hotkey;
mod logger;


#[derive(Clone)]  // auto copy/clone
pub struct Logic {
    signal_state: Arc<AtomicU8>,
    logger: Logger
}

impl Logic {
// ##### PRIVATE AREA #####
    fn set_signal_state(&self, signal_as_uint: u8) {
        self.signal_state.store(signal_as_uint, Ordering::Release);
    }

// ##### PUBLIC AREA #####
    pub fn get_signal_state(&self) -> u8 {
        self.signal_state.swap(NoSignal.as_num(),Ordering::Relaxed)
    }

    pub fn reset_signal(&self) {
        self.signal_state.store(NoSignal.as_num(), Ordering::Relaxed);
    }
    
    pub fn shutdown(&self) {
        self.logger.shutdown();
        println!("[logic]: Logger shutdown");
    }

    // add key to the log list
    pub fn logger_start(&self) -> Result<String, Error> {
        return self.logger.start();
    }
    
    pub fn log_key(&self, event: Event) {
        // !!!
        // Dosn't support no-English literals.
        // It prints only key name (as KeyA, Shift, Num1 etc.)

        match event.event_type {
            EventType::KeyPress(key) => {
                let key_name = format!("{:?}", key);
                self.logger.log_key(key_name.as_str());
            }
            _ => () // do nothing
        }
    }

    // key printer
    pub fn print_key_in_console(&self, key: Option<String>) {
        match key {
            Some(key) => println!("{}", key),
            None => (),
        };
    }

    // processes the keys state changes
    pub fn process_event(&self, event: EventType) {

        // lamda func: check does the key is a hotkey component
        let as_hotkey_component = move |key: Key| -> HotKey {
            let component: HotKey;
            match key {
                Key::Escape      => component = HotKey::Escape,
                Key::ControlLeft => component = HotKey::ControlLeft,
                Key::ShiftLeft   => component = HotKey::ShiftLeft,
                Key::KeyQ        => component = HotKey::KeyQ,
                Key::KeyC        => component = HotKey::KeyC,
                // default
                _ => component = HotKey::NoComponent,
            }
            return component;
        };

        // find event type
        match event {
            EventType::KeyPress(key) => {
                let comp: HotKey = as_hotkey_component(key);
                if comp != HotKey::NoComponent {
                    comp.press_key();
                    // save signal
                    let signal_as_uint = HotKey::get_hotkey_signal().as_num();
                    self.set_signal_state(signal_as_uint);
                }
            },
            EventType::KeyRelease(key) => {
                let comp: HotKey = as_hotkey_component(key);
                if comp != HotKey::NoComponent {
                    comp.release_key();
                }
            },
            _ => ()
        }

        // # Note: 
        // Signal reset only after Listener::signal_analyzer
        // processed it
        
    }

    // constructor
    pub fn new() -> Logic {
        Logic {
            signal_state: Arc::new(AtomicU8::new(NoSignal.as_num())),
            logger: Logger::new()
        }
    }
}