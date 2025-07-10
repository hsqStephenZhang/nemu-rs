#![no_std]
#![no_main]

use user_lib::{
    driver::{new_frame_buffer, sync_frame, write_frame, write_pixel},
    println,
};

const FPS: u64 = 30;

#[unsafe(no_mangle)]
fn main() -> i32 {
    let mut frame = new_frame_buffer();
    set_red(&mut frame);
    // init all pixels to red
    write_frame(&frame);
    sync_frame();

    let mut fps = 0;
    let mut last = 0;
    let mut fps_last = 0;
    let (width, height) = user_lib::driver::screen_width_height();
    let mut x = 0;
    let mut y = 0;

    loop {
        let us = user_lib::driver::get_time();
        let ms = us / 1000;

        if ms - last >= 1000 / FPS {
            last = ms;
            // Set a pixel at (100, 100) to blue
            write_pixel(x, y, 0xFF0000FF);
            sync_frame();
            fps += 1;
            x = (x + 1) % width;
            y = (y + 1) % height;
        }
        if ms - fps_last >= 1000 {
            println!("FPS: {}", fps);
            fps = 0;
            fps_last = ms;
        }
    }
}

fn set_red(framebuffer: &mut [u8]) {
    for pixel in framebuffer.chunks_exact_mut(4) {
        // ARGB8888
        pixel[0] = 0x00; // B
        pixel[1] = 0x00; // G
        pixel[2] = 0xFF; // R
        pixel[3] = 0x00; // A
    }
}
