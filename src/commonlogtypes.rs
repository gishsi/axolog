#[derive(Copy, Clone, clap::ValueEnum, strum_macros::Display, Default)]
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum CommonLogTypes {
    #[default] All,
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