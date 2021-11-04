pub mod addr;
pub mod page_table;
pub mod physical_mm_manager;
use crate::arch::riscv::register;

pub fn init() {
    init_physical_mm();
    init_kernel_table();
}

pub static KERNEL_PAGE_TABLES: spin::Mutex<page_table::PageTables> =
    spin::Mutex::new(page_table::PageTables {
        page_tables: [None, None, None, None, None, None, None, None, None, None],
    });

fn init_kernel_table() {
    let mut tables = page_table::PageTables::new();
    for i in 0x80200..0x80900 {
        tables.map(addr::VirtualPageNumber(i), addr::PhysicalPageNumber(i));
    }
    for i in 0x0..0x100 {
        tables.map(
            addr::VirtualPageNumber(i),
            addr::PhysicalPageNumber(i + 0x80400),
        );
    }
    register::SAtp::new()
        .with_root_ppn(tables.get_root().0)
        .set();
    kinfo!("map done!");
    KERNEL_PAGE_TABLES.lock().page_tables = tables.page_tables;
}

fn init_physical_mm() {
    extern "C" {
        fn kernel_end_addr_asm();
    }
    kinfo!("Kernel end_addr 0x{:x}.", kernel_end_addr_asm as usize);
    physical_mm_manager::PHYSICAL_MM_MANAGER.lock().init(
        addr::PhysicalAddr(kernel_end_addr_asm as usize),
        addr::PhysicalAddr(kernel_end_addr_asm as usize + 1 * (1 << 20)), // 1M 空间
    );
}
