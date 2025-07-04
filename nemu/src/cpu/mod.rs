use crate::memory::addr::VAddr;

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

    fn ifetch(&mut self, pc: VAddr) -> Option<Self::Instruction>;

    // execute one instruction
    fn exec_once(&mut self, pc: VAddr);

    // execute multiple instructions
    fn exec(&mut self, n: usize) -> usize {
        let mut i = 0;
        for _ in 0..n {
            self.exec_once(self.pc());
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
}
