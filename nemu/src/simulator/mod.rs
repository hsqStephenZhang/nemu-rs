use crate::{addr_space::AddressSpace, cpu::Cpu, timer::virtual_clock::VirtualClock};

pub struct Simulator<C: Cpu> {
    cpu: C,
    addr_space: AddressSpace,
    virtual_clock: VirtualClock,
}

impl<C: Cpu<Context = Ctx>, Ctx> Simulator<C> {
    pub fn new(cpu: C, addr_space: AddressSpace) -> Self {
        Self {
            cpu,
            addr_space,
            virtual_clock: VirtualClock::new(),
        }
    }

    pub fn run(&mut self, ctx: &mut Ctx, max_instructions: usize) -> usize {
        self.cpu.exec(ctx, &mut self.addr_space, max_instructions)
    }

    pub fn clock(&mut self) -> &mut VirtualClock {
        &mut self.virtual_clock
    }

    pub fn cpu(&self) -> &C {
        &self.cpu
    }

    pub fn addr_space(&self) -> &AddressSpace {
        &self.addr_space
    }
}

#[cfg(test)]
mod riscv64_tests {

    use std::env;
    use std::{io::Read, u32, u64};

    use once_cell::sync::Lazy;
    use tracing::info;

    use crate::cpu::riscv64::regs::GRegName;
    use crate::cpu::{Cpu, NemuState, riscv64::*};
    use crate::device;
    use crate::timer::virtual_clock::VirtualClock;
    use crate::{
        addr_space::*,
        config::{MBASE, MSIZE},
        init_log,
        memory::PhyMem,
    };

    static CPU_TESTS_DIR: Lazy<String> = Lazy::new(|| {
        let pa_home = env::var("PA_HOME").expect("Environment variable PA_HOME is not set");
        let path = std::path::PathBuf::new()
            .join(pa_home)
            .join("am-kernels")
            .join("tests")
            .join("cpu-tests")
            .join("build");
        path.to_str().unwrap().to_string()
    });

    static ALU_TESTS_DIR: Lazy<String> = Lazy::new(|| {
        let pa_home = env::var("PA_HOME").expect("Environment variable PA_HOME is not set");
        let path = std::path::PathBuf::new()
            .join(pa_home)
            .join("am-kernels")
            .join("tests")
            .join("alu-tests")
            .join("build");
        path.to_str().unwrap().to_string()
    });

    const TARGET_DIR: Lazy<String> = Lazy::new(|| {
        let mut project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        project_dir = project_dir.strip_suffix("/nemu").unwrap().to_string();
        let path = std::path::PathBuf::new()
            .join(project_dir)
            .join("target")
            .join("riscv64gc-unknown-none-elf")
            .join("release");
        path.to_str().unwrap().to_string()
    });

    fn run_cpu_test_once(testcase: &str) {
        let file = format!("{}{}", CPU_TESTS_DIR.as_str(), testcase);
        run_test_once(&file);
    }

    fn run_alu_test_once() {
        let file = format!("{}{}", ALU_TESTS_DIR.as_str(), "alutest-riscv64-nemu.bin");
        run_test_once(&file);
    }

    fn run_my_test_once(testcase: &str) {
        let file = format!("{}/{}", TARGET_DIR.as_str(), testcase);
        run_test_once(&file);
    }

    fn run_test_once(file: &str) {
        info!("testing test with {}", file);
        let mut addr_space =
            AddressSpace::new(PhyMem::new(PAddr(MBASE), MSIZE as usize)).with_default_mmio();
        let mut cpu = RISCV64::new(mmu::MMU::new(mmu::Mode::Bare));

        let mut file = std::fs::File::open(file).unwrap();
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();

        info!("loaded {} bytes", content.len());

        cpu.mmu_mut()
            .load_program(&mut addr_space, VAddr(MBASE), &content)
            .unwrap();

        let mut clock = VirtualClock::new();
        device::init(&mut clock);

        let res = cpu.exec(&mut clock, &mut addr_space, usize::MAX);
        info!("executed instructions count: {:?}", res);
        assert_eq!(cpu.state(), NemuState::End);
        assert_eq!(cpu.halt_ret(), 0x0);
    }

    #[test]
    fn cpu_tests() {
        run_cpu_test_once("add-riscv64-nemu.bin");
        run_cpu_test_once("add-longlong-riscv64-nemu.bin");
        run_cpu_test_once("bubble-sort-riscv64-nemu.bin");
        run_cpu_test_once("crc32-riscv64-nemu.bin");
        run_cpu_test_once("div-riscv64-nemu.bin");
        run_cpu_test_once("dummy-riscv64-nemu.bin");
        run_cpu_test_once("fact-riscv64-nemu.bin");
        // TODO: move it to cpu_tests after finish printf, vprintf, sprintf in PA
        // it's up the to the provider of the image
        // test_alu_once("hello-str-riscv64-nemu.bin");
        run_cpu_test_once("if-else-riscv64-nemu.bin");
        run_cpu_test_once("leap-year-riscv64-nemu.bin");
        run_cpu_test_once("load-store-riscv64-nemu.bin");
        run_cpu_test_once("matrix-mul-riscv64-nemu.bin");
        run_cpu_test_once("max-riscv64-nemu.bin");
        run_cpu_test_once("mersenne-riscv64-nemu.bin");
        run_cpu_test_once("min3-riscv64-nemu.bin");
        run_cpu_test_once("mov-c-riscv64-nemu.bin");
        run_cpu_test_once("movsx-riscv64-nemu.bin");
        run_cpu_test_once("mul-longlong-riscv64-nemu.bin");
        run_cpu_test_once("pascal-riscv64-nemu.bin");
        run_cpu_test_once("prime-riscv64-nemu.bin");
        run_cpu_test_once("quick-sort-riscv64-nemu.bin");
        run_cpu_test_once("recursion-riscv64-nemu.bin");
        run_cpu_test_once("select-sort-riscv64-nemu.bin");
        run_cpu_test_once("shift-riscv64-nemu.bin");
        run_cpu_test_once("sum-riscv64-nemu.bin");
        run_cpu_test_once("switch-riscv64-nemu.bin");
        run_cpu_test_once("switch-riscv64-nemu.bin");
        run_cpu_test_once("to-lower-case-riscv64-nemu.bin");
        run_cpu_test_once("unalign-riscv64-nemu.bin");
        run_cpu_test_once("wanshu-riscv64-nemu.bin");
    }

    #[test]
    fn test_alu() {
        init_log(tracing::Level::INFO);
        run_alu_test_once();
    }

    #[test]
    fn test_userlib_alu() {
        init_log(tracing::Level::INFO);

        run_my_test_once("add.bin");
        run_my_test_once("add-long.bin");
        run_my_test_once("bit.bin");
        run_my_test_once("crc32.bin");
        run_my_test_once("div.bin");
        run_my_test_once("fact.bin");
        run_my_test_once("fib.bin");
        run_my_test_once("goldbach.bin");
        run_my_test_once("dummy.bin");
        run_my_test_once("if-else.bin");
        run_my_test_once("leap-year.bin");
        run_my_test_once("load-store.bin");
        run_my_test_once("matrix-mul.bin");
        run_my_test_once("max.bin");
        run_my_test_once("mersenne.bin");
        run_my_test_once("min3.bin");
        run_my_test_once("mov-c.bin");
        run_my_test_once("mul-longlong.bin");
        run_my_test_once("pascal.bin");
        run_my_test_once("prime.bin");
        run_my_test_once("quick-sort.bin");
        run_my_test_once("recursion.bin");
        run_my_test_once("select-sort.bin");
        run_my_test_once("shift.bin");
        run_my_test_once("shuixianhua.bin");
        run_my_test_once("string.bin");
        run_my_test_once("sub-longlong.bin");
        run_my_test_once("sum.bin");
        run_my_test_once("switch.bin");
        run_my_test_once("to-lower-case.bin");
        run_my_test_once("unalign.bin");
        run_my_test_once("wanshu.bin");
    }

    #[test]
    fn test_userlib_atomics() {
        for case in &[
            "alloc.bin",
            "alloc-set.bin",
            "rand.bin",
            "atomics.bin",
            "future.bin",
        ] {
            run_my_test_once(case);
        }
    }

    #[test]
    fn test_userlib_ioe() {
        run_my_test_once("tracing.bin");
        run_my_test_once("time.bin");
        run_my_test_once("colorful.bin");
        run_my_test_once("dummy.bin");
        run_my_test_once("backtrace.bin");
    }

    // #[test]
    // fn test_am_kernel_demos() {
    //     run_test_once(
    //         "/workspaces/course/ics-pa/am-kernels/kernels/demo/build/demo-riscv64-nemu.elf",
    //     );
    // }

    // #[test]
    // fn run_benches() {
    //     // run_test_once("/workspaces/course/ics-pa/am-kernels/benchmarks/microbench/build/microbench-riscv64-nemu.bin");
    //     run_test_once(
    //         "/workspaces/course/ics-pa/am-kernels/benchmarks/coremark/build/coremark-riscv64-nemu.bin",
    //     );
    // }

    use difftest::*;

    #[repr(C)]
    #[derive(Debug, Default, Clone, Copy)]
    struct MockRegisters {
        general: [u64; 32],
        pc: u64,
    }

    impl MockRegisters {
        pub fn new(pc: u64) -> Self {
            let mut cpu = MockRegisters::default();
            cpu.pc = pc;
            cpu
        }

        pub fn as_mut_ptr(&mut self) -> *mut u8 {
            self as *mut MockRegisters as *mut u8
        }
    }

    const EBREAK: [u8; 4] = 0b000000000001_00000_000_00000_1110011u32.to_le_bytes();

    #[test]
    fn difftest() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let lib = "/workspaces/course/ics-pa/nemu/tools/spike-diff/build/-spike-so";
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

        let add_test = CPU_TESTS_DIR.to_string() + "hello-str-riscv64-nemu.bin";

        let mut file = std::fs::File::open(&add_test).unwrap();
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();
        memcpy_fn(0x80000000, content.as_ptr() as *mut u8, content.len(), true);

        let mut reset_cpu = MockRegisters::new(0x80000000);
        regcpy_fn(reset_cpu.as_mut_ptr(), true);

        for _ in 0..390 {
            let mut cpu = MockRegisters::default();
            regcpy_fn(cpu.as_mut_ptr(), false);
            let pc_offset = (cpu.pc - 0x80000000) as usize;
            disasm::disasm(&content[pc_offset..pc_offset + 4], Some(cpu.pc))?;
            if &content[pc_offset..pc_offset + 4] == &EBREAK {
                // end of program
                println!("result: {}", cpu.general[10]); // a0 is the return value
                break;
            }
            exec_fn(1);
        }

        Ok(())
    }

    #[test]
    fn test_difftests() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // init_log(tracing::Level::DEBUG);
        // hello-str is not supported, since it's related with device
        // and qemu and us have different device logic
        let testcases: &[&str] = &[
            "add-riscv64-nemu.bin",
            "add-longlong-riscv64-nemu.bin",
            "bubble-sort-riscv64-nemu.bin",
            "crc32-riscv64-nemu.bin",
            "div-riscv64-nemu.bin",
            "dummy-riscv64-nemu.bin",
            "fact-riscv64-nemu.bin",
            "if-else-riscv64-nemu.bin",
            "leap-year-riscv64-nemu.bin",
            "load-store-riscv64-nemu.bin",
            "matrix-mul-riscv64-nemu.bin",
            "max-riscv64-nemu.bin",
            "mersenne-riscv64-nemu.bin",
            "min3-riscv64-nemu.bin",
            "mov-c-riscv64-nemu.bin",
            "movsx-riscv64-nemu.bin",
            "mul-longlong-riscv64-nemu.bin",
            "pascal-riscv64-nemu.bin",
            "prime-riscv64-nemu.bin",
            "quick-sort-riscv64-nemu.bin",
            "recursion-riscv64-nemu.bin",
            "select-sort-riscv64-nemu.bin",
            "shift-riscv64-nemu.bin",
            "sum-riscv64-nemu.bin",
            "switch-riscv64-nemu.bin",
            "to-lower-case-riscv64-nemu.bin",
            "unalign-riscv64-nemu.bin",
            "wanshu-riscv64-nemu.bin",
        ];
        for i in testcases {
            let testcase = format!("{}{}", CPU_TESTS_DIR.as_str(), i);
            difftest_run(&testcase)?;
        }
        Ok(())
    }

    #[test]
    fn test_difftests_alloc() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let testcases: &[&str] = &["alloc.bin"];
        for i in testcases {
            let testcase = format!("{}{}", TARGET_DIR.as_str(), i);
            difftest_run(&dbg!(testcase))?;
        }
        Ok(())
    }

    fn difftest_run(testcase: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open(testcase).unwrap();
        let mut image = Vec::new();
        file.read_to_end(&mut image).unwrap();

        let mut addr_space =
            AddressSpace::new(PhyMem::new(PAddr(MBASE), MSIZE as usize)).with_default_mmio();
        let mut clock = VirtualClock::new();
        let mut cpu = RISCV64::new(mmu::MMU::new(mmu::Mode::Bare));
        cpu.mmu_mut()
            .load_program(&mut addr_space, VAddr(MBASE), &image)
            .unwrap();

        let mut qemu_diff = difftest::qemu::DiffTest::new(1234).unwrap();
        let mut init_regs = cpu.regs().all().to_vec();
        qemu_diff
            .difftest_write_general_regs(&mut init_regs)
            .unwrap();
        qemu_diff.difftest_memcpy_to(0x80_000_000, &image).unwrap();

        // now the state(CPU + MEM) is the same

        loop {
            let mut qemu_regs = vec![0u64; 32 + 1 + 32]; // 32 registers for RISC-V
            qemu_diff
                .difftest_read_general_regs(&mut qemu_regs)
                .unwrap();
            debug_assert_eq!(qemu_regs[32], cpu.regs().pc);
            debug_assert_eq_regs(&qemu_regs, cpu.regs().all());
            let pc_offset = (qemu_regs[32] - 0x80000000) as usize;
            disasm::disasm(&image[pc_offset..pc_offset + 4], Some(qemu_regs[32]))?;
            if &image[pc_offset..pc_offset + 4] == &EBREAK {
                // end of program
                println!("registers: {:x?}", qemu_regs);
                println!("result: {}", qemu_regs[10]); // a0 is the return value
                break;
            }
            cpu.exec(&mut clock, &mut addr_space, 1);
            qemu_diff.difftest_exec(1).unwrap();
        }

        Ok(())
    }

    fn debug_assert_eq_regs(a: &[u64], b: &[u64]) {
        assert_eq!(a.len(), b.len(), "Registers length mismatch");
        for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
            let register_name = if i < 32 {
                let reg = GRegName::from(i as u32);
                let name: &str = reg.into();
                name.to_string()
            } else if i == 32 {
                "pc".to_string()
            } else {
                format!("float{}", i - 32)
            };
            assert_eq!(
                *x, *y,
                "Register {} mismatch: {} != {}",
                register_name, x, y
            );
        }
    }
}
