use core::mem;

use crate::arch::trap;
use crate::mm::addr;
use crate::mm::addr::PAGE_SIZE;
use crate::mm::ppn_manager;

pub struct KernelStack {
    pub ppn: ppn_manager::PhysicalPageNumberGuard,
    raw: &'static mut [u8; addr::PAGE_SIZE],
}

impl KernelStack {
    pub fn get_top(&self) -> usize {
        self.ppn.as_addr() + PAGE_SIZE - mem::size_of::<trap::TrapContext>()
    }
    pub fn get_trap_context(&self) -> &'static trap::TrapContext {
        unsafe {
            (self.get_top() as *const trap::TrapContext)
                .as_ref()
                .unwrap()
        }
    }
    pub fn get_trap_context_mut(&mut self) -> &'static mut trap::TrapContext {
        unsafe { (self.get_top() as *mut trap::TrapContext).as_mut().unwrap() }
    }
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
