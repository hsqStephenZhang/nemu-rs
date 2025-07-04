use crate::memory::{
    PhyMem, Size,
    addr::{PAddr, VAddr},
};

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Bare,
    SV39,
}

#[derive(Debug)]
pub struct MMU {
    mode: Mode,
    phy_mem: PhyMem,
}

impl MMU {
    pub fn new(phy_mem: PhyMem, mode: Mode) -> Self {
        MMU { mode, phy_mem }
    }

    fn translate(&mut self, vaddr: VAddr) -> Option<PAddr> {
        match self.mode {
            Mode::Bare => Some(PAddr(vaddr.0)),
            Mode::SV39 => {
                todo!()
            }
        }
    }

    pub fn read(&mut self, addr: VAddr, size: Size) -> Option<u64> {
        let addr = self.translate(addr)?;
        self.phy_mem.read(addr, size)
    }

    pub fn write(&mut self, addr: VAddr, size: Size, value: u64) -> Option<()> {
        let addr = self.translate(addr)?;
        self.phy_mem.write(addr, size, value);
        Some(())
    }

    #[cfg(test)]
    pub fn load_program(&mut self, addr: VAddr, data: &[u8]) -> Option<()> {
        let mut offset = 0;
        while offset < data.len() {
            let size = Size::Byte; // Assuming byte size for simplicity
            self.write(VAddr(addr.0 + offset as u64), size, data[offset] as u64)?;
            offset += 1;
        }
        Some(())
    }
}
