//! Functions for decoding Jump instructions.
use super::{bits_to_condition, DecodeResult, Instruction, LOW_THREE, MID_THREE, TOP_TWO};
use byteorder::{ByteOrder, LE};

/// Attempt to decode a Jump instruction.
///
/// # Arguments
/// - `memory`: slice of memory with first byte of instruction at index 0
pub fn jump(memory: &[u8]) -> DecodeResult {
    match memory {
        [0xc3, rest @ ..] => {
            let nn = LE::read_u16(rest);
            Some((Instruction::JP_nn(nn), 3))
        }
        [0x18, e, ..] => Some((Instruction::JR_e((*e as i8) + 2), 2)),
        [0x38, e, ..] => Some((Instruction::JR_C_e((*e as i8) + 2), 2)),
        [0x30, e, ..] => Some((Instruction::JR_NC_e((*e as i8) + 2), 2)),
        [0x28, e, ..] => Some((Instruction::JR_Z_e((*e as i8) + 2), 2)),
        [0x20, e, ..] => Some((Instruction::JR_NZ_e((*e as i8) + 2), 2)),
        [0xe9, ..] => Some((Instruction::JP_HL, 1)),
        [0xdd, 0xe9, ..] => Some((Instruction::JP_IX, 2)),
        [0xfd, 0xe9, ..] => Some((Instruction::JP_IY, 2)),
        [0x10, e, ..] => Some((Instruction::DJNZ_e((*e as i8) + 2), 2)),
        _ => jp_cc_nn(memory),
    }
}

fn jp_cc_nn(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | LOW_THREE) != 0b11000010 {
        return None;
    }

    let cc = bits_to_condition((mem[0] & MID_THREE) >> 3)?;
    let nn = LE::read_u16(&mem[1..]);
    Some((Instruction::JP_cc_nn(cc, nn), 3))
}
