#![no_std]
#![no_main]
#![feature(global_asm)]

#[no_mangle]
fn main() {
    log::logger::LOGGER
        .lock()
        .init(log::logger::Level::Info, log::logger::LoggerType::SerialIO);
    kdebug!("Hello, world! {}. Score {}", "Tom", 100);
    kinfo!("Hello, world! {}. Score {}", "Tom", 100);
    kwarn!("Hello, world! {}. Score {}", "Tom", 100);
    kerror!("Hello, world! {}. Score {}", "Tom", 100);
    // arch::ecall::shutdown();
}
#[no_mangle]
fn trap_entry() {
    kinfo!("trap entry");
}

#[macro_use]
mod io; //出现在早点的位置，这样后面的模块就可以直接使用宏了;
#[macro_use]
mod log; //出现在早点的位置，这样后面的模块就可以直接使用宏了;
mod arch;
mod panic;
