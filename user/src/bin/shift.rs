#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

static TEST: [u32; 8] = [
    0x12345678, 0x98765432, 0x0, 0xeffa1000, 0x7fffffff, 0x80000000, 0x33, 0xffffffff,
];

static SRL_ANS: [u32; 8] = [
    0x2468ac, 0x130eca8, 0x0, 0x1dff420, 0xffffff, 0x1000000, 0x0, 0x1ffffff,
];

static SRLV_ANS: [u32; 8] = [
    0x1234567, 0x4c3b2a1, 0x0, 0x1dff420, 0x7fffff, 0x400000, 0x0, 0x1fffff,
];

static SRAV_ANS: [u32; 8] = [
    0x1234567, 0xfcc3b2a1, 0x0, 0xffdff420, 0x7fffff, 0xffc00000, 0x0, 0xffffffff,
];

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    for i in 0..TEST.len() {
        // Logical right shift
        nemu_assert!((TEST[i] >> 7) == SRL_ANS[i]);
    }

    for i in 0..TEST.len() {
        // Arithmetic right shift
        let res = (TEST[i] as i32) >> (i + 4);
        nemu_assert!(res as u32 == SRAV_ANS[i]);
    }

    for i in 0..TEST.len() {
        // Logical right shift by variable
        nemu_assert!((TEST[i] >> (i + 4)) == SRLV_ANS[i]);
    }

    println!("shift test passed!");
    0
}
