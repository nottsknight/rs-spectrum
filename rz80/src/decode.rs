//! Methods, macros, and helper functions for decoding Z80 instructions.
mod arith8;
mod exchange;
mod jump;
mod load8;

use super::{Instruction, Condition, Register, Z80};
use arith8::arith8;
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

/// Returns a [`Condition`] if the provided three-bit value maps to a condition.
/// 
/// # Arguments
/// - `bits`: the bits to convert
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
/// let x: Option<u8> = options!(Some(1), None);
/// assert_eq!(Some(1), x);
/// let y: Option<u8> = options!(None, None, Some(2), Some(3));
/// assert_eq!(Some(2), y);
/// # }
/// ```
///
/// The result can be unwrapped:
/// ```
/// # #[macro_use(options)] extern crate rz80;
/// # fn main() {
/// let x: u8 = options!(unwrap Some(1), None);
/// assert_eq!(1, x);
/// # }
/// ```
/// 
/// Unwrapping will panic in the normal way:
/// ```should_panic
/// # #[macro_use(options)] extern crate rz80;
/// # fn main() {
/// let x: u8 = options!(unwrap None, None);
/// # }
/// ```
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

    (unwrap $($opts:expr),+) => {
        options!($($opts),+).unwrap()
    }
}

/// If decoding succeeds, returns both the instruction and the number of bytes read.
type DecodeResult = Option<(Instruction, u8)>;

/// Bit mask `11000000`.
const TOP_TWO: u8 = 0b11000000;
/// Bit mask `00111000`.
const MID_THREE: u8 = 0b00111000;
/// Bit mask `00000111`.
const LOW_THREE: u8 = 0b00000111;

impl Z80 {
    /// Attempts to decode an instruction that begins at the start of the provided slice.
    ///
    /// # Arguments
    /// - `memory`: slice containing the instruction to decode
    pub fn decode(&self, memory: &[u8]) -> DecodeResult {
        options!(load8(memory), exchange(memory), jump(memory), arith8(memory))
    }
}
