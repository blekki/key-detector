use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use rdev::{EventType, Key};

use super::signals;
use signals::Signals::{*};

pub struct Logic;

impl Logic {
    // key printer
    pub fn print_key_in_console(key: Option<String>) {
        match key {
            Some(key) => println!("{}", key),
            None => (),
        };
    }

    // return a signal after process the event
    pub fn process_event(event: EventType, signal_ptr: Arc<AtomicU8>) {
        // find event type
        match event {
            EventType::KeyPress(key) => {
                // find key
                match key {
                    Key::Escape => signal_ptr.store(ShouldStop.into_num(), Ordering::Relaxed),
                    _ => ()
                }
            },
            _ => ()
        }
    }
}