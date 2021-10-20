#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {core::fmt::write(&mut $crate::io::stdout::StdOut{}, core::format_args!($($arg)*)).unwrap()}
}
#[macro_export]
macro_rules! println {
    ($fmt:literal$(,$($arg:tt)*)?) => {core::fmt::write(&mut $crate::io::stdout::StdOut{}, core::format_args!(core::concat!($fmt,"\n")$(,$($arg)*)?)).unwrap()}
}
