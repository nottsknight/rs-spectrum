# RS Spectrum

![Development workflow](https://github.com/nottsknight/rs-spectrum/actions/workflows/rust-develop.yml/badge.svg)
![Main workflow](https://github.com/nottsknight/rs-spectrum/actions/workflows/rust-main.yml/badge.svg)

A ZX Spectrum emulator written in Rust. This project is divided into three
separate crates:
- `rz80` implements the emulated Zilog Z80 CPU,
- `rspectrum` implements the Spectrum itself, utilising `librz80`, and 
- `rs-spectrum` provides the binary for running the emulator, utilising both 
  `librspectrum` and `librz80`
