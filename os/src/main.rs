#![no_std]
#![no_main]
#![feature(global_asm)]

global_asm!(include_str!("main.asm"));

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() -> ! {
    let s = "Hello, world!";
    for ch in s.chars() {
        print_char(ch);
    }
    loop {} //避免代码往下走走到了未知的指令
}

fn print_char(ch: char) {
    ecall(1 as usize, ch as usize, 0, 0);
}

fn ecall(ecall_id: usize, a0: usize, a1: usize, a2: usize) -> usize {
    extern "C" {
        fn ecall_asm(a0: usize, a1: usize, a2: usize, a3: usize) -> usize;
    }
    unsafe { ecall_asm(a0, a1, a2, ecall_id) }
}
