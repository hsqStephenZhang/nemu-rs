use crate::addr_space::{AddressSpace, VAddr};

pub mod riscv64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NemuState {
    Running = 1,
    Stopped = 2,
    End = 3,
    Abort = 4,
    Quit = 5,
}

pub trait Cpu {
    type Instruction;
    type Context;

    fn ifetch(&mut self, addr_space: &mut AddressSpace, pc: VAddr) -> Option<Self::Instruction>;

    // execute one instruction
    fn exec_once(&mut self, ctx: &mut Self::Context, addr_space: &mut AddressSpace, pc: VAddr);

    // execute multiple instructions
    fn exec(&mut self, ctx: &mut Self::Context, addr_space: &mut AddressSpace, n: usize) -> usize {
        let mut i = 0;
        for _ in 0..n {
            self.exec_once(ctx, addr_space, self.pc());
            i += 1;
            if self.state() != NemuState::Running {
                break;
            }
        }
        i
    }

    fn pc(&self) -> VAddr;

    fn state(&self) -> NemuState;

    fn logo(&self) -> &[u8];

    // get reg value by it's name/dialect
    fn get_reg_by_name(&self, reg: &str) -> Option<u64>;

    fn set_reg_by_name(&mut self, reg: &str, value: u64) -> Option<()>;

    fn raise_interrupt(&mut self, interrupt: u64);

    fn debug_load_img(
        &mut self,
        addr_space: &mut AddressSpace,
        vaddr: VAddr,
        data: &[u8],
    )-> Result<(), String>;
}
