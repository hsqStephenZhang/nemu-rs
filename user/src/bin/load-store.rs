#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate user_lib;

use core::ptr::{read, write};

#[allow(unsafe_op_in_unsafe_fn)]
#[unsafe(no_mangle)]
pub unsafe fn main() -> i32 {
    let mut mem: [u16; 8] = [0x0, 0x0258, 0x4abc, 0x7fff, 0x8000, 0x8100, 0xabcd, 0xffff];
    let lh_ans: [u32; 8] = [
        0x00000000, 0x00000258, 0x00004abc, 0x00007fff, 0xffff8000, 0xffff8100, 0xffffabcd,
        0xffffffff,
    ];
    let lhu_ans: [u32; 8] = [
        0x00000000, 0x00000258, 0x00004abc, 0x00007fff, 0x00008000, 0x00008100, 0x0000abcd,
        0x0000ffff,
    ];
    let sh_ans: [u16; 8] = [
        0xfffd, 0xfff7, 0xffdf, 0xff7f, 0xfdff, 0xf7ff, 0xdfff, 0x7fff,
    ];
    let lwlr_ans: [u32; 4] = [0xbc025800, 0x007fff4a, 0xcd810080, 0x00ffffab];

    for i in 0..mem.len() {
        let val = read(mem.as_ptr().add(i)) as i16 as i32 as u32;
        nemu_assert!(val == lh_ans[i]);
    }

    for i in 0..mem.len() {
        let val = read(mem.as_ptr().add(i)) as u32;
        nemu_assert!(val == lhu_ans[i]);
    }

    // 模拟从未对齐的地址加载
    let ptr = mem.as_ptr() as *const u8;
    for i in 0..((mem.len() / 2) - 1) {
        let unaligned_ptr = (ptr.add(1) as *const u32).add(i);
        let x = core::ptr::read_unaligned(unaligned_ptr);
        // 注意：字节序可能会影响此测试的结果
        nemu_assert!(x == lwlr_ans[i]);
    }

    for i in 0..mem.len() {
        let val_to_write = !(1u16 << (2 * i + 1));
        write(mem.as_mut_ptr().add(i), val_to_write);
        nemu_assert!(read(mem.as_ptr().add(i)) == sh_ans[i]);
    }

    println!("load-store test passed!");
    0
}
