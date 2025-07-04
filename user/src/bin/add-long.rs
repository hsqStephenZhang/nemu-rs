#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

/// Adds two 64-bit integers.
///

fn add(a: i64, b: i64) -> i64 {
    a.wrapping_add(b)
}

// Test data, equivalent to the C code's `long long test_data[]`.
static TEST_DATA: [u64; 8] = [
    0,
    1,
    2,
    0x7fffffffffffffff,
    0x8000000000000000,
    0x8000000000000001,
    0xfffffffffffffffe,
    0xffffffffffffffff,
];

// Expected answers, equivalent to the C code's `long long ans[]`.
static ANS: [u64; 64] = [
    0,
    0x1,
    0x2,
    0x7fffffffffffffff,
    0x8000000000000000,
    0x8000000000000001,
    0xfffffffffffffffe,
    0xffffffffffffffff,
    0x1,
    0x2,
    0x3,
    0x8000000000000000,
    0x8000000000000001,
    0x8000000000000002,
    0xffffffffffffffff,
    0,
    0x2,
    0x3,
    0x4,
    0x8000000000000001,
    0x8000000000000002,
    0x8000000000000003,
    0,
    0x1,
    0x7fffffffffffffff,
    0x8000000000000000,
    0x8000000000000001,
    0xfffffffffffffffe,
    0xffffffffffffffff,
    0,
    0x7ffffffffffffffd,
    0x7ffffffffffffffe,
    0x8000000000000000,
    0x8000000000000001,
    0x8000000000000002,
    0xffffffffffffffff,
    0,
    0x1,
    0x7ffffffffffffffe,
    0x7fffffffffffffff,
    0x8000000000000001,
    0x8000000000000002,
    0x8000000000000003,
    0,
    0x1,
    0x2,
    0x7fffffffffffffff,
    0x8000000000000000,
    0xfffffffffffffffe,
    0xffffffffffffffff,
    0,
    0x7ffffffffffffffd,
    0x7ffffffffffffffe,
    0x7fffffffffffffff,
    0xfffffffffffffffc,
    0xfffffffffffffffd,
    0xffffffffffffffff,
    0,
    0x1,
    0x7ffffffffffffffe,
    0x7fffffffffffffff,
    0x8000000000000000,
    0xfffffffffffffffd,
    0xfffffffffffffffe,
];

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let mut ans_idx = 0;
    // Iterate through a pairs of test data.
    for i in 0..TEST_DATA.len() {
        for j in 0..TEST_DATA.len() {
            // Add the numbers and check against the expected answer.
            let result = add(TEST_DATA[i] as i64, TEST_DATA[j] as i64);
            let expected = ANS[ans_idx];
            nemu_assert!(
                result == expected as i64,
                "add(test_data[{}]={}, test_data[{}]={}) = {}, expected ANS[{}]={}",
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
    println!("add-longlong test passed!");
    0 // Return 0 for success.
}
