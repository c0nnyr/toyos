use super::addr;

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
