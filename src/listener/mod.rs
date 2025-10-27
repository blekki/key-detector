use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use rdev::Event;

mod signals;
mod logic;
mod safe_term;
use signals::Signals::{*};
use logic::Logic;

use crate::listener::safe_term::SafeTerm;

pub struct Listener {
    logic: Arc<Logic>,
    safe_term: Arc<SafeTerm>,
    ready_to_stop: Arc<AtomicBool>,
}

impl Listener {
// ##### PRIVATE AREA #####
    fn run_signal_analyzer(&self) {

        // start all internal services
        self.logic.start_logger();
        self.logic.start_metrics();

        // create Arc's as a canal between the thread and class
        let logic_handle: Arc<Logic> = self.logic.clone();
        let safe_term_handle: Arc<SafeTerm> = self.safe_term.clone();
        let ready_to_stop_handle: Arc<AtomicBool> = self.ready_to_stop.clone();

        // create a "signal_analyzer" thread
        let _ = thread::spawn(move || {
            let mut signal_copy: u8;
            loop {
                // core signal "the program must be terminated" (highest priority)
                if safe_term_handle.has_stop_signal_arrived() {
                    break;
                }

                // command (signal) from the "Logic"
                signal_copy = logic_handle.get_signal_state();
                
                // process input signal
                if signal_copy == NoSignal.as_uint() {          // do nothing
                    continue;
                }
                if signal_copy == PrintHelloWorld.as_uint() {   // debug option
                    println!("Hello World!!!");
                    logic_handle.reset_signal();
                    continue;
                }
                if signal_copy == Shutdown.as_uint() {          // stop all internal systems
                    println!("[signal_analyzer] shutdown signal has come");
                    break;
                }
            }

            // shoutdown all internal processes
            logic_handle.shutdown();
            println!("[signal_analyzer]: Logic shutdown");
            
            // signal to the Listener, it can be completely stopped
            ready_to_stop_handle.store( true, Ordering::Release);
        });
    }

    fn run_keyboard_listener(&self) {
        // create a "keyboard_listener" thread
        let logic_handle: Arc<Logic> = self.logic.clone();
        let _ = thread::spawn(move || {
            // rdevListen callback
            let callback = move |event: Event| {
                // # Note: 
                // Need to use clones, because the "Logic" uses the threads.
                logic_handle.log_key(
                    event.clone()
                );
                logic_handle.print_key_in_console(
                    event.name.clone()
                );
                logic_handle.process_event(
                    event.event_type.clone()
                );
                logic_handle.update_metric(
                    event.clone()
                );
            };

            // create rdevListen (start a separated blocking thread)
            let _ = rdev::listen(callback);
        });

    }

    fn init(&self) {
        self.run_signal_analyzer();     // analyze various signals (especially "exit")
        self.run_keyboard_listener();   // keyboard listener
    }

// ##### PUBLIC AREA #####
    pub fn is_stop(&self) -> bool {
        self.ready_to_stop.load(Ordering::Acquire)
    }

    // constructor
    pub fn new() -> Self {
        // init parameter
        let listener = Listener{
            logic:          Arc::new(Logic::new()),
            safe_term:      Arc::new(SafeTerm::new()),
            ready_to_stop:  Arc::new(AtomicBool::new(false))
        };
        // init other essential
        listener.init();
        return listener;
    }
}