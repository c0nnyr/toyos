use super::ecall;

pub enum SyscallId {
    Putchar,
    Exit,
    Unsupported(usize),
}

pub struct SyscallParam {
    pub params: [usize; 4], //支持传递4个参数
    pub syscall_id: SyscallId,
}

impl From<usize> for SyscallId {
    fn from(v: usize) -> Self {
        match v {
            1 => SyscallId::Putchar,
            2 => SyscallId::Exit,
            _ => SyscallId::Unsupported(v),
        }
    }
}

impl SyscallParam {
    pub fn dispatch_syscall(&self) -> usize {
        match self.syscall_id {
            SyscallId::Putchar => ecall::putchar_serialio(self.params[0] as u8 as char),
            SyscallId::Exit | SyscallId::Unsupported(_) => {
                panic!("never here"); //外面处理
            }
        }
    }
}
