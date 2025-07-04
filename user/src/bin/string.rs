#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

// In a no_std environment, we mimic the C string operations using Rust's byte slices.
// This captures the logic of the tests without depending on a C standard library.

static S: [&[u8]; 6] = [
    b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab",
    b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    b", World!\n\0", // Include null terminator for C-style string logic
    b"Hello, World!\n\0",
    b"#####",
];

#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    // check(strcmp(s[0], s[2]) == 0);
    nemu_assert!(S[0] == S[2]);

    // check(strcmp(s[0], s[1]) < 0);
    nemu_assert!(S[0] < S[1]);
    nemu_assert!(&S[0][1..] < &S[1][1..]);
    nemu_assert!(&S[0][2..] < &S[1][2..]);
    nemu_assert!(&S[0][3..] < &S[1][3..]);

    // Mimic: check(strcmp( strcat(strcpy(str, str1), s[3]), s[4]) == 0);
    let str1 = b"Hello";
    let mut str_buf: [u8; 20] = [0; 20];

    // strcpy
    str_buf[..str1.len()].copy_from_slice(str1);

    // strcat
    let s3_no_null = &S[3][..S[3].len() - 1]; // Exclude null terminator for concat
    str_buf[str1.len()..str1.len() + s3_no_null.len()].copy_from_slice(s3_no_null);
    
    let s4_no_null = &S[4][..S[4].len() - 1];
    nemu_assert!(&str_buf[..s4_no_null.len()] == s4_no_null);

    // Mimic: check(memcmp(memset(str, '#', 5), s[5], 5) == 0);
    // memset
    str_buf[..5].fill(b'#');
    
    // memcmp
    nemu_assert!(&str_buf[..5] == S[5]);

    println!("string test passed!");
    0
}
