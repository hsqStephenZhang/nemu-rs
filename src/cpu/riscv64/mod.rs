use std::{ops::BitOr, str::FromStr};

use riscv_decode::{
    Instruction,
    types::{BType, IType, JType, RType, SType, UType},
};
use tracing::debug;

use crate::{
    cpu::{
        Cpu, NemuState,
        riscv64::{
            logo::LOGO,
            mmu::MMU,
            regs::{GRegName, GeneralRegs},
        },
    },
    memory::addr::VAddr,
};

mod csr;
mod insn;
mod logo;
mod mmu;
mod regs;

// TODO: CSR and system related state
pub struct RISCV64 {
    state: NemuState,
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
        let insn = riscv_decode::decode(instruction).unwrap();
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
            Instruction::Fence(_) => todo!(),
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
            Instruction::AmoswapD(_) => todo!(),
            Instruction::AmoaddD(_) => todo!(),
            Instruction::AmoxorD(_) => todo!(),
            Instruction::AmoandD(_) => todo!(),
            Instruction::AmoorD(_) => todo!(),
            Instruction::AmominD(_) => todo!(),
            Instruction::AmomaxD(_) => todo!(),
            Instruction::AmominuD(_) => todo!(),
            Instruction::AmomaxuD(_) => todo!(),
            Instruction::LrD(_) => todo!(),
            Instruction::ScD(_) => todo!(),
            Instruction::Illegal => todo!(),
            _ => todo!(),
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

    use std::{io::Read, u32, u64};

    use riscv_decode::*;
    use tracing::info;

    use super::*;
    use crate::{
        device::{consts::SERIAL_MMIO_START, serial::Serial},
        init_log,
        memory::{
            PhyMem,
            addr::PAddr,
            config::{MBASE, MSIZE},
        },
    };

    #[test]
    fn t1() {
        let ins = decode(0x00000097).unwrap();
        println!("{:?}", ins);
    }

    #[test]
    fn test_signed_ext() {
        let a = u32::MAX;
        let b = signed_ext_32_64(a);
        assert_eq!(b, 0xFFFFFFFFFFFFFFFF);
        assert_eq!(a as u64, 0x00000000FFFFFFFF);
    }

    #[test]
    fn test_mulh() {
        let mut cpu = RISCV64 {
            state: NemuState::Running,
            regs: GeneralRegs::new(),
            halt_pc: 0,
            halt_ret: 0,
            mmu: MMU::new(PhyMem::new(PAddr(MBASE), MSIZE as usize), mmu::Mode::Bare),
        };

        // Test LUI instruction
        let insn = Instruction::Mulh(RType(0).with(GRegName::a0, GRegName::a1, GRegName::a2));
        cpu.set_reg_by_name(GRegName::a1.into(), -1_i64 as u64);
        cpu.set_reg_by_name(GRegName::a2.into(), -1_i64 as u64);
        cpu.exec_and_get_next_pc(0, insn);
        // (-1)*(−1) >> 64
        // 0x00000000000000000000000000000001 >> 64
        assert_eq!(cpu.regs.get(GRegName::a0), 0);

        let insn = Instruction::Mulhu(RType(0).with(GRegName::a0, GRegName::a1, GRegName::a2));
        cpu.exec_and_get_next_pc(0, insn);
        // (2^64-1)*(2^64-1) >> 64
        // 0xFFFFFFFFFFFFFFFE0000000000000001 >> 64
        assert_eq!(cpu.regs.get(GRegName::a0), u64::MAX - 1);

        let insn = Instruction::Mulhsu(RType(0).with(GRegName::a0, GRegName::a1, GRegName::a2));
        cpu.exec_and_get_next_pc(0, insn);
        // −(2^64-1) >> 64
        // 0xFFFFFFFFFFFFFFFF0000000000000001 >> 64
        assert_eq!(cpu.regs.get(GRegName::a0), u64::MAX);
    }

    const CPU_TESTS_DIR: &str = "/workspaces/course/ics-pa/am-kernels/tests/cpu-tests/build/";
    const ALU_TESTS_DIR: &str = "/workspaces/course/ics-pa/am-kernels/tests/alu-tests/build/";

    fn run_cpu_test_once(testcase: &str) {
        let file = format!("{}{}", CPU_TESTS_DIR, testcase);
        run_test_once(&file);
    }

    fn run_alu_test_once() {
        let file = format!("{}{}", ALU_TESTS_DIR, "alutest-riscv64-nemu.bin");
        run_test_once(&file);
    }

    fn run_test_once(file: &str) {
        info!("testing test with {}", file);
        let mut phy_mem = PhyMem::new(PAddr(MBASE), MSIZE as usize);
        phy_mem
            .add_mmio(
                SERIAL_MMIO_START,
                SERIAL_MMIO_START + 1,
                "serial",
                Serial::new_mmio(),
            )
            .unwrap();
        let mut cpu = RISCV64 {
            state: NemuState::Running,
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
    fn test_failed_cases() {
        init_log(tracing::Level::DEBUG);
        // TODO: move it to cpu_tests after finish printf, vprintf, sprintf in PA
        // it's up the to the provider of the image
        run_cpu_test_once("hello-str-riscv64-nemu.bin");
    }

    #[test]
    fn test_alu() {
        init_log(tracing::Level::INFO);
        run_alu_test_once();
    }

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
            let testcase = format!("{}{}", CPU_TESTS_DIR, i);
            difftest_run(&testcase)?;
        }
        Ok(())
    }

    fn difftest_run(testcase: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open(testcase).unwrap();
        let mut image = Vec::new();
        file.read_to_end(&mut image).unwrap();

        let mut phy_mem = PhyMem::new(PAddr(MBASE), MSIZE as usize);
        phy_mem
            .add_mmio(
                SERIAL_MMIO_START,
                SERIAL_MMIO_START + 1,
                "serial",
                Serial::new_mmio(),
            )
            .unwrap();
        let mut cpu = RISCV64 {
            state: NemuState::Running,
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
            debug_assert_eq!(&qemu_regs, cpu.regs.all());
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
}
