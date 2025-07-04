#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

fn is_prime(n: i32) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    if n % 3 == 0 {
        return n == 3;
    }
    let mut d = 5;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
        if n % d == 0 {
            return false;
        }
        d += 4;
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> i32 {
    let mut i: i64;
    let mut d: i32;
    let mut p: i64;
    let mut r: i64;
    let q: i32 = 929;

    if !is_prime(q) {
        return 1;
    }

    r = q as i64;
    while r > 0 {
        r <<= 1;
    }

    d = 2 * q + 1;
    loop {
        p = r;
        i = 1;
        while p != 0 {
            i = (i * i) % d as i64;
            if p < 0 {
                i *= 2;
            }
            if i > d as i64 {
                i -= d as i64;
            }
            p <<= 1;
        }
        if i != 1 {
            d += 2 * q;
        } else {
            break;
        }
    }

    nemu_assert!(d == 13007, "Expected d = 13007, got {}", d);
    println!("test passed!");
    0
}
