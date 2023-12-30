//! This module defines values for emulating the Zilog Z80 CPU.
use std::{thread, time};

const CLOCK_SPEED: time::Duration = time::Duration::from_nanos(1_000_000_000 / 4_000);

macro_rules! upper {
    ($x:expr) => {
        ($x & 0xff00) >> 8
    };
}

macro_rules! set_upper {
    ($x:expr, $n:expr) => {
        $x = ($x & 0x00ff) | ($n << 8)
    };
}

macro_rules! lower {
    ($x:expr) => {
        $x & 0x00ff
    };
}

macro_rules! set_lower {
    ($x:expr, $n:expr) => {
        $x = ($x & 0xff00) | $n
    };
}

#[cfg(test)]
mod upper_lower_tests {
    use rstest::*;

    #[rstest]
    pub fn test_upper() {
        assert_eq!(0xab, upper!(0xabcd));
    }

    #[rstest]
    pub fn test_set_upper() {
        let mut n = 0xabcd;
        set_upper!(n, 0x12);
        assert_eq!(0x12cd, n);
    }

    #[rstest]
    pub fn test_lower() {
        assert_eq!(0xcd, lower!(0xabcd));
    }

    #[rstest]
    pub fn test_set_lower() {
        let mut n = 0xabcd;
        set_lower!(n, 0x01);
        assert_eq!(n, 0xab01);
    }
}

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

    fn fetch<'a>(&self, memory: &'a [u8]) -> &'a [u8] {
        &memory[self.prog_counter as usize..]
    }

    pub fn run(&mut self, mem: &mut [u8]) -> Option<()> {
        loop {
            let m = self.fetch(mem);
            let (inst, width) = self.decode(m)?;
            self.execute(inst);
            self.prog_counter += width as u16;
            thread::sleep(CLOCK_SPEED);
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
