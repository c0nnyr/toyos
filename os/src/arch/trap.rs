use super::riscv::register;
#[derive(Debug)] //方便打印
pub enum Exception {
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
