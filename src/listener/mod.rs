use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;

use rdev::Event;

mod signals;
mod logic;
use signals::Signals::{*};
use logic::Logic;

pub struct Listener {
    signal: Arc<AtomicU8>,
}

impl Listener {
// ##### PRIVATE AREA #####
    fn run_signal_analyzer(&self) {
        // create ptr as canal between thread and class
        let signal_ptr = Arc::clone(&self.signal);

        // create a "signal_analyzer" thread
        let _ = thread::spawn(move || {
            loop {
                let signal_copy = signal_ptr.load(Ordering::Acquire);
                
                if signal_copy == ShouldStop.as_num() {
                    break;
                } else if signal_copy == HelloWorld.as_num() {
                    println!("Hello World!!!");    
                }

                // have a tiny break
                thread::sleep(time::Duration::from_millis(50));
            }
        });
    }

    fn run_keyboard_listener(&self) {
        let signal_ptr = Arc::clone(&self.signal);
        
        // create a "keyboard_listener" thread
        let _ = thread::spawn(move || {
            // rdevListen callback
            let callback = move |event: Event| {
                Logic::print_key_in_console(
                    event.name
                );
                Logic::process_event(
                    event.event_type,
                    signal_ptr.clone()
                );
            };

            // create rdevListen
            let _ = rdev::listen(callback);     // start rdev::listener thread
        });
    }

    fn init(&self) {
        self.run_signal_analyzer();     // analyze various signals (especially "exit")
        self.run_keyboard_listener();   // keyboard listener
    }

// ##### PUBLIC AREA #####
    pub fn is_stop(&self) -> bool {
        let signal = self.signal.load(Ordering::Acquire);
        let stop_sign = ShouldStop.as_num();
        
        return signal == stop_sign;
    }

    // constructor
    pub fn new() -> Self {
        // init parameter
        let listener = Listener{
            signal: Arc::new(
                AtomicU8::new(NoSignal.as_num())
            ),
        };
        // init other essential
        listener.init();
        return listener;
    }
}