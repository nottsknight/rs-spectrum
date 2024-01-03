use crate::z80::{Flag, Register, Z80};
use crate::{set_lower, set_upper};
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
    set_lower!(cpu.index_x, lo as u16);
    let hi = mem[(cpu.stack_ptr + 1) as usize];
    set_upper!(cpu.index_x, hi as u16);
}

#[inline]
pub fn exchange_sp_iy(cpu: &mut Z80, mem: &[u8]) {
    let lo = mem[cpu.stack_ptr as usize];
    set_lower!(cpu.index_y, lo as u16);
    let hi = mem[(cpu.stack_ptr + 1) as usize];
    set_upper!(cpu.index_y, hi as u16);
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
    while cpu.bc != 0 {
        let src = cpu.hl as usize;
        let dest = cpu.de as usize;
        mem[dest] = mem[src];
        cpu.de += 1;
        cpu.hl += 1;
        cpu.bc -= 1;
        if cpu.bc != 0 {
            cpu.prog_counter -= 2;
        }
    }

    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::PV, cpu.bc - 1 != 0);
    cpu.set_flag(Flag::N, false);
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
    while cpu.bc != 0 {
        let src = cpu.hl as usize;
        let dest = cpu.de as usize;
        mem[dest] = mem[src];
        cpu.de -= 1;
        cpu.hl -= 1;
        cpu.bc -= 1;
        if cpu.bc != 0 {
            cpu.prog_counter -= 2;
        }
    }

    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::PV, cpu.bc != 0);
    cpu.set_flag(Flag::N, false);
}
