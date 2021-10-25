use crate::arch::trap::{self, TrapContextStore}; //引入TrapContextStore才能使用TrapContext身上对这个trait的实现

#[derive(Clone, Copy)]
pub enum TaskState {
    Init,
    Running,
    Stopped,
}

#[derive(Clone, Copy)]
pub struct Task {
    start_addr: usize,
    end_addr: usize,
    trap_context: trap::TrapContext,
    state: TaskState,
}

impl Task {
    pub fn new(start_addr: usize, end_addr: usize, stack_bottom: usize) -> Self {
        Task {
            start_addr,
            end_addr,
            trap_context: { //初始化为这个task最初应该的样子
                let mut ctx = trap::TrapContext::default();
                ctx.set_sp(stack_bottom as u64);
                ctx.set_pc(super::task_manager::TASK_RUNNING_ADDR as u64);
                ctx
            },
            state: TaskState::Init,
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

    pub fn get_trap_context(&self) -> trap::TrapContext {
        self.trap_context
    }

    pub fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }
}
