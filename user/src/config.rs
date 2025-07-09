pub const MMIO_START: u64 = 0xa0000000;

pub const RTC_PORT: *mut u32 = (MMIO_START + 0x48) as *mut u32;
pub const RTC_PORT_HIGH: *mut u32 = RTC_PORT;
pub const RTC_PORT_LOW: *mut u32 = (MMIO_START + 0x48 + 0x4) as *mut u32;
pub const SERIAL_PORT: *mut u8 = (MMIO_START + 0x3f8) as *mut u8;
