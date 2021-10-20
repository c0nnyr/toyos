use crate::arch::syscall;
pub struct StdOut {}
const STD_OUT_FILE_ID: usize = 1; //传统

impl core::fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            syscall::putchar(ch, STD_OUT_FILE_ID);
        }
        Ok(())
    }
}
