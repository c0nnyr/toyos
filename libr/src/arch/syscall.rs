use super::riscv::syscall as raw_syscall;

enum SyscallId {
    Putchar = 1,
}

fn _syscall(a0: usize, a1: usize, a2: usize, a3: usize, syscall_id: SyscallId) -> usize {
    raw_syscall::syscall(a0, a1, a2, a3, syscall_id as usize)
}

pub fn putchar(ch: char, file_id: usize) -> usize {
    //打印文字到什么文件中
    _syscall(ch as usize, file_id, 0, 0, SyscallId::Putchar)
}
