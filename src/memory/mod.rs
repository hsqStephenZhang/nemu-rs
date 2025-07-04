use crate::memory::addr::PAddr;

pub mod addr;
pub mod config;

pub trait IOMap: std::fmt::Debug {
    fn read(&self, offset: PAddr) -> u64;
    fn write(&mut self, offset: PAddr, value: u64);
}

#[derive(Debug)]
pub struct IOMapEntry {
    start: PAddr,
    end: PAddr,
    name: &'static str,
    io: Box<dyn IOMap>,
}

#[derive(Debug)]
pub struct PhyMem {
    base_addr: PAddr,
    mem: Box<[u8]>,
    // KISS
    mmio: Vec<IOMapEntry>,
}

impl PhyMem {
    pub fn new(base_addr: PAddr, size: usize) -> Self {
        PhyMem {
            base_addr,
            mem: vec![0; size].into_boxed_slice(),
            mmio: Vec::new(),
        }
    }

    pub fn add_mmio(
        &mut self,
        start: PAddr,
        end: PAddr,
        name: &'static str,
        io: Box<dyn IOMap>,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        match self.get_mmio(start) {
            Some(exist) => Err(format!(
                "MMIO region {} overlaps with existing region {}",
                name, exist.name
            )
            .into()),
            None => {
                self.mmio.push(IOMapEntry {
                    start,
                    end,
                    name,
                    io,
                });
                Ok(())
            }
        }
    }

    pub fn get_mmio(&self, addr: PAddr) -> Option<&IOMapEntry> {
        self.mmio
            .iter()
            .find(|entry| entry.start <= addr && addr < entry.end)
    }

    pub fn get_mmio_mut(&mut self, addr: PAddr) -> Option<&mut IOMapEntry> {
        self.mmio
            .iter_mut()
            .find(|entry| entry.start <= addr && addr < entry.end)
    }
}

#[derive(Clone, Debug)]
pub enum Size {
    Byte = 1,
    HalfWord = 2,
    Word = 4,
    DoubleWord = 8,
}

impl PhyMem {
    fn read_u8(&self, addr: PAddr) -> u8 {
        self.mem[addr.0 as usize]
    }
    fn read_u16(&self, addr: PAddr) -> u16 {
        self.mem[addr.0 as usize..addr.0 as usize + 2]
            .try_into()
            .map(u16::from_le_bytes)
            .unwrap_or(0)
    }
    fn read_u32(&self, addr: PAddr) -> u32 {
        self.mem[addr.0 as usize..addr.0 as usize + 4]
            .try_into()
            .map(u32::from_le_bytes)
            .unwrap_or(0)
    }
    fn read_u64(&self, addr: PAddr) -> u64 {
        self.mem[addr.0 as usize..addr.0 as usize + 8]
            .try_into()
            .map(u64::from_le_bytes)
            .unwrap_or(0)
    }
}

// TODO: memory alignment?
impl PhyMem {
    pub fn read(&self, addr: PAddr, size: Size) -> Option<u64> {
        if let Some(mmio) = self.get_mmio(addr) {
            // TODO: is boundary check necessary?
            let offset = addr.0 - mmio.start.0;
            return Some(mmio.io.read(offset.into()));
        }
        if addr < self.base_addr || addr.0 >= self.base_addr.0 + self.mem.len() as u64 {
            panic!("Address out of bounds: {:?}", addr);
        }
        let addr = PAddr(addr.0 - self.base_addr.0);

        let res = match size {
            Size::Byte => self.read_u8(addr) as u64,
            Size::HalfWord => self.read_u16(addr) as u64,
            Size::Word => self.read_u32(addr) as u64,
            Size::DoubleWord => self.read_u64(addr),
        };
        Some(res)
    }

    pub fn write(&mut self, addr: PAddr, size: Size, value: u64) {
        if let Some(mmio) = self.get_mmio_mut(addr) {
            // TODO: is boundary check necessary?
            let offset = addr.0 - mmio.start.0;
            mmio.io.write(offset.into(), value);
            return;
        }
        if addr < self.base_addr || addr.0 >= self.base_addr.0 + self.mem.len() as u64 {
            panic!(
                "Address out of bounds: {:x?}, range: [{:x?}, {:x?})",
                addr,
                self.base_addr,
                self.base_addr.0 + self.mem.len() as u64
            );
        }
        let addr = PAddr(addr.0 - self.base_addr.0);
        match size {
            Size::Byte => {
                if value > 0xFF {
                    panic!("Value out of bounds for byte: {}", value);
                }
                self.mem[addr.0 as usize] = value as u8;
            }
            Size::HalfWord => {
                if value > 0xFFFF {
                    panic!("Value out of bounds for half-word: {}", value);
                }
                self.mem[addr.0 as usize..addr.0 as usize + 2]
                    .copy_from_slice(&(value as u16).to_le_bytes());
            }
            Size::Word => {
                if value > 0xFFFFFFFF {
                    panic!("Value out of bounds for word: {}", value);
                }
                self.mem[addr.0 as usize..addr.0 as usize + 4]
                    .copy_from_slice(&(value as u32).to_le_bytes());
            }
            Size::DoubleWord => {
                self.mem[addr.0 as usize..addr.0 as usize + 8]
                    .copy_from_slice(&value.to_le_bytes());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::{PhyMem, addr::PAddr};

    #[test]
    fn t1() {
        let base = 0x80000000;
        let mut mem = PhyMem::new(PAddr(base), 0x8000000);
        mem.write(PAddr(base), crate::memory::Size::Word, 0x12345678);
        let value = mem.read(PAddr(base), crate::memory::Size::Word);
        assert_eq!(value, Some(0x12345678));
        let value = mem.read(PAddr(base), crate::memory::Size::HalfWord);
        assert_eq!(value, Some(0x5678));
        let value = mem.read(PAddr(base + 2), crate::memory::Size::HalfWord);
        assert_eq!(value, Some(0x1234));
        let value = mem.read(PAddr(base), crate::memory::Size::Byte);
        assert_eq!(value, Some(0x78));

        mem.write(
            PAddr(base),
            crate::memory::Size::DoubleWord,
            0x123456789abcdef0,
        );
        let value = mem.read(PAddr(base), crate::memory::Size::DoubleWord);
        assert_eq!(value, Some(0x123456789abcdef0));
        // let value = mem.read(PAddr(base + 8), crate::memory::Size::DoubleWord);
    }
}
