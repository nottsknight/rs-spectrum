use super::{bits_to_condition, DecodeResult, Instr, LOW_THREE, MID_THREE, TOP_TWO};
use byteorder::{ByteOrder, LE};

/// Attempt to decode a Jump instruction.
///
/// # Arguments
/// - `memory`: slice of memory with first byte of instruction at index 0
pub fn jump(memory: &[u8]) -> DecodeResult {
    match memory {
        [0xc3, rest @ ..] => {
            let nn = LE::read_u16(rest);
            Some((Instr::JP_nn(nn), 3))
        }
        [0x18, e, ..] => Some((Instr::JR_e((*e as i8) + 2), 2)),
        [0x38, e, ..] => Some((Instr::JR_C_e((*e as i8) + 2), 2)),
        [0x30, e, ..] => Some((Instr::JR_NC_e((*e as i8) + 2), 2)),
        [0x28, e, ..] => Some((Instr::JR_Z_e((*e as i8) + 2), 2)),
        [0x20, e, ..] => Some((Instr::JR_NZ_e((*e as i8) + 2), 2)),
        [0xe9, ..] => Some((Instr::JP_HL, 1)),
        [0xdd, 0xe9, ..] => Some((Instr::JP_IX, 2)),
        [0xfd, 0xe9, ..] => Some((Instr::JP_IY, 2)),
        [0x10, e, ..] => Some((Instr::DJNZ_e((*e as i8) + 2), 2)),
        _ => jp_cc_nn(memory),
    }
}

fn jp_cc_nn(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | LOW_THREE) != 0b11000010 {
        return None;
    }

    let cc = bits_to_condition((mem[0] & MID_THREE) >> 3)?;
    let nn = LE::read_u16(&mem[1..]);
    Some((Instr::JP_cc_nn(cc, nn), 3))
}
