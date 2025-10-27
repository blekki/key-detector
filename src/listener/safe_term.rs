use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use ctrlc;

// SafeTerm possible states
const RELAXED:        u8 = 0;
const TERMINATING:    u8 = 1;

// "term" is a shorthand of "termination"
pub struct SafeTerm {
    state: Arc<AtomicU8>
}

impl SafeTerm {
// ##### PRIVATE AREA #####
    fn init(&mut self) {
        
        // catch core signal to terminate program
        let state_handle = self.state.clone();
        let callback = move || {
            state_handle.store(TERMINATING, Ordering::Release);
        };

        // run handling core signal
        ctrlc::set_handler(
            callback
        ).expect("[ctrlc] error setting Ctrl-C handler");
    }

// ##### PUBLIC AREA #####
    pub fn has_stop_signal_arrived(&self) -> bool {
        self.state.load(Ordering::Acquire) == TERMINATING
    }

    // constructor
    pub fn new() -> SafeTerm {
        let mut safe_term = SafeTerm {
            state: Arc::new(AtomicU8::new(RELAXED))
        };
        safe_term.init();
        return safe_term;
    }
}