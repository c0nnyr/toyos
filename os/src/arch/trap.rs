use crate::arch::ecall;
use crate::arch::syscall;

use super::riscv::register;
use super::riscv::trap;

#[derive(Debug)] //方便打印
pub enum Exception {
    Syscall,
    Unsupported(usize), //暂时不对具体的cause做区分，将所有的信息都放在这里面
}
#[derive(Debug)]
pub enum Interrupt {
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
}

pub struct TrapContext {
    store: trap::TrapContextStoreImpl,
}

impl TrapContextStore for TrapContext {
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

impl TrapContext {
    pub fn new(store: &trap::TrapContextStoreImpl) -> Self {
        TrapContext { store: *store }
    }
    pub fn raw(&self) -> &trap::TrapContextStoreImpl {
        &self.store
    }
}

pub fn init() {
    trap::init();
}

pub fn dispatch_trap(ctx: &mut TrapContext) -> &mut TrapContext {
    let cause = TrapCause::get_current_cause();
    match cause {
        TrapCause::Exeption(v) => match v {
            Exception::Syscall => {
                let syscall_param = ctx.get_syscall_param();
                let return_code = syscall_param.dispatch_syscall();
                ctx.set_syscall_return_code(return_code);
                ctx.mv_pc_to_next();
            }
            Exception::Unsupported(v) => {
                panic!("Unsupported trap exception {:?}", v);
            }
        },
        TrapCause::Interrupt(v) => match v {
            Interrupt::Unsupported(v) => {
                panic!("Unsupported trap interrupt {:?}", v);
            }
        },
    }
    ctx
}
