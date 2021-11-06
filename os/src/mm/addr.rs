pub const PAGE_SIZE_BIT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SIZE_BIT; // 4K
#[derive(Copy, Clone, Debug)]
pub struct PhysicalPageNumber {
    pub bits: usize,
}
impl PhysicalPageNumber {
    pub fn ceil(addr: usize) -> Self {
        Self {
            bits: ((addr - 1) >> PAGE_SIZE_BIT) + 1,
        }
    }
    pub fn floor(addr: usize) -> Self {
        Self {
            bits: addr >> PAGE_SIZE_BIT,
        }
    }
}

impl From<usize> for PhysicalPageNumber {
    fn from(v: usize) -> Self {
        Self { bits: v }
    }
}
