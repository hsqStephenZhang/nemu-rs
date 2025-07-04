#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

const N: usize = 20;

static mut A: [i32; N] = [
    2, 12, 14, 6, 13, 15, 16, 10, 0, 18, 11, 19, 9, 1, 7, 5, 4, 3, 8, 17,
];

fn select_sort() {
    unsafe {
        for i in 0..(N - 1) {
            let mut k = i;
            for j in (i + 1)..N {
                if A[j] < A[k] {
                    k = j;
                }
            }
            // Swap A[i] and A[k]
            let t = A[i];
            A[i] = A[k];
            A[k] = t;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    select_sort();
    for i in 0..N {
        nemu_assert!(A[i] == i as i32);
    }

    // Sort again to check behavior on sorted array
    select_sort();
    for i in 0..N {
        nemu_assert!(A[i] == i as i32);
    }

    println!("select-sort test passed!");
    0
}
