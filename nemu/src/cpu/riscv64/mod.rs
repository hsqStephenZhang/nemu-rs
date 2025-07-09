use std::{ops::BitOr, str::FromStr};

use riscv_decode::{
    Instruction,
    types::{BType, IType, JType, RType, SType, UType},
};
use tracing::*;

use crate::{
    addr_space::{AddressSpace, VAddr},
    cpu::{
        Cpu, NemuState,
        riscv64::{
            logo::LOGO,
            mmu::MMU,
            regs::{GRegName, GeneralRegs},
            reserved::Reserve,
        },
    },
    timer::virtual_clock::VirtualClock,
};

pub mod csr;
pub mod insn;
mod logo;
pub mod mmu;
pub mod regs;
pub mod reserved;

// TODO: CSR and system related state
pub struct RISCV64 {
    state: NemuState,
    reserve: Reserve,
    halt_pc: u64,
    halt_ret: u64,
    // general registers
    regs: GeneralRegs,
    // memory management unit
    mmu: MMU,
}

impl RISCV64 {
    pub fn new(mmu: MMU) -> Self {
        RISCV64 {
            state: NemuState::Running,
            reserve: Reserve::default(),
            halt_pc: 0,
            halt_ret: 0,
            regs: GeneralRegs::new(),
            mmu,
        }
    }

    pub fn halt_pc(&self) -> u64 {
        self.halt_pc
    }

    pub fn halt_ret(&self) -> u64 {
        self.halt_ret
    }

    pub fn mmu_mut(&mut self) -> &mut MMU {
        &mut self.mmu
    }

    pub fn regs(&self) -> &GeneralRegs {
        &self.regs
    }

    pub fn vmem_enabled(&self) -> bool {
        // RISCV64 always has virtual memory enabled
        true
    }
}

impl Cpu for RISCV64 {
    type Instruction = u32;
    type Context = VirtualClock;

    fn ifetch(&mut self, addr_space: &mut AddressSpace, pc: VAddr) -> Option<Self::Instruction> {
        let res = self
            .mmu
            .read(addr_space, pc, crate::addr_space::Size::Word)?;
        Some(res as u32)
    }

    fn exec_once(&mut self, ctx: &mut Self::Context, addr_space: &mut AddressSpace, pc: VAddr) {
        let printer = |insn: &disasm::Insn<'_>| {
            debug!(
                "{:08x}: {:<8} {}",
                insn.address(),
                insn.mnemonic().unwrap_or(""),
                insn.op_str().unwrap_or("")
            );
        };
        let instruction = self.ifetch(addr_space, pc).unwrap();
        disasm::disasm_with(&instruction.to_le_bytes(), Some(pc.0), &printer).unwrap();
        let insn = match riscv_decode::decode(instruction) {
            Ok(insn) => insn,
            Err(e) => {
                panic!("Failed to decode instruction at {:#x}: {:?}", pc.0, e);
            }
        };
        let next_pc = self.exec_and_get_next_pc(addr_space, pc.0, insn);
        ctx.advance(1);
        self.regs.pc = next_pc;
    }

    fn pc(&self) -> VAddr {
        VAddr(self.regs.pc)
    }

    fn state(&self) -> super::NemuState {
        self.state
    }

    fn logo(&self) -> &[u8] {
        LOGO
    }

    fn get_reg_by_name(&self, reg: &str) -> Option<u64> {
        match GRegName::from_str(reg) {
            Ok(greg) => Some(self.regs.get(greg)),
            Err(_) => None,
        }
    }

    fn set_reg_by_name(&mut self, reg: &str, value: u64) -> Option<()> {
        match GRegName::from_str(reg) {
            Ok(greg) => {
                self.regs.set(greg, value);
                Some(())
            }
            Err(_) => None,
        }
    }

    fn raise_interrupt(&mut self, _interrupt: u64) {
        todo!()
    }

    fn debug_load_img(
        &mut self,
        addr_space: &mut AddressSpace,
        vaddr: VAddr,
        data: &[u8],
    ) -> Result<(), String> {
        match self.mmu.load_program(addr_space, vaddr, data) {
            Some(_) => Ok(()),
            None => Err(format!("Failed to load program at {:#x}", vaddr.0)),
        }
    }
}

impl RISCV64 {
    pub fn exec_and_get_next_pc(
        &mut self,
        addr_space: &mut AddressSpace,
        pc: u64,
        instruction: Instruction,
    ) -> u64 {
        let mut next_pc = pc.wrapping_add(4);
        let arith_reg = |rtype: RType, op: fn(u64, u64) -> u64| {
            let rs1_value = self.regs.get(rtype.rs1());
            let rs2_value = self.regs.get(rtype.rs2());
            op(rs1_value, rs2_value)
        };
        let arith_imm = |itype: IType, op: fn(u64, u64) -> u64| {
            let rs1_value = self.regs.get(itype.rs1());
            let imm = sign_extend(itype.imm(), itype.bits());
            op(rs1_value, imm)
        };
        let mut atomic_word = |state: &mut RISCV64, rtype: RType, op: fn(i32, i32) -> i32| {
            let addr = state.regs.get(rtype.rs1());
            let value1 = state
                .mmu
                .read(addr_space, VAddr(addr), crate::addr_space::Size::Word)
                .unwrap() as i32;
            state.regs.set(rtype.rd(), value1 as i64 as u64);
            let rs2_value = state.regs.get(rtype.rs2()) as i32;
            let result = op(value1, rs2_value);
            state
                .mmu
                .write(
                    addr_space,
                    VAddr(addr),
                    crate::addr_space::Size::Word,
                    signed_ext_32_64(result as u32),
                )
                .unwrap();
        };
        let atomic_double_word = |state: &mut RISCV64,
                                  addr_space: &mut AddressSpace,
                                  rtype: RType,
                                  op: fn(i64, i64) -> i64| {
            let addr = state.regs.get(rtype.rs1());
            let value1 = state
                .mmu
                .read(addr_space, VAddr(addr), crate::addr_space::Size::DoubleWord)
                .unwrap();
            state.regs.set(rtype.rd(), value1);
            let rs2_value = state.regs.get(rtype.rs2());
            let result = op(value1 as i64, rs2_value as i64);
            state
                .mmu
                .write(
                    addr_space,
                    VAddr(addr),
                    crate::addr_space::Size::DoubleWord,
                    result as u64,
                )
                .unwrap();
        };
        // x[rd]=sext(M[x[rs1]+sext(offset)][7:0])
        let unwrap_read = |state: &mut RISCV64,
                           addr_space: &mut AddressSpace,
                           itype: IType,
                           size: crate::addr_space::Size| {
            let rs1_value = state.regs.get(itype.rs1());
            let offset = sign_extend(itype.imm(), itype.bits());
            let addr = rs1_value.wrapping_add(offset);
            let value = state.mmu.read(addr_space, VAddr(addr), size).unwrap();
            value
        };
        let unwrap_write = |state: &mut RISCV64,
                            addr_space: &mut AddressSpace,
                            stype: SType,
                            size: crate::addr_space::Size,
                            value: u64| {
            let addr = state
                .regs
                .get(stype.rs1())
                .wrapping_add(sign_extend(stype.imm(), stype.bits()));
            state
                .mmu
                .write(addr_space, VAddr(addr), size, value)
                .unwrap();
        };
        match instruction {
            // x[rd] = sext(immediate[31:12] << 12)
            Instruction::Lui(utype) => {
                let imm = signed_ext_32_64(utype.imm());
                self.regs.set(utype.rd(), imm);
            }
            // x[rd] = PC + sext(immediate[31:12] << 12)
            Instruction::Auipc(utype) => {
                // Auipc: Add upper immediate to PC
                let imm = signed_ext_32_64(utype.imm());
                self.regs.set(utype.rd(), pc.wrapping_add(imm));
            }
            Instruction::Jal(jtype) => {
                self.regs.set(jtype.rd(), pc.wrapping_add(4));
                let imm = sign_extend(jtype.imm(), jtype.bits());
                next_pc = pc.wrapping_add(imm);
            }
            Instruction::Jalr(itype) => {
                let t = pc.wrapping_add(4);
                next_pc = self
                    .regs
                    .get(itype.rs1())
                    .wrapping_add(sign_extend(itype.imm(), itype.bits()))
                    & !1; // Clear the least significant bit
                self.regs.set(itype.rd(), t);
            }
            // if (rs1 == rs2) pc += sext(offset)
            Instruction::Beq(btype) => {
                let rs1_value = self.regs.get(btype.rs1());
                let rs2_value = self.regs.get(btype.rs2());
                if rs1_value == rs2_value {
                    next_pc = pc.wrapping_add(sign_extend(btype.imm(), btype.bits()));
                }
            }
            // if (rs1 != rs2) pc += sext(offset)
            Instruction::Bne(btype) => {
                let rs1_value = self.regs.get(btype.rs1());
                let rs2_value = self.regs.get(btype.rs2());
                if rs1_value != rs2_value {
                    next_pc = pc.wrapping_add(sign_extend(btype.imm(), btype.bits()));
                }
            }
            // if (rs1 <𝑠 rs2) pc += sext(offset)
            Instruction::Blt(btype) => {
                let rs1_value = self.regs.get(btype.rs1());
                let rs2_value = self.regs.get(btype.rs2());
                if (rs1_value as i64) < (rs2_value as i64) {
                    next_pc = pc.wrapping_add(sign_extend(btype.imm(), btype.bits()));
                }
            }
            // if (rs1 >=𝑠 rs2) pc += sext(offset)
            Instruction::Bge(btype) => {
                let rs1_value = self.regs.get(btype.rs1());
                let rs2_value = self.regs.get(btype.rs2());
                if (rs1_value as i64) >= (rs2_value as i64) {
                    next_pc = pc.wrapping_add(sign_extend(btype.imm(), btype.bits()));
                }
            }
            // if (rs1 <𝑢 rs2) pc += sext(offset)
            Instruction::Bltu(btype) => {
                let rs1_value = self.regs.get(btype.rs1());
                let rs2_value = self.regs.get(btype.rs2());
                if rs1_value < rs2_value {
                    next_pc = pc.wrapping_add(sign_extend(btype.imm(), btype.bits()));
                }
            }
            // if (rs1 >=𝑢 rs2) pc += sext(offset)
            Instruction::Bgeu(btype) => {
                let rs1_value = self.regs.get(btype.rs1());
                let rs2_value = self.regs.get(btype.rs2());
                if rs1_value >= rs2_value {
                    next_pc = pc.wrapping_add(sign_extend(btype.imm(), btype.bits()));
                }
            }

            // Load

            // x[rd]=sext(M[x[rs1]+sext(offset)][7:0])
            Instruction::Lb(itype) => {
                let value = unwrap_read(self, addr_space, itype, crate::addr_space::Size::Byte);
                self.regs.set(itype.rd(), (value as i8) as i64 as u64);
            }
            // x[rd]=sext(M[x[rs1]+sext(offset)][15:0])
            Instruction::Lh(itype) => {
                let value = unwrap_read(self, addr_space, itype, crate::addr_space::Size::HalfWord);
                self.regs.set(itype.rd(), (value as i16) as i64 as u64);
            }
            // x[rd]=sext(M[x[rs1]+sext(offset)][31:0])
            Instruction::Lw(itype) => {
                let value = unwrap_read(self, addr_space, itype, crate::addr_space::Size::Word);
                self.regs.set(itype.rd(), (value as i32) as i64 as u64);
            }
            // x[rd]=(M[x[rs1]+sext(offset)][7:0])
            Instruction::Lbu(itype) => {
                let value = unwrap_read(self, addr_space, itype, crate::addr_space::Size::Byte);
                self.regs.set(itype.rd(), value as u64);
            }
            // x[rd]=(M[x[rs1]+sext(offset)][15:0])
            Instruction::Lhu(itype) => {
                let value = unwrap_read(self, addr_space, itype, crate::addr_space::Size::HalfWord);
                self.regs.set(itype.rd(), value as u64);
            }
            // x[rd]=(M[x[rs1]+sext(offset)][31:0])
            Instruction::Lwu(itype) => {
                let value = unwrap_read(self, addr_space, itype, crate::addr_space::Size::Word);
                self.regs.set(itype.rd(), value as u64);
            }
            // x[rd]=(M[x[rs1]+sext(offset)][63:0])
            Instruction::Ld(itype) => {
                let value =
                    unwrap_read(self, addr_space, itype, crate::addr_space::Size::DoubleWord);
                self.regs.set(itype.rd(), value);
            }
            // Store
            // M[ x[rs1] + sext(offset)]  = x[rs2][7:0]
            Instruction::Sb(stype) => {
                let value = self.regs.get(stype.rs2()) as u8;
                unwrap_write(
                    self,
                    addr_space,
                    stype,
                    crate::addr_space::Size::Byte,
                    value as u64,
                );
            }
            // M[ x[rs1] + sext(offset)]  = x[rs2][15:0]
            Instruction::Sh(stype) => {
                let value = self.regs.get(stype.rs2()) as u16;
                unwrap_write(
                    self,
                    addr_space,
                    stype,
                    crate::addr_space::Size::HalfWord,
                    value as u64,
                );
            }
            // M[ x[rs1] + sext(offset)]  = x[rs2][31:0]
            Instruction::Sw(stype) => {
                let value = self.regs.get(stype.rs2()) as u32;
                unwrap_write(
                    self,
                    addr_space,
                    stype,
                    crate::addr_space::Size::Word,
                    value as u64,
                );
            }
            // M[ x[rs1] + sext(offset)]  = x[rs2][63:0]
            Instruction::Sd(stype) => {
                let value = self.regs.get(stype.rs2());
                unwrap_write(
                    self,
                    addr_space,
                    stype,
                    crate::addr_space::Size::DoubleWord,
                    value,
                );
            }

            // op immediate

            // x[rd] = x[rs1] + sext(imm)
            Instruction::Addi(itype) => {
                let dst = itype.rd();
                let res = arith_imm(itype, u64::wrapping_add);
                self.regs.set(dst, res);
            }

            // x[rd] = ( x[rs1] <𝑠 sext(imm) )
            Instruction::Slti(itype) => {
                self.regs.set(
                    itype.rd(),
                    arith_imm(itype, |a, b| {
                        let a_signed = a as i64;
                        let b_signed = b as i64;
                        if a_signed < b_signed { 1 } else { 0 }
                    }),
                );
            }

            // x[rd] = ( x[rs1] <𝑢 sext(imm) )
            Instruction::Sltiu(itype) => {
                self.regs.set(
                    itype.rd(),
                    arith_imm(itype, |a, b| if a < b { 1 } else { 0 }),
                );
            }

            // x[rd] = x[rs1] ^ sext(imm)
            Instruction::Xori(itype) => {
                self.regs.set(itype.rd(), arith_imm(itype, |a, b| a ^ b));
            }

            // x[rd] = x[rs1] | sext(imm)
            Instruction::Ori(itype) => {
                self.regs.set(itype.rd(), arith_imm(itype, |a, b| a | b));
            }

            // x[rd] = x[rs1] & sext(imm)
            Instruction::Andi(itype) => {
                self.regs.set(itype.rd(), arith_imm(itype, |a, b| a & b));
            }

            // x[rd] = x[rs1] <<𝑢 shamt
            Instruction::Slli(shift_type) => {
                let rs1 = self.regs.get(shift_type.rs1());
                let shamt = shift_type.shamt();
                self.regs
                    .set(shift_type.rd(), rs1.wrapping_shl(shamt as u32));
            }

            // x[rd] = x[rs1] >>𝑢 shamt
            Instruction::Srli(shift_type) => {
                let rs1 = self.regs.get(shift_type.rs1());
                let shamt = shift_type.shamt();
                self.regs
                    .set(shift_type.rd(), rs1.wrapping_shr(shamt as u32));
            }

            // x[rd] = x[rs1] >>𝑠 shamt
            Instruction::Srai(shift_type) => {
                let rs1 = self.regs.get(shift_type.rs1());
                let shamt = shift_type.shamt();
                let result = ((rs1 as i64) >> shamt) as u64;
                self.regs.set(shift_type.rd(), result);
            }

            // op
            // x[rd] = x[rs1] + x[rs2]
            Instruction::Add(rtype) => {
                self.regs
                    .set(rtype.rd(), arith_reg(rtype, u64::wrapping_add));
            }

            // x[rd] = x[rs1] - x[rs2]
            Instruction::Sub(rtype) => {
                self.regs
                    .set(rtype.rd(), arith_reg(rtype, u64::wrapping_sub));
            }

            // x[rd] = x[rs1] << x[rs2]
            Instruction::Sll(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let shamt = b & 0x3F; // 64-bit, shift amount is 6 bits (0~63)
                    a.wrapping_shl(shamt as u32)
                });
                self.regs.set(rtype.rd(), result);
            }

            // x[rd] = ( x[rs1] <𝑠 [rs2])
            Instruction::Slt(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let a_signed = a as i64;
                    let b_signed = b as i64;
                    if a_signed < b_signed { 1 } else { 0 }
                });
                self.regs.set(rtype.rd(), result);
            }

            // x[rd] = ( x[rs1] <𝑢 [rs2])
            Instruction::Sltu(rtype) => {
                let result = arith_reg(rtype, |a, b| if a < b { 1 } else { 0 });
                self.regs.set(rtype.rd(), result);
            }

            // x[rd] = x[rs1] ^ x[rs2]
            Instruction::Xor(rtype) => {
                self.regs.set(rtype.rd(), arith_reg(rtype, |a, b| a ^ b));
            }
            // x[rd] = ( x[rs1] >>𝑢 [rs2])
            Instruction::Srl(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let shamt = b & 0x3F;
                    a.wrapping_shr(shamt as u32)
                });
                self.regs.set(rtype.rd(), result);
            }
            // x[rd] = ( x[rs1] >>𝑠 [rs2])
            Instruction::Sra(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let shamt = b & 0x3F;
                    ((a as i64) >> shamt) as u64
                });
                self.regs.set(rtype.rd(), result);
            }
            // x[rd] = x[rs1] | x[rs2]
            Instruction::Or(rtype) => {
                self.regs.set(rtype.rd(), arith_reg(rtype, u64::bitor));
            }
            // x[rd] = x[rs1] ^ x[rs2]
            Instruction::And(rtype) => {
                self.regs.set(rtype.rd(), arith_reg(rtype, |a, b| a & b));
            }
            // x[rd] = x[rs1] * x[rs2]
            Instruction::Mul(rtype) => {
                self.regs
                    .set(rtype.rd(), arith_reg(rtype, u64::wrapping_mul));
            }
            // x[rd] = ( x[rs1]𝑠 * 𝑠 [rs2]) >>𝑠 XLEN
            Instruction::Mulh(rtype) => {
                self.regs.set(
                    rtype.rd(),
                    arith_reg(rtype, |a, b| {
                        let a128 = a as i64 as i128;
                        let b128 = b as i64 as i128;
                        let product = a128.wrapping_mul(b128);
                        println!(
                            "Mulh: a: {:#x}, b: {:#x}, product: {:#x}",
                            a128, b128, product
                        );
                        (product >> 64) as u64
                    }),
                );
            }
            // x[rd] = ( x[rs1]𝑠 *𝑢 [rs2]) >>𝑠 XLEN
            Instruction::Mulhsu(rtype) => {
                self.regs.set(
                    rtype.rd(),
                    arith_reg(rtype, |a, b| {
                        let a128 = signed_ext_64_128(a);
                        let b128 = b as u128;
                        let product = a128.wrapping_mul(b128);
                        (product >> 64) as u64
                    }),
                );
            }

            // x[rd] = (x[rs1] 𝑢 *𝑢 x[rs2]) >>𝑢 64
            Instruction::Mulhu(rtype) => {
                self.regs.set(
                    rtype.rd(),
                    arith_reg(rtype, |a, b| {
                        let a128 = a as u128;
                        let b128 = b as u128;
                        let product = a128.wrapping_mul(b128);
                        (product >> 64) as u64
                    }),
                );
            }
            Instruction::Div(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let a = a as i64;
                    let b = b as i64;
                    if b == 0 {
                        -1_i64 as u64
                    } else {
                        a.wrapping_div(b) as u64
                    }
                });
                self.regs.set(rtype.rd(), result);
            }
            Instruction::Divu(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    if b == 0 {
                        -1_i64 as u64
                    } else {
                        a.wrapping_div(b)
                    }
                });
                self.regs.set(rtype.rd(), result);
            }
            Instruction::Rem(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let a = a as i64;
                    let b = b as i64;
                    a.wrapping_rem(b) as u64
                });
                self.regs.set(rtype.rd(), result);
            }
            Instruction::Remu(rtype) => {
                let result = arith_reg(rtype, u64::wrapping_rem);
                self.regs.set(rtype.rd(), result);
            }

            // Fence
            Instruction::Fence(_) => {
                // TODO: make sure that the ordering of observation
            }
            Instruction::FenceI => todo!(),

            // System
            Instruction::Ecall => todo!(),
            Instruction::Ebreak => {
                self.state = NemuState::End;
                self.halt_pc = pc;
                self.halt_ret = self.regs.get(GRegName::a0);
            }
            Instruction::Uret => todo!(),
            Instruction::Sret => todo!(),
            Instruction::Mret => todo!(),
            Instruction::Wfi => todo!(),
            Instruction::SfenceVma(_) => todo!(),

            // CSR
            Instruction::Csrrw(_) => todo!(),
            Instruction::Csrrs(_) => todo!(),
            Instruction::Csrrc(_) => todo!(),
            Instruction::Csrrwi(_) => todo!(),
            Instruction::Csrrsi(_) => todo!(),
            Instruction::Csrrci(_) => todo!(),

            // op immediate 32
            Instruction::Addiw(itype) => {
                let res = arith_imm(itype, |a, b| {
                    let res = a.wrapping_add(b) as u32;
                    signed_ext_32_64(res)
                });
                self.regs.set(itype.rd(), res);
            }
            // x[rd] = sext((x[rs1][31:0] <<𝑢 shamt)[31:0])
            Instruction::Slliw(shift_type) => {
                let rs1_value = self.regs.get(shift_type.rs1()) as u32;
                let shifts = shift_type.shamt() as u32;
                assert!(shifts & 0x20 == 0, "Slliw shamt must be in range [0, 31]");
                let res = rs1_value.wrapping_shl(shifts);
                self.regs.set(shift_type.rd(), signed_ext_32_64(res));
            }
            // x[rd] = sext((x[rs1][31:0] >>𝑢 shamt)[31:0])
            Instruction::Srliw(shift_type) => {
                let rs1_value = self.regs.get(shift_type.rs1()) as u32;
                let shifts = shift_type.shamt() as u32;
                assert!(shifts & 0x20 == 0, "Srliw shamt must be in range [0, 31]");
                let res = rs1_value.wrapping_shr(shifts);
                self.regs.set(shift_type.rd(), signed_ext_32_64(res));
            }
            // x[rd] = sext(x[rs1][31:0] >>𝑠 shamt)
            Instruction::Sraiw(shift_type) => {
                let rs1_value = self.regs.get(shift_type.rs1()) as i32;
                let shifts = shift_type.shamt() as u32;
                assert!(shifts & 0x20 == 0, "Sraiw shamt must be in range [0, 31]");
                let res = rs1_value.wrapping_shr(shifts) as u32;
                self.regs.set(shift_type.rd(), signed_ext_32_64(res));
            }

            // op 32

            // x[rd] = sext (( x[rs1] + x[rs2])[ 31: 0 ])
            Instruction::Addw(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let res = a.wrapping_add(b) as u32;
                    signed_ext_32_64(res)
                });
                self.regs.set(rtype.rd(), result);
            }
            // x[rd] = sext (( x[rs1] - x[rs2])[ 31: 0 ])
            Instruction::Subw(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let res = a.wrapping_sub(b) as u32;
                    signed_ext_32_64(res)
                });
                self.regs.set(rtype.rd(), result);
            }
            // x[rd] = sext (( x[rs1] ≪ x[rs2][ 4: 0 ])[ 31:0 ])
            Instruction::Sllw(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let shamt = (b & 0x1F) as u32;
                    let res = (a as u32).wrapping_shl(shamt);
                    signed_ext_32_64(res)
                });
                self.regs.set(rtype.rd(), result);
            }
            // x[rd] = sext ( x[rs1][ 31: 0 ] >>𝑢 x[rs2][ 4: 0 ])
            Instruction::Srlw(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let shamt = (b & 0x1F) as u32;
                    let res = (a as u32).wrapping_shr(shamt);
                    signed_ext_32_64(res)
                });
                self.regs.set(rtype.rd(), result);
            }
            // x[rd] = sext ( x[rs1][ 31: 0 ] >>𝑠 [rs2][ 4:0 ])
            Instruction::Sraw(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let shamt = (b & 0x1F) as u32;
                    let res = truncate_i32(a).wrapping_shr(shamt) as u32;
                    signed_ext_32_64(res)
                });
                self.regs.set(rtype.rd(), result);
            }
            // x[rd] = sext (( x[rs1] × x[rs2])[ 31: 0 ])
            Instruction::Mulw(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let product = a.wrapping_mul(b) as u32;
                    signed_ext_32_64(product)
                });
                self.regs.set(rtype.rd(), result);
            }
            // x[rd]=sext(x[rs1][31:0] ÷𝑠 x[rs2][31:0])
            Instruction::Divw(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let a = truncate_i32(a);
                    let b = truncate_i32(b);
                    let res = if b == 0 {
                        u32::MAX
                    } else {
                        a.wrapping_div(b) as u32
                    };
                    signed_ext_32_64(res)
                });
                self.regs.set(rtype.rd(), result);
            }
            // x[rd]=sext(x[rs1][31:0] ÷𝑢 x[rs2][31:0])
            Instruction::Divuw(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let a = truncate_u32(a);
                    let b = truncate_u32(b);
                    let res = if b == 0 { u32::MAX } else { a.wrapping_div(b) };
                    signed_ext_32_64(res)
                });
                self.regs.set(rtype.rd(), result);
            }
            Instruction::Remw(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let a = truncate_i32(a);
                    let b = truncate_i32(b);
                    signed_ext_32_64(a.wrapping_rem(b) as u32)
                });
                self.regs.set(rtype.rd(), result);
            }
            Instruction::Remuw(rtype) => {
                let result = arith_reg(rtype, |a, b| {
                    let a = truncate_u32(a);
                    let b = truncate_u32(b);
                    signed_ext_32_64(a.wrapping_rem(b))
                });
                self.regs.set(rtype.rd(), result);
            }

            // RV64A Standard Extension
            Instruction::AmoswapD(rtype) => {
                let addr = self.regs.get(rtype.rs1());
                let value1 = self
                    .mmu
                    .read(addr_space, VAddr(addr), crate::addr_space::Size::DoubleWord)
                    .unwrap();
                self.regs.set(rtype.rd(), value1);
                let rs2_value = self.regs.get(rtype.rs2());
                self.mmu
                    .write(
                        addr_space,
                        VAddr(addr),
                        crate::addr_space::Size::DoubleWord,
                        rs2_value,
                    )
                    .unwrap();
            }
            Instruction::AmoaddD(rtype) => {
                atomic_double_word(self, addr_space, rtype, i64::wrapping_add);
            }
            Instruction::AmoxorD(rtype) => {
                atomic_double_word(self, addr_space, rtype, |a, b| a ^ b);
            }
            Instruction::AmoandD(rtype) => {
                atomic_double_word(self, addr_space, rtype, |a, b| a & b);
            }
            Instruction::AmoorD(rtype) => {
                atomic_double_word(self, addr_space, rtype, i64::bitor);
            }
            Instruction::AmominD(rtype) => {
                atomic_double_word(self, addr_space, rtype, i64::min);
            }
            Instruction::AmomaxD(rtype) => {
                atomic_double_word(self, addr_space, rtype, i64::max);
            }
            Instruction::AmominuD(rtype) => {
                atomic_double_word(self, addr_space, rtype, |a, b| {
                    (a as u64).min(b as u64) as i64
                });
            }
            Instruction::AmomaxuD(rtype) => {
                atomic_double_word(self, addr_space, rtype, |a, b| {
                    (a as u64).max(b as u64) as i64
                });
            }
            Instruction::LrD(rtype) => {
                let addr = self.regs.get(rtype.rs1());
                let value = self
                    .mmu
                    .read(addr_space, VAddr(addr), crate::addr_space::Size::DoubleWord)
                    .unwrap();
                self.reserve.reset(addr);
                self.regs.set(rtype.rd(), value);
            }
            Instruction::ScD(rtype) => {
                let addr = self.regs.get(rtype.rs1());
                let value = self.regs.get(rtype.rd());
                if self.reserve.check(addr) {
                    self.mmu
                        .write(
                            addr_space,
                            VAddr(addr),
                            crate::addr_space::Size::DoubleWord,
                            value as u64,
                        )
                        .unwrap();
                    self.regs.set(rtype.rd(), 0);
                } else {
                    // If reservation failed, we should not write to memory
                    // and set rd to 0
                    self.regs.set(rtype.rd(), 1);
                }
            }
            Instruction::LrW(rtype) => {
                let addr = self.regs.get(rtype.rs1());
                let value = self
                    .mmu
                    .read(addr_space, VAddr(addr), crate::addr_space::Size::Word)
                    .unwrap() as u32;
                self.reserve.reset(addr);
                self.regs.set(rtype.rd(), signed_ext_32_64(value));
            }
            Instruction::ScW(rtype) => {
                let addr = self.regs.get(rtype.rs1());
                let value = self.regs.get(rtype.rd()) as u32;
                if self.reserve.check(addr) {
                    self.mmu
                        .write(
                            addr_space,
                            VAddr(addr),
                            crate::addr_space::Size::Word,
                            value as u64,
                        )
                        .unwrap();
                    self.regs.set(rtype.rd(), 0);
                } else {
                    // If reservation failed, we should not write to memory
                    // and set rd to 0
                    self.regs.set(rtype.rd(), 1);
                }
            }
            Instruction::Illegal => todo!(),

            Instruction::AmoswapW(rtype) => {
                atomic_word(self, rtype, |_a, b| b);
            }
            Instruction::AmoaddW(rtype) => {
                atomic_word(self, rtype, i32::wrapping_add);
            }
            Instruction::AmoorW(rtype) => {
                atomic_word(self, rtype, i32::bitor);
            }
            Instruction::AmoxorW(rtype) => {
                atomic_word(self, rtype, |a, b| a ^ b);
            }
            Instruction::AmoandW(rtype) => {
                atomic_word(self, rtype, |a, b| a & b);
            }
            Instruction::AmominW(rtype) => {
                atomic_word(self, rtype, i32::min);
            }
            Instruction::AmomaxW(rtype) => {
                atomic_word(self, rtype, i32::max);
            }
            Instruction::AmominuW(rtype) => {
                atomic_word(self, rtype, |a, b| (a as u32).min(b as u32) as i32);
            }
            Instruction::AmomaxuW(rtype) => {
                atomic_word(self, rtype, |a, b| (a as u32).max(b as u32) as i32);
            }

            _ => {
                panic!("unknown instruction {:?} at PC {:#x}", instruction, pc);
            }
        }

        next_pc
    }
}

trait ImmBits {
    fn bits(&self) -> u8;
}

impl ImmBits for IType {
    fn bits(&self) -> u8 {
        12
    }
}

impl ImmBits for SType {
    fn bits(&self) -> u8 {
        12
    }
}

impl ImmBits for BType {
    fn bits(&self) -> u8 {
        13
    }
}

impl ImmBits for UType {
    fn bits(&self) -> u8 {
        20
    }
}

impl ImmBits for JType {
    fn bits(&self) -> u8 {
        21
    }
}

fn sign_extend(value: u32, bitwidth: u8) -> u64 {
    let shift = 64 - bitwidth;
    let res = ((value as i64) << shift) >> shift;
    res as u64
}

fn signed_ext_32_64(x: u32) -> u64 {
    (x as i32) as i64 as u64
}

fn signed_ext_64_128(x: u64) -> u128 {
    x as i64 as i128 as u128
}

// truncate to 32 bits signed int
fn truncate_i32(x: u64) -> i32 {
    (x & 0xFFFFFFFF) as i32
}

fn truncate_u32(x: u64) -> u32 {
    // Truncate to 32 bits
    (x & 0xFFFFFFFF) as u32
}

pub trait RTypeExt: Sized + Clone {
    fn set_rd(&mut self, rd: GRegName);
    fn set_rs1(&mut self, rs1: GRegName);
    fn set_rs2(&mut self, rs2: GRegName);

    fn with(&self, rd: GRegName, rs1: GRegName, rs2: GRegName) -> Self {
        let mut this = self.clone();
        this.set_rd(rd);
        this.set_rs1(rs1);
        this.set_rs2(rs2);
        this
    }
}

impl RTypeExt for RType {
    fn set_rd(&mut self, rd: GRegName) {
        let rd_index = rd as u32 & 0x1F;
        self.0 = self.0 | (rd_index << 7);
    }

    fn set_rs1(&mut self, rs1: GRegName) {
        let rs1_index = rs1 as u32 & 0x1F;
        self.0 = self.0 | (rs1_index << 15);
    }

    fn set_rs2(&mut self, rs2: GRegName) {
        let rs2_index = rs2 as u32 & 0x1F;
        self.0 = self.0 | (rs2_index << 20);
    }
}

pub trait ITypeExt: Sized + Clone {
    fn set_imm(&mut self, imm: u32);
    fn set_rd(&mut self, rd: GRegName);
    fn set_rs1(&mut self, rs1: GRegName);

    fn with(&self, rd: GRegName, rs1: GRegName) -> Self {
        let mut this = self.clone();
        this.set_rd(rd);
        this.set_rs1(rs1);
        this
    }
}

impl ITypeExt for IType {
    fn set_imm(&mut self, imm: u32) {
        // imm is 12 bits, so we mask it to fit
        self.0 = self.0 | (imm & 0xFFF) << 20;
    }

    fn set_rd(&mut self, rd: GRegName) {
        let rd_index = rd as u32 & 0x1F;
        self.0 = self.0 | (rd_index << 7);
    }

    fn set_rs1(&mut self, rs1: GRegName) {
        let rs1_index = rs1 as u32 & 0x1F;
        self.0 = self.0 | (rs1_index << 15);
    }
}
