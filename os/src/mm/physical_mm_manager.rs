use super::addr::{PhysicalAddr, PhysicalPageNumber};

const MAX_MANAGE_PAGE_NUM: usize = 2560; //能管理2560*4K=10M空间
                                         // 管理[start, end)的区域，start mod 4K = 0, end mod 4K = 0;
pub struct PhysicalMMManager {
    start: PhysicalPageNumber,
    end: PhysicalPageNumber,
    bitmap: [bool; MAX_MANAGE_PAGE_NUM],
}

pub static PHYSICAL_MM_MANAGER: spin::Mutex<PhysicalMMManager> =
    spin::Mutex::new(PhysicalMMManager {
        start: PhysicalPageNumber(0),
        end: PhysicalPageNumber(0),
        bitmap: [false; MAX_MANAGE_PAGE_NUM],
    });
pub struct PhysicalFrameGuard {
    guard_page_number: PhysicalPageNumber,
}

impl Drop for PhysicalFrameGuard {
    fn drop(&mut self) {
        PHYSICAL_MM_MANAGER.lock().free(self.guard_page_number);
        self.guard_page_number = PhysicalPageNumber(core::usize::MAX);
    }
}

impl PhysicalMMManager {
    pub fn init(&mut self, start: PhysicalAddr, end: PhysicalAddr) {
        assert!(start.is_page_start());
        assert!(end.is_page_start());
        self.start = start.into();
        self.end = end.into();
        assert!(self.end.0 - self.start.0 <= MAX_MANAGE_PAGE_NUM);
    }

    fn is_occupied(&self, cur: PhysicalPageNumber) -> bool {
        self.bitmap[cur.0 - self.start.0]
    }

    fn set_occupied(&mut self, cur: PhysicalPageNumber, b: bool) {
        self.bitmap[cur.0 - self.start.0] = b;
    }

    pub fn alloc(&mut self) -> Option<PhysicalFrameGuard> {
        let mut cur = self.start;
        while cur < self.end {
            if !self.is_occupied(cur) {
                self.set_occupied(cur, true);
                return Some(PhysicalFrameGuard {
                    guard_page_number: cur,
                });
            }
        }
        None
    }

    pub fn free(&mut self, cur: PhysicalPageNumber) {
        assert!(self.is_occupied(cur));
        self.set_occupied(cur, false);
    }
}
