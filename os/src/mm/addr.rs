pub const PAGE_MASK_BITS: usize = 12;
pub const PAGE_SIZE: usize = 1 << 12;
pub const PAGE_MASK: usize = PAGE_SIZE - 1;
#[derive(Clone, Copy)]
pub struct PhysicalAddr(pub usize);

impl PhysicalAddr {
    pub fn is_page_start(&self) -> bool {
        self.0 & PAGE_MASK == 0
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct PhysicalPageNumber(pub usize);
impl From<PhysicalAddr> for PhysicalPageNumber {
    fn from(v: PhysicalAddr) -> Self {
        PhysicalPageNumber(v.0 >> PAGE_MASK_BITS)
    }
}
