use super::addr;

#[derive(Debug)]
pub struct PhysicalPageNumberGuard {
    pub ppn: addr::PhysicalPageNumber,
}

impl Drop for PhysicalPageNumberGuard {
    fn drop(&mut self) {
        kinfo!("dropping ppn: {}", self.ppn);
        PPN_MANAGER.lock().free(self.ppn);
    }
}

impl core::fmt::Display for PhysicalPageNumberGuard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.ppn))
    }
}

pub trait PPNManager {
    fn add_memory(&mut self, start_addr: usize, end_addr: usize);
    fn alloc(&mut self) -> Result<PhysicalPageNumberGuard, &'static str>;
    fn free(&mut self, ppn: addr::PhysicalPageNumber);
}

const MAX_MANAGED_PAGE: usize = 256;
//简单的管理1M内存的PPNManager实现
pub struct PPNManagerImpl {
    start_ppn: Option<addr::PhysicalPageNumber>,
    bitmap: [bool; MAX_MANAGED_PAGE], //能力有限，管理1M
}

impl PPNManager for PPNManagerImpl {
    fn add_memory(&mut self, start_addr: usize, end_addr: usize) {
        if self.start_ppn.is_some() {
            //已经初始化了，报错
            panic!("must init only once");
        }
        self.start_ppn = Some(addr::PhysicalPageNumber::ceil(start_addr));
        let end_ppn = addr::PhysicalPageNumber::floor(end_addr);
        self.bitmap = [true; MAX_MANAGED_PAGE];
        for i in 0..self.bitmap.len() {
            if self.start_ppn.unwrap().bits + i >= end_ppn.bits {
                break;
            }
            self.bitmap[i] = false;
        }
        kinfo!(
            "memory added: from 0x{:x} to 0x{:x}",
            self.start_ppn.unwrap().bits,
            end_ppn.bits
        );
    }

    fn alloc(&mut self) -> Result<PhysicalPageNumberGuard, &'static str> {
        for i in 0..self.bitmap.len() {
            //遍历，获取一页没有占用的
            if !self.bitmap[i] {
                self.bitmap[i] = true;
                return Ok(PhysicalPageNumberGuard {
                    ppn: addr::PhysicalPageNumber::from(self.start_ppn.unwrap().bits + i),
                });
            }
        }
        Err("failed to alloc")
    }

    fn free(&mut self, ppn: addr::PhysicalPageNumber) {
        self.bitmap[ppn.bits - self.start_ppn.unwrap().bits] = false
    }
}

pub static PPN_MANAGER: spin::Mutex<PPNManagerImpl> = spin::Mutex::new(PPNManagerImpl {
    start_ppn: None,
    bitmap: [true; MAX_MANAGED_PAGE], //一开始全部都是占用的
});

pub fn init() {
    extern "C" {
        fn kernel_end_asm();
    }
    //将内核结束地址到引用程序开始地址管理起来
    PPN_MANAGER.lock().add_memory(
        kernel_end_asm as usize,
        crate::task::task_manager::TASK_RUNNING_ADDR,
    );
}
