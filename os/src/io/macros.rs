#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {core::fmt::write(&mut $crate::io::serial_io::SerialIO{}, core::format_args!($($arg)*)).unwrap()}
}
