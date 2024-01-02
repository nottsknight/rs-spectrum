use crate::options;
use super::{Instr, DecodeResult};

pub fn exchange(mem: &[u8]) -> DecodeResult {
    options!(
        exchange_de_hl(mem)
    )
}

fn exchange_de_hl(mem: &[u8]) -> DecodeResult {
    if mem[0] != 0xeb {
        None
    } else {
        Some((Instr::EX_DE_HL, 1))
    }
}