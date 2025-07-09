#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    drain_keys();
    return 0;
}

pub fn drain_keys() {
    loop {
        let key = user_lib::driver::get_key();
        match key {
            Some((key_code, is_down)) => {
                if is_down {
                    println!("Key pressed: {:?}", key_code);
                } else {
                    println!("Key released: {:?}", key_code);
                }
            }
            None => continue,
        }
    }
}
