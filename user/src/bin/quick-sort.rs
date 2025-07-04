#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

const N: usize = 20;

fn partition(a: &mut [i32]) -> usize {
    let pivot = a[0];
    let mut i = 0;
    let mut j = a.len() - 1;
    while i < j {
        while i < j && a[j] > pivot {
            j -= 1;
        }
        a[i] = a[j];

        while i < j && a[i] <= pivot {
            i += 1;
        }
        a[j] = a[i];
    }
    a[i] = pivot;
    i
}

fn quick_sort(a: &mut [i32]) {
    if a.len() <= 1 {
        return;
    }
    let m = partition(a);
    quick_sort(&mut a[0..m]);
    if m + 1 < a.len() {
        quick_sort(&mut a[m + 1..]);
    }
}

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut a: [i32; N] = [
        2, 12, 14, 6, 13, 15, 16, 10, 0, 18, 11, 19, 9, 1, 7, 5, 4, 3, 8, 17,
    ];
    quick_sort(&mut a);
    for i in 0..N {
        nemu_assert!(a[i] == i as i32);
    }

    // Sort again to test stability on an already sorted array
    quick_sort(&mut a);
    for i in 0..N {
        nemu_assert!(a[i] == i as i32);
    }

    println!("quick-sort test passed!");
    0
}
