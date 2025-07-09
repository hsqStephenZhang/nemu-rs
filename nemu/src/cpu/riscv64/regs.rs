use strum_macros::{EnumString, IntoStaticStr};
use tracing::*;

use crate::config::{MBASE, PC_RESET_OFFSET};

// register name by ABI
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString, IntoStaticStr)]
pub enum GRegName {
    zero,
    ra,
    sp,
    gp,
    tp,
    t0,
    t1,
    t2,
    s0,
    s1,
    a0,
    a1,
    a2,
    a3,
    a4,
    a5,
    a6,
    a7,
    s2,
    s3,
    s4,
    s5,
    s6,
    s7,
    s8,
    s9,
    s10,
    s11,
    t3,
    t4,
    t5,
    t6,
}

impl From<u32> for GRegName {
    fn from(value: u32) -> Self {
        match value {
            0 => GRegName::zero,
            1 => GRegName::ra,
            2 => GRegName::sp,
            3 => GRegName::gp,
            4 => GRegName::tp,
            5 => GRegName::t0,
            6 => GRegName::t1,
            7 => GRegName::t2,
            8 => GRegName::s0,
            9 => GRegName::s1,
            10 => GRegName::a0,
            11 => GRegName::a1,
            12 => GRegName::a2,
            13 => GRegName::a3,
            14 => GRegName::a4,
            15 => GRegName::a5,
            16 => GRegName::a6,
            17 => GRegName::a7,
            18 => GRegName::s2,
            19 => GRegName::s3,
            20 => GRegName::s4,
            21 => GRegName::s5,
            22 => GRegName::s6,
            23 => GRegName::s7,
            24 => GRegName::s8,
            25 => GRegName::s9,
            26 => GRegName::s10,
            27 => GRegName::s11,
            28 => GRegName::t3,
            29 => GRegName::t4,
            30 => GRegName::t5,
            31 => GRegName::t6, // t6 is the last one
            _ => panic!("Invalid GRegName value: {}", value),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GeneralRegs {
    pub general: [u64; 32],
    pub pc: u64,
    pub _float: [u64; 32], // Placeholder for floating-point registers
}

impl Default for GeneralRegs {
    fn default() -> Self {
        let mut general = [u64::MAX; 32];
        general[0] = 0; // x0 is always 0
        GeneralRegs {
            general,
            pc: MBASE + PC_RESET_OFFSET,
            _float: [0; 32], // Placeholder for floating-point registers
        }
    }
}

impl std::fmt::Display for GeneralRegs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pc: {:#x}\n", self.pc)?;
        for (i, &reg) in self.general.iter().enumerate() {
            write!(f, "x{:02}: {:#x}\n", i, reg)?;
        }
        Ok(())
    }
}

#[allow(unused)]
impl GeneralRegs {
    pub fn new() -> Self {
        GeneralRegs::default()
    }

    pub fn all(&self) -> &[u64] {
        let res = unsafe {
            std::slice::from_raw_parts(
                self as *const GeneralRegs as *const u64,
                std::mem::size_of::<GeneralRegs>() / std::mem::size_of::<u64>(),
            )
        };
        debug_assert_eq!(res.len(), 32 + 32 + 1); // 32 general, 32 float, 1 pc
        res
    }

    pub fn get<R: Into<GRegName>>(&self, reg: R) -> u64 {
        let index = reg.into() as usize;
        self.general[index]
    }

    pub fn set<R: Into<GRegName>>(&mut self, reg: R, value: u64) {
        let index = reg.into() as usize;
        if index == 0 {
            trace!("Cannot set x0 (zero register) to a non-zero value");
            return;
        }
        self.general[index] = value;
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn t1() {
        assert_eq!(GRegName::zero as usize, 0);
        assert_eq!(GRegName::t6 as usize, 31);

        assert_eq!(GRegName::from_str("zero").unwrap(), GRegName::zero);
        let name: &'static str = GRegName::a0.into();
        assert_eq!(name, "a0");
    }
}
