use crate::memory::IOMap;

#[derive(Debug)]
pub struct RTC;

impl IOMap for RTC {
    fn read(&self, offset: crate::memory::addr::PAddr) -> u64 {
        let us = get_time_us();
        match offset.0 {
            0x00 => (us >> 32) as u64,
            0x04 => us as u32 as u64,
            _ => panic!("RTC read from invalid offset: {:?}", offset),
        }
    }

    fn write(&mut self, _offset: crate::memory::addr::PAddr, _value: u64) {
        panic!("RTC does not support write operations");
    }
}

impl RTC {
    pub fn new_mmio() -> Box<dyn IOMap> {
        Box::new(RTC)
    }
}

fn get_time_us() -> u64 {
    let time = chrono::Utc::now();
    let us = time.timestamp_micros() as u64;
    us
}
