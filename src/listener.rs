use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub struct Listener {
    should_stop: Arc<AtomicBool>,
}

impl Listener {
// ##### private area #####
    fn run_signal_analyzer(&self) {
        // create ptr as canal between thread and class
        let should_stop_ptr = Arc::clone(&self.should_stop);

        // create a "handle_checker" thread
        let _ = thread::spawn(move || {
            loop {
                // check does signal to stop processes come
                if should_stop_ptr.load(Ordering::Acquire) == true {
                    break;
                }
                // have a tiny break
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });
    }

    fn run_keyboard_listener(&self) {
        let should_stop_ptr = Arc::clone(&self.should_stop);
        
        // create a thread "handle_reader"
        let _ = thread::spawn(move || {
            // rdevListen callback
            let callback = move |event: rdev::Event| {
                match event.name {
                    Some(key) => {
                        // print what user wrote
                        println!("{:?}", key);
                        
                        // exit key ("\u{1b}" = "esc" key code)
                        if key == "\u{1b}" {
                            should_stop_ptr.store(true, Ordering::Relaxed);
                        }
                    },
                    None => (),
                };
            };

            // create rdevListen
            let _ = rdev::listen(callback);     // start rdev::listener thread
        });
    }

    fn init(&self) {
        self.run_signal_analyzer();     // analyze various signals (especially "exit")
        self.run_keyboard_listener();   // keyborad listener
    }

// ##### public area #####
    pub fn is_stop(&self) -> bool {
        return self.should_stop.load(Ordering::Acquire);
    }

    // constructor
    pub fn new() -> Self {
        let listener = Listener{
            should_stop: Arc::new(AtomicBool::new(false)),
        };
        listener.init();
        return listener;
    }
}