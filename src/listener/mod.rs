use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;

mod signals;
use rdev::{Event, EventType, Key};
use signals::Signals::{*};

pub struct Listener {
    signal: Arc<AtomicU8>,
}

impl Listener {
// ##### PRIVATE AREA #####
    fn run_signal_analyzer(&self) {
        // create ptr as canal between thread and class
        let signal_ptr = Arc::clone(&self.signal);

        // create thread
        let signal_analyzer = thread::spawn(move || {
            loop {
                // check does signal to stop processes come
                let should_stop: bool = {
                    signal_ptr.load(Ordering::Acquire) == ShouldStop.into_num()
                };

                if should_stop {
                    break;
                }
                // have a tiny break
                thread::sleep(time::Duration::from_millis(50));
            }
        });
    }

    fn run_keyboard_listener(&self) {
        let signal_ptr = Arc::clone(&self.signal);
        
        // create a thread
        let keyboard_listener = thread::spawn(move || {
            // rdevListen callback
            let callback = move |event: Event| {
                Listener::print_pressed_key(
                    event.name
                );
                Listener::update_logic(
                    event.event_type, 
                    signal_ptr.clone()
                );
                
                // print what user wrote
                // match event.name {
                //     Some(key) => println!("{:?}", key),
                //     None => (),
                // };

                // update logic
                // match event.event_type {
                //     EventType::KeyPress(key) => {
                //         match key {
                //             // rdev::Key::KeyA => println!("key {:?} was pressed", Some(event.name)),
                //             Key::Escape => signal_ptr.store(ShouldStop.into_num(), Ordering::Relaxed),
                //             _ => ()
                //         }
                //     },
                //     // default
                //     _ => ()
                // }
            };

            // create rdevListen
            let _ = rdev::listen(callback);     // start rdev::listener thread
        });
    }

    fn print_pressed_key(key: Option<String>) {
        match key {
            Some(key) => println!("{:?}", key),
            None => (),
        };
    }

    fn update_logic(event: EventType, signal_ptr: Arc<AtomicU8>) {
        match event {
            EventType::KeyPress(key) => {
                match key {
                    Key::Escape => signal_ptr.store(ShouldStop.into_num(), Ordering::Relaxed),
                    _ => ()
                }
            },
            _ => ()
        }
    }

    fn init(&self) {
        self.run_signal_analyzer();     // analyze various signals (especially "exit")
        self.run_keyboard_listener();   // keyboard listener
    }

// ##### PUBLIC AREA #####
    pub fn is_stop(&self) -> bool {
        let signal = self.signal.load(Ordering::Acquire);
        let stop_sign = ShouldStop.into_num();
        
        return signal == stop_sign;
    }

    // constructor
    pub fn new() -> Self {
        // init parameter
        let listener = Listener{
            signal: Arc::new(
                AtomicU8::new(NoSignal.into_num())
            ),
        };
        // init other essential
        listener.init();
        return listener;
    }
}