global_asm!(include_str!("switch.asm"));

#[repr(C)]
pub struct TaskContext {
    //x0..=x31
    ctx: [u64; 32],
}

impl Default for TaskContext {
    fn default() -> Self {
        Self { ctx: [0; 32] }
    }
}

impl TaskContext {
    pub fn switch_to(&self, ctx: &'static TaskContext) {
        extern "C" {
            fn switch_asm(ctx_from: &TaskContext, ctx_to: &'static TaskContext);
        }
        unsafe { switch_asm(self, ctx) }
    }

    pub fn set_return_addr(&mut self, ra: usize) {
        self.ctx[1] = ra as u64;
    }

    pub fn set_sp(&mut self, sp: usize) {
        self.ctx[2] = sp as u64;
    }

    pub fn set_param0(&mut self, param0: usize) {
        self.ctx[10] = param0 as u64;
    }
}
