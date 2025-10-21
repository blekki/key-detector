use std::sync::atomic::{AtomicU8, AtomicBool, Ordering};
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
    // signal: Arc<AtomicU8>,  // todo: rename as "command"
    logic: Arc<Logic>,
    safe_term: Arc<SafeTerm>,

    ready_to_stop: Arc<AtomicBool>,
}

impl Listener {
// ##### PRIVATE AREA #####
    fn run_signal_analyzer(&self) {
        // create Arc-s as a canal between the thread and class
        // let signal_ptr: Arc<AtomicU8> = self.signal.clone();
        let logic_ptr: Arc<Logic> = self.logic.clone();
        let safe_term_ptr: Arc<SafeTerm> = self.safe_term.clone();
        let ready_to_stop_ptr: Arc<AtomicBool> = self.ready_to_stop.clone();

        // create a "signal_analyzer" thread
        let _ = thread::spawn(move || {
            // let mut safe_term = SafeTerm::new();
            let mut signal_copy: u8 = NoSignal.as_num();
            // signal_copy = StopListener.as_num();
            loop {
                // reset signal and wait a bit when new signal come
                // signal_ptr.store(NoSignal.as_num(), Ordering::Relaxed);
                signal_copy = logic_ptr.get_signal_state();

                // hotkeys signal
                // signal_copy = signal_ptr.load(Ordering::Relaxed);
                

                // core signal "the program must be terminated" (highest priority)
                if safe_term_ptr.has_stop_signal_arrived() {
                    break;
                }
                
                // process input signal
                if signal_copy == NoSignal.as_num() { continue; } // do nothing
                if signal_copy == StopListener.as_num() { // stop all internal systems
                    // exit from a loop
                    break;
                }
                if signal_copy == HelloWorld.as_num() { // debug option
                    println!("Hello World!!!");
                    logic_ptr.reset_signal();
                    // signal_ptr.store(NoSignal.as_num(), Ordering::Release);
                    continue;
                }

                // reset signal and wait a bit when new signal come
                // signal_ptr.store(NoSignal.as_num(), Ordering::Release);
                // thread::sleep(time::Duration::from_millis(50));
            }

            // shoutdown all processes
            logic_ptr.shutdown();
            println!("[signal_analyzer]: Logic shutdown");
            // signal to the Listener, it can be completely stopped
            // signal_ptr.store(
            //     AllSystemsIsStopped.as_num(), 
            //     Ordering::Release
            // );
            ready_to_stop_ptr.store(
                true,
                Ordering::Release
            );
        });
    }

    fn run_keyboard_listener(&self) {
        // let signal_ptr: Arc<AtomicU8> = self.signal.clone();
        
        // try to run logger
        let try_to_run = self.logic.logger_start();
        // if something goes wrong, stop processes
        match try_to_run {
            Ok(_) => (),
            Err(msg) => {
                println!("{}", msg);
                return;
            },
        }

        // create a "keyboard_listener" thread
        let logic_ptr: Arc<Logic> = self.logic.clone();
        let _ = thread::spawn(move || {
            // rdevListen callback
            let callback = move |event: Event| {
                // # Note: 
                // Need to use clones, because the "Logic"
                // uses the threads.
                logic_ptr.log_key(
                    event.clone()
                );
                logic_ptr.print_key_in_console(
                    event.name.clone()
                );
                logic_ptr.process_event(
                    event.event_type.clone()
                    // signal_ptr.clone()
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
        // let signal = self.signal.load(Ordering::Acquire);
        // let systems_stopped = AllSystemsIsStopped.as_num(); // signal
        // return signal == systems_stopped;

        self.ready_to_stop.load(Ordering::Acquire)
    }

    // constructor
    pub fn new() -> Self {
        // init parameter
        let listener = Listener{
            // signal: Arc::new(
            //     AtomicU8::new(NoSignal.as_num())
            // ),
            logic: Arc::new(Logic::new()),
            safe_term: Arc::new(SafeTerm::new()),
            ready_to_stop: Arc::new(AtomicBool::new(false))
        };
        // init other essential
        listener.init();
        return listener;
    }
}