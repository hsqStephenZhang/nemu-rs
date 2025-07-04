#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

// Global mutable state for recursion tracking
static mut REC_COUNT: i32 = 0;
static mut MAX_LEVEL: i32 = 0;

// Function pointer array for mutual recursion
static FUNC: [fn(i32, i32) -> i32; 4] = [f0, f1, f2, f3];

fn f0(n: i32, l: i32) -> i32 {
    unsafe {
        if l > MAX_LEVEL { MAX_LEVEL = l; }
        REC_COUNT += 1;
        if n <= 0 { 1 } else { FUNC[3](n / 3, l + 1) }
    }
}

fn f1(n: i32, l: i32) -> i32 {
    unsafe {
        if l > MAX_LEVEL { MAX_LEVEL = l; }
        REC_COUNT += 1;
        if n <= 0 { 1 } else { FUNC[0](n - 1, l + 1) }
    }
}

fn f2(n: i32, l: i32) -> i32 {
    unsafe {
        if l > MAX_LEVEL { MAX_LEVEL = l; }
        REC_COUNT += 1;
        if n <= 0 { 1 } else { FUNC[1](n, l + 1) + 9 }
    }
}

fn f3(n: i32, l: i32) -> i32 {
    unsafe {
        if l > MAX_LEVEL { MAX_LEVEL = l; }
        REC_COUNT += 1;
        if n <= 0 { 1 } else { FUNC[2](n / 2, l + 1) * 3 + FUNC[2](n / 2, l + 1) * 2 }
    }
}

static ANS: [i32; 3] = [38270, 218, 20];

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let x = FUNC[0](14371, 0);
    nemu_assert!(x == ANS[0]);       // answer
    nemu_assert!(REC_COUNT == ANS[1]); // # recursions
    nemu_assert!(MAX_LEVEL == ANS[2]); // max depth
    
    println!("recursion test passed!");
    0
}
