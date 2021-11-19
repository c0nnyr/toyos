use super::{
    addr::{self, PAGE_SIZE},
    page_table,
    ppn_manager::{self, PPNManager, PPN_MANAGER},
    raw_page,
};

#[derive(Clone, Copy)]
pub struct Permission {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user: bool,
}

pub const TEXT_PERMISSION: Permission = Permission {
    read: false,
    write: false,
    execute: true,
    user: false,
};
pub const RODATA_PERMISSION: Permission = Permission {
    read: true,
    write: false,
    execute: false,
    user: false,
};
pub const DATA_PERMISSION: Permission = Permission {
    read: true,
    write: true,
    execute: false,
    user: false,
};
pub const BSS_PERMISSION: Permission = Permission {
    read: true,
    write: true,
    execute: false,
    user: false,
};

impl Permission {
    pub fn for_user(&self) -> Self {
        let mut clone = self.clone();
        clone.user = true;
        clone
    }
    pub fn for_kernel(&self) -> Self {
        let mut clone = self.clone();
        clone.user = false;
        clone
    }
}
#[derive(Copy, Clone)]
pub enum MapTarget<'a> {
    Identity,
    Random(Option<&'a [u8]>),
}

// [start_vpn, end_vpn)
#[derive(Copy, Clone)]
pub struct VirtualSection<'a> {
    pub start_vpn: addr::VirtualPageNumber,
    pub end_vpn: addr::VirtualPageNumber,
    pub permission: Permission,
    pub map_target: MapTarget<'a>,
}

impl<'a> VirtualSection<'a> {
    pub fn new(
        start_addr: usize,
        end_addr: usize,
        map_target: MapTarget<'a>,
        permission: Permission,
    ) -> Self {
        Self {
            start_vpn: addr::VirtualPageNumber::floor(start_addr),
            end_vpn: addr::VirtualPageNumber::ceil(end_addr),
            map_target,
            permission,
        }
    }

    pub fn iter(&self) -> VirtualSectionIter {
        VirtualSectionIter {
            section: self,
            cur: self.start_vpn,
        }
    }
}

pub struct VirtualSectionIter<'a> {
    //定义a这个生命周期的变量，指明section的声明周期得和VirtualSectionIter一致。
    section: &'a VirtualSection<'a>,
    cur: addr::VirtualPageNumber,
}

impl Iterator for VirtualSectionIter<'_> {
    type Item = (
        addr::VirtualPageNumber,
        page_table::PageTableEntry,
        Option<raw_page::RawPage>,
    ); // Iterate Trait需要，用于定义next返回的对象
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.bits < self.section.end_vpn.bits {
            //还能迭代
            let ret = self.cur;
            let (ppn, raw_page) = match self.section.map_target {
                MapTarget::Identity => (addr::PhysicalPageNumber::from(ret.bits), None),
                MapTarget::Random(data) => {
                    let mut raw_page: raw_page::RawPage =
                        PPN_MANAGER.lock().alloc().unwrap().into();
                    if let Some(data) = data {
                        let start_addr = (ret.bits - self.section.start_vpn.bits) * PAGE_SIZE;
                        let mut end_addr = (ret.bits - self.section.start_vpn.bits + 1) * PAGE_SIZE;
                        if end_addr > data.len() {
                            end_addr = data.len();
                        }
                        if start_addr < data.len() {
                            raw_page.copy(&data[start_addr..end_addr]);
                        }
                    }
                    (raw_page.ppn.ppn, Some(raw_page))
                }
            };
            let entry = page_table::PageTableEntry {
                ppn: ppn,
                valid: true,
                read: self.section.permission.read,
                write: self.section.permission.write,
                execute: self.section.permission.execute,
                user: self.section.permission.user,
            };
            self.cur.bits += 1;
            Some((ret, entry, raw_page))
        } else {
            //迭代结束
            None
        }
    }
}
