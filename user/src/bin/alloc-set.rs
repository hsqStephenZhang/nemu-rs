#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

extern crate alloc;

#[unsafe(no_mangle)]
fn main() -> i32 {
    let mut set = hashbrown::HashSet::new();
    for i in 0..100 {
        set.insert(i);
    }

    for i in 0..100 {
        nemu_assert!(set.contains(&i), "HashSet should contain {}", i);
    }

    let mut set = alloc::collections::BTreeSet::new();
        for i in 0..100 {
        set.insert(i);
    }

    for i in 0..100 {
        nemu_assert!(set.contains(&i), "HashSet should contain {}", i);
    }

    println!("alloc-set test passed!");
    return 0;
}
