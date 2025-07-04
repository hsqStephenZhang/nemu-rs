use std::io::Write;

use crate::memory::{IOMap, addr::PAddr};

#[derive(Debug)]
pub struct Serial;

impl IOMap for Serial {
    fn read(&self, _: crate::memory::addr::PAddr) -> u64 {
        panic!("Serial port does not support read operations");
    }

    fn write(&mut self, offset: crate::memory::addr::PAddr, value: u64) {
        debug_assert_eq!(offset, PAddr(0));
        debug_assert_eq!(value & 0xff, value);
        print!("{}", value as u8 as char);
        let _ = std::io::stdout().flush();
    }
}

impl Serial {
    pub fn new_mmio() -> Box<dyn IOMap> {
        Box::new(Serial)
    }
}
