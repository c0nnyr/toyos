#![no_std]
#![no_main]
#![feature(global_asm)]

global_asm!(include_str!("main.asm"));

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

macro_rules! kprint {
    ($($arg:tt)*) => {core::fmt::write(&mut SerialIO{}, core::format_args!($($arg)*)).unwrap()}
}

struct SerialIO {}

impl core::fmt::Write for SerialIO {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            print_char(ch);
        }
        Ok(()) //这里是返回值，等价于return Ok(())。OK是Result的一个枚举类型
    }
}
#[no_mangle]
fn main() -> ! {
    kprint!("Hello, world! {}. Score {}", "Tom", 100);
    loop {} //避免代码往下走走到了未知的指令
}

fn print_char(ch: char) {
    ecall(1 as usize, ch as usize, 0, 0);
}

mod arch;
use arch::ecall::ecall;
