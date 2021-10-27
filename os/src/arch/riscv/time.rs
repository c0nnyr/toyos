pub fn get_now() -> core::time::Duration {
    super::register::Time::load().as_duration()
}

pub fn enable_time_interrupt() {
    super::register::SIe::enable_time_interrupt()
}
