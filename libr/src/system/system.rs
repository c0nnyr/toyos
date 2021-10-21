use crate::arch::syscall;
pub fn exit(code: usize) -> ! {
    syscall::exit(code);
    panic!("never here");
}
