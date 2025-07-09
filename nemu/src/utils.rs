use std::cell::UnsafeCell;

// SAFETY: used in a single-threaded context
pub struct UPSafeCellRaw<T> {
    inner: UnsafeCell<T>,
}

unsafe impl<T> Sync for UPSafeCellRaw<T> {}

impl<T> UPSafeCellRaw<T> {
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut (*self.inner.get()) }
    }

    pub fn get(&self) -> &T {
        unsafe { &(*self.inner.get()) }
    }
}
