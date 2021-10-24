use super::ecall;
use super::time;

pub enum SyscallId {
    Putchar,
    Exit,
    GetNow,
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
            3 => SyscallId::GetNow,
            _ => SyscallId::Unsupported(v),
        }
    }
}

impl SyscallParam {
    pub fn dispatch_syscall(&self) -> usize {
        match self.syscall_id {
            SyscallId::Putchar => ecall::putchar_serialio(self.params[0] as u8 as char),
            SyscallId::GetNow => time::get_now().as_millis() as usize,
            SyscallId::Exit | SyscallId::Unsupported(_) => {
                panic!("never here"); //外面处理
            }
        }
    }
}
