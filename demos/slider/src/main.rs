#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;

use user_lib::{
    driver::{get_time, sync_frame, vga::draw_area},
    println,
};

const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = 300;

static IMAGE1: &[u8] = include_bytes!("../images/litenes.bin");
static IMAGE2: &[u8] = include_bytes!("../images/projectn.bin");
struct ImageDisplay {
    images: Vec<&'static [u8]>,
    current_image: usize,
    last_time: u64,
}

impl ImageDisplay {
    fn new(images: Vec<&'static [u8]>) -> Self {
        println!("Loaded {} images", images.len());

        ImageDisplay {
            images,
            current_image: 0,
            last_time: 0,
        }
    }

    fn display_image(&self, index: usize) {
        if index >= self.images.len() {
            println!("Image index {} out of bounds", index);
            return;
        }

        let image_slice = self.images[index];

        draw_area(0, 0, IMAGE_WIDTH, IMAGE_HEIGHT, image_slice);

        sync_frame();

        println!("Displaying image {}", index);
    }

    fn update(&mut self) {
        let current_time = get_time() / 1000; // 转换为毫秒

        // 每5秒切换一次图像
        if current_time - self.last_time > 5000 {
            self.current_image = (self.current_image + 1) % self.images.len();
            self.display_image(self.current_image);
            self.last_time = current_time;
        }
    }
}

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Starting image display program...");

    let mut display = ImageDisplay::new(Vec::from([IMAGE1, IMAGE2]));
    display.display_image(0); // 显示第一张图像

    loop {
        display.update();
    }
}
