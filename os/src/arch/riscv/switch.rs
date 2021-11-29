global_asm!(include_str!("switch.asm"));

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TaskContextStoreImpl {
    //x0..=x31, idx 33个
    ctx: [u64; 33],
}

impl Default for TaskContextStoreImpl {
    fn default() -> Self {
        Self { ctx: [0; 33] }
    }
}
impl crate::arch::switch::TaskContextStore for TaskContextStoreImpl {
    fn switch_to(&self, ctx: &'static TaskContextStoreImpl) {
        extern "C" {
            fn switch_asm(ctx_from: &TaskContextStoreImpl, ctx_to: &'static TaskContextStoreImpl);
        }
        unsafe { switch_asm(self, ctx) }
    }

    fn set_idx(&mut self, idx: usize) {
        self.ctx[32] = idx as u64;
    }

    fn get_idx(&self) -> usize {
        self.ctx[32] as usize
    }

    fn set_ra(&mut self, ra: usize) {
        self.ctx[1] = ra as u64;
    }
    fn set_sp(&mut self, sp: usize) {
        self.ctx[2] = sp as u64;
    }
    fn set_param0(&mut self, param0: usize) {
        self.ctx[10] = param0 as u64;
    }
}
