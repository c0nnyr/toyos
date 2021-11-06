use super::{addr, ppn_manager};
use crate::arch;

#[derive(Copy, Clone)]
pub struct PageTableEntry {
    pub ppn: addr::PhysicalPageNumber,
    pub valid: bool,   //页表项是否有效
    pub read: bool,    //可读权限
    pub write: bool,   //可写权限
    pub execute: bool, //可执行权限
    pub user: bool,    //用户态是否能访问。用户态能访问的同时内核态不能访问。须知。
}

impl Default for PageTableEntry {
    fn default() -> Self {
        Self {
            ppn: addr::PhysicalPageNumber::from(0),
            valid: false,
            read: false,
            write: false,
            execute: false,
            user: false,
        }
    }
}

pub struct PageTable {
    ppn: ppn_manager::PhysicalPageNumberGuard,
    page_table: &'static mut arch::page_table::PageTableImpl, //赋予静态的生命周期才好管理，这块内存只本结构管理。一定要mut的，否则之后没法修改
}

impl PageTable {
    pub fn set_entry(&mut self, offset: usize, entry: PageTableEntry) {
        self.page_table.set_entry(offset, entry.into());
    }
    pub fn get_entry(&self, offset: usize) -> PageTableEntry {
        self.page_table.get_entry(offset).into()
    }
}

impl From<ppn_manager::PhysicalPageNumberGuard> for PageTable {
    fn from(v: ppn_manager::PhysicalPageNumberGuard) -> Self {
        let page_table = unsafe {
            (v.as_addr() as *mut arch::page_table::PageTableImpl)
                .as_mut()
                .unwrap()
        };
        page_table.clear(); //清空，全部都无效
        let mut ret = Self {
            ppn: v, //ownership move here
            page_table,
        };
        ret
    }
}
