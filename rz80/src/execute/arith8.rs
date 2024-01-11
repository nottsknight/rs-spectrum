use crate::{
    alu::{ALUAdd, ALU},
    Flag, Register, Z80,
};

#[inline]
pub fn add_a_r(cpu: &mut Z80, r: Register) {
    let a = cpu.reg(Register::A) as u8;
    let rval = cpu.reg(r) as u8;
    let ALUAdd {
        sum,
        carry3,
        carry7,
        overflow,
    } = ALU::add(a, rval);

    cpu.set_reg(Register::A, sum as u16);
    cpu.set_flag(Flag::S, (sum as i8) < 0);
    cpu.set_flag(Flag::Z, sum == 0);
    cpu.set_flag(Flag::H, carry3);
    cpu.set_flag(Flag::PV, overflow);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::C, carry7);
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
