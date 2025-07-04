#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

fn is_leap_year(n: i32) -> bool {
    (n % 4 == 0 && n % 100 != 0) || (n % 400 == 0)
}

static ANS: [i32; 125] = [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0];


#[allow(unsafe_op_in_unsafe_fn)]
#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    for i in 0..125 {
        let year = i + 1890;
        let result = if is_leap_year(year) { 1 } else { 0 };
        nemu_assert!(result == ANS[i as usize]);
    }
    println!("leap-year test passed!");
    0
}
