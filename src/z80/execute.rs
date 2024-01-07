use super::{insts::Instr, Flag, Z80};
use crate::min_time;
use std::time::{Duration, Instant};
mod exchange;
mod jump;
mod load8;

/// Constrain a function to run for a minimum number of nanoseconds.
///
/// This macro requires [`std::time::Duration`] and [`std::time::Instant`] which
/// should both be used in every module using this macro.
///
/// # Examples
/// These examples use an imagined function `foo` which adds two `u8`s.
///
/// Basic example forcing a call to `foo` to last at least 1000ns:
/// ```
/// # #[macro_use] extern crate spectrum;
/// # use std::time::{Duration, Instant};
/// #
/// # fn foo(x: u8, y: u8) -> u8 { x + y }
/// # fn main() {
/// min_time!(foo(2, 3); 1000);
/// # }
/// ```
///
/// The time limited function can still return its normal return value:
/// ```
/// # #[macro_use] extern crate spectrum;
/// # use std::time::{Duration, Instant};
/// #
/// # fn foo(x: u8, y: u8) -> u8 { x + y }
/// # fn main() {
/// let sum = min_time!(foo(2, 3); 1000);
/// assert_eq!(5, sum);
/// # }
/// ```
#[macro_export]
macro_rules! min_time {
    ($fun:expr; $dur:expr) => {{
        let d = Duration::from_nanos($dur);
        let t0 = Instant::now();
        let x = $fun;
        while t0.elapsed() < d {
            // pass
        }
        x
    }};
}

impl Z80 {
    /// Execute a single instruction.
    ///
    /// # Arguments
    /// - `instr`: the instruction to execute
    /// - `memory`: slice representing the entire memory
    pub fn execute(&mut self, instr: Instr, memory: &mut [u8]) {
        match instr {
            // 8-bit load
            Instr::LD_r_r(r, r1) => min_time!(load8::load_r_r(self, r, r1); 1000),
            Instr::LD_r_n(r, n) => min_time!(load8::load_r_n(self, r, n); 1750),
            Instr::LD_r_HL(r) => min_time!(load8::load_r_hl(self, r, memory); 1750),
            Instr::LD_r_IX(r, d) => min_time!(load8::load_r_ix(self, r, d, memory); 4750),
            Instr::LD_r_IY(r, d) => min_time!(load8::load_r_iy(self, r, d, memory); 4750),
            Instr::LD_HL_r(r) => min_time!(load8::load_hl_r(self, r, memory); 1750),
            Instr::LD_IX_r(d, r) => min_time!(load8::load_ix_r(self, d, r, memory); 4750),
            Instr::LD_IY_r(d, r) => min_time!(load8::load_iy_r(self, d, r, memory); 4750),
            Instr::LD_HL_n(n) => min_time!(load8::load_hl_n(self, n, memory); 2500),
            Instr::LD_IX_n(d, n) => min_time!(load8::load_ix_n(self, d, n, memory); 4750),
            Instr::LD_IY_n(d, n) => min_time!(load8::load_iy_n(self, d, n, memory); 2500),
            Instr::LD_A_BC => min_time!(load8::load_a_bc(self, memory); 1750),
            Instr::LD_A_DE => min_time!(load8::load_a_de(self, memory); 1750),
            Instr::LD_A_nn(nn) => min_time!(load8::load_a_nn(self, nn, memory); 3250),
            Instr::LD_BC_A => min_time!(load8::load_bc_a(self, memory); 1750),
            Instr::LD_DE_A => min_time!(load8::load_de_a(self, memory); 1750),
            Instr::LD_nn_A(nn) => min_time!(load8::load_nn_a(self, nn, memory); 3250),
            Instr::LD_A_I => min_time!(load8::load_a_i(self); 2250),
            Instr::LD_A_R => min_time!(load8::load_a_r(self); 2250),
            Instr::LD_I_A => min_time!(load8::load_i_a(self); 2250),
            Instr::LD_R_A => min_time!(load8::load_r_a(self); 2250),
            // Exchange, Swap, Search
            Instr::EX_DE_HL => min_time!(exchange::exchange_de_hl(self); 1000),
            Instr::EX_AF_AF1 => min_time!(exchange::exchange_af_af1(self); 1000),
            Instr::EXX => min_time!(exchange::exchange_exx(self); 1000),
            Instr::EX_SP_HL => min_time!(exchange::exchange_sp_hl(self, memory); 4750),
            Instr::EX_SP_IX => min_time!(exchange::exchange_sp_ix(self, memory); 5750),
            Instr::EX_SP_IY => min_time!(exchange::exchange_sp_iy(self, memory); 5750),
            Instr::LDI => min_time!(exchange::exchange_ldi(self, memory); 4000),
            Instr::LDIR => {
                if self.bc != 0 {
                    min_time!(exchange::exchange_ldir(self, memory); 5250)
                } else {
                    min_time!(exchange::exchange_ldir(self, memory); 4000)
                }
            }
            Instr::LDD => min_time!(exchange::exchange_ldd(self, memory); 4000),
            Instr::LDDR => {
                if self.bc != 0 {
                    min_time!(exchange::exchange_lddr(self, memory); 5250)
                } else {
                    min_time!(exchange::exchange_lddr(self, memory); 4000)
                }
            }
            Instr::CPI => min_time!(exchange::exchange_cpi(self, memory); 4000),
            Instr::CPIR => {
                if self.bc != 0 {
                    min_time!(exchange::exchange_cpir(self, memory); 5250)
                } else {
                    min_time!(exchange::exchange_cpir(self, memory); 4000)
                }
            }
            Instr::CPD => min_time!(exchange::exchange_cpd(self, memory); 4000),
            Instr::CPDR => {
                if self.bc != 0 {
                    min_time!(exchange::exchange_cpdr(self, memory); 5250)
                } else {
                    min_time!(exchange::exchange_cpdr(self, memory); 4000)
                }
            }
            // Jump
            Instr::JP_nn(nn) => min_time!(jump::jump_nn(self, nn); 2500),
            Instr::JP_cc_nn(cc, nn) => min_time!(jump::jump_cc_nn(self, cc, nn); 2500),
            Instr::JR_e(e) => min_time!(jump::jumpr_e(self, e); 3000),
            Instr::JR_C_e(e) => min_time!(jump::jr_flag_e(self, Flag::C, e); 3000),
            Instr::JR_NC_e(e) => min_time!(jump::jr_nflag_e(self, Flag::C, e); 3000),
            Instr::JR_Z_e(e) => min_time!(jump::jr_flag_e(self, Flag::Z, e); 3000),
            Instr::JR_NZ_e(e) => min_time!(jump::jr_nflag_e(self, Flag::Z, e); 3000),
            Instr::JP_HL => min_time!(jump::jump_hl(self); 1000),
            Instr::JP_IX => min_time!(jump::jump_ix(self); 1000),
            Instr::JP_IY => min_time!(jump::jump_iy(self); 1000),
            Instr::DJNZ_e(e) => min_time!(jump::djnz_e(self, e); 3250),
        }
    }
}
