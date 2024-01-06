use super::{bits_to_reg, DecodeResult, Instr, LOW_THREE, MID_THREE, TOP_TWO};
use crate::options;

/// Attempt to decode an 8-bit Arithmetic instruction.
///
/// # Arguments
/// - `memory`: slice of memory beginning with the first byte of the instruction
pub fn arith8(memory: &[u8]) -> DecodeResult {
    match memory {
        [0xc6, n, ..] => Some((Instr::ADD_A_n(*n), 2)),
        [0x86, ..] => Some((Instr::ADD_A_HL, 1)),
        [0xdd, rest @ ..] => match rest {
            [0x86, d, ..] => Some((Instr::ADD_A_IX(*d as i8), 3)),
            [0x8e, d, ..] => Some((Instr::ADC_A_IX(*d as i8), 3)),
            [0x96, d, ..] => Some((Instr::SUB_A_IX(*d as i8), 3)),
            [0x9e, d, ..] => Some((Instr::SBC_A_IX(*d as i8), 3)),
            [0x34, d, ..] => Some((Instr::INC_IX(*d as i8), 3)),
            _ => None,
        },
        [0xfd, rest @ ..] => match rest {
            [0x86, d, ..] => Some((Instr::ADD_A_IY(*d as i8), 3)),
            [0x8e, d, ..] => Some((Instr::ADC_A_IY(*d as i8), 3)),
            [0x96, d, ..] => Some((Instr::SUB_A_IY(*d as i8), 3)),
            [0x9e, d, ..] => Some((Instr::SBC_A_IY(*d as i8), 3)),
            [0x34, d, ..] => Some((Instr::INC_IY(*d as i8), 3)),
            _ => None,
        },
        [0xce, n, ..] => Some((Instr::ADC_A_n(*n), 2)),
        [0x8e, ..] => Some((Instr::ADC_A_HL, 1)),
        [0xd6, n, ..] => Some((Instr::SUB_A_n(*n), 2)),
        [0x96, ..] => Some((Instr::SUB_A_HL, 1)),
        [0xde, n, ..] => Some((Instr::SBC_A_n(*n), 2)),
        [0x9e, ..] => Some((Instr::SBC_A_HL, 1)),
        [0x34, ..] => Some((Instr::INC_HL, 1)),
        _ => options!(
            add_a_r(memory),
            addc_a_r(memory),
            sub_a_r(memory),
            subc_a_r(memory),
            inc_r(memory)
        )
    }
}

fn add_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & TOP_TWO != 0b10000000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::ADD_A_r(r), 1))
}

fn addc_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b10001000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::ADC_A_r(r), 1))
}

fn sub_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b10010000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::SUB_A_r(r), 1))
}

fn subc_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b10011000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::SBC_A_r(r), 1))
}

fn inc_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | LOW_THREE) != 0b00000100 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    Some((Instr::INC_r(r), 1))
}
