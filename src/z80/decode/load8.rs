use super::{bits_to_reg, DecodeResult, Instr, LOW_THREE, MID_THREE, TOP_TWO};

pub fn load_r_r(mem: &[u8]) -> DecodeResult {
    if mem[0] & TOP_TWO != 0b01000000 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    let r1 = bits_to_reg(mem[0] & LOW_THREE)?;
    Some((Instr::LD_r_r(r, r1), 1))
}

pub fn load_r_n(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | LOW_THREE) != 0b00000110 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    Some((Instr::LD_r_n(r, mem[1]), 2))
}

pub fn load_r_hl(mem: &[u8]) -> DecodeResult {
    if mem[0] & (TOP_TWO | LOW_THREE) != 0b01000110 {
        return None;
    }

    let r = bits_to_reg((mem[0] & MID_THREE) >> 3)?;
    Some((Instr::LD_r_HL(r), 1))
}
