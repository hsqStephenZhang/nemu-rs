#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[repr(C)]
struct StackFrame {
    prev_fp: *const StackFrame,
    return_addr: usize,
}

#[unsafe(no_mangle)]
fn backtrace(max_depth: usize) {
    let mut fp: *const StackFrame;

    fn is_valid(fp: *const StackFrame) -> bool {
        !fp.is_null() && (fp as usize) != usize::MAX
    }

    unsafe {
        // 读取当前 frame pointer (s0)
        core::arch::asm!(
            "mv {0}, fp",
            out(reg) fp,
        );
        // fp 为 frame pointer 结构的 end offset，调整为 start offset 即可使用
        fp = fp.offset(-1);

        let mut i = 0;
        while is_valid(fp) && i < max_depth {
            let ra = (*fp).return_addr;
            println!("Frame {}: return address = {:#x}", i, ra);
            fp = (*fp).prev_fp;
            i += 1;
        }
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
fn t1() -> i32 {
    t2();
    return 0;
}

#[unsafe(no_mangle)]
#[inline(never)]
fn t2() -> i32 {
    t3();
    return 0;
}

/**
 * here the return value is (), so the compiler will not keep the stack frame
 * the assembly looks like this, notice that `addi sp, sp,16` is called before `jr 8(t1)`
 * 000000008000006c <t3>:
    8000006c:   ff010113                addi    sp,sp,-16
    80000070:   00113423                sd      ra,8(sp)
    80000074:   00813023                sd      s0,0(sp)
    80000078:   01010413                addi    s0,sp,16
    8000007c:   00813083                ld      ra,8(sp)
    80000080:   00013403                ld      s0,0(sp)
    80000084:   01010113                addi    sp,sp,16
    80000088:   00000317                auipc   t1,0x0
    8000008c:   00830067                jr      8(t1) # 80000090 <t4>
 */
#[unsafe(no_mangle)]
#[inline(never)]
fn t3() {
    t4();
}

#[unsafe(no_mangle)]
#[inline(never)]
fn t4() {
    backtrace(10);
}

#[unsafe(no_mangle)]
fn main() -> i32 {
    let _ = t1();
    println!("backtrace test passed!");
    return 0;
}
