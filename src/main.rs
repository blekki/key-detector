use std::thread;
use std::time;

mod listener;
use listener::{*};

fn main() {
    // start listen a keyboard
    let listener = Listener::new();
    
    loop {
        // check when the listener is ready to stop
        if listener.is_stop() {
            break;
        }
        
        // take a break (free CPU to work with other processes)
        thread::sleep(time::Duration::from_millis(50));        
    }
    
    // final log msg
    println!("[main]: programm success shutdown");
}