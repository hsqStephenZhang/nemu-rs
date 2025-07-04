#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

// A mutable static array to store the Fibonacci sequence.
// We initialize the first two elements like in the C code.
static mut FIB: [i32; 40] = [
    1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

// Precomputed answers for verification.
static ANS: [i32; 40] = [
    1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
    17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578,
    5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
];

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    // Unsafe block is required to access and modify the static mut array `FIB`.
    unsafe {
        for i in 2..40 {
            FIB[i] = FIB[i - 1] + FIB[i - 2];
            nemu_assert!(FIB[i] == ANS[i]);
        }
    }

    println!("fib test passed!");
    0 // Return 0 for success.
}
