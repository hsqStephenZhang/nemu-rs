#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

fn add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

static TEST_DATA: [u32; 8] = [
    0, 1, 2, 0x7fffffff, 0x80000000, 0x80000001, 0xfffffffe, 0xffffffff,
];

// Expected answers, equivalent to the C code's `int ans[]`.
static ANS: [u32; 64] = [
    0, 0x1, 0x2, 0x7fffffff, 0x80000000, 0x80000001, 0xfffffffe, 0xffffffff, 0x1, 0x2, 0x3,
    0x80000000, 0x80000001, 0x80000002, 0xffffffff, 0, 0x2, 0x3, 0x4, 0x80000001, 0x80000002,
    0x80000003, 0, 0x1, 0x7fffffff, 0x80000000, 0x80000001, 0xfffffffe, 0xffffffff, 0, 0x7ffffffd,
    0x7ffffffe, 0x80000000, 0x80000001, 0x80000002, 0xffffffff, 0, 0x1, 0x7ffffffe, 0x7fffffff,
    0x80000001, 0x80000002, 0x80000003, 0, 0x1, 0x2, 0x7fffffff, 0x80000000, 0xfffffffe,
    0xffffffff, 0, 0x7ffffffd, 0x7ffffffe, 0x7fffffff, 0xfffffffc, 0xfffffffd, 0xffffffff, 0, 0x1,
    0x7ffffffe, 0x7fffffff, 0x80000000, 0xfffffffd, 0xfffffffe,
];

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let mut ans_idx = 0;
    // Iterate through all pairs of test data.
    for i in 0..TEST_DATA.len() {
        for j in 0..TEST_DATA.len() {
            // Add the numbers and check against the expected answer.
            let result = add(
                TEST_DATA[i] as i32,
                TEST_DATA[j] as i32,
            );
            let expected = ANS[ans_idx];
            nemu_assert!(
                result == expected as i32,
                "add(data[{}]={}, data[{}]={}) = {}, expected ANS[{}]={}",
                i,
                TEST_DATA[i],
                j,
                TEST_DATA[j],
                result,
                ans_idx,
                expected
            );
            ans_idx += 1;
        }
    }

    println!("add test passed!");
    0 // Return 0 for success.
}
