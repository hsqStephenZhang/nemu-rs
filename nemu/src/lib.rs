pub mod cpu;
pub mod device;
pub mod engine;
pub mod memory;
pub mod monitor;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use clap::{Parser, ValueEnum};
use tracing::{Level, info};

use crate::cpu::Cpu;
use crate::memory::config::{MBASE, RESET_VECTOR};

#[derive(Parser, Debug, Clone, ValueEnum)]
pub enum Difftester {
    Spike,
    Qemu,
    None,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// IMAGE
    #[arg(long)]
    pub(crate) image: Option<String>,

    /// run with difftest
    #[arg(short, long)]
    pub difftest: Difftester,

    /// run with batch mode
    #[arg(short, long)]
    pub batch: bool,

    /// output log to FILE
    #[arg(short, long, value_name = "FILE")]
    pub(crate) log: Option<String>,

    #[arg(long)]
    pub log_level: Level,
}

pub fn init_log(level: Level) {
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_file(true)
        .with_line_number(true)
        .without_time()
        .init();
}

// load image to RESET_VECTOR
pub(crate) fn load_img(img_file: &String, phy_mem: &mut [u8]) -> u64 {
    let path = Path::new(img_file);
    let f = File::open(path).unwrap();
    let size = f.metadata().unwrap().len();

    info!("loading image {}, size: {} bytes", img_file, size);

    let offset = RESET_VECTOR - MBASE;

    if phy_mem.len() < (offset + size) as usize {
        panic!(
            "Image size {} exceeds memory size {} at offset {}",
            size,
            phy_mem.len(),
            offset
        );
    }
    phy_mem[offset as usize..(offset + size) as usize]
        .copy_from_slice(&f.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>());

    size
}

pub struct Simulator<C: Cpu> {
    cpu: C,
    batch: bool,
}

impl<C: Cpu> Simulator<C> {
    pub fn run(mut self) {}
}
