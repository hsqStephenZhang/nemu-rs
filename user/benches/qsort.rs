#![no_std]
#![no_main]

use alloc::vec::Vec;
use user_lib::driver::get_time;

#[macro_use]
extern crate user_lib;

extern crate alloc;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    const SIZE: usize = 100;
    let start = get_time();
    let mut vec = (0..SIZE)
        .map(|_| user_lib::rand::rand())
        .collect::<Vec<u32>>();
    vec.sort();

    let end = get_time();
    let microseconds = end - start;
    let seconds = microseconds / 1_000_000;
    let microseconds = microseconds % 1_000_000;
    println!("add bench passed!, time elapsed: {}.{}", seconds, microseconds);
    0 // Return 0 for success.
}
