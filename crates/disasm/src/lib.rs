pub use capstone::{Insn, prelude::*};

#[derive(Debug, thiserror::Error)]
pub enum DisasmError {
    #[error("Capstone error: {0}")]
    CapstoneError(#[from] capstone::Error),
}

pub fn disasm_with(
    code: &[u8],
    addr: Option<u64>,
    printer: &dyn Fn(&Insn<'_>),
) -> Result<(), DisasmError> {
    let cs = Capstone::new()
        .riscv()
        .mode(arch::riscv::ArchMode::RiscV64)
        .extra_mode(std::iter::once(arch::riscv::ArchExtraMode::RiscVC))
        .detail(true)
        .build()?;

    let insns = cs.disasm_all(code, addr.unwrap_or_default())?;

    for i in insns.iter() {
        printer(i);
    }

    Ok(())
}

pub fn disasm(code: &[u8], addr: Option<u64>) -> Result<(), Box<dyn std::error::Error>> {
    disasm_with(code, addr, &|insn| {
        println!(
            "{:08x}: {:<8} {}",
            insn.address(),
            insn.mnemonic().unwrap_or(""),
            insn.op_str().unwrap_or("")
        );
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_ins_disasm() -> Result<(), Box<dyn std::error::Error>> {
        // int mul(int a, int b) {
        //     return a * b;
        // }
        #[rustfmt::skip]
        let code: &[u8] = &[
            0x01, 0x11, // addi sp, sp, -32
            0x22, 0xec, // sd s0, 24(sp)
            0x00, 0x10, // addi s0, sp, 32
            0xaa, 0x87, // mv a5, a0
            0x2e, 0x87, // mv a4, a1
            0x23, 0x26, 0xf4, 0xfe, // sw a5, -20(s0)
            0xba, 0x87, // mv a5, a4
            0x23, 0x24, 0xf4, 0xfe, // sw a5, -24(s0)
            0x83, 0x27, 0xc4, 0xfe, // lw a5, -20(s0)
            0x3e, 0x87, // mv a4, a5
            0x83, 0x27, 0x84, 0xfe, // lw a5, -24(s0)
            0xbb, 0x07, 0xf7, 0x02, // mulw a5, a4, a5
            0x81, 0x27, // sext.w a5, a5
            0x3e, 0x85, // mv a0, a5
            0x62, 0x64, // ld s0, 24(sp)
            0x05, 0x61, // addi sp, sp, 32
            0x82, 0x80, // ret
        ];

        disasm(&code, None)?;
        Ok(())
    }
}
