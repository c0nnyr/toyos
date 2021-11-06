use crate::mm::{addr, page_table};
#[derive(Copy, Clone)]
struct PageTableEntry {
    bits: u64,
}

impl From<page_table::PageTableEntry> for PageTableEntry {
    fn from(v: page_table::PageTableEntry) -> Self {
        let mut bits: u64 = v.ppn.bits as u64;
        bits = bits << 6; //忽略掉RSV和D、A、G的设置，共5bit
        bits = bits | (v.user as u64);
        bits = bits << 1;
        bits = bits | (v.execute as u64);
        bits = bits << 1;
        bits = bits | (v.write as u64);
        bits = bits << 1;
        bits = bits | (v.read as u64);
        bits = bits << 1;
        bits = bits | (v.valid as u64);
        Self { bits }
    }
}

impl Into<page_table::PageTableEntry> for PageTableEntry {
    fn into(self) -> page_table::PageTableEntry {
        let get_bit = |pos: u64| self.bits & (1 << pos) == 1;
        page_table::PageTableEntry {
            ppn: addr::PhysicalPageNumber::from((self.bits >> 10) as usize),
            user: get_bit(4),
            execute: get_bit(3),
            write: get_bit(2),
            read: get_bit(1),
            valid: get_bit(0),
        }
    }
}
