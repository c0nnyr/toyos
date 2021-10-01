#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kprint!("Panic: {}\n", info);
    loop {}
}
