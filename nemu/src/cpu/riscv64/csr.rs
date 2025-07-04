use strum_macros::{EnumIter, EnumString, FromRepr, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, EnumIter, EnumString, IntoStaticStr, FromRepr)]
pub enum CSRName {
    sstatus = 0x100,
    sie = 0x104,
    stvec = 0x105,
    scounteren = 0x106,
    sscratch = 0x140,
    sepc = 0x141,
    scause = 0x142,
    stval = 0x143,
    sip = 0x144,
    satp = 0x180,

    mcycle = 0xB00,
    mstatus = 0x300,
    misa = 0x301,
    medeleg = 0x302,
    mideleg = 0x303,
    mie = 0x304,
    mtvec = 0x305,
    mcounteren = 0x306,
    menvcfg = 0x30a,

    mscratch = 0x340,
    mepc = 0x341,
    mcause = 0x342,
    mtval = 0x343,
    mip = 0x344,

    pmpcfg0 = 0x3A0,
    // pmpcfg2 = 0x3A2,
    pmpaddr0 = 0x3B0,
    // pmpaddr1 = 0x3B1,
    // pmpaddr2 = 0x3B2,
    // pmpaddr3 = 0x3B3,
    cycle = 0xc00,
    time = 0xc01,

    mvendorid = 0xF11,
    marchid = 0xF12,
    mimpid = 0xF13,
    mhartid = 0xF14,
    // mnscratch = 0x740,
    // mnstatus = 0x744,
}
