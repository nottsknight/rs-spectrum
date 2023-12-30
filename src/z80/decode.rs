mod load8;
use super::{insts::Instr, Register, Z80};

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

type DecodeResult = Option<(Instr, u8)>;

const TOP_TWO: u8 = 0b11000000;
const MID_THREE: u8 = 0b00111000;
const LOW_THREE: u8 = 0b00000111;

impl Z80 {
    pub fn decode(&self, mem: &[u8]) -> DecodeResult {
        options!(
            load8::load_r_r(mem),
            load8::load_r_n(mem),
            load8::load_r_hl(mem)
        )
    }
}
