pub mod consts {
    use crate::memory::addr::PAddr;

    pub const MMIO_START: u64 = 0xa0000000;

    pub const RTC_MMIO_START: PAddr = PAddr::new(MMIO_START + 0x70);
    pub const SERIAL_MMIO_START: PAddr = PAddr::new(MMIO_START + 0x3f8);

}

pub mod rtc;
pub mod serial;

