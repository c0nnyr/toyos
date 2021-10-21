use crate::system;
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Panic: {}\n", info);
    system::exit(!0); //全1的错误吗
}
