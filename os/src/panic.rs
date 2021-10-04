use crate::arch;
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kprint!("Panic: {}\n", info);
    arch::ecall::shutdown();
}
