use crate::z80::{Condition, Flag, Register, Z80};

#[inline]
pub fn jump_nn(cpu: &mut Z80, nn: u16) {
    cpu.prog_counter = nn;
}

#[inline]
pub fn jump_cc_nn(cpu: &mut Z80, cc: Condition, nn: u16) {
    let flag = match cc {
        Condition::Z => cpu.flag(Flag::Z),
        Condition::NZ => !cpu.flag(Flag::Z),
        Condition::C => cpu.flag(Flag::C),
        Condition::NC => !cpu.flag(Flag::C),
        Condition::PO => cpu.flag(Flag::PV),
        Condition::PE => !cpu.flag(Flag::PV),
        Condition::P => cpu.flag(Flag::S),
        Condition::M => !cpu.flag(Flag::S),
    };

    if flag {
        cpu.prog_counter = nn;
    }
}

#[inline]
pub fn jumpr_e(cpu: &mut Z80, e: i8) {
    let new_pc = (cpu.prog_counter as i32) + (e as i32);
    cpu.prog_counter = new_pc as u16;
}

#[inline]
pub fn jr_flag_e(cpu: &mut Z80, flag: Flag, e: i8) {
    if cpu.flag(flag) {
        jumpr_e(cpu, e);
    }
}

#[inline]
pub fn jr_nflag_e(cpu: &mut Z80, flag: Flag, e: i8) {
    if !cpu.flag(flag) {
        jumpr_e(cpu, e);
    }
}

#[inline]
pub fn jump_hl(cpu: &mut Z80) {
    cpu.prog_counter = cpu.hl;
}

#[inline]
pub fn jump_ix(cpu: &mut Z80) {
    cpu.prog_counter = cpu.index_x;
}

#[inline]
pub fn jump_iy(cpu: &mut Z80) {
    cpu.prog_counter = cpu.index_y;
}

#[inline]
pub fn djnz_e(cpu: &mut Z80, e: i8) {
    let b = cpu.reg(Register::B);
    cpu.set_reg(Register::B, b - 1);
    if b - 1 != 0 {
        jumpr_e(cpu, e);
    }
}
