#![no_std]
#![no_main]

use core::{
    pin::Pin,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

#[macro_use]
extern crate user_lib;

struct AddOne(i32);

impl Future for AddOne {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        this.0 += 1;
        Poll::Ready(this.0)
    }
}

#[unsafe(no_mangle)]
fn main() -> i32 {
    let raw_waker = RawWaker::new(
        core::ptr::null(),
        &RawWakerVTable::new(
            |_: *const ()| todo!(),
            |_: *const ()| {},
            |_: *const ()| {},
            |_: *const ()| {},
        ),
    );
    let waker = unsafe { Waker::from_raw(raw_waker) };
    let mut ctx = Context::from_waker(&waker);
    let mut fut = AddOne(0);
    for i in 0..100 {
        match Pin::new(&mut fut).poll(&mut ctx) {
            Poll::Ready(result) => {
                nemu_assert!(result == i + 1);
            }
            Poll::Pending => {
                panic!("Future should not be pending");
            }
        }
    }

    println!("Future test passed");

    return 0;
}
