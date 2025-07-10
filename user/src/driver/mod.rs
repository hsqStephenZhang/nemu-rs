// pub mod keyboard;
pub mod chardev;
pub mod keyboard;
pub mod time;
pub mod vga;

pub use chardev::putchar;
pub use keyboard::{KeyCode, get_key};
pub use time::get_time;
pub use vga::{new_frame_buffer, screen_width_height, sync_frame, write_frame, write_pixel};
