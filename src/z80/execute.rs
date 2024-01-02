use super::{Z80, insts::Instr};
mod load8;

impl Z80 {
    /// Execute a single instruction.
    /// 
    /// # Arguments
    /// - `inst`: the instruction to execute
    /// - `mem`: slice representing the entire memory
    pub fn execute(&mut self, inst: Instr, mem: &mut [u8]) {
        match inst {
            // 8-bit load
            Instr::LD_r_r(r, r1) => load8::load_r_r(self, r, r1),
            Instr::LD_r_n(r, n) => load8::load_r_n(self, r, n),
            Instr::LD_r_HL(r) => load8::load_r_hl(self, r, mem),
            Instr::LD_r_IX(r, d) => load8::load_r_ix(self, r, d, mem),
            Instr::LD_r_IY(r, d) => load8::load_r_iy(self, r, d, mem),
            Instr::LD_HL_r(r) => load8::load_hl_r(self, r, mem),
            Instr::LD_IX_r(d, r) => load8::load_ix_r(self, d, r, mem),
            Instr::LD_IY_r(d, r) => load8::load_iy_r(self, d, r, mem),
            Instr::LD_HL_n(n) => load8::load_hl_n(self, n, mem),
            Instr::LD_IX_n(d, n) => load8::load_ix_n(self, d, n, mem),
            Instr::LD_IY_n(d, n) => load8::load_iy_n(self, d, n, mem),
            Instr::LD_A_BC => load8::load_a_bc(self, mem),
            Instr::LD_A_DE => load8::load_a_de(self, mem),
            Instr::LD_A_nn(nn) => load8::load_a_nn(self, nn, mem),
            Instr::LD_BC_A => load8::load_bc_a(self, mem),
            Instr::LD_DE_A => load8::load_de_a(self, mem),
            Instr::LD_nn_A(nn) => load8::load_nn_a(self, nn, mem),
            Instr::LD_A_I => load8::load_a_i(self),
            Instr::LD_A_R => load8::load_a_r(self),
            Instr::LD_I_A => load8::load_i_a(self),
            Instr::LD_R_A => load8::load_r_a(self),
        }
    }
}