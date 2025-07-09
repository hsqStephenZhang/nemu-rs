use crate::addr_space::{AddressSpace, Size};
use crate::addr_space::{PAddr, VAddr};

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Bare,
    SV39,
}

#[derive(Debug)]
pub struct MMU {
    mode: Mode,
}

#[allow(unused)]
impl MMU {
    pub fn new(mode: Mode) -> Self {
        MMU { mode }
    }

    fn translate(&mut self, vaddr: VAddr) -> Option<PAddr> {
        match self.mode {
            Mode::Bare => Some(PAddr(vaddr.0)),
            Mode::SV39 => {
                todo!()
            }
        }
    }

    pub fn read(&mut self, addr_space: &AddressSpace, addr: VAddr, size: Size) -> Option<u64> {
        let addr = self.translate(addr)?;
        addr_space.read(addr, size).ok()
    }

    pub fn write(
        &mut self,
        addr_space: &mut AddressSpace,
        addr: VAddr,
        size: Size,
        value: u64,
    ) -> Option<()> {
        let addr = self.translate(addr)?;
        addr_space.write(addr, size, value).ok()
    }

    pub fn load_program(
        &mut self,
        addr_space: &mut AddressSpace,
        addr: VAddr,
        data: &[u8],
    ) -> Option<()> {
        let mut offset = 0;
        while offset < data.len() {
            let size = Size::Byte; // Assuming byte size for simplicity
            self.write(
                addr_space,
                VAddr(addr.0 + offset as u64),
                size,
                data[offset] as u64,
            )?;
            offset += 1;
        }
        Some(())
    }
}
