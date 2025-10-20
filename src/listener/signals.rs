#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub(crate) enum Signals {
    // default
    NoSignal,
    
    // for the Listener
    StopListener,
    AllSystemsIsStopped,

    // for the Logger
    StopLogger,
    LoggerReadyShoutdown,

    // test (debug) signal
    HelloWorld,
}

impl Signals {
    pub fn as_num(&self) -> u8 {
        return *self as u8;
    }
}