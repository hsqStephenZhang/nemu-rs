use std::{ops::BitOr, str::FromStr};

use riscv_decode::{
    Instruction,
    types::{BType, IType, JType, RType, SType, UType},
};
use tracing::*;

use crate::{
    cpu::{
        Cpu, NemuState,
        riscv64::{
            logo::LOGO,
            mmu::MMU,
            regs::{GRegName, GeneralRegs},
            reserved::Reserve,
        },
    },
    memory::addr::VAddr,
};

mod csr;
mod insn;
mod logo;
mod mmu;
mod regs;
mod reserved;

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
    pub fn vmem_enabled(&self) -> bool {
        // RISCV64 always has virtual memory enabled
        true
    }
}

impl Cpu for RISCV64 {
    type Instruction = u32;

    fn ifetch(&mut self, pc: VAddr) -> Option<Self::Instruction> {
        let res = self.mmu.read(pc, crate::memory::Size::Word)?;
        Some(res as u32)
    }

    fn exec_once(&mut self, pc: VAddr) {
        let printer = |insn: &disasm::Insn<'_>| {
            debug!(
                "{:08x}: {:<8} {}",
                insn.address(),
                insn.mnemonic().unwrap_or(""),
                insn.op_str().unwrap_or("")
            );
        };
        let instruction = self.ifetch(pc).unwrap();
        disasm::disasm_with(&instruction.to_le_bytes(), Some(pc.0), &printer).unwrap();
        let insn = match riscv_decode::decode(instruction) {
            Ok(insn) => insn,
            Err(e) => {
                panic!("Failed to decode instruction at {:#x}: {:?}", pc.0, e);
            }
        };
        let next_pc = self.exec_and_get_next_pc(pc.0, insn);
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
}

impl RISCV64 {
    pub fn exec_and_get_next_pc(&mut self, pc: u64, instruction: Instruction) -> u64 {
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
        let atomic_word = |state: &mut RISCV64, rtype: RType, op: fn(i32, i32) -> i32| {
            let addr = state.regs.get(rtype.rs1());
            let value1 = state
                .mmu
                .read(VAddr(addr), crate::memory::Size::Word)
                .unwrap() as i32;
            state.regs.set(rtype.rd(), value1 as i64 as u64);
            let rs2_value = state.regs.get(rtype.rs2()) as i32;
            let result = op(value1, rs2_value);
            state
                .mmu
                .write(
                    VAddr(addr),
                    crate::memory::Size::Word,
                    signed_ext_32_64(result as u32),
                )
                .unwrap();
        };
        let atomic_double_word = |state: &mut RISCV64, rtype: RType, op: fn(i64, i64) -> i64| {
            let addr = state.regs.get(rtype.rs1());
            let value1 = state
                .mmu
                .read(VAddr(addr), crate::memory::Size::DoubleWord)
                .unwrap();
            state.regs.set(rtype.rd(), value1);
            let rs2_value = state.regs.get(rtype.rs2());
            let result = op(value1 as i64, rs2_value as i64);
            state
                .mmu
                .write(VAddr(addr), crate::memory::Size::DoubleWord, result as u64)
                .unwrap();
        };
        // x[rd]=sext(M[x[rs1]+sext(offset)][7:0])
        let unwrap_read = |state: &mut RISCV64, itype: IType, size: crate::memory::Size| {
            let rs1_value = state.regs.get(itype.rs1());
            let offset = sign_extend(itype.imm(), itype.bits());
            let addr = rs1_value.wrapping_add(offset);
            let value = state.mmu.read(VAddr(addr), size).unwrap();
            value
        };
        let unwrap_write =
            |state: &mut RISCV64, stype: SType, size: crate::memory::Size, value: u64| {
                let addr = state
                    .regs
                    .get(stype.rs1())
                    .wrapping_add(sign_extend(stype.imm(), stype.bits()));
                state.mmu.write(VAddr(addr), size, value).unwrap();
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
                let value = unwrap_read(self, itype, crate::memory::Size::Byte);
                self.regs.set(itype.rd(), (value as i8) as i64 as u64);
            }
            // x[rd]=sext(M[x[rs1]+sext(offset)][15:0])
            Instruction::Lh(itype) => {
                let value = unwrap_read(self, itype, crate::memory::Size::HalfWord);
                self.regs.set(itype.rd(), (value as i16) as i64 as u64);
            }
            // x[rd]=sext(M[x[rs1]+sext(offset)][31:0])
            Instruction::Lw(itype) => {
                let value = unwrap_read(self, itype, crate::memory::Size::Word);
                self.regs.set(itype.rd(), (value as i32) as i64 as u64);
            }
            // x[rd]=(M[x[rs1]+sext(offset)][7:0])
            Instruction::Lbu(itype) => {
                let value = unwrap_read(self, itype, crate::memory::Size::Byte);
                self.regs.set(itype.rd(), value as u64);
            }
            // x[rd]=(M[x[rs1]+sext(offset)][15:0])
            Instruction::Lhu(itype) => {
                let value = unwrap_read(self, itype, crate::memory::Size::HalfWord);
                self.regs.set(itype.rd(), value as u64);
            }
            // x[rd]=(M[x[rs1]+sext(offset)][31:0])
            Instruction::Lwu(itype) => {
                let value = unwrap_read(self, itype, crate::memory::Size::Word);
                self.regs.set(itype.rd(), value as u64);
            }
            // x[rd]=(M[x[rs1]+sext(offset)][63:0])
            Instruction::Ld(itype) => {
                let value = unwrap_read(self, itype, crate::memory::Size::DoubleWord);
                self.regs.set(itype.rd(), value);
            }
            // Store
            // M[ x[rs1] + sext(offset)]  = x[rs2][7:0]
            Instruction::Sb(stype) => {
                let value = self.regs.get(stype.rs2()) as u8;
                unwrap_write(self, stype, crate::memory::Size::Byte, value as u64);
            }
            // M[ x[rs1] + sext(offset)]  = x[rs2][15:0]
            Instruction::Sh(stype) => {
                let value = self.regs.get(stype.rs2()) as u16;
                unwrap_write(self, stype, crate::memory::Size::HalfWord, value as u64);
            }
            // M[ x[rs1] + sext(offset)]  = x[rs2][31:0]
            Instruction::Sw(stype) => {
                let value = self.regs.get(stype.rs2()) as u32;
                unwrap_write(self, stype, crate::memory::Size::Word, value as u64);
            }
            // M[ x[rs1] + sext(offset)]  = x[rs2][63:0]
            Instruction::Sd(stype) => {
                let value = self.regs.get(stype.rs2());
                unwrap_write(self, stype, crate::memory::Size::DoubleWord, value);
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
                    .read(VAddr(addr), crate::memory::Size::DoubleWord)
                    .unwrap();
                self.regs.set(rtype.rd(), value1);
                let rs2_value = self.regs.get(rtype.rs2());
                self.mmu
                    .write(VAddr(addr), crate::memory::Size::DoubleWord, rs2_value)
                    .unwrap();
            }
            Instruction::AmoaddD(rtype) => {
                atomic_double_word(self, rtype, i64::wrapping_add);
            }
            Instruction::AmoxorD(rtype) => {
                atomic_double_word(self, rtype, |a, b| a ^ b);
            }
            Instruction::AmoandD(rtype) => {
                atomic_double_word(self, rtype, |a, b| a & b);
            }
            Instruction::AmoorD(rtype) => {
                atomic_double_word(self, rtype, i64::bitor);
            }
            Instruction::AmominD(rtype) => {
                atomic_double_word(self, rtype, i64::min);
            }
            Instruction::AmomaxD(rtype) => {
                atomic_double_word(self, rtype, i64::max);
            }
            Instruction::AmominuD(rtype) => {
                atomic_double_word(self, rtype, |a, b| (a as u64).min(b as u64) as i64);
            }
            Instruction::AmomaxuD(rtype) => {
                atomic_double_word(self, rtype, |a, b| (a as u64).max(b as u64) as i64);
            }
            Instruction::LrD(rtype) => {
                let addr = self.regs.get(rtype.rs1());
                let value = self
                    .mmu
                    .read(VAddr(addr), crate::memory::Size::DoubleWord)
                    .unwrap();
                self.reserve.reset(addr);
                self.regs.set(rtype.rd(), value);
            }
            Instruction::ScD(rtype) => {
                let addr = self.regs.get(rtype.rs1());
                let value = self.regs.get(rtype.rd());
                if self.reserve.check(addr) {
                    self.mmu
                        .write(VAddr(addr), crate::memory::Size::DoubleWord, value as u64)
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
                    .read(VAddr(addr), crate::memory::Size::Word)
                    .unwrap() as u32;
                self.reserve.reset(addr);
                self.regs.set(rtype.rd(), signed_ext_32_64(value));
            }
            Instruction::ScW(rtype) => {
                let addr = self.regs.get(rtype.rs1());
                let value = self.regs.get(rtype.rd()) as u32;
                if self.reserve.check(addr) {
                    self.mmu
                        .write(VAddr(addr), crate::memory::Size::Word, value as u64)
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

#[cfg(test)]
mod tests {

    use std::env;
    use std::{io::Read, u32, u64};

    use once_cell::sync::Lazy;
    use tracing::info;

    use super::*;
    use crate::{
        init_log,
        memory::{
            PhyMem,
            addr::PAddr,
            config::{MBASE, MSIZE},
        },
    };

    static CPU_TESTS_DIR: Lazy<String> = Lazy::new(|| {
        let pa_home = env::var("PA_HOME").expect("Environment variable PA_HOME is not set");
        let path = std::path::PathBuf::new()
            .join(pa_home)
            .join("am-kernels/tests/cpu-tests/build/");
        path.to_str().unwrap().to_string()
    });

    static ALU_TESTS_DIR: Lazy<String> = Lazy::new(|| {
        let pa_home = env::var("PA_HOME").expect("Environment variable PA_HOME is not set");
        let path = std::path::PathBuf::new()
            .join(pa_home)
            .join("am-kernels/tests/alu-tests/build/");
        path.to_str().unwrap().to_string()
    });

    const TARGET_DIR: Lazy<String> = Lazy::new(|| {
        let mut project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        project_dir = project_dir.strip_suffix("/nemu").unwrap().to_string();
        project_dir + "/target/riscv64gc-unknown-none-elf/release/"
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
        let phy_mem = PhyMem::new(PAddr(MBASE), MSIZE as usize).with_default_mmios();
        let mut cpu = RISCV64 {
            state: NemuState::Running,
            reserve: Reserve::default(),
            regs: GeneralRegs::new(),
            halt_pc: 0,
            halt_ret: 0,
            mmu: MMU::new(phy_mem, mmu::Mode::Bare),
        };

        let mut file = std::fs::File::open(file).unwrap();
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();

        info!("loaded {} bytes", content.len());

        cpu.mmu.load_program(VAddr(MBASE), &content).unwrap();

        let res = cpu.exec(usize::MAX);
        info!("executed instructions count: {:?}", res);
        assert_eq!(cpu.state, NemuState::End);
        assert_eq!(cpu.halt_ret, 0x0);
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
        run_my_test_once("alloc.bin");
        run_my_test_once("alloc-set.bin");
        run_my_test_once("rand.bin");
        run_my_test_once("atomics.bin");
        run_my_test_once("future.bin");
    }

    #[test]
    fn test_userlib_ioe() {
        run_my_test_once("tracing.bin");
        run_my_test_once("time.bin");
        run_my_test_once("colorful.bin");
        run_my_test_once("dummy.bin");
        run_my_test_once("backtrace.bin");
        run_my_test_once("qsort_bench.bin");
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

        let phy_mem = PhyMem::new(PAddr(MBASE), MSIZE as usize).with_default_mmios();
        let mut cpu = RISCV64 {
            state: NemuState::Running,
            reserve: Reserve::default(),
            regs: GeneralRegs::new(),
            halt_pc: 0,
            halt_ret: 0,
            mmu: MMU::new(phy_mem, mmu::Mode::Bare),
        };
        cpu.mmu.load_program(VAddr(MBASE), &image).unwrap();

        let mut qemu_diff = difftest::qemu::DiffTest::new(1234).unwrap();
        let mut init_regs = cpu.regs.all().to_vec();
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
            debug_assert_eq!(qemu_regs[32], cpu.regs.pc);
            debug_assert_eq_regs(&qemu_regs, cpu.regs.all());
            let pc_offset = (qemu_regs[32] - 0x80000000) as usize;
            disasm::disasm(&image[pc_offset..pc_offset + 4], Some(qemu_regs[32]))?;
            if &image[pc_offset..pc_offset + 4] == &EBREAK {
                // end of program
                println!("registers: {:x?}", qemu_regs);
                println!("result: {}", qemu_regs[10]); // a0 is the return value
                break;
            }
            cpu.exec(1);
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
