use tracing::*;

use crate::{
    addr_space::{IOMap, PAddr, Size},
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
    vmem: Vec<u8>,
    width: usize,
    height: usize,
    sync: bool,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Framebuffer {
            vmem: vec![0; (width * height * 4) as usize],
            width: width as usize,
            height: height as usize,
            sync: false,
        }
    }

    pub fn vmem(&self) -> &[u8] {
        &self.vmem
    }

    pub fn vmem_mut(&mut self) -> &mut [u8] {
        &mut self.vmem
    }

    pub fn should_sync(&self) -> bool {
        self.sync
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
            std::slice::from_raw_parts(framebuffer.vmem().as_ptr(), framebuffer.vmem().len())
        };
        if offset.0 as usize >= pixels.len() {
            error!("VGA read from invalid offset: {:?}", offset);
            return 0;
        } else {
            pixels[offset.0 as usize] as u64
        }
    }

    fn write(&mut self, offset: crate::addr_space::PAddr, size: Size, value: u64) {
        // info!("VGA write to offset {:?} with value: {}", offset, value as u8);
        let framebuffer = FRAME_BUFFER.get_mut();
        let pixels = unsafe {
            std::slice::from_raw_parts_mut(
                framebuffer.vmem_mut().as_mut_ptr(),
                framebuffer.vmem().len(),
            )
        };
        let num_bytes = size as usize;
        if offset.0 as usize >= pixels.len() || offset.0 as usize + num_bytes > pixels.len() {
            error!(
                "VGA read from invalid offset: {:?}, size: {:?}",
                offset, size
            );
        } else {
            match size {
                Size::Byte => {
                    pixels[offset.0 as usize] = (value & 0xFF) as u8;
                }
                Size::HalfWord => pixels[offset.0 as usize..offset.0 as usize + 2]
                    .copy_from_slice(&((value & 0xFFFF) as u16).to_le_bytes()),
                Size::Word => pixels[offset.0 as usize..offset.0 as usize + 4]
                    .copy_from_slice(&((value & 0xFFFFFFFF) as u32).to_le_bytes()),
                Size::DoubleWord => {
                    pixels[offset.0 as usize..offset.0 as usize + 8]
                        .copy_from_slice(&value.to_le_bytes());
                }
            }
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
            PAddr(0) => {
                let framebuffer = FRAME_BUFFER.get();
                let width = framebuffer.width as u32;
                let height = framebuffer.height as u32;
                ((width << 16) | height) as u64 // Return width in high 16 bits and height in low 16 bits
            }
            PAddr(4) => {
                FRAME_BUFFER.get().should_sync() as u64 // Return sync status
            }
            _ => {
                eprintln!("VGA control read from invalid offset: {:?}", offset);
                0
            }
        }
    }

    fn write(&mut self, offset: PAddr, size: Size, value: u64) {
        match offset {
            PAddr(0) => {
                // This is where you would set the width and height.
                // For now, we just log the operation.
                error!("VGA control write to offset 0 is not implemented");
            }
            PAddr(4) => {
                debug!(
                    "VGA control write to offset 4 with value: {}, size:{:?}",
                    value, size
                );
                // Toggle sync status
                let framebuffer = FRAME_BUFFER.get_mut();
                framebuffer.sync = value != 0;
            }
            _ => {
                error!("VGA control write to invalid offset: {:?}", offset);
            }
        }
    }

    fn len(&self) -> usize {
        8 // u32 for width and height, plus u32 for sync status
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
            let framebuffer = FRAME_BUFFER.get_mut();
            if !framebuffer.should_sync() {
                return; // Skip if sync is not required
            } else {
                framebuffer.sync = false; // Reset sync status after processing
            }
            debug!("Updating screen");
            let sdl_device = super::SDL_DEVICE.get_mut();
            sdl_device.update_screen(framebuffer.vmem());
        }))
    }
}
