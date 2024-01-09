use super::{DecodeResult, Instr};

/// Attempt to decode an Exchange, Transfer, or Search instruction.
///
/// # Arguments
/// - `mem`: slice of memory with the instruction to decode beginning at `mem[0]`
pub fn exchange(mem: &[u8]) -> DecodeResult {
    match mem {
        [0xeb, ..] => Some((Instr::EX_DE_HL, 1)),
        [0x08, ..] => Some((Instr::EX_AF_AF1, 1)),
        [0xd9, ..] => Some((Instr::EXX, 1)),
        [0xe3, ..] => Some((Instr::EX_SP_HL, 1)),
        [0xdd, 0xe3, ..] => Some((Instr::EX_SP_IX, 2)),
        [0xfd, 0xe3, ..] => Some((Instr::EX_SP_IY, 2)),
        [0xed, n, ..] => match *n {
            0xa0 => Some((Instr::LDI, 2)),
            0xb0 => Some((Instr::LDIR, 2)),
            0xa8 => Some((Instr::LDD, 2)),
            0xb8 => Some((Instr::LDDR, 2)),
            0xa1 => Some((Instr::CPI, 2)),
            0xb1 => Some((Instr::CPIR, 2)),
            0xa9 => Some((Instr::CPD, 2)),
            0xb9 => Some((Instr::CPDR, 2)),
            _ => None,
        },
        _ => None,
    }
}
