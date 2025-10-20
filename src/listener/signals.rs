#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub(crate) enum Signals {
    NoSignal,

    // ReadyToExit,
    ShouldStop,
    AllSystemsIsStopped,

    StopLogger,
    LoggerReadyShoutdown,

    HelloWorld, // test signal
}

impl Signals {
    pub fn as_num(&self) -> u8 {
        return *self as u8;
    }
}