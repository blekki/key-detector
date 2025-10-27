use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use rdev::{Event, EventType, Key};

mod hotkey;
mod logger;
mod metrics;

use super::signals::{Signals::*};
use crate::listener::logic::hotkey::HotKey;
use logger::Logger;
use metrics::Metrics;



#[derive(Clone)]  // auto copy/clone
pub struct Logic {
    signal_state: Arc<AtomicU8>,
    logger: Logger,
    metrics: Metrics
}

impl Logic {
// ##### PRIVATE AREA #####
    fn set_signal_state(&self, signal_as_uint: u8) {
        self.signal_state.store(signal_as_uint, Ordering::Release);
    }

// ##### PUBLIC AREA #####
    pub fn get_signal_state(&self) -> u8 {
        self.signal_state.swap(NoSignal.as_uint(),Ordering::Relaxed)
    }

    pub fn reset_signal(&self) {
        self.signal_state.store(NoSignal.as_uint(), Ordering::Relaxed);
    }
    
    pub fn shutdown(&self) {
        self.logger.shutdown();
        self.metrics.shutdown();
    }

    // ----- run internal sistems -----
    pub fn start_logger(&self) {
        // catch all errors if logger starts wrong
        match self.logger.start() {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err);
                self.set_signal_state(Shutdown.as_uint());
            }
        }
    }

    pub fn start_metrics(&self) {
        // catch all errors if logger starts wrong
        match self.metrics.start() {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err);
                self.set_signal_state(Shutdown.as_uint());
            }
        }
    }
    // --------------------------------

    // add key to the log list    
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

    // add key to the log list
    pub fn update_metric(&self, event: Event) {
        match event.event_type {
            EventType::KeyPress(key) => {
                let key_name = format!("{:?}", key);
                self.metrics.update_metric(key_name.as_str());
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
            return match key {
                Key::Escape      => HotKey::Escape,
                Key::ControlLeft => HotKey::ControlLeft,
                Key::ShiftLeft   => HotKey::ShiftLeft,
                Key::KeyQ        => HotKey::KeyQ,
                Key::KeyC        => HotKey::KeyC,
                _ => HotKey::NoComponent,
            };
        };

        // find event type
        match event {
            EventType::KeyPress(key) => {
                let comp: HotKey = as_hotkey_component(key);
                if comp != HotKey::NoComponent {
                    comp.press_key();
                    // save signal
                    let signal = HotKey::get_hotkey_signal().as_uint();
                    self.set_signal_state(signal);
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
            signal_state: Arc::new(AtomicU8::new(NoSignal.as_uint())),
            logger: Logger::new(),
            metrics: Metrics::new()
        }
    }
}