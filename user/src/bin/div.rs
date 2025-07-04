#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const N: usize = 10;
// A mutable static array to hold our numbers.
static mut A: [i32; N] = [0; N];

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    // Unsafe block is needed to access the static mut array `A`.
    unsafe {
        // Initialize the array with values 0 to N-1.
        for i in 0..N {
            A[i] = i as i32;
        }

        // Multiply each element by numbers from 1 to N.
        for i in 0..N {
            for j in 1..(N + 1) {
                A[i] *= j as i32;
            }
        }

        // Divide each element by the same numbers, which should
        // restore the original values.
        for i in 0..N {
            for j in 1..(N + 1) {
                A[i] /= j as i32;
            }
        }

        // Check if the array elements are back to their original values.
        for i in 0..N {
            nemu_assert!(A[i] == i as i32);
        }
    }

    println!("div test passed!");
    0 // Return 0 for success.
}
