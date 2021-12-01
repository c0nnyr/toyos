use crate::mm::addr;
use crate::mm::ppn_manager;

pub struct KernelStack {
    pub ppn: ppn_manager::PhysicalPageNumberGuard,
    raw: &'static mut [u8; addr::PAGE_SIZE],
}

impl From<ppn_manager::PhysicalPageNumberGuard> for KernelStack {
    fn from(v: ppn_manager::PhysicalPageNumberGuard) -> Self {
        let raw = unsafe {
            (v.ppn.as_addr() as *mut [u8; addr::PAGE_SIZE])
                .as_mut()
                .unwrap()
        };
        Self { ppn: v, raw }
    }
}
