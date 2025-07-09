#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PAddr(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VAddr(pub u64);

impl PAddr {
    pub const fn new(addr: u64) -> Self {
        PAddr(addr)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for PAddr {
    fn from(addr: u64) -> Self {
        PAddr(addr)
    }
}

impl From<PAddr> for u64 {
    fn from(addr: PAddr) -> Self {
        addr.0
    }
}

impl std::ops::Add<u64> for PAddr {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        PAddr(self.0 + rhs)
    }
}

impl std::ops::Sub<PAddr> for PAddr {
    type Output = Self;

    fn sub(self, rhs: PAddr) -> Self::Output {
        PAddr(self.0 - rhs.0)
    }
}

impl VAddr {
    pub fn new(addr: u64) -> Self {
        VAddr(addr)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for VAddr {
    fn from(addr: u64) -> Self {
        VAddr(addr)
    }
}

impl From<VAddr> for u64 {
    fn from(addr: VAddr) -> Self {
        addr.0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Byte = 1,
    HalfWord = 2,
    Word = 4,
    DoubleWord = 8,
}

#[allow(unused)]
pub trait Translate {
    fn translate(&mut self, vaddr: VAddr) -> Option<PAddr>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyTranslator;
    impl Translate for DummyTranslator {
        fn translate(&mut self, vaddr: VAddr) -> Option<PAddr> {
            // Dummy translation logic for testing
            Some(PAddr(vaddr.as_u64() + 0x1000)) // Just an example offset
        }
    }

    #[test]
    fn test_translate() {
        let mut translator = DummyTranslator;
        let vaddr = VAddr::new(0x2000);
        let paddr = translator.translate(vaddr);
        assert_eq!(paddr, Some(PAddr::new(0x3000)));
    }
}
