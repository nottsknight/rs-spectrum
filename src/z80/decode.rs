use super::{Z80, insts::Instr};

impl Z80 {
    pub fn decode(&self, mem: &[u8]) -> Option<(Instr, u8)> {
        None
    }
}