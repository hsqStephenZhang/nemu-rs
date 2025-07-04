#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

use core::cmp::max;

static TEST_DATA: [u32; 8] = [
    0, 1, 2, 0x7fffffff, 0x80000000, 0x80000001, 0xfffffffe, 0xffffffff,
];
static ANS: [u32; 64] = [
    0, 0x1, 0x2, 0x7fffffff, 0, 0, 0, 0, 0x1, 0x1, 0x2, 0x7fffffff, 0x1, 0x1, 0x1, 0x1, 0x2, 0x2,
    0x2, 0x7fffffff, 0x2, 0x2, 0x2, 0x2, 0x7fffffff, 0x7fffffff, 0x7fffffff, 0x7fffffff,
    0x7fffffff, 0x7fffffff, 0x7fffffff, 0x7fffffff, 0, 0x1, 0x2, 0x7fffffff, 0x80000000,
    0x80000001, 0xfffffffe, 0xffffffff, 0, 0x1, 0x2, 0x7fffffff, 0x80000001, 0x80000001,
    0xfffffffe, 0xffffffff, 0, 0x1, 0x2, 0x7fffffff, 0xfffffffe, 0xfffffffe, 0xfffffffe,
    0xffffffff, 0, 0x1, 0x2, 0x7fffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
];

#[allow(unsafe_op_in_unsafe_fn)]
#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut ans_idx = 0;
    for &i in TEST_DATA.iter() {
        for &j in TEST_DATA.iter() {
            // 修正原始 C 代码中的答案数组以匹配正确的 max 行为
            let correct_ans = max(i as i32, j as i32);
            nemu_assert!(
                correct_ans == ANS[ans_idx] as i32,
                "max({}, {}) = {}, expected {}",
                i,
                j,
                correct_ans,
                ANS[ans_idx]
            );
            ans_idx += 1;
        }
    }
    println!("max test passed!");
    0
}
