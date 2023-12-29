//! This module defines values for emulating the Zilog Z80 CPU.
use super::{lower, set_lower, set_upper, upper};

#[derive(Default)]
pub struct Z80 {
    pub af: u16,
    pub af1: u16,
    pub bc: u16,
    pub bc1: u16,
    pub de: u16,
    pub de1: u16,
    pub hl: u16,
    pub hl1: u16,
    pub index_x: u16,
    pub index_y: u16,
    pub stack_ptr: u16,
    pub interrupt: u8,
    pub refresh: u8,
    pub prog_counter: u16,
}

impl Z80 {
    pub fn reg(&self, reg: Register) -> u16 {
        match reg {
            Register::A => upper!(self.af),
            Register::F => lower!(self.af),
            Register::B => upper!(self.bc),
            Register::C => lower!(self.bc),
            Register::BC => self.bc,
            Register::D => upper!(self.de),
            Register::E => lower!(self.de),
            Register::DE => self.de,
            Register::H => upper!(self.hl),
            Register::L => lower!(self.hl),
            Register::HL => self.hl,
        }
    }

    pub fn set_reg(&mut self, reg: Register, val: u16) {
        match reg {
            Register::A => set_upper!(self.af, val),
            Register::F => set_lower!(self.af, val),
            Register::B => set_upper!(self.bc, val),
            Register::C => set_lower!(self.bc, val),
            Register::BC => self.bc = val,
            Register::D => set_upper!(self.de, val),
            Register::E => set_lower!(self.de, val),
            Register::DE => self.de = val,
            Register::H => set_upper!(self.hl, val),
            Register::L => set_lower!(self.hl, val),
            Register::HL => self.hl = val,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Register {
    A,
    F,
    B,
    C,
    BC,
    D,
    E,
    DE,
    H,
    L,
    HL,
}

mod decode;
mod execute;
mod insts;
