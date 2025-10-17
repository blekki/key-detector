#[derive(Copy, Clone)]
pub(crate) enum Signals {
    NoSignal,
    ShouldStop,
    HelloWorld, // test signal
}

impl Signals {
    pub(crate) fn as_num(&self) -> u8 {
        return *self as u8;
    }
}