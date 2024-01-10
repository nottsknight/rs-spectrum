//! Functions for decoding Exchange instructions.
use super::{DecodeResult, Instruction};

/// Attempt to decode an Exchange, Transfer, or Search instruction.
///
/// # Arguments
/// - `mem`: slice of memory with the instruction to decode beginning at `mem[0]`
pub fn exchange(mem: &[u8]) -> DecodeResult {
    match mem {
        [0xeb, ..] => Some((Instruction::EX_DE_HL, 1)),
        [0x08, ..] => Some((Instruction::EX_AF_AF1, 1)),
        [0xd9, ..] => Some((Instruction::EXX, 1)),
        [0xe3, ..] => Some((Instruction::EX_SP_HL, 1)),
        [0xdd, 0xe3, ..] => Some((Instruction::EX_SP_IX, 2)),
        [0xfd, 0xe3, ..] => Some((Instruction::EX_SP_IY, 2)),
        [0xed, 0xa0, ..] => Some((Instruction::LDI, 2)),
        [0xed, 0xb0, ..] => Some((Instruction::LDIR, 2)),
        [0xed, 0xa8, ..] => Some((Instruction::LDD, 2)),
        [0xed, 0xb8, ..] => Some((Instruction::LDDR, 2)),
        [0xed, 0xa1, ..] => Some((Instruction::CPI, 2)),
        [0xed, 0xb1, ..] => Some((Instruction::CPIR, 2)),
        [0xed, 0xa9, ..] => Some((Instruction::CPD, 2)),
        [0xed, 0xb9, ..] => Some((Instruction::CPDR, 2)),
        _ => None,
    }
}
