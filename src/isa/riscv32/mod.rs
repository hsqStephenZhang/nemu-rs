pub const NUM_REGISTERS: usize = 32;

#[repr(C)]
pub struct CpuState {
    regs: [i64; NUM_REGISTERS],
    pc: usize,
}

pub struct I(i32);
pub struct S(i32);
pub struct U(i32);

pub enum Instruction {
    I(I),
    S(S),
    U(U),
}

const MASK2: i32 = 2 ^ 2 - 1;
const MASK3: i32 = 2 ^ 3 - 1;
const MASK5: i32 = 2 ^ 5 - 1;
const MASK7: i32 = 2 ^ 7 - 1;
const MASK12: i32 = 2 ^ 12 - 1;
const MASK20: i32 = 2 ^ 20 - 1;

/*
uint32_t opcode1_0 : 2;
uint32_t opcode6_2 : 5;
uint32_t rd        : 5;
uint32_t funct3    : 3;
uint32_t rs1       : 5;
int32_t  simm11_0  :12;
*/
impl I {
    // 11 00000 00000 000 00000 000000000000
    pub fn op1(&self) -> i32 {
        self.0 >> 30 & MASK3
    }

    // 00 11111 00000 000 00000 000000000000
    pub fn op2(&self) -> i32 {
        self.0 >> 25 & MASK5
    }

    // 00 00000 11111 000 00000 000000000000
    pub fn rd(&self) -> i32 {
        self.0 >> 20 & MASK5
    }

    // 00 00000 00000 111 00000 000000000000
    pub fn funct(&self) -> i32 {
        self.0 >> 17 & MASK3
    }

    // 00 00000 00000 000 11111 000000000000
    pub fn rs1(&self) -> i32 {
        self.0 >> 12 & MASK5
    }

    // 00 00000 00000 000 00000 111111111111
    pub fn simm(&self) -> i32 {
        self.0 & MASK12
    }
}

impl S {
    // 11 00000 00000 000 00000 00000 0000000
    pub fn op1(&self) -> i32 {
        self.0 >> 30 & MASK2
    }

    // 00 11111 00000 000 00000 000000000000
    pub fn op2(&self) -> i32 {
        self.0 >> 25 & MASK5
    }

    // 00 00000 11111 000 00000 00000 0000000
    pub fn rd(&self) -> i32 {
        self.0 >> 20 & MASK5
    }

    // 00 00000 00000 111 00000 00000 0000000
    pub fn funct(&self) -> i32 {
        self.0 >> 17 & MASK3
    }

    // 00 00000 00000 000 11111 00000 0000000
    pub fn rs1(&self) -> i32 {
        self.0 >> 12 & MASK5
    }

    // 00 00000 00000 000 00000 11111 0000000
    pub fn rs2(&self) -> i32 {
        self.0 >> 7 & MASK5
    }

    // 00 11111 00000 000 00000 00000 0000000
    pub fn simm(&self) -> i32 {
        self.0 & MASK7
    }
}


impl U {
    // 11 00000 00000 00000000000000000000
    pub fn op1(&self) -> i32 {
        self.0 >> 30 & MASK2
    }

    // 00 11111 00000 00000000000000000000
    pub fn op2(&self) -> i32 {
        self.0 >> 25 & MASK5
    }

    // 00 00000 11111 00000000000000000000
    pub fn rd(&self) -> i32 {
        self.0 >> 20 & MASK5
    }

    // 00 00000 00000 1111111111111111111
    pub fn simm(&self) -> i32 {
        self.0 & MASK20
    }
}