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
    logic: Arc<Logic>,
}

impl Listener {
// ##### PRIVATE AREA #####
    fn run_signal_analyzer(&self) {
        // create ptr as canal between thread and class

        let signal_ptr: Arc<AtomicU8> = self.signal.clone();
        let logic_ptr: Arc<Logic>     = self.logic.clone();

        // create a "signal_analyzer" thread
        let _ = thread::spawn(move || {
            loop {
                let signal_state = signal_ptr.load(Ordering::Acquire);

                // # Note:
                // Here better use if..else..if..else structure,
                // then match.
                
                if signal_state == NoSignal.as_num() {
                    // do nothing
                    continue;
                } else if signal_state == StopListener.as_num() {
                    // stop all internal systems
                    logic_ptr.shoutdown();
                    println!("[signal_analyzer]: Logic shoutdown");

                    break;
                } else if signal_state == HelloWorld.as_num() {
                    println!("Hello World!!!");
                }
                
                // reset signal
                signal_ptr.store(NoSignal.as_num(), Ordering::Release);

                // have a tiny break
                thread::sleep(time::Duration::from_millis(50));
            }

            // signal to the Listener, it can be completely stopped
            signal_ptr.store(
                AllSystemsIsStopped.as_num(), 
                Ordering::Release
            );
        });
    }

    fn run_keyboard_listener(&self) {
        let signal_ptr: Arc<AtomicU8> = self.signal.clone();
        
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
                    event.event_type.clone(),
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
        let systems_stopped = AllSystemsIsStopped.as_num(); // signal
        
        return signal == systems_stopped;
    }

    // constructor
    pub fn new() -> Self {
        // init parameter
        let listener = Listener{
            signal: Arc::new(
                AtomicU8::new(NoSignal.as_num())
            ),
            logic: Arc::new(Logic::new()),
        };
        // init other essential
        listener.init();
        return listener;
    }
}