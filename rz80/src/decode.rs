//! Methods, macros, and helper functions for decoding Z80 instructions.

mod exchange;
mod jump;
mod load8;

use super::{insts::Instr, Condition, Register, Z80};
use exchange::exchange;
use jump::jump;
use load8::load8;

/// Returns a [`Register`] if the provided three-bit value maps to a register name.
///
/// # Arguments
/// - `bits`: the bits to convert
#[inline]
fn bits_to_reg(bits: u8) -> Option<Register> {
    match bits {
        0b111 => Some(Register::A),
        0b000 => Some(Register::B),
        0b001 => Some(Register::C),
        0b010 => Some(Register::D),
        0b011 => Some(Register::E),
        0b100 => Some(Register::H),
        0b101 => Some(Register::L),
        _ => None,
    }
}

#[inline]
fn bits_to_condition(bits: u8) -> Option<Condition> {
    match bits {
        0b000 => Some(Condition::NZ),
        0b001 => Some(Condition::Z),
        0b010 => Some(Condition::NC),
        0b011 => Some(Condition::C),
        0b100 => Some(Condition::PO),
        0b101 => Some(Condition::PE),
        0b110 => Some(Condition::P),
        0b111 => Some(Condition::M),
        _ => None,
    }
}

/// Try a sequence of provided [`Option`] expressions in order and return the first [`Some`]
/// value it finds, or [`None`] if none of the expressions succeed.
///
/// # Examples
/// ```
/// # #[macro_use(options)] extern crate rz80;
/// # fn main() {
/// let x = options!(Some(1), None::<u8>);
/// assert_eq!(Some(1), x);
/// let y = options!(None::<u8>, None::<u8>, Some(2), Some(3));
/// assert_eq!(Some(2), y);
/// # }
/// ```
///
/// The result can be `unwrapped`, which will panic in the normal way:
/// ```
/// # #[macro_use(options)] extern crate rz80;
/// # fn main() {
/// let x = options!(Some(1), None::<u8>; unwrap);
/// assert_eq!(1, x);
/// # }
///
#[macro_export]
macro_rules! options {
    ($opt:expr) => {
        $opt
    };

    ($opt:expr, $($opts:expr),+) => {
        {
            let x = $opt;
            match x {
                Some(_) => x,
                None => options!($($opts),+)
            }
        }
    };

    ($($opts:expr),+; unwrap) => {
        options!($($opts),+).unwrap()
    }
}

#[cfg(test)]
mod options_tests {
    use rstest::*;

    #[rstest]
    fn test_single_option() {
        let result: Option<u8> = options!(Some(1));
        assert_eq!(Some(1), result);
    }

    #[rstest]
    fn test_several_option() {
        let result: Option<u8> = options!(None::<u8>, None::<u8>, Some(2));
        assert_eq!(Some(2), result);
    }
}

/// If decoding succeeds, returns both the instruction and the number of bytes read.
type DecodeResult = Option<(Instr, u8)>;

const TOP_TWO: u8 = 0b11000000;
const MID_THREE: u8 = 0b00111000;
const LOW_THREE: u8 = 0b00000111;

impl Z80 {
    /// Attempts to decode an instruction that begins at the start of the provided slice.
    ///
    /// # Arguments
    /// - `memory`: slice containing the instruction to decode
    pub fn decode(&self, memory: &[u8]) -> DecodeResult {
        options!(load8(memory), exchange(memory), jump(memory))
    }
}
