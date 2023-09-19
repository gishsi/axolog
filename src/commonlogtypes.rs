/// Common log types in an unstructured log file, defaults to All 
#[derive(Copy, Clone, clap::ValueEnum, strum_macros::Display, Default, Hash, Eq, PartialEq, Debug)]
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