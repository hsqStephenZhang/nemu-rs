#![no_std]
#![no_main]

use user_lib::driver::get_time;

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    let time = get_time();
    println!("Current time: {} seconds since boot", time);

    return 0;
}
