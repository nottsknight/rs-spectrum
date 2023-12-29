use super::Register;

#[derive(Clone, Copy, Debug)]
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
}