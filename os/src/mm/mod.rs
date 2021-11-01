pub mod addr;
pub mod page_table;
pub mod physical_mm_manager;

pub fn init() {
    extern "C" {
        fn kernel_end_addr_asm();
    }
    kinfo!("Kernel end_addr 0x{:x}.", kernel_end_addr_asm as usize);
    let mgr = physical_mm_manager::PHYSICAL_MM_MANAGER.lock();
    mgr.init(
        addr::PhysicalAddr(kernel_end_addr_asm as usize),
        addr::PhysicalAddr(kernel_end_addr_asm as usize + 10 * (1 << 20)), // 10M 空间
    );
    let table: page_table::PageTable = mgr.alloc().unwrap().into();
    table.entries.0[0] = table.entries.0[0]
        .with_validation(true)
        .with_permission(true, true, true)
        .with_access_mode(true)
        .with_page_number(0x8050)
}
