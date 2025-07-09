use tracing::error;

use crate::{
    addr_space::{IOMap, PAddr},
    device::AsyncDevice,
    utils::UPSafeCellRaw,
};

lazy_static::lazy_static! {
    pub static ref FRAME_BUFFER: UPSafeCellRaw<Framebuffer> = unsafe {
        let buffer = Framebuffer::new(800, 600);
        UPSafeCellRaw::new(buffer)
    };

    pub static ref VGA_DEVICE: UPSafeCellRaw<VGADevice> = unsafe {
        UPSafeCellRaw::new(VGADevice)
    };
}

#[allow(unused)]
#[derive(Debug)]
pub struct Framebuffer {
    vmem: Vec<u32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Framebuffer {
            vmem: vec![0; (width * height) as usize],
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn vmem(&self) -> &[u32] {
        &self.vmem
    }

    pub fn vmem_mut(&mut self) -> &mut [u32] {
        &mut self.vmem
    }
}

#[derive(Debug)]
pub struct VGAIOMap;

impl VGAIOMap {
    pub fn new_mmio() -> Box<dyn IOMap> {
        Box::new(VGAIOMap)
    }
}

impl IOMap for VGAIOMap {
    fn read(&self, offset: crate::addr_space::PAddr) -> u64 {
        // For simplicity, we return a dummy value.
        // In a real implementation, this would read from the VGA memory.
        let framebuffer = FRAME_BUFFER.get();
        let pixels = unsafe {
            std::slice::from_raw_parts(
                framebuffer.vmem().as_ptr() as *const u8,
                framebuffer.vmem().len() * 4,
            )
        };
        if offset.0 as usize >= pixels.len() {
            error!("VGA read from invalid offset: {:?}", offset);
            return 0;
        } else {
            pixels[offset.0 as usize] as u64
        }
    }

    fn write(&mut self, offset: crate::addr_space::PAddr, value: u64) {
        let framebuffer = FRAME_BUFFER.get_mut();
        let pixels = unsafe {
            std::slice::from_raw_parts_mut(
                framebuffer.vmem_mut().as_mut_ptr() as *mut u8,
                framebuffer.vmem().len() * 4,
            )
        };
        if offset.0 as usize >= pixels.len() {
            error!("VGA read from invalid offset: {:?}", offset);
        } else {
            pixels[offset.0 as usize] = (value & 0xFF) as u8;
        }
    }

    fn len(&self) -> usize {
        FRAME_BUFFER.get().vmem().len() * 4
    }
}

#[derive(Debug)]
pub struct VGACtrlIOMap;

impl VGACtrlIOMap {
    pub fn new_mmio() -> Box<dyn IOMap> {
        Box::new(VGACtrlIOMap)
    }
}

impl IOMap for VGACtrlIOMap {
    fn read(&self, offset: crate::addr_space::PAddr) -> u64 {
        // For simplicity, we return a dummy value.
        // In a real implementation, this would read from the VGA control registers.
        match offset {
            PAddr(0) => FRAME_BUFFER.get().width as u64,
            PAddr(4) => FRAME_BUFFER.get().height as u64,
            _ => {
                eprintln!("VGA control read from invalid offset: {:?}", offset);
                0
            }
        }
    }

    fn write(&mut self, offset: PAddr, _value: u64) {
        error!(
            "VGA control does not support write operations, offset: {:?}",
            offset
        );
    }

    fn len(&self) -> usize {
        8 // 2 u32 for width and height
    }
}

pub struct VGADevice;

impl AsyncDevice for VGADevice {
    fn name(&self) -> &'static str {
        "vga"
    }

    fn period(&self) -> Option<u64> {
        Some(100)
    }

    // read frame buffer and update screen
    fn callback(&self) -> Option<Box<dyn FnMut(u64, u64) + 'static>> {
        Some(Box::new(move |_, _| {
            let pixel_data = unsafe {
                std::slice::from_raw_parts(
                    FRAME_BUFFER.get().vmem().as_ptr() as *const u8,
                    FRAME_BUFFER.get().vmem().len() * 4,
                )
            };
            let sdl_device = super::SDL_DEVICE.get_mut();
            sdl_device.update_screen(pixel_data);
        }))
    }
}
