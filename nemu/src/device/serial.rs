use std::io::Write;

use tracing::error;

use crate::{
    addr_space::{IOMap, PAddr},
    device::AsyncDevice,
    utils::UPSafeCellRaw,
};

lazy_static::lazy_static! {
    pub static ref BYTES_QUEUE: crossbeam_queue::ArrayQueue<u8> = crossbeam_queue::ArrayQueue::new(1024);

    pub static ref SERIAL_DEVICE: UPSafeCellRaw<SerialDevice> = unsafe {
        UPSafeCellRaw::new(SerialDevice)
    };
}

#[derive(Debug)]
pub struct SerialIOMap;

impl IOMap for SerialIOMap {
    fn read(&self, _: crate::addr_space::PAddr) -> u64 {
        panic!("Serial port does not support read operations");
    }

    fn write(&mut self, offset: crate::addr_space::PAddr, value: u64) {
        debug_assert_eq!(offset, PAddr(0));
        debug_assert_eq!(value & 0xff, value);
        if BYTES_QUEUE.push(value as u8).is_err() {
            error!("Serial port queue is full, dropping byte: {}", value);
        }
    }

    fn len(&self) -> usize {
        1
    }
}

impl SerialIOMap {
    pub fn new_mmio() -> Box<dyn IOMap> {
        Box::new(SerialIOMap)
    }
}

pub struct SerialDevice;

impl SerialDevice {
    pub fn flush() -> std::io::Result<()> {
        let mut printed = false;
        while let Some(byte) = BYTES_QUEUE.pop() {
            print!("{}", byte as char);
            printed = true;
        }
        if printed {
            std::io::stdout().flush()
        } else {
            Ok(())
        }
    }
}

impl AsyncDevice for SerialDevice {
    fn name(&self) -> &'static str {
        "serial"
    }

    fn period(&self) -> Option<u64> {
        Some(1)
    }

    fn callback(&self) -> Option<Box<dyn FnMut(u64, u64) + 'static>> {
        Some(Box::new(move |_, _| {
            if let Err(e) = SerialDevice::flush() {
                error!("Failed to flush serial output: {}", e);
            }
        }))
    }
}
