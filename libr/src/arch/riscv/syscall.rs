global_asm!(include_str!("syscall.asm"));

pub fn syscall(a0: usize, a1: usize, a2: usize, a3: usize, syscall_id: usize) -> usize {
    extern "C" {
        fn syscall_asm(a0: usize, a1: usize, a2: usize, a3: usize, syscall_id: usize) -> usize;
    }
    unsafe { syscall_asm(a0, a1, a2, a3, syscall_id) }
}
