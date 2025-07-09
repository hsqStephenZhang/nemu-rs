#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

#[macro_use]
extern crate user_lib;

macro_rules! gen_atomic_alu_test_simple {
    ($name:ident, $bits:expr, $atomic_type:ty, $base_type:ty) => {
        #[unsafe(no_mangle)]
        fn $name() -> i32 {
            let atomic = <$atomic_type>::new(0);

            // Initial value test
            nemu_assert!(
                atomic.load(Ordering::Relaxed) == 0,
                "Initial value should be 0"
            );

            // Compare and exchange - successful
            let result = atomic.compare_exchange(0, 10, Ordering::Relaxed, Ordering::Relaxed);
            nemu_assert!(
                result.is_ok() && result.unwrap() == 0,
                "Compare exchange should succeed and return old value 0"
            );
            nemu_assert!(
                atomic.load(Ordering::Relaxed) == 10,
                "Value should be 10 after compare_exchange"
            );

            // Fetch add
            let old_val = atomic.fetch_add(5, Ordering::Relaxed);
            nemu_assert!(old_val == 10, "fetch_add should return old value 10");
            nemu_assert!(
                atomic.load(Ordering::Relaxed) == 15,
                "Value should be 15 after fetch_add(5)"
            );

            // Fetch sub
            let old_val = atomic.fetch_sub(3, Ordering::Relaxed);
            nemu_assert!(old_val == 15, "fetch_sub should return old value 15");
            nemu_assert!(
                atomic.load(Ordering::Relaxed) == 12,
                "Value should be 12 after fetch_sub(3)"
            );

            // Bitwise operations
            let old_val = atomic.fetch_and(0b1111, Ordering::Relaxed);
            nemu_assert!(old_val == 12, "fetch_and should return old value 12");

            let old_val = atomic.fetch_or(0b10000, Ordering::Relaxed);
            nemu_assert!(old_val == 12, "fetch_or should return old value 12");
            nemu_assert!(
                atomic.load(Ordering::Relaxed) == 28,
                "Value should be 28 after fetch_or(0b10000)"
            );

            let old_val = atomic.fetch_xor(0b11111, Ordering::Relaxed);
            nemu_assert!(old_val == 28, "fetch_xor should return old value 28");
            nemu_assert!(
                atomic.load(Ordering::Relaxed) == 3,
                "Value should be 3 after fetch_xor(0b11111)"
            );

            // Min/Max operations
            atomic.store(5, Ordering::Relaxed);
            let old_val = atomic.fetch_max(10, Ordering::Relaxed);
            nemu_assert!(old_val == 5, "fetch_max should return old value 5");
            nemu_assert!(
                atomic.load(Ordering::Relaxed) == 10,
                "Value should be 10 after fetch_max(10)"
            );

            let old_val = atomic.fetch_min(7, Ordering::Relaxed);
            nemu_assert!(old_val == 10, "fetch_min should return old value 10");
            nemu_assert!(
                atomic.load(Ordering::Relaxed) == 7,
                "Value should be 7 after fetch_min(7)"
            );

            // Swap
            let old_val = atomic.swap(42, Ordering::Relaxed);
            nemu_assert!(old_val == 7, "swap should return old value 7");
            nemu_assert!(
                atomic.load(Ordering::Relaxed) == 42,
                "Value should be 42 after swap"
            );

            println!(
                "{}-bit {} ALU test passed!",
                $bits,
                stringify!($atomic_type)
            );
            0
        }
    };
}

gen_atomic_alu_test_simple!(test_i8, 8, core::sync::atomic::AtomicU8, u8);
gen_atomic_alu_test_simple!(test_i16, 16, core::sync::atomic::AtomicU16, u16);
gen_atomic_alu_test_simple!(test_i32, 32, core::sync::atomic::AtomicU32, u32);
gen_atomic_alu_test_simple!(test_i64, 64, core::sync::atomic::AtomicU64, u64);
gen_atomic_alu_test_simple!(test_u8, 8, core::sync::atomic::AtomicI8, i8);
gen_atomic_alu_test_simple!(test_u16, 16, core::sync::atomic::AtomicI16, i16);
gen_atomic_alu_test_simple!(test_u32, 32, core::sync::atomic::AtomicI32, i32);
gen_atomic_alu_test_simple!(test_u64, 64, core::sync::atomic::AtomicI64, i64);

#[unsafe(no_mangle)]
fn main() -> i32 {
    let atomic = AtomicU32::new(0);
    atomic.fetch_add(0b10, Ordering::Relaxed);
    atomic.fetch_xor(0b01, Ordering::Relaxed);
    atomic.fetch_or(0b100, Ordering::Relaxed);
    nemu_assert!(
        atomic.load(Ordering::Relaxed) == 0b111,
        "Final value should be 0b111"
    );
    atomic.fetch_min(0b100, Ordering::Relaxed);
    nemu_assert!(
        atomic.load(Ordering::Relaxed) == 0b100,
        "Final value should be 0b100 after fetch_min(0b100)"
    );
    atomic.fetch_max(0b1111111, Ordering::Relaxed);
    nemu_assert!(
        atomic.load(Ordering::Relaxed) == 0b1111111,
        "Final value should be 0b1111111 after fetch_max(0b1111111)"
    );

    let atomic = AtomicU64::new(0);
    atomic.fetch_add(0b10, Ordering::Relaxed);
    atomic.fetch_xor(0b01, Ordering::Relaxed);
    atomic.fetch_or(0b100, Ordering::Relaxed);
    nemu_assert!(
        atomic.load(Ordering::Relaxed) == 0b111,
        "Final value should be 0b111"
    );
    atomic.fetch_min(0b100, Ordering::Relaxed);
    nemu_assert!(
        atomic.load(Ordering::Relaxed) == 0b100,
        "Final value should be 0b100 after fetch_min(0b100)"
    );
    atomic.fetch_max(0b1111111, Ordering::Relaxed);
    nemu_assert!(
        atomic.load(Ordering::Relaxed) == 0b1111111,
        "Final value should be 0b1111111 after fetch_max(0b1111111)"
    );

    println!("atomics test passed!");
    return 0;
}
