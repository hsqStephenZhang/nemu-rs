#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

use core::ptr::{read_volatile, write_volatile};

#[macro_use]
extern crate user_lib;

/// Gets the value of a bit at a specific offset in a byte buffer.
///
/// # Arguments
/// * `buf` - A mutable pointer to the buffer.
/// * `offset` - The bit offset from the start of the buffer.
///
/// # Safety
/// This function is unsafe because it dereferences a raw pointer.
#[inline(never)]
unsafe fn get_bit(buf: *const u8, offset: i32) -> bool {
    let byte_index = (offset >> 3) as isize;
    let bit_offset = offset & 7;
    let mask = 1 << bit_offset;
    // Read the byte from the buffer.
    let byte = read_volatile(buf.offset(byte_index));
    (byte & mask) != 0
}

/// Sets the value of a bit at a specific offset in a byte buffer.
///
/// # Arguments
/// * `buf` - A mutable pointer to the buffer.
/// * `offset` - The bit offset from the start of the buffer.
/// * `bit` - The boolean value to set the bit to.
///
/// # Safety
/// This function is unsafe because it dereferences and writes to a raw pointer.
#[inline(never)]
unsafe fn set_bit(buf: *mut u8, offset: i32, bit: bool) {
    let byte_index = (offset >> 3) as isize;
    let bit_offset = offset & 7;
    let mask = 1 << bit_offset;
    let p = buf.offset(byte_index);

    // Volatile read-modify-write to match the C code's behavior.
    let mut current_byte = read_volatile(p);
    if bit {
        current_byte |= mask;
    } else {
        current_byte &= !mask;
    }
    write_volatile(p, current_byte);
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    // Create a buffer on the stack.
    let mut buf: [u8; 2] = [0; 2];

    buf[0] = 0xaa;
    buf[1] = 0x00;

    // The `unsafe` block is required to call our unsafe functions.
    unsafe {
        // Test get_bit
        nemu_assert!(get_bit(buf.as_ptr(), 0) == false);
        nemu_assert!(get_bit(buf.as_ptr(), 1) == true);
        nemu_assert!(get_bit(buf.as_ptr(), 2) == false);
        nemu_assert!(get_bit(buf.as_ptr(), 3) == true);
        nemu_assert!(get_bit(buf.as_ptr(), 4) == false);
        nemu_assert!(get_bit(buf.as_ptr(), 5) == true);
        nemu_assert!(get_bit(buf.as_ptr(), 6) == false);
        nemu_assert!(get_bit(buf.as_ptr(), 7) == true);

        // Test set_bit
        set_bit(buf.as_mut_ptr(), 8, true);
        set_bit(buf.as_mut_ptr(), 9, false);
        set_bit(buf.as_mut_ptr(), 10, true);
        set_bit(buf.as_mut_ptr(), 11, false);
        set_bit(buf.as_mut_ptr(), 12, true);
        set_bit(buf.as_mut_ptr(), 13, false);
        set_bit(buf.as_mut_ptr(), 14, true);
        set_bit(buf.as_mut_ptr(), 15, false);
    }

    // Check the final state of the buffer.
    nemu_assert!(buf[1] == 0x55);

    println!("bit test passed!");
    0 // Return 0 for success.
}
