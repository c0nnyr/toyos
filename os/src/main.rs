#![no_std]
#![no_main]
#![feature(global_asm)]

#[no_mangle]
fn main() -> ! {
    log::logger::LOGGER
        .lock()
        .init(log::logger::Level::Info, log::logger::LoggerType::SerialIO);
    kprint!("Hello, world! {}. Score {}", "Tom", 100);
    loop {} //避免代码往下走走到了未知的指令
}

#[macro_use]
mod io; //出现在早点的位置，这样后面的模块就可以直接使用宏了;
#[macro_use]
mod log; //出现在早点的位置，这样后面的模块就可以直接使用宏了;
mod arch;
mod panic;
