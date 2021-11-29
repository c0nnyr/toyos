use super::riscv::switch;

pub trait TaskContextStore {
    fn switch_to(&self, ctx: &'static switch::TaskContextStoreImpl);
    fn set_idx(&mut self, idx: usize);
    fn get_idx(&self) -> usize;
    fn set_ra(&mut self, ra: usize);
    fn set_sp(&mut self, sp: usize);
    fn set_param0(&mut self, param0: usize);
}

pub struct TaskContext {
    store: switch::TaskContextStoreImpl,
}
impl Default for TaskContext {
    fn default() -> Self {
        Self {
            store: switch::TaskContextStoreImpl::default(),
        }
    }
}

impl TaskContext {
    pub fn switch_to(&self, ctx: &'static TaskContext) {
        self.store.switch_to(&ctx.store)
    }
    pub fn set_idx(&mut self, idx: usize) {
        self.store.set_idx(idx)
    }
    pub fn get_idx(&self) -> usize {
        self.store.get_idx()
    }
    pub fn set_ra(&mut self, ra: usize) {
        self.store.set_ra(ra);
    }
    pub fn set_sp(&mut self, sp: usize) {
        self.store.set_sp(sp);
    }
    pub fn set_param0(&mut self, param0: usize) {
        self.store.set_param0(param0);
    }
}
