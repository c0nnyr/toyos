#![no_std]
#![no_main]
#![feature(global_asm)]

use crate::{
    arch::trap::{self, TrapContextStore},
    mm::addr::PhysicalAddr,
};

#[no_mangle]
fn main() {
    extern "C" {
        fn kernel_end_addr_asm();
    }
    log::logger::LOGGER
        .lock()
        .init(log::logger::Level::Info, log::logger::LoggerType::SerialIO);
    kinfo!("Kernel end_addr 0x{:x}.", kernel_end_addr_asm as usize);
    arch::trap::init();
    mm::physical_mm_manager::PHYSICAL_MM_MANAGER.lock().init(
        PhysicalAddr(kernel_end_addr_asm as usize),
        PhysicalAddr(kernel_end_addr_asm as usize + 10 * (1 << 20)), // 10M 空间
    );
    task::task_manager::init();
    arch::time::enable_time_interrupt();
    arch::time::set_next_timer(core::time::Duration::from_millis(500));
    build_first_task_trap_context().restore_trap();
}
// 这里必须依靠一个函数，及时释放task_manager获得的锁。
fn build_first_task_trap_context() -> trap::TrapContext {
    let mut task_manager = task::task_manager::TASK_MANAGER.lock();
    task_manager.switch_to_task(0).unwrap(); //至少得有一个应用
    task_manager.get_current_trap_context()
}
#[macro_use]
mod io; //出现在早点的位置，这样后面的模块就可以直接使用宏了;
#[macro_use]
mod log; //出现在早点的位置，这样后面的模块就可以直接使用宏了;
mod arch;
mod mm;
mod panic;
mod task;
