#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

use core::ptr::{read_volatile, write_volatile};

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut a: [i32; 10] = [0; 10];
    let mut b: i32 = 0;
    let mut c: [i8; 10] = [0; 10];
    write_volatile(a.as_mut_ptr().add(0), 0);
    write_volatile(a.as_mut_ptr().add(1), 1);
    write_volatile(a.as_mut_ptr().add(2), 2);
    write_volatile(a.as_mut_ptr().add(3), 3);
    write_volatile(a.as_mut_ptr().add(4), 4);

    let b_val = read_volatile(a.as_ptr().add(3));
    write_volatile(&mut b as *mut i32, b_val);
    write_volatile(a.as_mut_ptr().add(5), b_val);

    write_volatile(c.as_mut_ptr().add(0), 'a' as i8);
    nemu_assert!(read_volatile(c.as_ptr().add(0)) == 'a' as i8);

    let c0_val = read_volatile(c.as_ptr().add(0));
    write_volatile(c.as_mut_ptr().add(1), c0_val);
    nemu_assert!(read_volatile(c.as_ptr().add(1)) == 'a' as i8);

    // Sign extension from i8 to i32
    let a0_val = read_volatile(c.as_ptr().add(0)) as i32;
    write_volatile(a.as_mut_ptr().add(0), a0_val);
    nemu_assert!(read_volatile(a.as_ptr().add(0)) == 'a' as i32);

    // Write 0x80 (-128) to C[1]
    write_volatile(c.as_mut_ptr().add(1), 0x80u8 as i8);
    // Sign extension from i8 to i32
    let a0_val_signed = read_volatile(c.as_ptr().add(1)) as i32;
    write_volatile(a.as_mut_ptr().add(0), a0_val_signed);

    nemu_assert!(read_volatile(a.as_ptr().add(1)) == 1);
    nemu_assert!(read_volatile(a.as_ptr().add(2)) == 2);
    nemu_assert!(read_volatile(a.as_ptr().add(3)) == 3);
    nemu_assert!(read_volatile(a.as_ptr().add(4)) == 4);
    nemu_assert!(read_volatile(&b as *const i32) == 3);
    nemu_assert!(read_volatile(a.as_ptr().add(5)) == 3);

    // In C, comparing a signed char 0x80 with an int 0xffffff80 involves promotion.
    // Rust's cast `as i32` performs the same sign extension.
    nemu_assert!((read_volatile(c.as_ptr().add(1)) as i32) == 0xffffff80u32 as i32);
    nemu_assert!(read_volatile(a.as_ptr().add(0)) == 0xffffff80u32 as i32);

    println!("movsx test passed!");
    0
}
