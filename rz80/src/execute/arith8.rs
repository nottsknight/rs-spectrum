use crate::{
    alu::{ALUAdd, ALU},
    Flag, Register, Z80,
};

#[inline]
fn add(cpu: &mut Z80, rhs: u8) {
    let ALUAdd {
        sum,
        carry3,
        carry7,
        overflow,
    } = ALU::add(cpu.reg(Register::A) as u8, rhs);

    cpu.set_reg(Register::A, sum as u16);
    cpu.set_flag(Flag::S, (sum as i8) < 0);
    cpu.set_flag(Flag::Z, sum == 0);
    cpu.set_flag(Flag::H, carry3);
    cpu.set_flag(Flag::PV, overflow);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::C, carry7);
}

#[inline]
fn add_carry(cpu: &mut Z80, rhs: u8) {
    let c = if cpu.flag(Flag::C) { 1 } else { 0 };
    let (rhs, _) = rhs.overflowing_add(c);
    add(cpu, rhs);
}

#[inline]
pub fn add_a_r(cpu: &mut Z80, r: Register, carry: bool) {
    let rhs = cpu.reg(r) as u8;
    if carry {
        add_carry(cpu, rhs);
    } else {
        add(cpu, rhs);
    }
}

#[inline]
pub fn add_a_n(cpu: &mut Z80, n: u8, carry: bool) {
    if carry {
        add_carry(cpu, n);
    } else {
        add(cpu, n);
    }
}

#[inline]
pub fn add_a_hl(cpu: &mut Z80, mem: &[u8], carry: bool) {
    let idx = cpu.reg(Register::HL) as usize;
    let rhs = mem[idx];
    if carry {
        add_carry(cpu, rhs);
    } else {
        add(cpu, rhs);
    }
}

#[inline]
pub fn add_a_ix(cpu: &mut Z80, d: i8, mem: &[u8], carry: bool) {
    let (idx, _) = cpu.index_x.overflowing_add_signed(d as i16);
    let rhs = mem[idx as usize];
    if carry {
        add_carry(cpu, rhs);
    } else {
        add(cpu, rhs);
    }
}

#[inline]
pub fn add_a_iy(cpu: &mut Z80, d: i8, mem: &[u8], carry: bool) {
    let (idx, _) = cpu.index_y.overflowing_add_signed(d as i16);
    let rhs = mem[idx as usize];
    if carry {
        add_carry(cpu, rhs);
    } else {
        add(cpu, rhs);
    }
}

#[inline]
pub fn inc_r(cpu: &mut Z80, r: Register) {
    let ALUAdd { sum, carry3, .. } = ALU::increment(cpu.reg(r) as u8);
    cpu.set_reg(r, sum as u16);
    cpu.set_flag(Flag::S, (sum as i8) < 0);
    cpu.set_flag(Flag::Z, sum == 0);
    cpu.set_flag(Flag::H, carry3);
    cpu.set_flag(Flag::PV, sum - 1 == 0x7f);
    cpu.set_flag(Flag::N, false);
}

#[inline]
pub fn inc_hl(cpu: &mut Z80, memory: &mut [u8]) {
    let idx = cpu.reg(Register::HL) as usize;
    let ALUAdd { sum, carry3, .. } = ALU::increment(memory[idx]);
    memory[idx] = sum;
    cpu.set_flag(Flag::S, (sum as i8) < 0);
    cpu.set_flag(Flag::Z, sum == 0);
    cpu.set_flag(Flag::H, carry3);
    cpu.set_flag(Flag::PV, sum - 1 == 0x7f);
    cpu.set_flag(Flag::N, false);
}
