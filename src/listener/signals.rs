#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub(crate) enum Signals {
    // default
    NoSignal,
    StopListener,

    // test (debug) signal
    HelloWorld,
}

impl Signals {
    pub fn as_num(&self) -> u8 {
        return *self as u8;
    }
}