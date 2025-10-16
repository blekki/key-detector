use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::sync::Arc;

use rdev;

fn main() {
    let atomic = Arc::new(AtomicBool::new(false));
    let arc_atomic = Arc::clone(&atomic);

    let handle_checker = thread::spawn(move || {
        println!("in loop");
            loop {
                if atomic.load(Ordering::Acquire) == true {
                    println!("log: signal to stop was got");
                    break;
                }
                // have a tiny break
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
    });

    let handle_listen = thread::spawn(move || {
        let callback = move |event: rdev::Event| {
            match event.name {
                Some(key) => {
                    println!("{:?}", key);  // print what user wrote
                    if key == "e" {
                        println!("log: signal to stop was sent");
                        arc_atomic.store(true, Ordering::Relaxed);
                    }
                },
                None => (),
            };
        };
        println!("log: call listener");
        let _ = rdev::listen(callback);     // start rdev::listener thread
        println!("log: after calling");

    });

    let _ = handle_checker.join();
    println!("log: programm success shutdown");
}