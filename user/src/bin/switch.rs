#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

fn switch_case(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 2,
        2 | 3 => 5,
        4..=7 => 8,
        8..=11 => 10,
        12 => 15,
        _ => -1,
    }
}

static ANS: [i32; 15] = [-1, 0, 2, 5, 5, 8, 8, 8, 8, 10, 10, 10, 10, 15, -1];

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    for i in 0..15 {
        nemu_assert!(switch_case(i - 1) == ANS[i as usize]);
    }

    println!("switch test passed!");
    0
}
