use super::riscv::page_table as raw_page_table;

pub use raw_page_table::split_vpn;
pub use raw_page_table::PageTableEntryImpl;
pub use raw_page_table::PageTableImpl;
