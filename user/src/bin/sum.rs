#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

use core::ptr::{read_volatile, write_volatile};

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut i = 1;
    let mut sum = 0; // In Rust, local variables are on the stack.
                     // To mimic `volatile`, we use volatile reads and writes.

    while i <= 100 {
        let current_sum = read_volatile(&sum);
        write_volatile(&mut sum, current_sum + i);
        i += 1;
    }

    nemu_assert!(read_volatile(&sum) == 5050);

    println!("sum test passed!");
    0
}
