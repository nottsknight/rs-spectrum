mod z80;
use z80::Z80;

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

pub(crate) use {lower, set_lower, set_upper, upper};

const MEM_SIZE: usize = (16 + 48) * 1024;

pub struct RSSpectrum {
    cpu: Z80,
    memory: [u8; MEM_SIZE],
}

impl RSSpectrum {
    pub fn new() -> RSSpectrum {
        RSSpectrum {
            cpu: Default::default(),
            memory: [0; MEM_SIZE],
        }
    }
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
