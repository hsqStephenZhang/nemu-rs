use thiserror::Error;

#[derive(Error, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecodeError {
    #[error("custom")]
    /// Instruction's opcode is reserved for custom extentions and thus can't be decoded further.
    Custom,
    /// Instruction's opcode is reserved for future standard extentions.
    #[error("reserved")]
    Reserved,
    /// Instruction bit pattern not defined in current specification.
    #[error("unknown")]
    Unknown,
    /// More bits from the instruction are required to fully decode it.
    #[error("truncated")]
    Truncated,
    /// Instruction type is well defined but is part of some extension this library doesn't support
    /// decoding yet.
    #[error("unimplemented")]
    Unimplemented,
}
