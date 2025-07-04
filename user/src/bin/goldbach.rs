#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

/// Checks if a number is prime.
/// Returns true if prime, false otherwise.
fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Checks if Goldbach's conjecture holds for a given even number `n`.
/// The conjecture states that every even integer greater than 2 is the sum of two primes.
/// Returns true if `n` can be expressed as the sum of two primes, false otherwise.
fn goldbach(n: i32) -> bool {
    for i in 2..n {
        if is_prime(i) && is_prime(n - i) {
            return true;
        }
    }
    false
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    // Test Goldbach's conjecture for even numbers from 4 to 30.
    // The `step_by(2)` makes the loop increment by 2.
    for n in (4..=30).step_by(2) {
        nemu_assert!(goldbach(n) == true);
    }

    println!("goldbach test passed!");
    0 // Return 0 for success.
}
