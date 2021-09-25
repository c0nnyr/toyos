#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn _start() -> ! {
    let s = "Hello, world!";
    for ch in s.chars() {
        print_char(ch);
    }
    loop {} //避免代码往下走走到了未知的指令
}

fn print_char(ch: char) {
    //TODO print a characture
}
