//! Functions for decoding 8-bit load instructions.
use super::{bits_to_reg, DecodeResult, Instr, LOW_THREE, MID_THREE, TOP_TWO};
use crate::options;

/// Attempt to decode an 8-bit load instruction from the provided memory slice.
///
/// # Arguments
/// - `mem`: slice of memory with the instruction to decode beginning at `mem[0]`
pub fn load8(mem: &[u8]) -> DecodeResult {
    match mem {
        [0xdd, mem1 @ ..] => match mem1 {
            [0x36, d, n, ..] => Some((Instr::LD_IX_n(*d as i8, *n), 4)),
            _ => options!(load_r_ix(mem1), load_ix_r(mem1)),
        },
        [0xfd, mem1 @ ..] => match mem1 {
            [0x36, d, n, ..] => Some((Instr::LD_IY_n(*d as i8, *n), 4)),
            _ => options!(load_r_iy(mem1), load_iy_r(mem1)),
        },
        [0x36, n, ..] => Some((Instr::LD_HL_n(*n), 1)),
        [0x0a, ..] => Some((Instr::LD_A_BC, 1)),
        [0x1a, ..] => Some((Instr::LD_A_DE, 1)),
        [0x3a, hi, lo, ..] => {
            let nn = ((*hi as u16) << 8) | *lo as u16;
            Some((Instr::LD_A_nn(nn), 3))
        }
        [0x02, ..] => Some((Instr::LD_BC_A, 1)),
        [0x12, ..] => Some((Instr::LD_DE_A, 1)),
        [0x32, hi, lo, ..] => {
            let nn = ((*hi as u16) << 8) | *lo as u16;
            Some((Instr::LD_nn_A(nn), 3))
        }
        [0xed, x, ..] => match *x {
            0x57 => Some((Instr::LD_A_I, 2)),
            0x5f => Some((Instr::LD_A_R, 2)),
            0x47 => Some((Instr::LD_I_A, 2)),
            0x4f => Some((Instr::LD_R_A, 2)),
            _ => None,
        },
        _ => options!(load_r_r(mem), load_r_n(mem), load_r_hl(mem)),
    }
}

fn load_r_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & TOP_TWO != 0b01000000 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    let r1 = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::LD_r_r(r, r1), 1))
}

fn load_r_n(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | LOW_THREE) != 0b00000110 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    Some((Instr::LD_r_n(r, mem[1]), 2))
}

fn load_r_hl(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | LOW_THREE) != 0b01000110 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    Some((Instr::LD_r_HL(r), 1))
}

fn load_r_ix(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | LOW_THREE) != 0b01000110 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    Some((Instr::LD_r_IX(r, mem[1] as i8), 3))
}

fn load_r_iy(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | LOW_THREE) != 0b01000110 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    Some((Instr::LD_r_IY(r, mem[1] as i8), 3))
}

fn load_ix_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b01110000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::LD_IX_r(mem[1] as i8, r), 3))
}

fn load_iy_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b01110000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::LD_IY_r(mem[1] as i8, r), 3))
}
