pub enum SyscallId {
    Putchar,
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
            _ => SyscallId::Unsupported(v),
        }
    }
}
