#[cfg(target_arch = "riscv64")]
#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    crate::println!("Panic occurred: {}", panic_info);
    crate::halt(-1)
}
