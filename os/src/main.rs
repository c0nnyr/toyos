#![no_std]
#![no_main]
#![feature(global_asm)]

use crate::arch::trap::TrapContextStore;

#[no_mangle]
fn main() {
    log::logger::LOGGER
        .lock()
        .init(log::logger::Level::Info, log::logger::LoggerType::SerialIO);
    arch::trap::init();
    kdebug!("Hello, world! {}. Score {}", "Tom", 100);
    kinfo!("Hello, world! {}. Score {}", "Tom", 100);
    kwarn!("Hello, world! {}. Score {}", "Tom", 100);
    kerror!("Hello, world! {}. Score {}", "Tom", 100);
    let mut ctx = arch::trap::TrapContext::default();
    ctx.set_sp(USER_STACK.as_ptr() as u64 + USER_STACK.len() as u64);
    ctx.set_pc(0x80400000);
    ctx.restore_trap();
}
static USER_STACK: [u8; 1024] = [0; 1024];
#[macro_use]
mod io; //出现在早点的位置，这样后面的模块就可以直接使用宏了;
#[macro_use]
mod log; //出现在早点的位置，这样后面的模块就可以直接使用宏了;
mod arch;
mod panic;
