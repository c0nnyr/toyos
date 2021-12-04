use core::mem;

use crate::arch::trap;
use crate::mm::addr::PAGE_SIZE;
use crate::mm::addr::{self, TRAP_ADDR};
use crate::mm::ppn_manager;

pub struct KernelStack {
    pub ppn: ppn_manager::PhysicalPageNumberGuard,
    pub task_idx: Option<usize>,
    raw: &'static mut [u8; addr::PAGE_SIZE],
}

impl KernelStack {
    pub fn get_idx() -> usize {
        let i = 0;
        (TRAP_ADDR - addr::VirtualPageNumber::floor(&i as *const i32 as usize).as_addr())
            / PAGE_SIZE
            / 2
            - 1
    }

    pub fn get_top(&self) -> usize {
        TRAP_ADDR - 2 * PAGE_SIZE * (self.task_idx.unwrap() + 1) + PAGE_SIZE
            - mem::size_of::<trap::TrapContext>()
    }

    pub fn get_bottom(&self) -> usize {
        TRAP_ADDR - 2 * PAGE_SIZE * (self.task_idx.unwrap() + 1) //乘以2是想留着一页作栈之间的分隔，以便防止栈溢出
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
        Self {
            ppn: v,
            raw,
            task_idx: None,
        }
    }
}
