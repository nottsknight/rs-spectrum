pub mod carry_borrow;
pub mod z80;
use z80::Z80;

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
