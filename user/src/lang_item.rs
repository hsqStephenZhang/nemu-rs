use crate::halt;

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    crate::println!("Panic occurred: {}", panic_info);
    halt(-1)
}
