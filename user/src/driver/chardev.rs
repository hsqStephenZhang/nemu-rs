pub fn putchar(c: u8) {
    unsafe {
        core::ptr::write_volatile(crate::config::SERIAL_PORT, c);
    }
}