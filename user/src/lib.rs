#![no_std]
#![feature(linkage)]
#![feature(alloc_error_handler)]

use core::{
    arch::{asm, global_asm},
    ptr::addr_of_mut,
};

use buddy_system_allocator::LockedHeap;
extern crate alloc;

mod config;
#[macro_use]
pub mod console;
pub mod driver;
mod lang_item;
pub mod rand;

#[inline(always)]
pub fn nemu_trap(code: usize) {
    unsafe {
        asm!(
            "mv a0, {0}",
            "ebreak",
            in(reg) code,
        );
    }
}

#[macro_export]
macro_rules! nemu_assert {
    ($cond:expr) => {
        if !$cond {
            $crate::nemu_trap(1);
        }
    };
    ($cond:expr, $msg:expr) => {
        if !$cond {
            $crate::println!("Assertion failed: {}", $msg);
            $crate::nemu_trap(1);
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            $crate::println!("Assertion failed: {}", format_args!($($arg)+));
            $crate::nemu_trap(1);
        }
    };
}

#[unsafe(no_mangle)]
fn halt(code: i32) -> ! {
    nemu_trap(code as usize);
    loop {}
}

global_asm!(include_str!("Start.S"));

#[unsafe(no_mangle)]
fn _trm_init() -> ! {
    unsafe {
        HEAP.lock()
            .init(addr_of_mut!(HEAP_SPACE) as usize, USER_HEAP_SIZE);
    }
    halt(main());
}

const USER_HEAP_SIZE: usize = 32768;

#[unsafe(no_mangle)]
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap<32> = LockedHeap::empty();

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Cannot find main!");
}
