use crate::mm::{addr::PAGE_SIZE, ppn_manager::PhysicalPageNumberGuard};

pub struct RawPage {
    pub ppn: PhysicalPageNumberGuard,
    pub raw: &'static mut [u8; PAGE_SIZE],
}

impl From<PhysicalPageNumberGuard> for RawPage {
    fn from(v: PhysicalPageNumberGuard) -> Self {
        let raw = unsafe { (v.ppn.as_addr() as *mut [u8; PAGE_SIZE]).as_mut().unwrap() };
        Self { ppn: v, raw }
    }
}

impl RawPage {
    pub fn copy(&mut self, data: &[u8]) {
        assert!(data.len() <= self.raw.len());
        self.raw[..data.len()].copy_from_slice(data);
    }
}
