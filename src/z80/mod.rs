//! This module defines values for emulating the Zilog Z80 CPU.
use std::{thread, time};

const CLOCK_SPEED: time::Duration = time::Duration::from_nanos(1_000_000_000 / 4_000);

#[macro_export]
macro_rules! upper {
    ($x:expr) => {
        ($x & 0xff00) >> 8
    };
}

#[macro_export]
macro_rules! set_upper {
    ($x:expr, $n:expr) => {
        $x = ($x & 0x00ff) | (($n & 0x00ff) << 8)
    };
}

#[macro_export]
macro_rules! lower {
    ($x:expr) => {
        $x & 0x00ff
    };
}

#[macro_export]
macro_rules! set_lower {
    ($x:expr, $n:expr) => {
        $x = ($x & 0xff00) | ($n & 0x00ff)
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
    /// Returns the value of the specified register.
    ///
    /// Note that if a single-width register is specified, only the lower 8 bits
    /// of the return value will be set.
    ///
    /// # Arguments
    /// - `reg`: register to get
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

    /// Sets the value of the specified register.
    ///
    /// When setting a single-width register, only the lower 8 bits of `val` will be
    /// used. The argument is `u16` to allow for setting double-width registers in the
    /// same method.
    ///
    /// # Arguments
    /// - `reg`: register to set
    /// - `val`: value to set
    ///
    /// # Example
    /// ```
    /// # use spectrum::z80::{Register, Z80};
    /// # let mut cpu: Z80 = Default::default();
    /// cpu.set_reg(Register::C, 0xabcd);
    /// assert_eq!(0xcd, cpu.reg(Register::C));
    /// ```
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

    /// Return whether the given status flag is set or not.
    /// 
    /// # Argument
    /// - `f`: flag to check
    pub fn flag(&self, f: Flag) -> bool {
        (lower!(self.af) & (f as u16)) != 0
    }

    /// Set the value of the given status flag.
    /// 
    /// # Arguments
    /// - `f`: flag to set
    /// - `val`: value to set the flag to
    /// 
    /// # Examples
    /// ```
    /// # use spectrum::z80::{Z80, Flag};
    /// # let mut z80: Z80 = Default::default();
    /// z80.set_flag(Flag::N, true);
    /// assert!(z80.flag(Flag::N)); 
    /// z80.set_flag(Flag::N, false);
    /// assert!(!z80.flag(Flag::N));
    /// ```
    pub fn set_flag(&mut self, f: Flag, val: bool) {
        let mask = f as u16;
        let old = lower!(self.af);
        let new = if val {
            old | mask
        } else {
            (old ^ mask) & !mask
        };
        set_lower!(self.af, new);
    }

    /// Return a slice of memory beginning at the current program counter.
    /// 
    /// # Arguments
    /// - `memory`: slice representing the entire memory
    fn fetch<'a>(&self, memory: &'a [u8]) -> &'a [u8] {
        &memory[self.prog_counter as usize..]
    }

    pub fn run(&mut self, mem: &mut [u8]) -> Option<()> {
        loop {
            let m = self.fetch(mem);
            let (inst, width) = self.decode(m)?;
            self.prog_counter += width as u16;
            self.execute(inst, mem);
            thread::sleep(CLOCK_SPEED);
        }
    }
}

#[cfg(test)]
mod z80_tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn z80() -> Z80 {
        Default::default()
    }

    #[rstest]
    #[case::a(Register::A, 0x01)]
    #[case::b(Register::B, 0x45)]
    #[case::c(Register::C, 0x67)]
    #[case::bc(Register::BC, 0x4567)]
    #[case::d(Register::D, 0x89)]
    #[case::e(Register::E, 0xab)]
    #[case::de(Register::DE, 0x89ab)]
    #[case::h(Register::H, 0xcd)]
    #[case::l(Register::L, 0xef)]
    #[case::hl(Register::HL, 0xcdef)]
    fn test_get_reg(mut z80: Z80, #[case] rname: Register, #[case] expected: u16) {
        z80.af = 0x0123;
        z80.bc = 0x4567;
        z80.de = 0x89ab;
        z80.hl = 0xcdef;
        let r = z80.reg(rname);
        assert_eq!(expected, r);
    }

    #[rstest]
    #[case::a(Register::A, 0x01)]
    #[case::b(Register::B, 0x45)]
    #[case::c(Register::C, 0x67)]
    #[case::bc(Register::BC, 0x4567)]
    #[case::d(Register::D, 0x89)]
    #[case::e(Register::E, 0xab)]
    #[case::de(Register::DE, 0x89ab)]
    #[case::h(Register::H, 0xcd)]
    #[case::l(Register::L, 0xef)]
    #[case::hl(Register::HL, 0xcdef)]
    fn test_set_reg(mut z80: Z80, #[case] rname: Register, #[case] val: u16) {
        z80.set_reg(rname, val);
        assert_eq!(val, z80.reg(rname));
    }
    
    #[rstest]
    fn test_fetch(mut z80: Z80) {
        z80.prog_counter = 2;
        let mem = vec![0x12, 0x34, 0x56, 0x78, 0x9a];
        let s = z80.fetch(&mem);
        assert_eq!(0x56, s[0]);
        assert_eq!(0x78, s[1]);
        assert_eq!(0x9a, s[2]);
    }
}

/// Enums for identifying specific registers in other methods.
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

/// Enums for identifying different status flags.
#[derive(Clone, Copy, Debug)]
pub enum Flag {
    C = 0,
    N = 1,
    PV = 2,
    H = 4,
    Z = 6,
    S = 7
}

mod decode;
mod execute;
mod insts;
