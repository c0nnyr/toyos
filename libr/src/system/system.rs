use crate::arch::syscall;
pub fn exit(code: usize) -> ! {
    syscall::exit(code);
    panic!("never here");
}

pub fn get_now() -> core::time::Duration {
    syscall::get_now()
}

pub fn sleep(duration: core::time::Duration) {
    let wakeup_time = get_now() + duration;
    while get_now() < wakeup_time {
        reschedule();
    }
}

pub fn reschedule() -> usize {
    syscall::reschedule()
}
