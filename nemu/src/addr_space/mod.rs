mod addr;
pub use addr::{PAddr, Size, VAddr};

use crate::{
    config::{FB_START, KBD_MMIO_START, RTC_MMIO_START, SERIAL_MMIO_START, VGACTL_MMIO_START},
    device::{
        keyboard::KeyboardIOMap,
        rtc::RTCIOMap,
        serial::SerialIOMap,
        vga::{VGACtrlIOMap, VGAIOMap},
    },
    memory::PhyMem,
};

#[derive(thiserror::Error, Debug)]
pub enum AddrError {
    #[error("Conflict MMIO address")]
    Conflict,
    #[error("Address out of bounds addr: {0:?}, size {1:?}")]
    OutOfBounds(PAddr, Size),
}

pub trait IOMap: std::fmt::Debug {
    fn len(&self) -> usize;
    fn read(&self, offset: PAddr) -> u64;
    // write operation of multiple bytes maybe optimized by compiler
    // to be Size of 4, or 8 bytes
    // therefore, we need to specify the size of the write operation
    // in practice, this is only used for MMIO devices with continuous buffer
    // e.g. VGA framebuffer
    fn write(&mut self, offset: PAddr, size: Size, value: u64);
}

#[derive(Debug)]
pub struct IOMapEntry {
    start: PAddr,
    end: PAddr,
    name: &'static str,
    ops: Box<dyn IOMap>,
}

impl std::fmt::Display for IOMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IOMapEntry {{ start: {}, end: {}, name: {}, ops: {:?} }}",
            self.start.0, self.end.0, self.name, self.ops
        )
    }
}

#[derive(Debug)]
pub struct AddressSpace {
    phy_mem: PhyMem,
    mmio: Vec<IOMapEntry>,
}

impl AddressSpace {
    pub fn new(phy_mem: PhyMem) -> Self {
        AddressSpace {
            phy_mem,
            mmio: Vec::new(),
        }
    }

    pub fn with_default_mmio(self) -> Self {
        let mut space = self;
        space
            .add_mmio(SERIAL_MMIO_START, "serial", SerialIOMap::new_mmio())
            .unwrap();
        space
            .add_mmio(RTC_MMIO_START, "rtc", RTCIOMap::new_mmio())
            .unwrap();
        space
            .add_mmio(KBD_MMIO_START, "kbd", KeyboardIOMap::new_mmio())
            .unwrap();
        space
            .add_mmio(VGACTL_MMIO_START, "vga_ctl", VGACtrlIOMap::new_mmio())
            .unwrap();
        space
            .add_mmio(FB_START, "vga", VGAIOMap::new_mmio())
            .unwrap();

        space
    }

    pub fn add_mmio(
        &mut self,
        start: PAddr,
        name: &'static str,
        ops: Box<dyn IOMap>,
    ) -> Result<(), AddrError> {
        let end = PAddr(start.0 + ops.len() as u64);
        if self.get_mmio(start).is_some() || self.get_mmio(end).is_some() {
            return Err(AddrError::Conflict);
        }

        self.mmio.push(IOMapEntry {
            start,
            end,
            name,
            ops,
        });
        Ok(())
    }

    pub fn get_mmio(&self, addr: PAddr) -> Option<&IOMapEntry> {
        self.mmio
            .iter()
            .find(|mmio| addr >= mmio.start && addr < mmio.end)
    }

    pub fn get_mmio_mut(&mut self, addr: PAddr) -> Option<&mut IOMapEntry> {
        self.mmio
            .iter_mut()
            .find(|mmio| addr >= mmio.start && addr < mmio.end)
    }

    pub fn read(&self, addr: PAddr, size: Size) -> std::result::Result<u64, AddrError> {
        if let Some(mmio) = self.get_mmio(addr) {
            let offset = addr - mmio.start;
            return Ok(mmio.ops.read(offset));
        } else if self.phy_mem.contains(addr, size) {
            return Ok(self
                .phy_mem
                .read(addr, size)
                .ok_or(AddrError::OutOfBounds(addr, size))?);
        }
        Err(AddrError::OutOfBounds(addr, size))
    }

    pub fn write(
        &mut self,
        addr: PAddr,
        size: Size,
        value: u64,
    ) -> std::result::Result<(), AddrError> {
        if let Some(mmio) = self.get_mmio_mut(addr) {
            let offset = addr - mmio.start;
            mmio.ops.write(offset, size, value);
        } else if self.phy_mem.contains(addr, size) {
            self.phy_mem.write(addr, size, value);
        } else {
            return Err(AddrError::OutOfBounds(addr, size));
        }
        Ok(())
    }
}
