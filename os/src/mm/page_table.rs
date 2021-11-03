use super::{
    addr::{self, PhysicalPageNumber, PAGE_SIZE},
    physical_mm_manager::{self, PHYSICAL_MM_MANAGER},
};

#[derive(Clone, Copy)]
pub struct PageTableEntry(usize);

impl PageTableEntry {
    pub fn clear(&mut self) {
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

impl Into<PhysicalPageNumber> for PageTableEntry {
    fn into(self) -> PhysicalPageNumber {
        PhysicalPageNumber(self.0>>44)
    }
}

pub type InnerPageTable = [PageTableEntry; PAGE_SIZE / core::mem::size_of::<PageTableEntry>()];
pub struct PageTable {
    pub physical_page: physical_mm_manager::PhysicalPageGuard, //own
    pub entries: &'static mut InnerPageTable,
    pub level: usize,
}

pub struct PageTables {
    page_tables: [Option<PageTable>; 10],
}

impl PageTables {
    pub fn new()->Self{
        Self{
            page_tables:[Some(PHYSICAL_MM_MANAGER.lock().alloc().unwrap().into()),None,None,None,None,None,None,None,None,None],
        }
    }
    pub fn map(&mut self, vpn: addr::VirtualPageNumber, ppn: addr::PhysicalPageNumber) {
        let mut offsets: [usize; 3] = [0; 3];
        for i in 0..3 {
            offsets[i] = (vpn.0 >> (i * 9)) & (1 << 9 - 1);
        }
        let mut cur_ppn = self.page_tables[0].as_ref().unwrap().physical_page.guard_page_number;
        for (level, &offset) in offsets.iter().skip(1).rev().enumerate() {
            let cur_table = self.find_page_table(level, cur_ppn);
            match cur_table{
                Some(cur_table)=>{
                    let entry = &mut cur_table.entries[offset];
                    if entry.is_valid(){
                        cur_ppn = (*entry).into();
                    } else {
                        let mut page_table :PageTable = PHYSICAL_MM_MANAGER.lock().alloc().unwrap().into();
                        *entry = entry.with_access_mode(true)
                                    .with_validation(true)
                                    .with_permission(true, true, true)
                                    .with_access_mode(true)
                                    .with_page_number(page_table.physical_page.guard_page_number);
                        cur_ppn = (*entry).into();
                        page_table.level = level+1;
                        self.save_page_table(page_table).unwrap();
                    }
                },
                None=>{
                    panic!("bug")
                }
            }
        }
        let cur_table = self.find_page_table(2, cur_ppn).unwrap();
        let entry = &mut cur_table.entries[offsets[0]];
        *entry = entry.with_access_mode(true)
                    .with_validation(true)
                    .with_permission(true, true, true)
                    .with_access_mode(true)
                    .with_page_number(ppn);

    }

    fn save_page_table(
        &mut self,
        table:PageTable,
    ) -> Result<(),&'static str>  {
        for page_table in &mut self.page_tables {
            if let None = page_table{
                *page_table = Some(table);
                return Ok(());
            }
        }
        Err("not enough")
    }
    fn find_page_table(
        &mut self,
        level: usize,
        ppn: addr::PhysicalPageNumber,
    ) -> Option<&mut PageTable> {
        for page_table in &mut self.page_tables {
            if let Some(page_table) = page_table {
                if page_table.level == level && page_table.physical_page.guard_page_number == ppn {
                    return Some(page_table);
                }
            }
        }
        None
    }
}
