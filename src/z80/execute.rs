use super::{insts::Instr, Z80};
use crate::time_limit;
use std::time::{Duration, Instant};
mod exchange;
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
            Instr::LD_r_r(r, r1) => {
                time_limit!(load8::load_r_r(self, r, r1); Duration::from_nanos(1000))
            }
            Instr::LD_r_n(r, n) => {
                time_limit!(load8::load_r_n(self, r, n); Duration::from_nanos(1750))
            }
            Instr::LD_r_HL(r) => {
                time_limit!(load8::load_r_hl(self, r, mem); Duration::from_nanos(1750))
            }
            Instr::LD_r_IX(r, d) => {
                time_limit!(load8::load_r_ix(self, r, d, mem); Duration::from_nanos(4750))
            }
            Instr::LD_r_IY(r, d) => {
                time_limit!(load8::load_r_iy(self, r, d, mem); Duration::from_nanos(4750))
            }
            Instr::LD_HL_r(r) => {
                time_limit!(load8::load_hl_r(self, r, mem); Duration::from_nanos(1750))
            }
            Instr::LD_IX_r(d, r) => {
                time_limit!(load8::load_ix_r(self, d, r, mem); Duration::from_nanos(4750))
            }
            Instr::LD_IY_r(d, r) => {
                time_limit!(load8::load_iy_r(self, d, r, mem); Duration::from_nanos(4750))
            }
            Instr::LD_HL_n(n) => {
                time_limit!(load8::load_hl_n(self, n, mem); Duration::from_nanos(2500))
            }
            Instr::LD_IX_n(d, n) => time_limit!(
                load8::load_ix_n(self, d, n, mem);
                Duration::from_nanos(4750)
            ),
            Instr::LD_IY_n(d, n) => {
                time_limit!(load8::load_iy_n(self, d, n, mem); Duration::from_nanos(2500))
            }
            Instr::LD_A_BC => time_limit!(load8::load_a_bc(self, mem); Duration::from_nanos(1750)),
            Instr::LD_A_DE => time_limit!(load8::load_a_de(self, mem); Duration::from_nanos(1750)),
            Instr::LD_A_nn(nn) => {
                time_limit!(load8::load_a_nn(self, nn, mem); Duration::from_nanos(3250))
            }
            Instr::LD_BC_A => time_limit!(load8::load_bc_a(self, mem); Duration::from_nanos(1750)),
            Instr::LD_DE_A => time_limit!(load8::load_de_a(self, mem); Duration::from_nanos(1750)),
            Instr::LD_nn_A(nn) => {
                time_limit!(load8::load_nn_a(self, nn, mem); Duration::from_nanos(3250))
            }
            Instr::LD_A_I => time_limit!(load8::load_a_i(self); Duration::from_nanos(2250)),
            Instr::LD_A_R => time_limit!(load8::load_a_r(self); Duration::from_nanos(2250)),
            Instr::LD_I_A => time_limit!(load8::load_i_a(self); Duration::from_nanos(2250)),
            Instr::LD_R_A => time_limit!(load8::load_r_a(self); Duration::from_nanos(2250)),
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
