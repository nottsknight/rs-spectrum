//! Functions for decoding 8-bit load instructions.
use crate::options;
use super::{bits_to_reg, DecodeResult, Instr, LOW_THREE, MID_THREE, TOP_TWO};

/// Attempt to decode an 8-bit load instruction from the provided memory slice.
/// 
/// # Arguments
/// - `mem`: slice of memory with the instruction to decode beginning at `mem[0]`
pub fn load8(mem: &[u8]) -> DecodeResult {
    options!(
        load_r_r(mem),
        load_r_n(mem),
        load_r_hl(mem),
        load_r_ix(mem),
        load_r_iy(mem),
        load_hl_r(mem),
        load_ix_r(mem),
        load_iy_r(mem),
        load_hl_n(mem),
        load_ix_n(mem),
        load_iy_n(mem),
        load_a_bc(mem),
        load_a_de(mem),
        load_a_nn(mem),
        load_bc_a(mem),
        load_de_a(mem),
        load_nn_a(mem),
        load_a_i(mem),
        load_a_r(mem)
    )
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
    if mem[0] != 0xdd {
        return None;
    }

    if mem[1] & (TOP_TWO | LOW_THREE) != 0b01000110 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    Some((Instr::LD_r_IX(r, mem[2] as i8), 3))
}

fn load_r_iy(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0xfd {
        return None;
    }

    if mem[1] & (TOP_TWO | LOW_THREE) != 0b01000110 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    Some((Instr::LD_r_IY(r, mem[2] as i8), 3))
}

fn load_hl_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | MID_THREE) != 0b01110000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::LD_HL_r(r), 1))
}

fn load_ix_r(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0xdd {
        return None;
    }

    if mem[1] & (TOP_TWO | MID_THREE) != 0b01110000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::LD_IX_r(mem[2] as i8, r), 3))
}

fn load_iy_r(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0xfd {
        return None;
    }

    if mem[1] & (TOP_TWO | MID_THREE) != 0b01110000 {
        return None;
    }

    let r = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::LD_IY_r(mem[2] as i8, r), 3))
}

fn load_hl_n(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0x36 {
        return None;
    }

    return Some((Instr::LD_HL_n(mem[1]), 2));
}

fn load_ix_n(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0xdd || mem[1] != 0x36 {
        return None;
    }

    return Some((Instr::LD_IX_n(mem[2] as i8, mem[3]), 4));
}

fn load_iy_n(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0xfd || mem[1] != 0x36 {
        return None;
    }

    return Some((Instr::LD_IY_n(mem[2] as i8, mem[3]), 4));
}

fn load_a_bc(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0x0a {
        None
    } else {
        Some((Instr::LD_A_BC, 1))
    }
}

fn load_a_de(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0x1a {
        None
    } else {
        Some((Instr::LD_A_DE, 1))
    }
}

fn load_a_nn(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0x3a {
        return None;
    }

    let nn = mem[1] as u16 | ((mem[2] as u16) << 8);
    Some((Instr::LD_A_nn(nn), 3))
}

fn load_bc_a(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0x02 {
        None
    } else {
        Some((Instr::LD_BC_A, 1))
    }
}

fn load_de_a(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0x12 {
        None
    } else {
        Some((Instr::LD_DE_A, 1))
    }
}

fn load_nn_a(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0x32 {
        return None;
    }

    let nn = mem[1] as u16 | ((mem[2] as u16) << 8);
    Some((Instr::LD_nn_A(nn), 3))
}

fn load_a_i(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0xed || mem[1] != 0x57 {
        None
    } else {
        Some((Instr::LD_A_I, 1))
    }
}

fn load_a_r(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0xed || mem[1] != 0x5f {
        None
    } else {
        Some((Instr::LD_A_R, 1))
    }
}
