pub mod consts {
    use crate::memory::addr::PAddr;

    pub const SERIAL_MMIO_START: PAddr = PAddr::new(0xa00003f8);
}

pub mod serial;
