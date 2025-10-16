#[derive(Copy, Clone)]
pub(crate) enum Signals {
    NoSignal,
    ShouldStop,
}

impl Signals {
    pub(crate) fn into_num(&self) -> u8 {
        return *self as u8;
    }
} 