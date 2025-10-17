use std::thread;
use std::time;

mod listener;

fn main() {
    
    // start listen a keyboard
    let listener = listener::Listener::new();
    
    // check when the listener is ready to stop
    loop {

        if listener.is_stop() { 
            break;
        }
        // take a break (free CPU to work with other processes)
        else {
            thread::sleep(time::Duration::from_millis(50));
        }
        
    }
    
    // final log msg
    println!("[log]: programm success shutdown");
}