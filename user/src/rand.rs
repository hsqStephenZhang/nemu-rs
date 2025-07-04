use core::sync::atomic::{AtomicU32, Ordering};

static SEED: AtomicU32 = AtomicU32::new(1);

pub fn srand(new_seed: u32) {
    SEED.store(new_seed & 0x7fff, Ordering::Relaxed);
}

pub fn rand() -> u32 {
    // Load the current seed value.
    let mut current_seed = SEED.load(Ordering::Relaxed);
    loop {
        let next_seed = current_seed.wrapping_mul(214013).wrapping_add(2531011);
        match SEED.compare_exchange_weak(
            current_seed,
            next_seed,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                return (next_seed >> 16) & 0x7fff;
            }
            Err(newly_read_seed) => {
                current_seed = newly_read_seed;
            }
        }
    }
}
