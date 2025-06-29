/***
 *
 * void (*ref_difftest_memcpy)(paddr_t addr, void *buf, size_t n, bool direction) = NULL;
 * void (*ref_difftest_regcpy)(void *dut, bool direction) = NULL;
 * void (*ref_difftest_exec)(uint64_t n) = NULL;
 * void (*ref_difftest_raise_intr)(uint64_t NO) = NULL;
 */

pub mod qemu;

// fixed to use u64 for paddr_t
// TODO: use compile flags to determine the paddr_t type
type Paddr = u64;

pub type DifftestMemcpy = extern "C" fn(Paddr, *mut u8, usize, bool);
pub type DifftestRegcpy = extern "C" fn(*mut u8, bool);
pub type DifftestExec = extern "C" fn(u64);
pub type DifftestRaiseIntr = extern "C" fn(u64);
pub type DifftestInit = extern "C" fn(u32);

pub struct DifftestUtilFns<'lib> {
    pub memcpy_fn: libloading::Symbol<'lib, DifftestMemcpy>,
    pub regcpy_fn: libloading::Symbol<'lib, DifftestRegcpy>,
    pub exec_fn: libloading::Symbol<'lib, DifftestExec>,
    pub raise_intr_fn: libloading::Symbol<'lib, DifftestRaiseIntr>,
    pub init_fn: libloading::Symbol<'lib, DifftestInit>,
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn load_difftest_functions<'lib>(
    lib: &'lib libloading::Library,
) -> std::result::Result<DifftestUtilFns<'lib>, Box<dyn std::error::Error>> {
    let init_fn: libloading::Symbol<DifftestInit> = lib.get(b"difftest_init")?;
    let memcpy_fn: libloading::Symbol<DifftestMemcpy> = lib.get(b"difftest_memcpy")?;
    let regcpy_fn: libloading::Symbol<DifftestRegcpy> = lib.get(b"difftest_regcpy")?;
    let exec_fn: libloading::Symbol<DifftestExec> = lib.get(b"difftest_exec")?;
    let raise_intr_fn: libloading::Symbol<DifftestRaiseIntr> = lib.get(b"difftest_raise_intr")?;
    println!(
        "fn difftest_init: {:?}, memcpy: {:?}, regcpy: {:?}, exec: {:?}, raise_intr: {:?}",
        init_fn, memcpy_fn, regcpy_fn, exec_fn, raise_intr_fn
    );

    let res = DifftestUtilFns {
        memcpy_fn,
        regcpy_fn,
        exec_fn,
        raise_intr_fn,
        init_fn,
    };

    Ok(res)
}

#[cfg(test)]
mod tests {

    use super::*;

    // cpu state for riscv64
    // CAVEAT: should build the spike-diff with ISA riscv64
    // or the spike-diff's cpu register might be 32 bit long
    #[repr(C)]
    #[derive(Debug, Default, Clone, Copy)]
    struct Cpu {
        general: [u64; 32],
        pc: u64,
    }

    impl Cpu {
        pub fn new(pc: u64) -> Self {
            let mut cpu = Cpu::default();
            cpu.pc = pc;
            cpu
        }

        pub fn as_mut_ptr(&mut self) -> *mut u8 {
            self as *mut Cpu as *mut u8
        }
    }

    #[test]
    fn test_load_spike() -> std::result::Result<(), Box<dyn std::error::Error>> {
        test_diff_so("/workspaces/course/ics-pa/nemu/tools/spike-diff/build/-spike-so")?;
        test_diff_so("/workspaces/course/ics-pa/nemu/tools/qemu-diff/build/-qemu-so")?;
        Ok(())
    }

    fn test_diff_so(lib: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        println!("loading diff so: {}", lib);
        let lib = unsafe { libloading::Library::new(lib).unwrap() };

        let DifftestUtilFns {
            init_fn,
            memcpy_fn,
            regcpy_fn,
            exec_fn,
            raise_intr_fn: _,
        } = unsafe { load_difftest_functions(&lib)? };
        // init with gdb port (unused in spike)
        init_fn(1234);

        let boring: &[u8] = &[
            0x13, 0x01, 0x01, 0xfe, // addi    sp,sp,-32
        ]
        .repeat(10);

        memcpy_fn(0x80000000, boring.as_ptr() as *mut u8, boring.len(), true);

        let mut reset_cpu = Cpu::new(0x80000000);
        reset_cpu.general[2] = 0x80000000 - 32 * 10; // sp = 0x80000000 - 32 * 10
        regcpy_fn(reset_cpu.as_mut_ptr(), true);

        for i in 0..10 {
            exec_fn(1);
            let mut cpu = Cpu::default();
            regcpy_fn(cpu.as_mut_ptr(), false);
            assert_eq!(cpu.pc, 0x80000000 + (i + 1) * 4);
            assert_eq!(cpu.general[2], 0x80000000 - 32 * 10 - 32 * (i + 1));
            println!("cpu: {:x?}", cpu);
        }
        println!("");

        Ok(())
    }
}
