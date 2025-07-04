#![no_std]
#![no_main]

use colorz::Colorize;

#[macro_use]
extern crate user_lib;

extern crate alloc;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("{}", "This line is blue.".blue());
    println!("{}", "This line is red.".red());
    println!("{}", "This line is red on blue.".red().on_blue());
    return 0;
}
