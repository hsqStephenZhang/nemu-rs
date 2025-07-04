#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

use core::ptr::{read_volatile, write_volatile};

#[allow(unsafe_op_in_unsafe_fn)]
#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut a: [i32; 10] = [0; 10];
    let mut b: i32 = 0;
    write_volatile(a.as_mut_ptr().add(0), 0);
    write_volatile(a.as_mut_ptr().add(1), 1);
    write_volatile(a.as_mut_ptr().add(2), 2);
    write_volatile(a.as_mut_ptr().add(3), 3);
    write_volatile(a.as_mut_ptr().add(4), 4);

    let b_val = read_volatile(a.as_ptr().add(3));
    write_volatile(&mut b as *mut i32, b_val);
    write_volatile(a.as_mut_ptr().add(5), b_val);

    nemu_assert!(read_volatile(a.as_ptr().add(0)) == 0);
    nemu_assert!(read_volatile(a.as_ptr().add(1)) == 1);
    nemu_assert!(read_volatile(a.as_ptr().add(2)) == 2);
    nemu_assert!(read_volatile(a.as_ptr().add(3)) == 3);
    nemu_assert!(read_volatile(a.as_ptr().add(4)) == 4);
    nemu_assert!(read_volatile(&b as *const i32) == 3);
    nemu_assert!(read_volatile(a.as_ptr().add(5)) == 3);

    println!("mov-c test passed!");
    0
}
