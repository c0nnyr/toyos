use core::mem::size_of;

use crate::mm::{addr, page_table};

use super::register::SAtp;
#[derive(Copy, Clone)]
pub struct PageTableEntryImpl {
    bits: u64,
}

impl From<page_table::PageTableEntry> for PageTableEntryImpl {
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

impl Into<page_table::PageTableEntry> for PageTableEntryImpl {
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

pub struct PageTableImpl {
    entries: [PageTableEntryImpl; addr::PAGE_SIZE / size_of::<PageTableEntryImpl>()],
}

impl PageTableImpl {
    pub fn set_entry(&mut self, offset: usize, entry: PageTableEntryImpl) {
        self.entries[offset] = entry;
    }
    pub fn get_entry(&self, offset: usize) -> PageTableEntryImpl {
        self.entries[offset]
    }
    pub fn clear(&mut self) {
        for entry in &mut self.entries {
            *entry = PageTableEntryImpl { bits: 0 };
        }
    }
}

pub fn split_vpn(addr: usize) -> [usize; 3] {
    let mut offsets = [0; 3];
    for i in 0..3 {
        offsets[2 - i] = (addr >> i * 9) & 511; //提取9bit，最高9bit放在[0]，最低放在[2]
    }
    offsets
}

pub fn active_page_table_root(ppn:addr::PhysicalPageNumber){
    SAtp::from_ppn(ppn.bits).set()
}
