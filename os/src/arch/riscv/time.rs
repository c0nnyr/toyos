pub fn get_now() -> core::time::Duration {
    super::register::Time::load().as_duration()
}
