pub mod addr;
pub mod page_table;
pub mod physical_mm_manager;

pub fn init() {
    extern "C" {
        fn kernel_end_addr_asm();
    }
    kinfo!("Kernel end_addr 0x{:x}.", kernel_end_addr_asm as usize);
    {
        let mut mgr = physical_mm_manager::PHYSICAL_MM_MANAGER.lock();
        mgr.init(
            addr::PhysicalAddr(kernel_end_addr_asm as usize),
            addr::PhysicalAddr(kernel_end_addr_asm as usize + 10 * (1 << 20)), // 10M 空间
        );
    }
    {
        let mut tables = page_table::PageTables::new();
        tables.map(addr::VirtualPageNumber(0), addr::PhysicalPageNumber(1));
    }
}
