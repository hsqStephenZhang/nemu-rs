#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

// Array to store calculated factorials.
static mut F: [i32; 15] = [0; 15];
// Precomputed answers for verification.
static ANS: [i32; 13] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600,
];

/// Calculates the factorial of a number recursively.
fn fact(n: i32) -> i32 {
    if n == 0 || n == 1 {
        1
    } else {
        fact(n - 1) * n
    }
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    for i in 0..13 {
        let result = fact(i as i32);
        // Safety: Accessing static mut `F` is unsafe.
        unsafe {
            F[i] = result;
            nemu_assert!(F[i] == ANS[i]);
        }
    }

    println!("fact test passed!");
    0 // Return 0 for success.
}
