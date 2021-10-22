#[derive(Clone, Copy)]
pub struct Task {
    start_addr: usize,
    end_addr: usize,
}

impl Task {
    pub fn new(start_addr: usize, end_addr: usize) -> Self {
        Task {
            start_addr,
            end_addr,
        }
    }

    pub fn get_code(&self) -> &[u8] {
        unsafe {
            // 从直接裸的指针返回slice，不安全，因为这块内存谁是owner，谁会改动，rust不知道，只有我们写程序的作者知道。
            // 我们是确信的，这块内存是只读的，没人改，因而返回一个只读的slice引用是安全的。
            core::slice::from_raw_parts(
                self.start_addr as *const u8,
                self.end_addr - self.start_addr,
            )
        }
    }
}
