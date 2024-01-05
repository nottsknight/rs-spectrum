use super::{insts::Instr, Z80};
use crate::time_limit;
use std::time::{Duration, Instant};
mod exchange;
mod load8;

impl Z80 {
    /// Execute a single instruction.
    ///
    /// # Arguments
    /// - `instr`: the instruction to execute
    /// - `memory`: slice representing the entire memory
    pub fn execute(&mut self, instr: Instr, memory: &mut [u8]) {
        match instr {
            // 8-bit load
            Instr::LD_r_r(r, r1) => {
                time_limit!(load8::load_r_r(self, r, r1); Duration::from_nanos(1000))
            }
            Instr::LD_r_n(r, n) => {
                time_limit!(load8::load_r_n(self, r, n); Duration::from_nanos(1750))
            }
            Instr::LD_r_HL(r) => {
                time_limit!(load8::load_r_hl(self, r, memory); Duration::from_nanos(1750))
            }
            Instr::LD_r_IX(r, d) => {
                time_limit!(load8::load_r_ix(self, r, d, memory); Duration::from_nanos(4750))
            }
            Instr::LD_r_IY(r, d) => {
                time_limit!(load8::load_r_iy(self, r, d, memory); Duration::from_nanos(4750))
            }
            Instr::LD_HL_r(r) => {
                time_limit!(load8::load_hl_r(self, r, memory); Duration::from_nanos(1750))
            }
            Instr::LD_IX_r(d, r) => {
                time_limit!(load8::load_ix_r(self, d, r, memory); Duration::from_nanos(4750))
            }
            Instr::LD_IY_r(d, r) => {
                time_limit!(load8::load_iy_r(self, d, r, memory); Duration::from_nanos(4750))
            }
            Instr::LD_HL_n(n) => {
                time_limit!(load8::load_hl_n(self, n, memory); Duration::from_nanos(2500))
            }
            Instr::LD_IX_n(d, n) => time_limit!(
                load8::load_ix_n(self, d, n, memory);
                Duration::from_nanos(4750)
            ),
            Instr::LD_IY_n(d, n) => {
                time_limit!(load8::load_iy_n(self, d, n, memory); Duration::from_nanos(2500))
            }
            Instr::LD_A_BC => {
                time_limit!(load8::load_a_bc(self, memory); Duration::from_nanos(1750))
            }
            Instr::LD_A_DE => {
                time_limit!(load8::load_a_de(self, memory); Duration::from_nanos(1750))
            }
            Instr::LD_A_nn(nn) => {
                time_limit!(load8::load_a_nn(self, nn, memory); Duration::from_nanos(3250))
            }
            Instr::LD_BC_A => {
                time_limit!(load8::load_bc_a(self, memory); Duration::from_nanos(1750))
            }
            Instr::LD_DE_A => {
                time_limit!(load8::load_de_a(self, memory); Duration::from_nanos(1750))
            }
            Instr::LD_nn_A(nn) => {
                time_limit!(load8::load_nn_a(self, nn, memory); Duration::from_nanos(3250))
            }
            Instr::LD_A_I => time_limit!(load8::load_a_i(self); Duration::from_nanos(2250)),
            Instr::LD_A_R => time_limit!(load8::load_a_r(self); Duration::from_nanos(2250)),
            Instr::LD_I_A => time_limit!(load8::load_i_a(self); Duration::from_nanos(2250)),
            Instr::LD_R_A => time_limit!(load8::load_r_a(self); Duration::from_nanos(2250)),
            // Exchange, Swap, Search
            Instr::EX_DE_HL => {
                time_limit!(exchange::exchange_de_hl(self); Duration::from_nanos(1000))
            }
            Instr::EX_AF_AF1 => {
                time_limit!(exchange::exchange_af_af1(self); Duration::from_nanos(1000))
            }
            Instr::EXX => time_limit!(exchange::exchange_exx(self); Duration::from_nanos(1000)),
            Instr::EX_SP_HL => {
                time_limit!(exchange::exchange_sp_hl(self, memory); Duration::from_nanos(4750))
            }
            Instr::EX_SP_IX => {
                time_limit!(exchange::exchange_sp_ix(self, memory); Duration::from_nanos(5750))
            }
            Instr::EX_SP_IY => {
                time_limit!(exchange::exchange_sp_iy(self, memory); Duration::from_nanos(5750))
            }
            Instr::LDI => {
                time_limit!(exchange::exchange_ldi(self, memory); Duration::from_nanos(4000))
            }
            Instr::LDIR => {
                if self.bc != 0 {
                    time_limit!(exchange::exchange_ldir(self, memory); Duration::from_nanos(5250))
                } else {
                    time_limit!(exchange::exchange_ldir(self, memory); Duration::from_nanos(4000))
                }
            }
            Instr::LDD => {
                time_limit!(exchange::exchange_ldd(self, memory); Duration::from_nanos(4000))
            }
            Instr::LDDR => {
                if self.bc != 0 {
                    time_limit!(exchange::exchange_lddr(self, memory); Duration::from_nanos(5250))
                } else {
                    time_limit!(exchange::exchange_lddr(self, memory); Duration::from_nanos(4000))
                }
            }
            Instr::CPI => {
                time_limit!(exchange::exchange_cpi(self, memory); Duration::from_nanos(4000))
            }
            Instr::CPIR => {
                if self.bc != 0 {
                    time_limit!(exchange::exchange_cpir(self, memory); Duration::from_nanos(5250))
                } else {
                    time_limit!(exchange::exchange_cpir(self, memory); Duration::from_nanos(4000))
                }
            }
            Instr::CPD => {
                time_limit!(exchange::exchange_cpd(self, memory); Duration::from_nanos(4000))
            }
            Instr::CPDR => {
                if self.bc != 0 {
                    time_limit!(exchange::exchange_cpdr(self, memory); Duration::from_nanos(5250))
                } else {
                    time_limit!(exchange::exchange_cpdr(self, memory); Duration::from_nanos(4000))
                }
            }
            _ => todo!("Implement jump instructions"),
        }
    }
}
