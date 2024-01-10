//! Creates a binary for running an emulated ZX Spectrum written in Rust.
use rspectrum::RSSpectrum;

fn main() {
    let mut emu = RSSpectrum::new();
    println!("Hello, RS Spectrum!");
    emu.run();
}
