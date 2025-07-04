#![no_std]
#![no_main]

use alloc::vec::Vec;

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

#[unsafe(no_mangle)]
fn main() -> i32 {
    let a: Vec<i32> = vec![1, 2, 3];
    nemu_assert!(a.len() == 3, "Vector should have length 3");
    nemu_assert!(a.iter().sum::<i32>() == 6, "Sum of vector elements should be 6");

    println!("alloc test passed!");
    return 0;
}
