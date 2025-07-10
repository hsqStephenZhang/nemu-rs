pub fn get_time() -> u64 {
    unsafe {
        let t1 = core::ptr::read_volatile(crate::config::RTC_PORT_HIGH);
        let t2 = core::ptr::read_volatile(crate::config::RTC_PORT_LOW);
        ((t1 as u64) << 32) | (t2 as u64)
    }
}