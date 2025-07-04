#![no_std]
#![no_main]

use user_lib::rand;

#[macro_use]
extern crate user_lib;

extern crate alloc;
use alloc::vec::Vec;

#[unsafe(no_mangle)]
fn main() -> i32 {
    rand::srand(1);
    let mut vec = (0..100).map(|_| rand::rand()).collect::<Vec<u32>>();
    vec.sort();
    for i in 0..99 {
        nemu_assert!(vec[i] <= vec[i + 1], "Vector should be sorted");
    }
    nemu_assert!(vec.len() == 100, "Vector should have length 100");
    println!("rand test passed!");
    return 0;
}
