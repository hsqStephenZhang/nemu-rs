// real time clock
pub mod rtc;
// sdl2 keyboard, vga, audio
pub mod sdl2;
pub use sdl2::{keyboard, vga};

use crate::timer::virtual_clock::VirtualClock;

// serial device
pub mod serial;
// local interrupt
pub mod clint;

pub trait AsyncDevice {
    fn name(&self) -> &'static str;
    // how many cpu cycles to wait before the next callback
    fn period(&self) -> Option<u64>;
    fn callback(&self) -> Option<Box<dyn FnMut(u64, u64) + 'static>>;
}

pub fn init(clock: &mut VirtualClock) {
    clock.register_timer(
        1,
        serial::SERIAL_DEVICE.get().callback().unwrap(),
        serial::SERIAL_DEVICE.get().period(),
    );
    // sdl2 does not support rust test since it acquires the main thread
    // to run the event loop
    #[cfg(not(test))]
    {
        sdl2::SDL_DEVICE.get();
        // clock.register_timer(
        //     100,
        //     vga::VGA_DEVICE.get().callback().unwrap(),
        //     vga::VGA_DEVICE.get().period(),
        // );
        clock.register_timer(
            100,
            keyboard::KEY_BOARD_DEVICE.get().callback().unwrap(),
            keyboard::KEY_BOARD_DEVICE.get().period(),
        );
    }
}

pub fn dummy_init(clock: &mut VirtualClock) {
    clock.register_timer(
        0,
        |_, _| {
            println!("cycle+100");
        },
        Some(100),
    );
}
