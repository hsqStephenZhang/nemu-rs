use lazy_static::lazy_static;

#[macro_use]
extern crate lazy_static;

pub mod am;
pub mod cpu;
pub mod device;
pub mod isa;
pub mod memory;
pub mod runtime;
pub mod utils;

fn main() {
    println!("Hello, world!");
}
