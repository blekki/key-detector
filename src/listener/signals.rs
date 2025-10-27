#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub(crate) enum Signals {
    // default
    NoSignal,
    Shutdown,

    // test (debug) signal
    PrintHelloWorld,
}

impl Signals {
    pub fn as_uint(&self) -> u8 {
        return *self as u8;
    }
}