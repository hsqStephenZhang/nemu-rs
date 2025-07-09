use crate::{addr_space::IOMap, device::AsyncDevice, utils::UPSafeCellRaw};

#[derive(Debug)]
pub struct RTCIOMap;

impl IOMap for RTCIOMap {
    fn read(&self, offset: crate::addr_space::PAddr) -> u64 {
        let us = get_time_us();
        match offset.0 {
            0x00 => (us >> 32) as u64,
            0x04 => us as u32 as u64,
            _ => panic!("RTC read from invalid offset: {:?}", offset),
        }
    }

    fn write(&mut self, _offset: crate::addr_space::PAddr, _value: u64) {
        panic!("RTC does not support write operations");
    }

    fn len(&self) -> usize {
        8
    }
}

impl RTCIOMap {
    pub fn new_mmio() -> Box<dyn IOMap> {
        Box::new(RTCIOMap)
    }
}

fn get_time_us() -> u64 {
    let time = chrono::Utc::now();
    let us = time.timestamp_micros() as u64;
    us
}

pub struct RTC;

impl AsyncDevice for RTC {
    fn name(&self) -> &'static str {
        "rtc"
    }

    fn period(&self) -> Option<u64> {
        None
    }

    fn callback(&self) -> Option<Box<dyn FnMut(u64, u64) + 'static>> {
        None
    }
}

lazy_static::lazy_static! {
    pub static ref RTC_DEVICE: UPSafeCellRaw<RTC> = unsafe { UPSafeCellRaw::new(RTC) };
}
