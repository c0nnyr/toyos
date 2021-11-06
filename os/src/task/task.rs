use crate::{
    arch::trap::{self, TrapContextStore},
    mm::{
        addr::{PhysicalAddr, PAGE_MASK, PAGE_SIZE},
        physical_mm_manager::{PhysicalPageGuard, PHYSICAL_MM_MANAGER},
    },
}; //引入TrapContextStore才能使用TrapContext身上对这个trait的实现

#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    Init,
    Running,
    Stopped,
}

pub struct Task {
    start_addr: usize,
    kernel_stack: KernelStack,
    end_addr: usize,
    trap_context: trap::TrapContext,
    state: TaskState,
}

pub struct KernelStack {
    pub guard_page: PhysicalPageGuard,
    pub array: &'static [u8; PAGE_SIZE],
}

impl KernelStack {
    pub fn get_top(&self) -> usize {
        return self.array.as_ptr() as usize + self.array.len();
    }
}

impl Into<KernelStack> for PhysicalPageGuard {
    fn into(self) -> KernelStack {
        unsafe {
            let addr: PhysicalAddr = self.guard_page_number.into();
            let array = (addr.0 as *const [u8; PAGE_SIZE]).as_ref().unwrap();
            KernelStack {
                guard_page: self, //move
                array,
            }
        }
    }
}
impl Task {
    pub fn new(start_addr: usize, end_addr: usize, stack_bottom: usize) -> Self {
        let kernel_stack: KernelStack = PHYSICAL_MM_MANAGER.lock().alloc().unwrap().into();
        let trap_context = {
            //初始化为这个task最初应该的样子
            let mut ctx = trap::TrapContext::default();
            // ctx.set_sp(stack_bottom as u64);
            ctx.set_sp(0xfffff);
            // ctx.set_pc(super::task_manager::TASK_RUNNING_ADDR as u64);
            ctx.set_pc(0);
            ctx.set_kernel_stack(kernel_stack.get_top() as u64);
            ctx
        };
        Task {
            start_addr,
            end_addr,
            kernel_stack,
            trap_context,
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

    pub fn save_trap_context(&mut self, ctx: &trap::TrapContext) {
        self.trap_context = *ctx;
    }

    pub fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }

    pub fn is_runnable(&self) -> bool {
        self.state == TaskState::Init || self.state == TaskState::Running
    }
}
