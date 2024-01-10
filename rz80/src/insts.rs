//! Defines constants for representing Z80 instructions with their arguments.
use super::{Register, Condition};

pub type Instr = Instruction;

/// A single Z80 instruction with its arguments.
/// 
/// Instructions implement `Copy` so there is no need to worry about passing references.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    // 8-Bit Load
    /// `LD r, r`
    LD_r_r(Register, Register),
    /// `LD r, n`
    LD_r_n(Register, u8),
    /// `LD r, (HL)`
    LD_r_HL(Register),
    /// `LD r, (IX+d)`
    LD_r_IX(Register, i8),
    /// `LD r, (IX+y)`
    LD_r_IY(Register, i8),
    /// `LD (HL), r)`
    LD_HL_r(Register),
    /// `LD (IX+d), r`
    LD_IX_r(i8, Register),
    /// `LD (IY+d), r`
    LD_IY_r(i8, Register),
    /// `LD (HL), n`
    LD_HL_n(u8),
    /// `LD (IX+d), n`
    LD_IX_n(i8, u8),
    /// `LD (IY+d), n`
    LD_IY_n(i8, u8),
    /// `LD A, (BC)`
    LD_A_BC,
    /// `LD A, (DE)`
    LD_A_DE,
    /// `LD A, nn`
    LD_A_nn(u16),
    /// `LD (BC), A`
    LD_BC_A,
    /// `LD (DE), A`
    LD_DE_A,
    /// `LD nn, A`
    LD_nn_A(u16),
    /// `LD A, I`
    LD_A_I,
    /// `LD A, R`
    LD_A_R,
    /// `LD I, A`
    LD_I_A,
    /// `LD R, A`
    LD_R_A,
    // Exchange and Transfer
    /// `EX DE, HL`
    EX_DE_HL,
    /// `EX AF, AF'`
    EX_AF_AF1,
    /// `EXX`
    EXX,
    /// `EX (SP), HL`
    EX_SP_HL,
    /// `EX (SP), IX`
    EX_SP_IX,
    /// `EX (SP), IY`
    EX_SP_IY,
    /// `LDI`
    LDI,
    /// `LDIR`
    LDIR,
    /// `LDD`
    LDD,
    /// `LDDR`
    LDDR,
    /// `CPI`
    CPI,
    /// `CPIR`
    CPIR,
    /// `CPD`
    CPD,
    /// `CPDR`
    CPDR,
    // 8-bit Arithmetic
    // TODO: Implement method of representing 's' param
    /// `ADD A, r`
    ADD_A_r(Register),
    /// `ADD A, n`
    ADD_A_n(u8),
    /// `ADD A, (HL)`
    ADD_A_HL,
    /// `ADD A, (IX+d)`
    ADD_A_IX(i8),
    /// `ADD A, (IY+d)`
    ADD_A_IY(i8),
    /// `ADC A, r`
    ADC_A_r(Register),
    /// `ADC A, n`
    ADC_A_n(u8),
    /// `ADC A, (HL)`
    ADC_A_HL,
    /// `ADC A, (IX+d)`
    ADC_A_IX(i8),
    /// `ADC A, (IY+d)`
    ADC_A_IY(i8),
    /// `SUB A, r`
    SUB_A_r(Register),
    /// `SUB A, n` 
    SUB_A_n(u8),
    /// `SUB A, (HL)`
    SUB_A_HL,
    /// `SUB A, (IX+d)`
    SUB_A_IX(i8),
    /// `SUB A, (IY+d)`
    SUB_A_IY(i8),
    /// `SBC A, r`
    SBC_A_r(Register),
    /// `SBC A, n`
    SBC_A_n(u8),
    /// `SBC A, (HL)`
    SBC_A_HL,
    /// `SBC A, (IX+d)`
    SBC_A_IX(i8),
    /// `SBC A, (IY+d)`
    SBC_A_IY(i8),
    /// `AND A, r`
    AND_A_r(Register),
    /// `AND A, n`
    AND_A_n(u8),
    /// `AND A, (HL)`
    AND_A_HL,
    /// `AND A, (IX+d)`
    AND_A_IX(i8),
    /// `AND A, (IY+d)`
    AND_A_IY(i8),
    /// `OR A, r`
    OR_A_r(Register),
    /// `OR A, n`
    OR_A_n(u8),
    /// `OR A, (HL)`
    OR_A_HL,
    /// `OR A, (IX+d)`
    OR_A_IX(i8),
    /// `OR A, (IY+d)`
    OR_A_IY(i8),
    /// `XOR A, r`
    XOR_A_r(Register),
    /// `XOR A, n`
    XOR_A_n(u8),
    /// `XOR A, (HL)`
    XOR_A_HL,
    /// `XOR A, (IX+d)`
    XOR_A_IX(i8),
    /// `XOR A, (IY+d)`
    XOR_A_IY(i8),
    /// `CP r`
    CP_r(Register),
    /// `CP n`
    CP_n(u8),
    /// `CP (HL)`
    CP_HL,
    /// `CP (IX+d)`
    CP_IX(i8),
    /// `CP (IY+d)`
    CP_IY(i8),
    /// `INC r`
    INC_r(Register),
    /// `INC (HL)`
    INC_HL,
    /// `INC (IX+d)`
    INC_IX(i8),
    /// `INC (IY+d)`
    INC_IY(i8),
    /// `DEC r`
    DEC_r(Register),
    /// `DEC (HL)`
    DEC_HL,
    /// `DEC (IX+d)`
    DEC_IX(i8),
    /// `DEC (IY+d)`
    DEC_IY(i8),
    // Jump
    /// `JP nn`
    JP_nn(u16),
    /// `JP cc, nn`
    JP_cc_nn(Condition, u16),
    /// `JR e`
    JR_e(i8),
    /// `JR C, e`
    JR_C_e(i8),
    /// `JR NC, e`
    JR_NC_e(i8),
    /// `JR Z, e`
    JR_Z_e(i8),
    /// `JR NZ, e`
    JR_NZ_e(i8),
    /// `JP (HL` 
    JP_HL,
    /// `JP (IX)`
    JP_IX,
    /// `JP (IY)`
    JP_IY,
    /// `DJNZ e`
    DJNZ_e(i8)
}