//! This module defines values for emulating the Zilog Z80 CPU.
mod decode;
mod execute;
mod insts;

pub use insts::{Instruction, Instr};
use std::{thread, time};

const CLOCK_SPEED: time::Duration = time::Duration::from_nanos(1_000_000_000 / 4_000);

/// Get or set the upper 8 bits of a 16-bit value.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate spectrum;
/// # fn main() {
/// let mut x = 0xabcd;
/// assert_eq!(0xab, upper!(get x));
/// upper!(set x => 0xef);
/// assert_eq!(0xef, upper!(get x));
/// # }
/// ```
#[macro_export]
macro_rules! upper {
    (get $x:expr) => {
        ($x & 0xff00) >> 8
    };

    (set $x:expr => $y:expr) => {
        $x = ($x & 0x00ff) | (($y & 0x00ff) << 8)
    };
}

/// Get or set the lower 8 bits of a 16-bit value.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate spectrum;
/// # fn main() {
/// let mut x = 0xabcd;
/// assert_eq!(0xcd, lower!(get x));
/// lower!(set x => 0xef);
/// assert_eq!(0xef, lower!(get x));
/// # }
#[macro_export]
macro_rules! lower {
    (get $x:expr) => {
        $x & 0x00ff
    };

    (set $x:expr => $y:expr) => {
        $x = ($x & 0xff00) | ($y & 0x00ff)
    };
}

/// Emulated Z80 CPU
#[derive(Default)]
pub struct Z80 {
    /// Main AF register pair
    pub af: u16,
    /// Alternate AF register pair
    pub af1: u16,
    /// Main BC register pair
    pub bc: u16,
    /// Alternate BC register pair
    pub bc1: u16,
    /// Main DE register pair
    pub de: u16,
    /// Alternate DE register pair
    pub de1: u16,
    /// Main HL register pair
    pub hl: u16,
    /// Alternate HL register pair
    pub hl1: u16,
    /// Index register X
    pub index_x: u16,
    /// Index register Y
    pub index_y: u16,
    /// Stack pointer
    pub stack_ptr: u16,
    /// Interrupt vector
    pub interrupt: u8,
    /// Memory refresh
    pub refresh: u8,
    /// Program counter
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
            Register::A => upper!(get self.af),
            Register::F => lower!(get self.af),
            Register::B => upper!(get self.bc),
            Register::C => lower!(get self.bc),
            Register::BC => self.bc,
            Register::D => upper!(get self.de),
            Register::E => lower!(get self.de),
            Register::DE => self.de,
            Register::H => upper!(get self.hl),
            Register::L => lower!(get self.hl),
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
            Register::A => upper!(set self.af => val),
            Register::F => lower!(set self.af => val),
            Register::B => upper!(set self.bc => val),
            Register::C => lower!(set self.bc => val),
            Register::BC => self.bc = val,
            Register::D => upper!(set self.de => val),
            Register::E => lower!(set self.de => val),
            Register::DE => self.de = val,
            Register::H => upper!(set self.hl => val),
            Register::L => lower!(set self.hl => val),
            Register::HL => self.hl = val,
        }
    }

    /// Return whether the given status flag is set or not.
    ///
    /// # Argument
    /// - `f`: flag to check
    pub fn flag(&self, f: Flag) -> bool {
        (lower!(get self.af) & (f as u16)) != 0
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
        let old = lower!(get self.af);
        let new = if val {
            old | mask
        } else {
            (old ^ mask) & !mask
        };
        lower!(set self.af => new);
    }

    /// Return a slice of memory beginning at the current program counter.
    ///
    /// # Arguments
    /// - `memory`: slice representing the entire memory
    pub fn fetch<'a>(&self, memory: &'a [u8]) -> &'a [u8] {
        &memory[self.prog_counter as usize..]
    }

    /// Start the cpu running the fetch-decode-execute cycle.
    ///
    /// This method loops infinitely unless an error occurs.
    ///
    /// # Arguments
    /// - `memory`: slice of the entire memory available to the CPU
    pub fn run(&mut self, memory: &mut [u8]) -> Option<()> {
        loop {
            let m = self.fetch(memory);
            let (inst, width) = self.decode(m)?;
            self.prog_counter += width as u16;
            self.execute(inst, memory);
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
    /// Accumulator register
    A,
    /// Flag register
    F,
    /// B register
    B,
    /// C register
    C,
    /// BC register pair
    BC,
    /// D register
    D,
    /// E register
    E,
    /// DE register pair
    DE,
    /// H register
    H,
    /// L register
    L,
    /// HL register pair
    HL,
}

/// Enums for identifying different status flags.
#[derive(Clone, Copy, Debug)]
pub enum Flag {
    /// Carry flag
    C = 0,
    /// Add/subtract flag
    N = 1,
    /// Parity/overflow flag
    PV = 2,
    /// Half-carry flag
    H = 4,
    /// Zero flag
    Z = 6,
    /// Sign flag
    S = 7,
}

/// Enums for identifying different jump conditions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Condition {
    /// Non-zero
    NZ,
    /// Zero
    Z,
    /// No carry
    NC,
    /// Carry
    C,
    /// Parity odd
    PO,
    /// Parity even
    PE,
    /// Sign positive
    P,
    /// Sign negative
    M,
}
