use crate::arch::ecall;
use crate::arch::syscall;
use crate::task::task;
use crate::task::task_manager;

use super::riscv::register;
use super::riscv::trap;
use super::riscv::trap::TrapContextStoreImpl;

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

pub trait TrapContextStore {
    fn set_sp(&mut self, sp: u64);
    fn set_pc(&mut self, pc: u64);
    fn mv_pc_to_next(&mut self);
    fn restore_trap(&self) -> !;
    fn get_syscall_param(&self) -> syscall::SyscallParam;
    fn set_syscall_return_code(&mut self, code: usize);
    fn set_page_table_root_ppn(&mut self, root_user: u64, root_kernel: u64);
}

pub struct TrapContext {
    store: trap::TrapContextStoreImpl,
}

impl TrapContextStore for TrapContext {
    fn set_page_table_root_ppn(&mut self, root_user: u64, root_kernel: u64) {
        self.store.set_page_table_root_ppn(root_user, root_kernel)
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

pub fn init() {
    trap::init();
}

fn build_next_trap_context() -> &'static TrapContext {
    let mut task_manager = task_manager::TASK_MANAGER.lock();
    let current_idx = task_manager.get_current_idx();
    match task_manager.switch_to_next_task(current_idx + 1) {
        Ok(_) => task_manager.get_current_trap_context(),
        Err(_) => {
            kinfo!("Legacy shutdown now!"); //没有更多应用了
            ecall::shutdown();
        }
    }
}

pub fn dispatch_trap(ctx: &'static mut TrapContext) -> &'static TrapContext {
    let cause = TrapCause::get_current_cause();
    let set_current_task_state = |state: task::TaskState| {
        task_manager::TASK_MANAGER.lock().set_current_state(state);
    };
    match cause {
        TrapCause::Exeption(v) => match v {
            Exception::Syscall => {
                let syscall_param = ctx.get_syscall_param();
                match syscall_param.syscall_id {
                    syscall::SyscallId::Exit => {
                        kinfo!("Task exiting.");
                        set_current_task_state(task::TaskState::Stopped);
                        build_next_trap_context()
                    }
                    syscall::SyscallId::Unsupported(v) => {
                        kerror!("Unsupported syscall {}.", v);
                        set_current_task_state(task::TaskState::Stopped);
                        build_next_trap_context()
                    }
                    syscall::SyscallId::Reschedule => {
                        ctx.mv_pc_to_next();
                        build_next_trap_context()
                    }
                    _ => {
                        let return_code = syscall_param.dispatch_syscall();
                        ctx.set_syscall_return_code(return_code);
                        ctx.mv_pc_to_next();
                        ctx
                    }
                }
            }
            Exception::Unsupported(v) => {
                kerror!("Unsupported trap exception {:?}", v);
                set_current_task_state(task::TaskState::Stopped);
                build_next_trap_context()
            }
        },
        TrapCause::Interrupt(v) => match v {
            Interrupt::Timer => {
                kinfo!("Timer at {:?}", super::time::get_now());
                super::time::set_next_timer(core::time::Duration::from_millis(500));
                build_next_trap_context()
            }
            Interrupt::Unsupported(v) => {
                kerror!("Unsupported trap interrupt {:?}", v);
                set_current_task_state(task::TaskState::Stopped);
                build_next_trap_context()
            }
        },
    }
}
