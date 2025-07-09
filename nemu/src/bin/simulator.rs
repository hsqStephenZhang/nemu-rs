use std::io::Read;

use clap::Parser;
use nemu_rs::{
    addr_space::{AddressSpace, PAddr}, config::{MBASE, MSIZE}, cpu::riscv64::{mmu, RISCV64}, device, init_log, memory::PhyMem, simulator, timer::virtual_clock::VirtualClock
};

const DEFAULT_IMG: &[u32] = &[
    0x00000297, // auipc t0,0
    0x00028823, // sb  zero,16(t0)
    0x0102c503, // lbu a0,16(t0)
    0x00100073, // ebreak (used as nemu_trap)
    0xdeadbeef, // some data
];

#[derive(clap::Parser, Debug, Clone)]
pub struct Opt {
    #[arg(short, long)]
    pub image: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_log(tracing::Level::INFO);
    // let opt = Opt::parse();
    let opt = Opt{
        image: Some("/Users/zc/codespace/rust/nemu-rs/target/riscv64gc-unknown-none-elf/release/io_test_keyboard.bin".to_string())
    };
    let addr_space =
        AddressSpace::new(PhyMem::new(PAddr(MBASE), MSIZE as usize)).with_default_mmio();
    let cpu = RISCV64::new(mmu::MMU::new(mmu::Mode::Bare));
    let mut simulator = simulator::Simulator::new(cpu, addr_space);

    let image = opt
        .image
        .map(|path| read_file(&path))
        .unwrap_or_else(|| {
            eprintln!("No image provided, using default image.");
            let bytes = DEFAULT_IMG
                .iter()
                .flat_map(|&x| x.to_le_bytes())
                .collect::<Vec<u8>>();
            Ok(bytes)
        })
        .unwrap_or_else(|err| {
            eprintln!("Error reading image: {}", err);
            std::process::exit(1);
        });

    simulator.load_img(&image)?;
    let mut clock = VirtualClock::new();
    device::init(&mut clock);
    let _ = simulator.run(&mut clock, usize::MAX);
    let res = simulator.cpu().halt_ret();
    println!("Simulation finished with result: {:#x}", res);

    Ok(())
}

fn read_file(path: &str) -> Result<Vec<u8>, String> {
    let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    Ok(buffer)
}
