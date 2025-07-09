extern crate sdl2;

use std::time::Duration;

use nemu_rs::{
    device::{
        keyboard::{KeyCode, MyScancode, key_dequeue, send_key},
        sdl2::SdlDevice,
    },
    utils::UPSafeCellRaw,
};
use num_traits::FromPrimitive;
use sdl2::event::Event;

// global single instance of SdlDevice
lazy_static::lazy_static! {
    static ref DEVICE: UPSafeCellRaw<SdlDevice> = unsafe {
        let device = SdlDevice::new(800, 600).expect("Failed to initialize SDL device");
        UPSafeCellRaw::new(device)
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::thread::spawn(|| {
        loop {
            if let Some(key) = key_dequeue() {
                let is_pressed = key & 0x8000 != 0;
                let key = KeyCode::from_u32(key & !0x8000).unwrap();
                println!("Key dequeued: {:?}, is_pressed {}", key, is_pressed);
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    });

    let mut events = DEVICE.get_mut().ctx().event_pump()?;

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("Quit event received, exiting...");
                    break 'running;
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

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
