//! Defines constants for representing Z80 instructions with their arguments.
use super::Register;

/// A single Z80 instruction with its arguments.
/// 
/// Instructions implement `Copy` so there is no need to worry about passing references.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Instr {
    // 8-Bit Load
    LD_r_r(Register, Register),
    LD_r_n(Register, u8),
    LD_r_HL(Register),
    LD_r_IX(Register, i8),
    LD_r_IY(Register, i8),
    LD_HL_r(Register),
    LD_IX_r(i8, Register),
    LD_IY_r(i8, Register),
    LD_HL_n(u8),
    LD_IX_n(i8, u8),
    LD_IY_n(i8, u8),
    LD_A_BC,
    LD_A_DE,
    LD_A_nn(u16),
    LD_BC_A,
    LD_DE_A,
    LD_nn_A(u16),
    LD_A_I,
    LD_A_R,
    LD_I_A,
    LD_R_A,
    // Exchange and Transfer
    EX_DE_HL,
    EX_AF_AF1,
    EXX,
    EX_SP_HL,
    EX_SP_IX,
    EX_SP_IY,
    LDI,
    LDIR,
    LDD,
    LDDR,
    CPI,
    CPIR,
    CPD,
    CPDR,
    // 8-bit Arithmetic
    // TODO: Implement method of representing 's' param
    ADD_A_r(Register),
    ADD_A_n(u8),
    ADD_A_HL,
    ADD_A_IX(i8),
    ADD_A_IY(i8),
    ADC_A_r(Register),
    ADC_A_n(u8),
    ADC_A_HL,
    ADC_A_IX(i8),
    ADC_A_IY(i8),
    SUB_s,
    SBC_A_s,
    AND_s,
    OR_s,
    XOR_s,
    CP_s,
    INC_r(Register),
    INC_HL,
    INC_IX(i8),
    INC_IY(i8),
    DEC_m
}