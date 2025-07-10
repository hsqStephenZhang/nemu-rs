#![no_std]
#![no_main]

use user_lib::driver::get_time;

#[macro_use]
extern crate user_lib;

mod generated;

use generated::*;

const FPS: u64 = 30;
const CHAR_WHITE: u8 = b'.';
const CHAR_BLACK: u8 = b'X';

// we could not use OUT_DIR beause it is not supported in no_std environment
const VIDEO: &[u8] = include_bytes!("../build/video.frame"); // 包含视频数据的二进制文件

fn getbit(p: &[u8], idx: usize) -> u8 {
    let byte_idx = idx / 8;
    let bit_idx = 7 - (idx % 8);
    let byte = p[byte_idx];
    (byte >> bit_idx) & 1
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    print!("\x1B[2J\x1B[1;1H");
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let frame_size = VIDEO_ROW * VIDEO_COL / 8;
    let frame_count = VIDEO.len() / frame_size;
    println!("Bad Apple - Frame Count: {}, Frame Size: {}, total bytes: {}", frame_count, frame_size, VIDEO.len());
    let frames = VIDEO;

    clear_screen();

    let mut now = get_time(); // 微秒计时器
    for i in 0..frame_count {
        print!("\x1B[0;0H"); // Reset cursor
        let frame = &frames[i * frame_size..(i + 1) * frame_size];

        for y in 0..VIDEO_ROW {
            for x in 0..VIDEO_COL {
                let idx = y * VIDEO_COL + x;
                let bit = getbit(frame, idx);
                let ch = if bit == 1 { CHAR_BLACK } else { CHAR_WHITE };
                print!("{}", ch as char);
            }
            println!();
        }

        let next = now + 1_000_000 / FPS;
        while get_time() < next {}
        now = next;
    }

    println!("Bad Apple playback finished!");

    0
}
