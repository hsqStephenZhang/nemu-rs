#[derive(Debug, Clone, Copy, Default)]
pub struct Reserve {
    addr: u64,
    valid: bool
}

#[allow(unused)]
impl Reserve {
    pub fn new(addr: u64) -> Self {
        Reserve {
            addr,
            valid: true
        }
    }

    pub fn reset(&mut self, addr: u64) {
        self.addr = addr;
        self.valid = true;
    }

    pub fn check(&self, addr: u64) -> bool {
        self.valid && self.addr == addr
    }

    pub fn invalidate(&mut self) {
        self.valid = false;
    }
}