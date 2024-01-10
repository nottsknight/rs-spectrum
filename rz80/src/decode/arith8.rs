//! Functions for decoding 8-bit Arithmetic instructions.
use super::{bits_to_reg, DecodeResult, Instruction, LOW_THREE, MID_THREE, TOP_TWO};
use crate::options;

/// Attempt to decode an 8-bit Arithmetic instruction.
///
/// # Arguments
/// - `memory`: slice of memory beginning with the first byte of the instruction
pub fn arith8(memory: &[u8]) -> DecodeResult {
    match memory {
        [0xc6, n, ..] => Some((Instruction::ADD_A_n(*n), 2)),
        [0x86, ..] => Some((Instruction::ADD_A_HL, 1)),
        [0xdd, op, d, ..] => match op {
            0x86 => Some((Instruction::ADD_A_IX(*d as i8), 3)),
            0x8e => Some((Instruction::ADC_A_IX(*d as i8), 3)),
            0x96 => Some((Instruction::SUB_A_IX(*d as i8), 3)),
            0x9e => Some((Instruction::SBC_A_IX(*d as i8), 3)),
            0x34 => Some((Instruction::INC_IX(*d as i8), 3)),
            0xa6 => Some((Instruction::AND_A_IX(*d as i8), 3)),
            0xb6 => Some((Instruction::OR_A_IX(*d as i8), 3)),
            0xae => Some((Instruction::XOR_A_IX(*d as i8), 3)),
            0xbe => Some((Instruction::CP_IX(*d as i8), 3)),
            0x35 => Some((Instruction::DEC_IX(*d as i8), 3)),
            _ => None,
        },
        [0xfd, op, d, ..] => match op {
            0x86 => Some((Instruction::ADD_A_IY(*d as i8), 3)),
            0x8e => Some((Instruction::ADC_A_IY(*d as i8), 3)),
            0x96 => Some((Instruction::SUB_A_IY(*d as i8), 3)),
            0x9e => Some((Instruction::SBC_A_IY(*d as i8), 3)),
            0x34 => Some((Instruction::INC_IY(*d as i8), 3)),
            0xa6 => Some((Instruction::AND_A_IY(*d as i8), 3)),
            0xb6 => Some((Instruction::OR_A_IY(*d as i8), 3)),
            0xae => Some((Instruction::XOR_A_IY(*d as i8), 3)),
            0xbe => Some((Instruction::CP_IY(*d as i8), 3)),
            0x35 => Some((Instruction::DEC_IY(*d as i8), 3)),
            _ => None,
        },
        [0xce, n, ..] => Some((Instruction::ADC_A_n(*n), 2)),
        [0x8e, ..] => Some((Instruction::ADC_A_HL, 1)),
        [0xd6, n, ..] => Some((Instruction::SUB_A_n(*n), 2)),
        [0x96, ..] => Some((Instruction::SUB_A_HL, 1)),
        [0xde, n, ..] => Some((Instruction::SBC_A_n(*n), 2)),
        [0x9e, ..] => Some((Instruction::SBC_A_HL, 1)),
        [0x34, ..] => Some((Instruction::INC_HL, 1)),
        [0xe6, n, ..] => Some((Instruction::AND_A_n(*n), 2)),
        [0xa6, ..] => Some((Instruction::AND_A_HL, 1)),
        [0xf6, n, ..] => Some((Instruction::OR_A_n(*n), 2)),
        [0xb6, ..] => Some((Instruction::OR_A_HL, 1)),
        [0xee, n, ..] => Some((Instruction::XOR_A_n(*n), 2)),
        [0xae, ..] => Some((Instruction::XOR_A_HL, 1)),
        [0xfe, n, ..] => Some((Instruction::CP_n(*n), 2)),
        [0xbe, ..] => Some((Instruction::CP_HL, 1)),
        [0x35, ..] => Some((Instruction::DEC_HL, 1)),
        _ => options!(
            add_a_r(memory),
            addc_a_r(memory),
            sub_a_r(memory),
            subc_a_r(memory),
            and_a_r(memory),
            or_a_r(memory),
            xor_a_r(memory),
            cp_r(memory),
            inc_r(memory),
            dec_r(memory)
        ),
    }
}

fn add_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & TOP_TWO != 0b10000000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instruction::ADD_A_r(r), 1))
}

fn addc_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b10001000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instruction::ADC_A_r(r), 1))
}

fn sub_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b10010000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instruction::SUB_A_r(r), 1))
}

fn subc_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b10011000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instruction::SBC_A_r(r), 1))
}

fn and_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b10100000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instruction::AND_A_r(r), 1))
}

fn or_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b10110000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instruction::OR_A_r(r), 1))
}

fn xor_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b10101000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instruction::XOR_A_r(r), 1))
}

fn cp_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b10111000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instruction::CP_r(r), 1))
}

fn inc_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | LOW_THREE) != 0b00000100 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    Some((Instruction::INC_r(r), 1))
}

fn dec_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | LOW_THREE) != 0b00000101 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    Some((Instruction::DEC_r(r), 1))
}