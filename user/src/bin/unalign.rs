#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

use core::ptr::{read_unaligned, write_unaligned};

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut buf: [u8; 16] = [0; 16];
    let mut x: u32 = 0xffffffff;
    for _ in 0..4 {
        // Pointer to the 3rd byte of the buffer
        let p = buf.as_mut_ptr().add(3) as *mut u32;

        // Perform an unaligned write
        write_unaligned(p, 0xaabbccdd);

        // Perform an unaligned read
        let x_val = read_unaligned(p);
        write_unaligned(&mut x, x_val);

        nemu_assert!(read_unaligned(&x) == 0xaabbccdd);

        // Clear first two bytes of buffer
        write_unaligned(buf.as_mut_ptr().add(0), 0);
        write_unaligned(buf.as_mut_ptr().add(1), 0);
    }
    println!("unalign test passed!");
    0
}
