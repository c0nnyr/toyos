#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {
        let logger:&mut $crate::log::logger::Logger = &mut $crate::log::logger::LOGGER.lock(); //提示编译器先转换成&mut Logger
        core::fmt::write(logger, core::format_args!($($arg)*)).unwrap()}
}
