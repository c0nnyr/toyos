#[macro_export]
macro_rules! klog {
    ($level:expr, $fmt:literal$(, $($arg:tt)+)?) => {{
        let level = $level;
        let logger:&mut $crate::log::logger::Logger = &mut $crate::log::logger::LOGGER.lock(); //提示编译器先转换成&mut Logger
        if logger.should_log(&level){
            let prefix = logger.get_fmt_prefix(&level);
            let suffix = logger.get_fmt_suffix(&level);
            core::fmt::write(logger, core::format_args!(core::concat!("{}", $fmt, "{}\n"), prefix$(, $($arg)+)?, suffix)).unwrap();
        }
    }}
}

#[macro_export]
macro_rules! kdebug {
    ($fmt:literal$(, $($arg:tt)+)?) => {{
        $crate::klog!($crate::log::logger::Level::Debug, $fmt$(, $($arg)+)?);
    }};
}

#[macro_export]
macro_rules! kinfo {
    ($fmt:literal$(, $($arg:tt)+)?) => {{
        $crate::klog!($crate::log::logger::Level::Info, $fmt$(, $($arg)+)?);
    }};
}

#[macro_export]
macro_rules! kwarn {
    ($fmt:literal$(, $($arg:tt)+)?) => {{
        $crate::klog!($crate::log::logger::Level::Warn, $fmt$(, $($arg)+)?);
    }};
}

#[macro_export]
macro_rules! kerror {
    ($fmt:literal$(, $($arg:tt)+)?) => {{
        $crate::klog!($crate::log::logger::Level::Error, $fmt$(, $($arg)+)?);
    }};
}
