use crate::arch::ecall;
use crate::arch::syscall;
use crate::task::kernel_stack;
use crate::task::task;
use crate::task::task_manager;

use super::riscv::register;
use super::riscv::switch;
use super::riscv::trap;

#[derive(Debug)] //方便打印
pub enum Exception {
    Syscall,
    Unsupported(usize), //暂时不对具体的cause做区分，将所有的信息都放在这里面
}
#[derive(Debug)]
pub enum Interrupt {
    Timer,
    Unsupported(usize),
}

#[derive(Debug)]
pub enum TrapCause {
    Exeption(Exception),
    Interrupt(Interrupt),
}

pub trait TrapCauseLoader {
    fn load() -> TrapCause;
}

impl TrapCause {
    pub fn get_current_cause() -> Self {
        //Self是对TrapCause的另一种写法，毕竟是在TrapCause的impl上下文里面了
        register::TrapCauseLoaderImpl::load()
    }
}

pub trait TrapContextStore: Default {
    fn set_sp(&mut self, sp: u64);
    fn set_pc(&mut self, pc: u64);
    fn mv_pc_to_next(&mut self);
    fn restore_trap(&self) -> !;
    fn get_syscall_param(&self) -> syscall::SyscallParam;
    fn set_syscall_return_code(&mut self, code: usize);
    fn set_page_table_root_ppn(&mut self, root: u64, is_user: bool);
    fn set_trap_handler(&mut self, handler: u64);
    fn get_task_context_mut(&mut self) -> &mut switch::TaskContext;
    fn get_task_context(&self) -> &switch::TaskContext;
}

pub struct TrapContext {
    store: trap::TrapContextStoreImpl,
}

impl TrapContextStore for TrapContext {
    fn get_task_context_mut(&mut self) -> &mut switch::TaskContext {
        self.store.get_task_context_mut()
    }
    fn get_task_context(&self) -> &switch::TaskContext {
        self.store.get_task_context()
    }
    fn set_page_table_root_ppn(&mut self, root: u64, is_user: bool) {
        self.store.set_page_table_root_ppn(root, is_user)
    }
    fn set_trap_handler(&mut self, handler: u64) {
        self.store.set_trap_handler(handler);
    }
    fn set_sp(&mut self, sp: u64) {
        self.store.set_sp(sp)
    }
    fn set_pc(&mut self, pc: u64) {
        self.store.set_pc(pc)
    }
    fn mv_pc_to_next(&mut self) {
        self.store.mv_pc_to_next()
    }
    fn restore_trap(&self) -> ! {
        self.store.restore_trap()
    }
    fn get_syscall_param(&self) -> syscall::SyscallParam {
        self.store.get_syscall_param()
    }
    fn set_syscall_return_code(&mut self, code: usize) {
        self.store.set_syscall_return_code(code)
    }
}

impl Default for TrapContext {
    fn default() -> Self {
        TrapContext {
            store: trap::TrapContextStoreImpl::default(),
        }
    }
}

pub fn init() {
    trap::init();
}

fn schedule(cur_trap_ctx: &'static TrapContext) {
    let mut task_manager = task_manager::TASK_MANAGER.lock();
    let current_idx = kernel_stack::KernelStack::get_idx();
    match task_manager.switch_to_next_task(current_idx + 1) {
        Ok(idx) => {
            let next_trap_ctx = task_manager.get_task(idx).unwrap().get_trap_context();
            drop(task_manager); //否则切走了
            cur_trap_ctx
                .get_task_context()
                .switch_to(next_trap_ctx.get_task_context());
        }
        Err(_) => {
            kinfo!("Legacy shutdown now!"); //没有更多应用了
            ecall::shutdown();
        }
    }
}

#[no_mangle]
pub fn trap_entry(ctx: &'static mut TrapContext) -> &'static TrapContext {
    let current_idx = kernel_stack::KernelStack::get_idx();
    let cause = TrapCause::get_current_cause();
    let set_current_task_state = |state: task::TaskState| {
        task_manager::TASK_MANAGER
            .lock()
            .get_task_mut(current_idx)
            .unwrap()
            .set_state(state);
    };
    match cause {
        TrapCause::Exeption(v) => match v {
            Exception::Syscall => {
                let syscall_param = ctx.get_syscall_param();
                match syscall_param.syscall_id {
                    syscall::SyscallId::Exit => {
                        kinfo!("Task exiting.");
                        set_current_task_state(task::TaskState::Stopped);
                        schedule(ctx)
                    }
                    syscall::SyscallId::Unsupported(v) => {
                        kerror!("Unsupported syscall {}.", v);
                        set_current_task_state(task::TaskState::Stopped);
                        schedule(ctx)
                    }
                    syscall::SyscallId::Reschedule => {
                        ctx.mv_pc_to_next();
                        schedule(ctx)
                    }
                    _ => {
                        let return_code = syscall_param.dispatch_syscall();
                        ctx.set_syscall_return_code(return_code);
                        ctx.mv_pc_to_next();
                    }
                }
            }
            Exception::Unsupported(v) => {
                kerror!("Unsupported trap exception {:?}", v);
                set_current_task_state(task::TaskState::Stopped);
                schedule(ctx)
            }
        },
        TrapCause::Interrupt(v) => match v {
            Interrupt::Timer => {
                kinfo!("Timer at {:?}", super::time::get_now());
                super::time::set_next_timer(core::time::Duration::from_millis(500));
                schedule(ctx)
            }
            Interrupt::Unsupported(v) => {
                kerror!("Unsupported trap interrupt {:?}", v);
                set_current_task_state(task::TaskState::Stopped);
                schedule(ctx)
            }
        },
    }
    ctx
}
