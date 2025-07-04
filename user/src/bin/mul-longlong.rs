#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

fn mul(a: i64, b: i64) -> i64 {
    a.wrapping_mul(b)
}

static TEST_DATA: [u32; 4] = [0xaeb1c2aa, 0x4500ff2b, 0x877190af, 0x11f42438];
static ANS: [u64; 10] = [
    0x19d29ab9db1a18e4, 0xea15986d3ac3088e, 0x2649e980fc0db236, 0xfa4c43da0a4a7d30,
    0x1299898e2c56b139, 0xdf8123d50a319e65, 0x04d6dfa84c15dd68, 0x38c5d79b9e4357a1,
    0xf78b91cb1efc4248, 0x014255a47fdfcc40
];

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut ans_idx = 0;
    for i in 0..TEST_DATA.len() {
        for j in i..TEST_DATA.len() {
            // Cast i32 to i64 before multiplication, mimicking C's type promotion.
            let res = mul(TEST_DATA[i] as i32 as i64, TEST_DATA[j] as i32 as i64);
            nemu_assert!(res == ANS[ans_idx] as i64);
            ans_idx += 1;
        }
    }
    println!("mul-longlong test passed!");
    0
}
