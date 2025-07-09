use num_traits::FromPrimitive;
use sdl2::{event::Event, pixels::PixelFormatEnum, render::Canvas, video::Window};
use tracing::info;

use crate::{
    device::keyboard::{MyScancode, send_key},
    utils::UPSafeCellRaw,
};

pub mod audio;
pub mod keyboard;
pub mod vga;

lazy_static::lazy_static! {
    pub(crate) static ref SDL_DEVICE: UPSafeCellRaw<SdlDevice> = unsafe {
        let device = SdlDevice::new(800, 600).expect("Failed to initialize SDL device");
        UPSafeCellRaw::new(device)
    };
}

pub struct SdlDevice {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
    screen_w: u32,
    screen_h: u32,
    exit_on_quit: bool,
}

impl SdlDevice {
    pub fn new(screen_w: u32, screen_h: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("REMU", screen_w, screen_h)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().unwrap();

        info!(
            "SDL initialized with screen size: {}x{}",
            screen_w, screen_h
        );

        Ok(SdlDevice {
            sdl_context,
            canvas,
            screen_h,
            screen_w,
            exit_on_quit: true,
        })
    }

    pub fn ctx(&self) -> &sdl2::Sdl {
        &self.sdl_context
    }

    pub fn poll_events(&mut self) -> Result<(), String> {
        let mut events = self.sdl_context.event_pump().map_err(|e| e.to_string())?;

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("Quit event received, exiting...");
                    if self.exit_on_quit {
                        std::process::exit(0);
                    }
                }
                Event::KeyDown {
                    scancode: Some(code),
                    ..
                } => {
                    send_key(MyScancode::from_i32(code as i32).unwrap(), true);
                }
                Event::KeyUp {
                    scancode: Some(code),
                    ..
                } => {
                    send_key(MyScancode::from_i32(code as i32).unwrap(), false);
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn update_screen(&mut self, pixel_data: &[u8]) {
        let texture_creator = self.canvas.texture_creator();

        // create new texture each time
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                self.screen_w as u32,
                self.screen_h as u32,
            )
            .unwrap();

        texture
            .update(None, pixel_data, (self.screen_w * 4) as usize)
            .unwrap();

        self.canvas.clear();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }
}
