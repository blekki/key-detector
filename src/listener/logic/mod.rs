use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::io::Error;

use rdev::{Event, EventType, Key};

use crate::listener::logic::hotkey::HotKey;
use super::signals::Signals::{*};
use logger::Logger;

mod hotkey;
mod logger;


#[derive(Clone)]  // auto copy/clone
pub struct Logic {
    logger: Logger
}

impl Logic {
// ##### PUBLIC AREA #####
    // add key to the log list
    pub fn logger_start(&self) -> Result<String, Error> {
        return self.logger.start();
    }

    pub fn log_key(&self, event: Event) {
        // This method is a key name adapter.
        // If a pressed key is standard one, save it without changes.
        // If it's a special key (as Shift, Fn, F1 etc), 
        // convert name, for example, from "\u{1b}" to the "Esc".

        match event.event_type {
            EventType::KeyPress(key) => {
                let key_name = format!("{:?}", key);
                self.logger.log_key(key_name.as_str());
            }
            _ => () // do nothing
        }
    }

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
                    
                    // send 
                    let signal_as_num = HotKey::get_hotkey_signal().as_num();
                    signal_ptr.store(signal_as_num, Ordering::Relaxed);
                }
            },
            EventType::KeyRelease(key) => {
                let comp: HotKey = as_hotkey_component(key);
                if comp != HotKey::NoComponent {
                    comp.release_key();
                    
                    // stop use hotkey
                    signal_ptr.store(NoSignal.as_num(), Ordering::Relaxed);
                }
            },
            _ => ()
        }
    }

    // constructor
    pub fn new() -> Logic {
        // init all parameters
        Logic {
            logger: Logger::new()
        }
    }
    
    
}