pub mod carry_borrow;
pub mod z80;
use z80::Z80;

/// Constrain a function to run for a minimum length of time.
///
/// This macro relies on [`std::time::Instant`] for timing and expects a [`std::time::Duration`]
/// to indicate the duration to last. Both these structs should be used in every module
/// calling this macro.
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
/// time_limit!(foo(2, 3); Duration::from_nanos(1000));
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
/// let sum = time_limit!(foo(2, 3); Duration::from_nanos(1000));
/// assert_eq!(5, sum);
/// # }
/// ```
#[macro_export]
macro_rules! time_limit {
    ($fun:expr; $dur:expr) => {{
        let t0 = Instant::now();
        let x = $fun;
        while t0.elapsed() < $dur {
            // pass
        }
        x
    }};
}

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

    pub fn run(&mut self) -> Option<()> {
        self.cpu.run(&mut self.memory)
    }
}
