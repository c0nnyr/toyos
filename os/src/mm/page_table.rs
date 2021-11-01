use crate::task::task_manager::MAX_TASK_NUM;

use super::{
    addr::{self, PhysicalPageNumber, PAGE_SIZE},
    physical_mm_manager::{self, PHYSICAL_MM_MANAGER},
};

#[derive(Clone, Copy)]
struct PageTableEntry(usize);

impl PageTableEntry {
    pub fn empty(&mut self) {
        self.0 = 0;
    }
    fn set_bit(&mut self, pos: usize, v: bool) {
        assert!(pos < 63);
        if v {
            self.0 = self.0 | (1 << pos);
        } else {
            self.0 = self.0 & !(1 << pos);
        }
    }
    fn get_bit(&self, pos: usize) -> bool {
        self.0 & (1 << pos) == 1
    }
    //访问权限
    pub fn with_permission(&self, read: bool, write: bool, execute: bool) -> Self {
        let mut new = *self;
        new.set_bit(1, read);
        new.set_bit(2, write);
        new.set_bit(3, execute);
        new
    }

    //用户态是否能访问
    pub fn with_access_mode(&self, user: bool) -> Self {
        let mut new = *self;
        new.set_bit(4, user);
        new
    }

    //是否有效
    pub fn with_validation(&self, validation: bool) -> Self {
        let mut new = *self;
        new.set_bit(0, validation);
        new
    }
    pub fn is_valid(&self) -> bool {
        self.get_bit(0)
    }
    pub fn with_page_number(&self, page_number: PhysicalPageNumber) -> Self {
        assert!(page_number.0 < (1 << 44));
        Self(page_number.0 << 44 | (self.0 & ((1 << 44) - 1)))
    }
}

pub struct InnerPageTable(pub [PageTableEntry; PAGE_SIZE / core::mem::size_of::<PageTableEntry>()]);
pub struct PageTable {
    pub physical_page: physical_mm_manager::PhysicalPageGuard,
    pub entries: &'static InnerPageTable,
    level: usize,
}

pub struct PageTables {
    page_tables: [Option<PageTable>; 10],
}

impl PageTables {
    pub fn map(&mut self, vpn: addr::VirtualPageNumber, ppn: addr::PhysicalPageNumber) {
        let mut offsets: [usize; 3] = [0; 3];
        for i in (0..3).rev() {
            offsets[i] = (vpn.0 >> (i * 9)) & (1 << 9 - 1);
        }
        for offset in offsets {
            let page_table = &self.page_tables[0].unwrap();
        }
    }

    fn find_or_create_page_table(
        &mut self,
        level: usize,
        ppn: addr::PhysicalPageNumber,
    ) -> Option<&PageTable> {
        for page_table in &self.page_tables {
            if let Some(page_table) = page_table {
                if page_table.level == level && page_table.physical_page.guard_page_number == ppn {
                    return Some(page_table);
                }
            }
        }
        let table: PageTable = PHYSICAL_MM_MANAGER.lock().alloc().unwrap().into();
        for (i, page_table) in self.page_tables.iter().enumerate() {
            if let None = page_table {
                self.page_tables[i] = Some(table);
                return Some(&self.page_tables[i].unwrap());
            }
        }
        return None;
    }
}

impl PageTable {
    pub fn map(&mut self, addr: addr::PhysicalPageNumber) {
        let offset = addr.0 >> (self.level * 9) & (1 << 9 - 1);
        let entry = &mut self.entries.0[offset];
        if self.level == 0 {
            assert!(!entry.is_valid()); //不允许重复映射
            *entry = entry
                .with_validation(true)
                .with_permission(true, true, true)
                .with_access_mode(true)
                .with_page_number(addr);
        } else {
            if entry.is_valid() {
                // PHYSICAL_MM_MANAGER.lock().alloc()
            } else {
                let mgr = PHYSICAL_MM_MANAGER.lock();
                let table: PageTable = mgr.alloc().unwrap().into();
                *entry = entry
                    .with_validation(true)
                    .with_permission(true, true, true)
                    .with_access_mode(true)
                    .with_page_number(table.physical_page.guard_page_number);
            }
        }
    }
}
