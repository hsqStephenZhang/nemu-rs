#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

static ANS: [i32; 10] = [101, 103, 107, 109, 113, 127, 131, 137, 139, 149];

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut n = 0;
    for m in (101..=150).step_by(2) {
        let mut is_prime = true;
        let mut i = 2;
        while i < m {
            if m % i == 0 {
                is_prime = false;
                break;
            }
            i += 1;
        }

        if is_prime {
            // The original C code checks `i == ans[n]`. When a number `m` is prime,
            // the inner loop finishes when `i` becomes equal to `m`.
            nemu_assert!(m == ANS[n]);
            n += 1;
        }
    }

    nemu_assert!(n == 10);
    println!("prime test passed!");
    0
}
