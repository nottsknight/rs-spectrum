//! Provides an emulated ZX Spectrum.
use rz80::Z80;

const MEM_SIZE: usize = (16 + 48) * 1024;

/// Struct representing a complete ZX Spectrum.
pub struct RSSpectrum {
    /// Zilog Z80 CPU
    cpu: Z80,
    /// Complete 64KB memory
    memory: [u8; MEM_SIZE],
}

impl RSSpectrum {
    /// Construct a new Spectrum.
    pub fn new() -> RSSpectrum {
        RSSpectrum {
            cpu: Default::default(),
            memory: [0; MEM_SIZE],
        }
    }

    /// Launch the Spectrum.
    pub fn run(&mut self) -> Option<()> {
        self.cpu.run(&mut self.memory)
    }
}
