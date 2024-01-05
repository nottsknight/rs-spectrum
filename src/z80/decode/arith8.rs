use super::{bits_to_reg, DecodeResult, Instr, LOW_THREE, TOP_TWO};
use crate::options;

/// Attempt to decode an 8-bit Arithmetic instruction.
///
/// # Arguments
/// - `memory`: slice of memory beginning with the first byte of the instruction
pub fn arith8(memory: &[u8]) -> DecodeResult {
    match memory {
        [0xc6, n, ..] => Some((Instr::ADD_A_n(*n), 2)),
        [0x86, ..] => Some((Instr::ADD_A_HL, 1)),
        [0xdd, 0x86, d, ..] => Some((Instr::ADD_A_IX(*d as i8), 3)),
        [0xfd, 0x86, d, ..] => Some((Instr::ADD_A_IY(*d as i8), 3)),
        _ => options!(add_a_r(memory))
    }
}

fn add_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & TOP_TWO != 0b10000000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::ADD_A_r(r), 1))
}
