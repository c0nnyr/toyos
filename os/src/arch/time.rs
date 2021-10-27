use super::{ecall, riscv::time as raw_time};

pub fn get_now() -> core::time::Duration {
    raw_time::get_now()
}

pub fn enable_time_interrupt() {
    raw_time::enable_time_interrupt()
}

pub fn set_next_timer(duration: core::time::Duration) -> usize {
    ecall::set_next_timer(duration)
}
