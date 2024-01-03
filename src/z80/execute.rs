use super::{Z80, insts::Instr};
mod load8;
mod exchange;

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
            // Exchange, Swap, Search
            Instr::EX_DE_HL => exchange::exchange_de_hl(self),
            Instr::EX_AF_AF1 => exchange::exchange_af_af1(self),
            Instr::EXX => exchange::exchange_exx(self),
            Instr::EX_SP_HL => exchange::exchange_sp_hl(self, mem),
            Instr::EX_SP_IX => exchange::exchange_sp_ix(self, mem),
            Instr::EX_SP_IY => exchange::exchange_sp_iy(self, mem),
            Instr::LDI => exchange::exchange_ldi(self, mem),
            Instr::LDIR => exchange::exchange_ldir(self, mem),
            Instr::LDD => exchange::exchange_ldd(self, mem),
            Instr::LDDR => exchange::exchange_lddr(self, mem),
            Instr::CPI => exchange::exchange_cpi(self, mem),
            Instr::CPIR => exchange::exchange_cpir(self, mem),
            Instr::CPD => exchange::exchange_cpd(self, mem),
            Instr::CPDR => exchange::exchange_cpdr(self, mem),
        }
    }
}