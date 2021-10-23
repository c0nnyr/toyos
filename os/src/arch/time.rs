use super::riscv::time as raw_time;

pub fn get_now() -> core::time::Duration {
    raw_time::get_now()
}
