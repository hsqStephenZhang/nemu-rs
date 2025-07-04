#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

static ANS: [i32; 4] = [153, 370, 371, 407];

fn cube(n: i32) -> i32 {
    n * n * n
}

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut k = 0;
    for n in 100..500 {
        let n2 = n / 100;
        let n1 = (n / 10) % 10;
        let n0 = n % 10;

        if n == cube(n2) + cube(n1) + cube(n0) {
            nemu_assert!(n == ANS[k]);
            k += 1;
        }
    }

    nemu_assert!(k == 4);
    println!("shuixianhua test passed!");
    0
}
