pub const NUM_REGISTERS: usize = 32;

#[repr(C)]
pub struct CpuState {
    regs: [i64; NUM_REGISTERS],
    pc: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct RType(u32);
#[derive(Clone, Copy, Debug)]
pub struct IType(u32);
#[derive(Clone, Copy, Debug)]
pub struct SType(u32);
#[derive(Clone, Copy, Debug)]
pub struct BType(u32);
#[derive(Clone, Copy, Debug)]
pub struct UType(u32);
#[derive(Clone, Copy, Debug)]
pub struct JType(u32);

pub enum Instruction {
    R(RType),
    I(IType),
    S(SType),
    B(BType),
    U(UType),
    J(JType),
}

const MASK1: u32 = 0b1;
const MASK2: u32 = 0b11;
const MASK3: u32 = 0b111;
const MASK5: u32 = 0b11111;
const MASK6: u32 = 0b111111;
const MASK7: u32 = 0b1111111;
const MASK8: u32 = 0b11111111;
const MASK10: u32 = 0b1111111111;
const MASK12: u32 = 0b111111111111;
const MASK20: u32 = 0b11111111111111111111;

/*
int32_t  funct7    : 7;
uint32_t rs2       : 5;
uint32_t rs1       : 5;
uint32_t funct3    : 3;
uint32_t rd        : 5;
uint32_t imm       : 5;
uint32_t opcode1_0 : 2;
*/
impl RType {
    pub fn op1(&self) -> u32 {
        self.0 & MASK2
    }

    pub fn imm(&self) -> u32 {
        self.0 >> 2 & MASK5
    }

    pub fn rd(&self) -> u32 {
        self.0 >> 7 & MASK5
    }

    pub fn funct(&self) -> u32 {
        self.0 >> 12 & MASK3
    }

    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & MASK5
    }

    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & MASK5
    }

    pub fn simm(&self) -> u32 {
        self.0 >> 25 & MASK7
    }
}

/*
int32_t  simm11_0  : 12;
uint32_t rs1       : 5;
uint32_t funct3    : 3;
uint32_t rd        : 5;
uint32_t op2       : 5;
uint32_t opcode1_0 : 2;
*/
impl IType {
    pub fn op1(&self) -> u32 {
        self.0 & MASK3
    }

    pub fn op2(&self) -> u32 {
        self.0 >> 2 & MASK5
    }

    pub fn rd(&self) -> u32 {
        self.0 >> 7 & MASK5
    }

    pub fn funct(&self) -> u32 {
        self.0 >> 12 & MASK3
    }

    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & MASK5
    }

    pub fn imm(&self) -> u32 {
        self.0 >> 20 & MASK12
    }
}

/*
int32_t  imm2      : 7;
uint32_t rs2       : 5;
uint32_t rs1       : 5;
uint32_t funct3    : 3;
uint32_t imm1      : 5;
uint32_t op2       : 5;
uint32_t opcode1_0 : 2;
*/
impl SType {
    pub fn op1(&self) -> u32 {
        self.0 & MASK2
    }

    pub fn op2(&self) -> u32 {
        self.0 >> 2 & MASK5
    }

    pub fn imm1(&self) -> u32 {
        self.0 >> 7 & MASK5
    }

    pub fn funct(&self) -> u32 {
        self.0 >> 12 & MASK3
    }

    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & MASK5
    }

    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & MASK5
    }

    pub fn imm2(&self) -> u32 {
        self.0 >> 25 & MASK7
    }

    pub fn imm(&self) -> u32 {
        ((self.0 >> 20) & 0xfe0) | self.imm1()
    }
}

/*
int32_t  simm7_0   : 7;
uint32_t rs2       : 5;
uint32_t rs1       : 5;
uint32_t funct3    : 3;
uint32_t rd        : 5;
uint32_t imm       : 5;
uint32_t opcode1_0 : 2;
*/
impl BType {
    pub fn op1(&self) -> u32 {
        self.0 & MASK2
    }

    pub fn op2(&self) -> u32 {
        self.0 >> 2 & MASK5
    }

    pub fn imm1(&self) -> u32 {
        self.0 >> 7 & MASK1
    }

    pub fn imm2(&self) -> u32 {
        self.0 >> 8 & MASK3
    }

    pub fn funct(&self) -> u32 {
        self.0 >> 12 & MASK3
    }

    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & MASK5
    }

    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & MASK5
    }

    pub fn imm3(&self) -> u32 {
        self.0 >> 25 & MASK6
    }

    pub fn imm4(&self) -> u32 {
        self.0 >> 31 & MASK1
    }

    pub fn imm(&self) -> u32 {
        ((self.0 & 0x8000_0000) >> 19)
            | ((self.0 & 0x7e00_0000) >> 20)
            | ((self.0 & 0x0000_0f00) >> 7)
            | ((self.0 & 0x0000_0080) << 4)
    }
}

/*
int32_t  simm20_0  : 20;
uint32_t rd        : 5;
uint32_t imm       : 5;
uint32_t opcode1_0 : 2;
*/
impl UType {
    pub fn op1(&self) -> u32 {
        self.0 & MASK2
    }

    pub fn op2(&self) -> u32 {
        self.0 >> 2 & MASK5
    }

    pub fn rd(&self) -> u32 {
        self.0 >> 7 & MASK5
    }

    pub fn simm(&self) -> u32 {
        self.0 >> 12 & MASK20
    }
}

/*
int32_t  imm4      : 1;
int32_t  imm3      : 10;
int32_t  imm2      : 1;
int32_t  imm1      : 8;
uint32_t rd        : 5;
uint32_t imm       : 5;
uint32_t opcode1_0 : 2;
*/
impl JType {
    pub fn op1(&self) -> u32 {
        self.0 & MASK2
    }

    pub fn op2(&self) -> u32 {
        self.0 >> 2 & MASK5
    }

    pub fn rd(&self) -> u32 {
        self.0 >> 7 & MASK5
    }

    pub fn imm1(&self) -> u32 {
        self.0 >> 12 & MASK8
    }

    pub fn imm2(&self) -> u32 {
        self.0 >> 20 & MASK1
    }

    pub fn imm3(&self) -> u32 {
        self.0 >> 21 & MASK10
    }

    pub fn imm4(&self) -> u32 {
        self.0 >> 31 & MASK1
    }

    pub fn imm(&self) -> u32 {
        ((self.0 & 0x8000_0000) >> 11)
            | ((self.0 & 0x7fe0_0000) >> 20)
            | ((self.0 & 0x0010_0000) >> 9)
            | (self.0 & 0x000f_f000)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rtype() {
        // let a = RType(0x00c58633);
        // dbg!(a.rs1(), a.rs2());
        // dbg!((0x00c58633 >> 15) & 0x1f);
        // dbg!(0x1f, (1 << 5) - 1);
        assert_eq!(RType(0x00c58633).rs1(), 11); // add x12,x11,x12
        assert_eq!(RType(0x40b50533).rs1(), 10); // sub x10,x10,x11
        assert_eq!(RType(0x00209f33).rs1(), 1); // sll x30,x1,x2
        assert_eq!(RType(0x0020af33).rs1(), 1); // slt x30,x1,x2
        assert_eq!(RType(0x0020bf33).rs1(), 1); // sltu x30,x1,x2
        assert_eq!(RType(0x00f647b3).rs1(), 12); // xor x15,x12,x15
        assert_eq!(RType(0x0020d0b3).rs1(), 1); // srl x1,x1,x2
        assert_eq!(RType(0x4020df33).rs1(), 1); // sra x30,x1,x2
        assert_eq!(RType(0x00b7e5b3).rs1(), 15); // or x11,x15,x11
        assert_eq!(RType(0x00d57533).rs1(), 10); // and x10,x10,x13

        assert_eq!(RType(0x00c58633).rs2(), 12); // add x12,x11,x12
        assert_eq!(RType(0x40b50533).rs2(), 11); // sub x10,x10,x11
        assert_eq!(RType(0x00209f33).rs2(), 2); // sll x30,x1,x2
        assert_eq!(RType(0x0020af33).rs2(), 2); // slt x30,x1,x2
        assert_eq!(RType(0x0020bf33).rs2(), 2); // sltu x30,x1,x2
        assert_eq!(RType(0x00f647b3).rs2(), 15); // xor x15,x12,x15
        assert_eq!(RType(0x0020d0b3).rs2(), 2); // srl x1,x1,x2
        assert_eq!(RType(0x4020df33).rs2(), 2); // sra x30,x1,x2
        assert_eq!(RType(0x00b7e5b3).rs2(), 11); // or x11,x15,x11
        assert_eq!(RType(0x00d57533).rs2(), 13); // and x10,x10,x13

        assert_eq!(RType(0x00c58633).rd(), 12); // add x12,x11,x12
        assert_eq!(RType(0x40b50533).rd(), 10); // sub x10,x10,x11
        assert_eq!(RType(0x00209f33).rd(), 30); // sll x30,x1,x2
        assert_eq!(RType(0x0020af33).rd(), 30); // slt x30,x1,x2
        assert_eq!(RType(0x0020bf33).rd(), 30); // sltu x30,x1,x2
        assert_eq!(RType(0x00f647b3).rd(), 15); // xor x15,x12,x15
        assert_eq!(RType(0x0020d0b3).rd(), 1); // srl x1,x1,x2
        assert_eq!(RType(0x4020df33).rd(), 30); // sra x30,x1,x2
        assert_eq!(RType(0x00b7e5b3).rd(), 11); // or x11,x15,x11
        assert_eq!(RType(0x00d57533).rd(), 10); // and x10,x10,x13
    }

    #[test]
    fn test_itype() {
        assert_eq!(IType(0x02008283).rd(), 5); // lb x5,32(x1)
        assert_eq!(IType(0x00708283).rd(), 5); // lb x5,7(x1)
        assert_eq!(IType(0x00108f03).rd(), 30); // lb x30,1(x1)
        assert_eq!(IType(0x00411f03).rd(), 30); // Lh x30,4(x2)
        assert_eq!(IType(0x00611f03).rd(), 30); // Lh x30,6(x2)
        assert_eq!(IType(0x00811f03).rd(), 30); // Lh x30,8(x2)
        assert_eq!(IType(0x02052403).rd(), 8); // Lw x8,32(x10)
        assert_eq!(IType(0x03452683).rd(), 13); // Lw x13,52(x10)
        assert_eq!(IType(0x0006a703).rd(), 14); // Lw x14,0(x13)
        assert_eq!(IType(0x0006c783).rd(), 15); // Lbu x15,0(x13)
        assert_eq!(IType(0x0006c703).rd(), 14); // Lbu x14,0(x13)
        assert_eq!(IType(0x0007c683).rd(), 13); // Lbu x13,0(x15)
        assert_eq!(IType(0x0060df03).rd(), 30); // Lhu x30,6(x1)
        assert_eq!(IType(0xffe0df03).rd(), 30); // Lhu x30,-2(x1)
        assert_eq!(IType(0x0002d303).rd(), 6); // Lhu x6,0(x5)
        assert_eq!(IType(0x00346303).rd(), 6); // Lwu x6,3(x8)
        assert_eq!(IType(0x0080ef03).rd(), 30); // Lwu x30,8(x1)
        assert_eq!(IType(0x0000ef03).rd(), 30); // Lwu x30,0(x1)
        assert_eq!(IType(0x01853683).rd(), 13); // Ld x13,24(x10)
        assert_eq!(IType(0x02013c03).rd(), 24); // Ld x24,32(x2)
        assert_eq!(IType(0x0007b703).rd(), 14); // Ld x14,0(x15)

        assert_eq!(IType(0x02008283).rs1(), 1); // lb x5,32(x1)
        assert_eq!(IType(0x00708283).rs1(), 1); // lb x5,7(x1)
        assert_eq!(IType(0x00108f03).rs1(), 1); // lb x30,1(x1)
        assert_eq!(IType(0x00411f03).rs1(), 2); // Lh x30,4(x2)
        assert_eq!(IType(0x00611f03).rs1(), 2); // Lh x30,6(x2)
        assert_eq!(IType(0x00811f03).rs1(), 2); // Lh x30,8(x2)
        assert_eq!(IType(0x02052403).rs1(), 10); // Lw x8,32(x10)
        assert_eq!(IType(0x03452683).rs1(), 10); // Lw x13,52(x10)
        assert_eq!(IType(0x0006a703).rs1(), 13); // Lw x14,0(x13)
        assert_eq!(IType(0x0006c783).rs1(), 13); // Lbu x15,0(x13)
        assert_eq!(IType(0x0006c703).rs1(), 13); // Lbu x14,0(x13)
        assert_eq!(IType(0x0007c683).rs1(), 15); // Lbu x13,0(x15)
        assert_eq!(IType(0x0060df03).rs1(), 1); // Lhu x30,6(x1)
        assert_eq!(IType(0xffe0df03).rs1(), 1); // Lhu x30,-2(x1)
        assert_eq!(IType(0x0002d303).rs1(), 5); // Lhu x6,0(x5)
        assert_eq!(IType(0x00346303).rs1(), 8); // Lwu x6,3(x8)
        assert_eq!(IType(0x0080ef03).rs1(), 1); // Lwu x30,8(x1)
        assert_eq!(IType(0x0000ef03).rs1(), 1); // Lwu x30,0(x1)
        assert_eq!(IType(0x01853683).rs1(), 10); // Ld x13,24(x10)
        assert_eq!(IType(0x02013c03).rs1(), 2); // Ld x24,32(x2)
        assert_eq!(IType(0x0007b703).rs1(), 15); // Ld x14,0(x15)

        assert_eq!(IType(0x02008283).imm(), 32); // lb x5,32(x1)
        assert_eq!(IType(0x00708283).imm(), 7); // lb x5,7(x1)
        assert_eq!(IType(0x00108f03).imm(), 1); // lb x30,1(x1)
        assert_eq!(IType(0x00411f03).imm(), 4); // Lh x30,4(x2)
        assert_eq!(IType(0x00611f03).imm(), 6); // Lh x30,6(x2)
        assert_eq!(IType(0x00811f03).imm(), 8); // Lh x30,8(x2)
        assert_eq!(IType(0x02052403).imm(), 32); // Lw x8,32(x10)
        assert_eq!(IType(0x03452683).imm(), 52); // Lw x13,52(x10)
        assert_eq!(IType(0x0006a703).imm(), 0); // Lw x14,0(x13)
        assert_eq!(IType(0x0006c783).imm(), 0); // Lbu x15,0(x13)
        assert_eq!(IType(0x0006c703).imm(), 0); // Lbu x14,0(x13)
        assert_eq!(IType(0x0007c683).imm(), 0); // Lbu x13,0(x15)
        assert_eq!(IType(0x0060df03).imm(), 6); // Lhu x30,6(x1)
        assert_eq!(IType(0xffe0df03).imm(), (-2i32) as u32 & 0xfff); // Lhu x30,-2(x1)
        assert_eq!(IType(0x0002d303).imm(), 0); // Lhu x6,0(x5)
        assert_eq!(IType(0x00346303).imm(), 3); // Lwu x6,3(x8)
        assert_eq!(IType(0x0080ef03).imm(), 8); // Lwu x30,8(x1)
        assert_eq!(IType(0x0000ef03).imm(), 0); // Lwu x30,0(x1)
        assert_eq!(IType(0x01853683).imm(), 24); // Ld x13,24(x10)
        assert_eq!(IType(0x02013c03).imm(), 32); // Ld x24,32(x2)
        assert_eq!(IType(0x0007b703).imm(), 0); // Ld x14,0(x15)
    }

    #[test]
    #[allow(overflowing_literals)]
    fn btype() {
        assert_eq!(BType(0x0420c063).imm(), 0x80002ea4 - 0x80002e64); // blt x1,x2,80002ea4
        assert_eq!(BType(0x06f58063).imm(), 0x80002724 - 0x800026c4); // beq x11,x15,80002724
        assert_eq!(BType(0x06f58063).imm(), 0x80002648 - 0x800025e8); // beq x11,x15,80002648
        assert_eq!(BType(0x00050a63).imm(), 0x800024e8 - 0x800024d4); // beq x10,x0,800024e8
        assert_eq!(BType(0x03ff0663).imm(), 0x80000040 - 0x80000014); // beq x30,x31,80000040
        assert_eq!(
            BType(0xfe069ae3).imm(),
            (0x800026f0i32 - 0x800026fci32) as u32 & 0x1fff
        ); // bne x13,x0,800026f0
        assert_eq!(BType(0x00f5f463).imm(), 0x80002290 - 0x80002288); // bgeu x11,x15,80002290
        assert_eq!(BType(0x1e301c63).imm(), 0x800003c4 - 0x800001cc); // bne x0,x3,800003c4
        assert_eq!(BType(0x13df1063).imm(), 0x800030dc - 0x80002fbc); // bne x30,x29,800030dc
        assert_eq!(BType(0x37df1263).imm(), 0x80002f90 - 0x80002c2c); // bne x30,x29,80002f90
    }

    #[test]
    fn utype() {
        assert_eq!(UType(0x00001a37).rd(), 20); // lui x20,0x1
        assert_eq!(UType(0x800002b7).rd(), 5); // lui x5,0x80000
        assert_eq!(UType(0x212120b7).rd(), 1); // lui x1,0x21212
        assert_eq!(UType(0xffffe517).rd(), 10); // auipc x10,0xffffe
        assert_eq!(UType(0xfffff797).rd(), 15); // auipc x15,0xfffff
        assert_eq!(UType(0xfffff797).rd(), 15); // auipc x15,0xfffff

        assert_eq!(UType(0x00001a37).rd(), 20); // lui x20,0x1
        assert_eq!(UType(0x800002b7).rd(), 5); // lui x5,0x80000
        assert_eq!(UType(0x212120b7).rd(), 1); // lui x1,0x21212
        assert_eq!(UType(0xffffe517).rd(), 10); // auipc x10,0xffffe
        assert_eq!(UType(0xfffff797).rd(), 15); // auipc x15,0xfffff
        assert_eq!(UType(0xfffff797).rd(), 15); // auipc x15,0xfffff
    }

    #[test]
    #[allow(overflowing_literals)]
    fn jtype() {
        assert_eq!(
            JType(0xfe1ff06f).imm(),
            (0x800029eci32 - 0x80002a0ci32) as u32 & 0x1fffff
        ); // jal x0,800029ec
        assert_eq!(JType(0x0000006f).imm(), 0x80002258 - 0x80002258); // jal x0,80002258
        assert_eq!(
            JType(0xf89ff06f).imm(),
            (0x800027aci32 - 0x80002824i32) as u32 & 0x1fffff
        ); // jal x0,800027ac
        assert_eq!(JType(0x0240006f).imm(), 0x8000215c - 0x80002138); // jal x0,8000215c
        assert_eq!(
            JType(0xd89ff0ef).imm(),
            (0x80002230i32 - 0x800024a8i32) as u32 & 0x1fffff
        ); // jal x1,80002230
        assert_eq!(JType(0x008007ef).imm(), 0x8000265c - 0x80002654); // jal x15,8000265c
        assert_eq!(JType(0x0240006f).imm(), 0x80002154 - 0x80002130); // jal x0,80002154
        assert_eq!(
            JType(0xf71ff06f).imm(),
            (0x80002750i32 - 0x800027e0i32) as u32 & 0x1fffff
        ); // jal x0,80002750
        assert_eq!(JType(0x00c0006f).imm(), 0x8000000c - 0x80000000); // jal x0,8000000c

        assert_eq!(JType(0xfe1ff06f).rd(), 0); // jal x0,800029ec
        assert_eq!(JType(0x0000006f).rd(), 0); // jal x0,80002258
        assert_eq!(JType(0xf89ff06f).rd(), 0); // jal x0,800027ac
        assert_eq!(JType(0x0240006f).rd(), 0); // jal x0,8000215c
        assert_eq!(JType(0xd89ff0ef).rd(), 1); // jal x1,80002230
        assert_eq!(JType(0x008007ef).rd(), 15); // jal x15,8000265c
        assert_eq!(JType(0x0240006f).rd(), 0); // jal x0,80002154
        assert_eq!(JType(0xf71ff06f).rd(), 0); // jal x0,80002750
        assert_eq!(JType(0x00c0006f).rd(), 0); // jal x0,8000000c
    }
}
