use crate::z80::{Flag, Register, Z80};
use crate::{lower, upper};
use std::mem::swap;

#[inline]
pub fn exchange_de_hl(cpu: &mut Z80) {
    swap(&mut cpu.de, &mut cpu.hl);
}

#[inline]
pub fn exchange_af_af1(cpu: &mut Z80) {
    swap(&mut cpu.af, &mut cpu.af1);
}

#[inline]
pub fn exchange_exx(cpu: &mut Z80) {
    swap(&mut cpu.bc, &mut cpu.bc1);
    swap(&mut cpu.de, &mut cpu.de1);
    swap(&mut cpu.hl, &mut cpu.hl1);
}

#[inline]
pub fn exchange_sp_hl(cpu: &mut Z80, mem: &[u8]) {
    let lo = mem[cpu.stack_ptr as usize];
    cpu.set_reg(Register::L, lo as u16);
    let hi = mem[(cpu.stack_ptr + 1) as usize];
    cpu.set_reg(Register::H, hi as u16);
}

#[inline]
pub fn exchange_sp_ix(cpu: &mut Z80, mem: &[u8]) {
    let lo = mem[cpu.stack_ptr as usize];
    lower!(set cpu.index_x; lo as u16);
    let hi = mem[(cpu.stack_ptr + 1) as usize];
    upper!(set cpu.index_x; hi as u16);
}

#[inline]
pub fn exchange_sp_iy(cpu: &mut Z80, mem: &[u8]) {
    let lo = mem[cpu.stack_ptr as usize];
    lower!(set cpu.index_y; lo as u16);
    let hi = mem[(cpu.stack_ptr + 1) as usize];
    upper!(set cpu.index_y; hi as u16);
}

#[inline]
pub fn exchange_ldi(cpu: &mut Z80, mem: &mut [u8]) {
    let src = cpu.hl as usize;
    let dest = cpu.de as usize;
    mem[dest] = mem[src];
    cpu.de += 1;
    cpu.hl += 1;
    cpu.bc -= 1;

    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::PV, cpu.bc - 1 != 0);
    cpu.set_flag(Flag::N, false);
}

#[inline]
pub fn exchange_ldir(cpu: &mut Z80, mem: &mut [u8]) {
    let src = cpu.hl as usize;
    let dest = cpu.de as usize;
    mem[dest] = mem[src];
    cpu.de += 1;
    cpu.hl += 1;
    cpu.bc -= 1;

    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::PV, cpu.bc - 1 != 0);
    cpu.set_flag(Flag::N, false);

    if cpu.bc != 0 {
        cpu.prog_counter -= 2;
    }
}

#[inline]
pub fn exchange_ldd(cpu: &mut Z80, mem: &mut [u8]) {
    let src = cpu.hl as usize;
    let dest = cpu.de as usize;
    mem[dest] = mem[src];
    cpu.de -= 1;
    cpu.hl -= 1;
    cpu.bc -= 1;

    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::PV, cpu.bc != 0);
    cpu.set_flag(Flag::N, false);
}

#[inline]
pub fn exchange_lddr(cpu: &mut Z80, mem: &mut [u8]) {
    let src = cpu.hl as usize;
    let dest = cpu.de as usize;
    mem[dest] = mem[src];
    cpu.de -= 1;
    cpu.hl -= 1;
    cpu.bc -= 1;

    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::PV, cpu.bc != 0);
    cpu.set_flag(Flag::N, false);

    if cpu.bc != 0 {
        cpu.prog_counter -= 2;
    }
}

#[inline]
pub fn exchange_cpi(cpu: &mut Z80, mem: &[u8]) {
    let addr = cpu.reg(Register::HL) as usize;
    let val = mem[addr] as i32;
    let cmp = (cpu.reg(Register::A) as i32) - val;

    cpu.hl += 1;
    cpu.bc -= 1;

    cpu.set_flag(Flag::S, cmp < 0);
    cpu.set_flag(Flag::Z, cmp == 0);
    cpu.set_flag(Flag::PV, cpu.bc != 0);
    cpu.set_flag(Flag::N, true);
}

#[inline]
pub fn exchange_cpir(cpu: &mut Z80, mem: &[u8]) {
    let addr = cpu.reg(Register::HL) as usize;
    let val = mem[addr] as i32;
    let cmp = (cpu.reg(Register::A) as i32) - val;
    cpu.set_flag(Flag::N, cmp < 0);
    cpu.set_flag(Flag::Z, cmp == 0);

    cpu.hl += 1;
    cpu.bc -= 1;
    if cpu.bc != 0 || cmp != 0 {
        cpu.prog_counter -= 2;
    }
}

#[inline]
pub fn exchange_cpd(cpu: &mut Z80, mem: &[u8]) {
    let addr = cpu.reg(Register::HL) as usize;
    let val = mem[addr] as i32;
    let cmp = (cpu.reg(Register::A) as i32) - val;

    cpu.hl -= 1;
    cpu.bc -= 1;

    cpu.set_flag(Flag::S, cmp < 0);
    cpu.set_flag(Flag::Z, cmp == 0);
    cpu.set_flag(Flag::PV, cpu.bc != 0);
    cpu.set_flag(Flag::N, true);
}

#[inline]
pub fn exchange_cpdr(cpu: &mut Z80, mem: &[u8]) {
    let addr = cpu.reg(Register::HL) as usize;
    let val = mem[addr] as i32;
    let cmp = (cpu.reg(Register::A) as i32) - val;

    cpu.hl -= 1;
    cpu.bc -= 1;

    cpu.set_flag(Flag::S, cmp < 0);
    cpu.set_flag(Flag::Z, cmp == 0);
    cpu.set_flag(Flag::PV, cpu.bc != 0);
    cpu.set_flag(Flag::N, true);

    if cpu.bc != 0 || cmp != 0 {
        cpu.prog_counter -= 2;
    }
}
