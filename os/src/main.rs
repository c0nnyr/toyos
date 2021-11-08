#![no_std]
#![no_main]
#![feature(global_asm)]

use mm::{addr, page_table::PageTableEntry, page_table::PageTableTree};

use crate::arch::trap::{self, TrapContextStore};

#[no_mangle]
fn main() {
    log::logger::LOGGER
        .lock()
        .init(log::logger::Level::Info, log::logger::LoggerType::SerialIO);
    mm::ppn_manager::init();
    {
        let mut page_table_tree = PageTableTree::default();
        page_table_tree.init().unwrap();
        page_table_tree
            .map(
                addr::VirtualPageNumber::from(0x0),
                PageTableEntry {
                    ppn: addr::PhysicalPageNumber::from(0x1),
                    valid: true,
                    read: true,
                    write: true,
                    execute: true,
                    user: true,
                },
            )
            .unwrap();
        kinfo!(
            "translate 0 => 0x{:x}",
            page_table_tree.translate(0).unwrap()
        );
        kinfo!(
            "translate 1 => 0x{:x}",
            page_table_tree.translate(1).unwrap()
        );
        kinfo!("unmap 0");
        page_table_tree.unmap(addr::VirtualPageNumber::from(0x0));
        kinfo!("translate 1 => {:?}", page_table_tree.translate(1));
    }
    arch::trap::init();
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
