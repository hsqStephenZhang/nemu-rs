#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

fn if_else(n: i32) -> i32 {
    if n > 500 {
        150
    } else if n > 300 {
        100
    } else if n > 100 {
        75
    } else if n > 50 {
        50
    } else {
        0
    }
}

static TEST_DATA: [i32; 14] = [-1, 0, 49, 50, 51, 99, 100, 101, 299, 300, 301, 499, 500, 501];
static ANS: [i32; 14] = [0, 0, 0, 0, 50, 50, 50, 75, 75, 75, 100, 100, 100, 150];

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut ans_idx = 0;
    for &data in TEST_DATA.iter() {
        nemu_assert!(if_else(data) == ANS[ans_idx]);
        ans_idx += 1;
    }

    nemu_assert!(ans_idx == TEST_DATA.len());

    println!("if-else test passed!");
    0
}
