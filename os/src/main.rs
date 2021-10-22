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
    task::task_manager::init();
    let mut task_manager = task::task_manager::TASK_MANAGER.lock();
    task_manager.switch_to_task(0).unwrap(); //至少得有一个应用
    let ctx = task_manager.get_default_trap_context();
    ctx.restore_trap();
}
#[macro_use]
mod io; //出现在早点的位置，这样后面的模块就可以直接使用宏了;
#[macro_use]
mod log; //出现在早点的位置，这样后面的模块就可以直接使用宏了;
mod arch;
mod panic;
mod task;
