#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

const N: usize = 31;

static mut A: [i32; N] = [0; N];
static ANS: [i32; N] = [
    1, 30, 435, 4060, 27405, 142506, 593775, 2035800, 5852925, 14307150, 30045015,
    54627300, 86493225, 119759850, 145422675, 155117520, 145422675, 119759850,
    86493225, 54627300, 30045015, 14307150, 5852925, 2035800, 593775, 142506,
    27405, 4060, 435, 30, 1,
];

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    A[0] = 1;
    A[1] = 1;

    for i in 2..N {
        let mut t0 = 1;
        for j in 1..i {
            let t1 = A[j];
            A[j] = t0 + t1;
            t0 = t1;
        }
        A[i] = 1;
    }

    for j in 0..N {
        nemu_assert!(A[j] == ANS[j]);
    }

    println!("pascal test passed!");
    0
}
