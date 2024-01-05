//! Functions for decoding 8-bit load instructions.
use super::{bits_to_reg, DecodeResult, Instr, LOW_THREE, MID_THREE, TOP_TWO};
use crate::options;

/// Attempt to decode an 8-bit load instruction from the provided memory slice.
///
/// # Arguments
/// - `mem`: slice of memory with the instruction to decode beginning at `mem[0]`
pub fn load8(mem: &[u8]) -> DecodeResult {
    match mem {
        [0xdd, 0x36, d, n, ..] => Some((Instr::LD_IX_n(*d as i8, *n), 4)),
        [0xdd, rest @ ..] => options!(load_r_ix(rest), load_ix_r(rest)),
        [0xfd, 0x36, d, n, ..] => Some((Instr::LD_IY_n(*d as i8, *n), 4)),
        [0xfd, rest @ ..] => options!(load_r_iy(rest), load_iy_r(rest)),
        [0x36, n, ..] => Some((Instr::LD_HL_n(*n), 2)),
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
        _ => options!(load_r_r(mem), load_r_n(mem), load_hl_r(mem), load_r_hl(mem)),
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

fn load_hl_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b01110000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::LD_HL_r(r), 1))
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

#[cfg(test)]
mod load8_tests {
    use super::*;
    use crate::z80::{Register, Z80};
    use rstest::*;

    #[rstest]
    #[case::ld_r_r(vec![0x42], Instr::LD_r_r(Register::B, Register::D), 1)]
    #[case::ld_r_n(vec![0x0e, 0x12], Instr::LD_r_n(Register::C, 0x12), 2)]
    #[case::ld_r_hl(vec![0x5e], Instr::LD_r_HL(Register::E), 1)]
    #[case::ld_r_ix(vec![0xdd, 0x66, 0xca], Instr::LD_r_IX(Register::H, -54), 3)]
    #[case::ld_r_iy(vec![0xfd, 0x66, 0xca], Instr::LD_r_IY(Register::H, -54), 3)]
    #[case::ld_hl_r(vec![0x75], Instr::LD_HL_r(Register::L), 1)]
    #[case::ld_ix_r(vec![0xdd, 0x77, 0x9b], Instr::LD_IX_r(-101, Register::A), 3)]
    #[case::ld_iy_r(vec![0xfd, 0x77, 0x9b], Instr::LD_IY_r(-101, Register::A), 3)]
    #[case::ld_hl_n(vec![0x36, 0x5d], Instr::LD_HL_n(0x5d), 2)]
    #[case::ld_ix_n(vec![0xdd, 0x36, 0xca, 0x1f], Instr::LD_IX_n(-54, 0x1f), 4)]
    #[case::ld_iy_n(vec![0xfd, 0x36, 0xca, 0x1f], Instr::LD_IY_n(-54, 0x1f), 4)]
    #[case::ld_a_bc(vec![0x0a], Instr::LD_A_BC, 1)]
    #[case::ld_a_de(vec![0x1a], Instr::LD_A_DE, 1)]
    #[case::ld_a_nn(vec![0x3a, 0x24, 0xa1], Instr::LD_A_nn(0x24a1), 3)]
    #[case::ld_bc_a(vec![0x02], Instr::LD_BC_A, 1)]
    #[case::ld_de_a(vec![0x12], Instr::LD_DE_A, 1)]
    #[case::ld_nn_a(vec![0x32, 0xfa, 0xce], Instr::LD_nn_A(0xface), 3)]
    #[case::ld_a_i(vec![0xed, 0x57], Instr::LD_A_I, 2)]
    #[case::ld_a_r(vec![0xed, 0x5f], Instr::LD_A_R, 2)]
    #[case::ld_i_a(vec![0xed, 0x47], Instr::LD_I_A, 2)]
    #[case::ld_i_r(vec![0xed, 0x4f], Instr::LD_R_A, 2)]
    fn test_load(#[case] mem: Vec<u8>, #[case] expected_inst: Instr, #[case] expected_width: u8) {
        let cpu = Z80::default();
        let (inst, width) = cpu.decode(&mem).unwrap();
        assert_eq!(expected_inst, inst);
        assert_eq!(expected_width, width);
    }
}
