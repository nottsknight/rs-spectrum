use crate::{
    carry_borrow::AddCarry,
    z80::{Flag, Register, Z80},
};

#[inline]
pub fn add_a_r(cpu: &mut Z80, r: Register) {
    let a = cpu.reg(Register::A) as u8;
    let rval = cpu.reg(r) as u8;
    let (sum, carry3, carry7) = a.add_carry(rval);

    cpu.set_reg(Register::A, sum as u16);
    todo!("Set S flag");
    cpu.set_flag(Flag::Z, sum == 0);
    cpu.set_flag(Flag::H, carry3);
    todo!("Set PV flag");
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::C, carry7);
}
