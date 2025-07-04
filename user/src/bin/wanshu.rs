#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

static ANS: [i32; 2] = [6, 28];

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut k = 0;
    for n in 1..30 {
        let mut sum = 0;
        for i in 1..n {
            if n % i == 0 {
                sum += i;
            }
        }

        if sum == n {
            nemu_assert!(n == ANS[k]);
            k += 1;
        }
    }

    nemu_assert!(k == 2);
    println!("wanshu test passed!");
    0
}
