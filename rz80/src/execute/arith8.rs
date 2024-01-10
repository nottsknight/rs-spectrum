use crate::{carry_borrow::AddCarry, Flag, Register, Z80};

#[inline]
pub fn add_a_r(cpu: &mut Z80, r: Register) {
    let a = cpu.reg(Register::A) as u8;
    let rval = cpu.reg(r) as u8;
    let (sum, carry3, carry7) = a.add_carry(rval);

    cpu.set_reg(Register::A, sum as u16);
    cpu.set_flag(Flag::S, (sum as i8) < 0);
    cpu.set_flag(Flag::Z, sum == 0);
    cpu.set_flag(Flag::H, carry3);
    todo!("Set PV flag");
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::C, carry7);
}

#[inline]
pub fn inc_r(cpu: &mut Z80, r: Register) {
    let (val, carry3, _) = (cpu.reg(r) as u8).add_carry(1);
    cpu.set_reg(r, val as u16);
    cpu.set_flag(Flag::S, (val as i8) < 0);
    cpu.set_flag(Flag::Z, val == 0);
    cpu.set_flag(Flag::H, carry3);
    cpu.set_flag(Flag::PV, val - 1 == 0x7f);
    cpu.set_flag(Flag::N, false);
}

#[inline]
pub fn inc_hl(cpu: &mut Z80, memory: &mut [u8]) {
    let idx = cpu.reg(Register::HL) as usize;
    let (val, carry3, _) = memory[idx].add_carry(1);
    memory[idx] = val;
    cpu.set_flag(Flag::S, (val as i8) < 0);
    cpu.set_flag(Flag::Z, val == 0);
    cpu.set_flag(Flag::H, carry3);
    cpu.set_flag(Flag::PV, val - 1 == 0x7f);
    cpu.set_flag(Flag::N, false);
}
