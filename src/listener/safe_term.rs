use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use ctrlc;

const RELAXED:        u8 = 0;
const TERMINATING:    u8 = 1;

// "term" is a shorthand of "terminator"
pub struct SafeTerm {
    state: Arc<AtomicU8>
}

impl SafeTerm {
// ##### PRIVATE AREA #####
    fn init(&mut self) {
        
        // ctrlc callback
        let state_ptr = self.state.clone();
        let callback = move || {
            state_ptr.store(TERMINATING, Ordering::Release); // core send signal to terminate program
        };

        // run handling core signal
        ctrlc::set_handler(
            callback
        ).expect("[ctrlc] error setting Ctrl-C handler");
    }


// ##### PUBLIC AREA #####
    pub fn has_stop_signal_arrived(&self) -> bool {
        return self.state.load(Ordering::Acquire) == TERMINATING;
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