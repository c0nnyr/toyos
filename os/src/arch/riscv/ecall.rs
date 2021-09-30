global_asm!(include_str!("ecall.asm"));
pub fn ecall(ecall_id: usize, a0: usize, a1: usize, a2: usize) -> usize {
    extern "C" {
        fn ecall_asm(a0: usize, a1: usize, a2: usize, a3: usize) -> usize;
    }
    unsafe { ecall_asm(a0, a1, a2, ecall_id) }
}
