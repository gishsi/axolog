#[derive(Copy, Clone, clap::ValueEnum, strum_macros::Display)]
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum CommonLogTypes {
    All,
    Info,
    Debug,
    Warn,
    Error,
    Fatal,
    Main,
    MainInfo,
    MainDebug,
    MainWarn,
    MainError
}

impl Default for CommonLogTypes {
    fn default() -> Self {
        CommonLogTypes::All
    }
}