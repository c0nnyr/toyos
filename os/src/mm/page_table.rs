use super::{
    addr,
    ppn_manager::{self, PPNManager},
};
use crate::arch::{
    self,
    page_table::{self, PageTableImpl},
};

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

const BASE_PAGE_TABLE_COUNT: usize = 10;
pub struct PageTableTree {
    page_tables: [Option<PageTable>; BASE_PAGE_TABLE_COUNT], //先定义10个，不满足以后在像动态增加的办法
}

impl PageTableTree {
    pub const fn default() -> Self {
        Self {
            page_tables: [None, None, None, None, None, None, None, None, None, None], //PageTable不是Copy的，所以不能用[None;BASE_PAGE_TABLE_COUNT]初始化
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        self.page_tables[0] = Some(ppn_manager::PPN_MANAGER.lock().alloc()?.into());
        Ok(())
    }

    fn create_table(&mut self) -> Result<usize, &'static str> {
        for (i, table) in self.page_tables.iter().enumerate() {
            if table.is_none() {
                //找到空位
                self.page_tables[i] = Some(ppn_manager::PPN_MANAGER.lock().alloc()?.into());
                return Ok(i);
            }
        }
        Err("no area to save page_table")
    }

    fn find_table(&self, ppn: addr::PhysicalPageNumber) -> Option<usize> {
        for (i, table) in self.page_tables.iter().enumerate() {
            if let Some(table) = table {
                //一种简化的match，这里我们只关心Some的情况
                if table.ppn.as_addr() == ppn.as_addr() {
                    return Some(i);
                }
            }
        }
        None
    }

    fn get_table_mut(&mut self, idx: usize) -> Option<&mut PageTable> {
        self.page_tables[idx].as_mut() //转换&Option<T>为Option<&mut T>
    }

    fn get_table(&self, idx: usize) -> Option<&PageTable> {
        self.page_tables[idx].as_ref() //转换&Option<T>为Option<&T>
    }

    fn get_leaf_table(
        &self,
        offsets: &[usize], //vpn拆解的VPN[0]、VPN[1]、VPN[2]，高位在前
        vpn: addr::VirtualPageNumber,
    ) -> Result<usize, &'static str> {
        let mut cur_table_idx = 0; //从root开始
        for i in 0..offsets.len() - 1 {
            //统一处理前面len-1个offset，页目录。最后一个offset用于直接索引得到entry拿结果
            let offset = offsets[i];
            let entry = self.get_table(cur_table_idx).unwrap().get_entry(offset);
            if entry.valid {
                cur_table_idx = self.find_table(entry.ppn).unwrap(); //此处必然应该能找到，定位到下一个table
            } else {
                return Err("failed to find leaf");
            }
        }
        Ok(cur_table_idx)
    }

    fn get_or_create_leaf_table(
        &mut self,
        offsets: &[usize],
        vpn: addr::VirtualPageNumber,
    ) -> Result<usize, &'static str> {
        let mut cur_table_idx = 0;
        for i in 0..offsets.len() - 1 {
            //统一处理前面len-1个offset，页目录
            let offset = offsets[i];
            let entry = self.get_table_mut(cur_table_idx).unwrap().get_entry(offset);
            if entry.valid {
                cur_table_idx = self.find_table(entry.ppn).unwrap(); //此处必然应该能找到，定位到下一个table
            } else {
                match self.create_table() {
                    Ok(idx) => {
                        //只需要设置这两项，作为目录项
                        let mut entry = PageTableEntry::default();
                        entry.valid = true;
                        entry.ppn = self.get_table_mut(idx).unwrap().ppn.ppn;
                        self.get_table_mut(cur_table_idx)
                            .unwrap()
                            .set_entry(offset, entry);
                        cur_table_idx = idx; //可以指向新的table了
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }
        Ok(cur_table_idx)
    }

    pub fn map(
        &mut self,
        vpn: addr::VirtualPageNumber,
        entry: PageTableEntry,
    ) -> Result<(), &'static str> {
        let offsets = arch::page_table::split_vpn(vpn.bits);
        match self.get_or_create_leaf_table(&offsets, vpn) {
            Ok(idx) => {
                self.get_table_mut(idx)
                    .unwrap()
                    .set_entry(offsets[offsets.len() - 1], entry); //设置一下leaf table的entry
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn unmap(&mut self, vpn: addr::VirtualPageNumber) -> Result<(), &'static str> {
        let offsets = arch::page_table::split_vpn(vpn.bits);
        match self.get_or_create_leaf_table(&offsets, vpn) {
            Ok(idx) => {
                self.get_table_mut(idx)
                    .unwrap()
                    .set_entry(offsets[offsets.len() - 1], PageTableEntry::default()); //清空leaf table的entry
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn translate(&self, virtual_addr: usize) -> Option<usize> {
        let vpn = addr::VirtualPageNumber::floor(virtual_addr);
        let addr_offset = virtual_addr - vpn.as_addr();

        let offsets = arch::page_table::split_vpn(vpn.bits);
        match self.get_leaf_table(&offsets, vpn) {
            Ok(idx) => {
                let entry = self
                    .get_table(idx)
                    .unwrap()
                    .get_entry(offsets[offsets.len() - 1]);
                if entry.valid {
                    Some(entry.ppn.as_addr() + addr_offset)
                } else {
                    None
                }
            }
            Err(err) => None,
        }
    }

    pub fn active(&self){
        arch::page_table::active_page_table_root(self.page_tables[0].as_ref().unwrap().ppn.ppn);
    }
}
