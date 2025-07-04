#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const N: usize = 20;

// The array to be sorted. It must be mutable, so we declare it as `static mut`.
// Accessing `static mut` is unsafe and requires an `unsafe` block.
static mut A: [i32; N] = [
    2, 12, 14, 6, 13, 15, 16, 10, 0, 18, 11, 19, 9, 1, 7, 5, 4, 3, 8, 17,
];

/// Sorts the global array `A` using the bubble sort algorithm.
fn bubble_sort() {
    // Safety: Accessing and modifying a `static mut` is unsafe. We must ensure
    // that no other thread is accessing `A` at the same time. In this single-threaded
    // context, it is safe.
    unsafe {
        for j in 0..N {
            for i in 0..(N - 1 - j) {
                if A[i] > A[i + 1] {
                    // Swap elements
                    let t = A[i];
                    A[i] = A[i + 1];
                    A[i + 1] = t;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    bubble_sort();

    // Check if the array is sorted correctly.
    for i in 0..N {
        // Safety: Accessing `static mut` is unsafe.
        unsafe {
            nemu_assert!(A[i] == i as i32);
        }
    }

    // Sort again to ensure it remains sorted.
    bubble_sort();

    for i in 0..N {
        unsafe {
            nemu_assert!(A[i] == i as i32);
        }
    }

    println!("bubble-sort test passed!");
    0 // Return 0 for success.
}
