pub const MBASE: u64 = 0x80_000_000;
pub const MSIZE: u64 = 0x8_000_000;
pub const PC_RESET_OFFSET: u64 = 0x0;
pub const RESET_VECTOR: u64 = PC_RESET_OFFSET + MBASE;

use crate::addr_space::PAddr;

pub const MMIO_START: u64 = 0xa0000000;

pub const SERIAL_MMIO_START: PAddr = PAddr::new(MMIO_START + 0x3f8);
pub const KBD_MMIO_START: PAddr = PAddr::new(MMIO_START + 0x60);
pub const RTC_MMIO_START: PAddr = PAddr::new(MMIO_START + 0x48);
pub const VGACTL_MMIO_START: PAddr = PAddr::new(MMIO_START + 0x100);
pub const AUDIO_MMIO_START: PAddr = PAddr::new(MMIO_START + 0x200);
pub const DISK_MMIO_START: PAddr = PAddr::new(MMIO_START + 0x300);

pub const FB_START: PAddr = PAddr::new(MMIO_START + 0x1000000);
pub const AUDIO_BUF_START: PAddr = PAddr::new(MMIO_START + 0x1200000);
