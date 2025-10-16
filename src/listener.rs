use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;


pub struct Listener {
    should_stop: Arc<AtomicBool>,
}

impl Listener {
// ##### private area #####
    fn create_checker(&self) {
        let should_stop_ptr = Arc::clone(&self.should_stop);

        // create a "handle_checker" thread
        let _ = thread::spawn(move || {
            println!("in loop");
            loop {
                if should_stop_ptr.load(Ordering::Acquire) == true {
                    println!("log: signal to stop was got");
                    break;
                }
                // have a tiny break
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });
    }

    fn create_reader(&self) {
        let should_stop_ptr = Arc::clone(&self.should_stop);
        
        // create a thread "handle_reader"
        let _ = thread::spawn(move || {
            let callback = move |event: rdev::Event| {
                match event.name {
                    Some(key) => {
                        println!("{:?}", key);  // print what user wrote
                        if key == "e" {
                            println!("log: signal to stop was sent");
                            should_stop_ptr.store(true, Ordering::Relaxed);
                        }
                    },
                    None => (),
                };
            };
            // create rdevListen
            println!("log: call listener");
            let _ = rdev::listen(callback);     // start rdev::listener thread
            println!("log: after calling");
        });
    }

    fn init(&self) {
        self.create_checker();
        self.create_reader();
    }

// ##### public area #####
    pub fn is_stop(&self) -> bool {
        // return self.should_stop;
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