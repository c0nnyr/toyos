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
    fn restore_trap(&self) -> !;
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
    fn restore_trap(&self) -> ! {
        self.store.restore_trap()
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
}

pub fn init() {
    trap::init();
}

pub fn dispatch_trap(ctx: &TrapContext) {
    let cause = TrapCause::get_current_cause();
    kinfo!("trap entry with cause {:?}", cause);
    match cause {
        TrapCause::Exeption(v) => match v {
            Exception::Syscall => {
                kinfo!("handling syscall");
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
}
