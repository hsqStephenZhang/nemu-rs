#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

const STR: &str = "The quick brown fox jumps over the lazy dog";

// Global mutable state for the CRC table and initialization flag.
// Accessing these is unsafe.
static mut TABLE: [u32; 256] = [0; 256];
static mut HAVE_TABLE: bool = false;

/// Generates the CRC32 lookup table.
/// This function should only be called once.
/// # Safety
/// This function is unsafe because it writes to static mutable variables.
/// It is not thread-safe.
unsafe fn make_table() {
    for i in 0..256 {
        let mut rem = i as u32;
        for _ in 0..8 {
            if (rem & 1) == 1 {
                rem >>= 1;
                rem ^= 0xedb88320;
            } else {
                rem >>= 1;
            }
        }
        TABLE[i] = rem;
    }
    HAVE_TABLE = true;
}

/// Calculates the CRC32 checksum for a byte slice.
fn rc_crc32(mut crc: u32, buf: &[u8]) -> u32 {
    // Safety: This block accesses and modifies static mutable variables.
    // The check `HAVE_TABLE` and the call to `make_table` are not thread-safe,
    // matching the behavior of the original C code.
    unsafe {
        if !HAVE_TABLE {
            make_table();
        }
    }

    crc = !crc;
    for &octet in buf {
        // Safety: Accessing the static TABLE is unsafe.
        unsafe {
            crc = (crc >> 8) ^ TABLE[((crc & 0xff) as u8 ^ octet) as usize];
        }
    }
    !crc
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    // Calculate CRC32 for the string.
    let res = rc_crc32(0, STR.as_bytes());

    // Check against the known correct value.
    nemu_assert!(res == 0x414FA339);

    println!("crc32 test passed!");
    0 // Return 0 for success.
}
