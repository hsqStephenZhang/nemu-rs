#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

fn sub(a: i64, b: i64) -> i64 {
    a.wrapping_sub(b)
}

const TEST_DATA: &[u64] = &[
    0,
    1,
    2,
    0x7fffffffffffffff,
    0x8000000000000000,
    0x8000000000000001,
    0xfffffffffffffffe,
    0xffffffffffffffff,
];
const ANS: &[u64] = &[
    0,
    0xffffffffffffffff,
    0xfffffffffffffffe,
    0x8000000000000001,
    0x8000000000000000,
    0x7fffffffffffffff,
    0x2,
    0x1,
    0x1,
    0,
    0xffffffffffffffff,
    0x8000000000000002,
    0x8000000000000001,
    0x8000000000000000,
    0x3,
    0x2,
    0x2,
    0x1,
    0,
    0x8000000000000003,
    0x8000000000000002,
    0x8000000000000001,
    0x4,
    0x3,
    0x7fffffffffffffff,
    0x7ffffffffffffffe,
    0x7ffffffffffffffd,
    0,
    0xffffffffffffffff,
    0xfffffffffffffffe,
    0x8000000000000001,
    0x8000000000000000,
    0x8000000000000000,
    0x7fffffffffffffff,
    0x7ffffffffffffffe,
    0x1,
    0,
    0xffffffffffffffff,
    0x8000000000000002,
    0x8000000000000001,
    0x8000000000000001,
    0x8000000000000000,
    0x7fffffffffffffff,
    0x2,
    0x1,
    0,
    0x8000000000000003,
    0x8000000000000002,
    0xfffffffffffffffe,
    0xfffffffffffffffd,
    0xfffffffffffffffc,
    0x7fffffffffffffff,
    0x7ffffffffffffffe,
    0x7ffffffffffffffd,
    0,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xfffffffffffffffe,
    0xfffffffffffffffd,
    0x8000000000000000,
    0x7fffffffffffffff,
    0x7ffffffffffffffe,
    0x1,
    0,
];

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut ans_idx = 0;
    for &i in TEST_DATA.iter() {
        for &j in TEST_DATA.iter() {
            let result = sub(i as i64, j as i64);
            nemu_assert!(result == ANS[ans_idx] as i64);
            ans_idx += 1;
        }
    }
    println!("sub-longlong test passed!");
    0
}
