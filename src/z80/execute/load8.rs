//! Functions for executing 8-bit load instructions.
use crate::z80::{Z80, Register};

#[inline]
pub fn load_r_r(cpu: &mut Z80, r: Register, r1: Register) {
    let x = cpu.reg(r1);
    cpu.set_reg(r, x);
}

#[inline]
pub fn load_r_n(cpu: &mut Z80, r: Register, n: u8) {
    cpu.set_reg(r, n as u16);
}

#[inline]
pub fn load_r_hl(cpu: &mut Z80, r: Register, mem: &[u8]) {
    let addr = cpu.reg(Register::HL);
    let nn = mem[addr as usize];
    cpu.set_reg(r, nn as u16);
}

#[inline]
pub fn load_r_ix(cpu: &mut Z80, r: Register, d: i8, mem: &[u8]) {
    let addr = cpu.index_x as i32 + d as i32;
    let nn = mem[addr as usize];
    cpu.set_reg(r, nn as u16);
}

#[inline]
pub fn load_r_iy(cpu: &mut Z80, r: Register, d: i8, mem: &[u8]) {
    let addr = cpu.index_y as i32 + d as i32;
    let nn = mem[addr as usize];
    cpu.set_reg(r, nn as u16);
}

#[inline]
pub fn load_hl_r(cpu: &mut Z80, r: Register, mem: &mut [u8]) {
    let addr = cpu.reg(Register::HL);
    mem[addr as usize] = cpu.reg(r) as u8;
}

#[inline]
pub fn load_ix_r(cpu: &mut Z80, d: i8, r: Register, mem: &mut [u8]) {
    let addr = cpu.index_x as i32 + d as i32;
    mem[addr as usize] = cpu.reg(r) as u8;
}

#[inline]
pub fn load_iy_r(cpu: &mut Z80, d: i8, r: Register, mem: &mut [u8]) {
    let addr = cpu.index_y as i32 + d as i32;
    mem[addr as usize] = cpu.reg(r) as u8;
}

#[inline]
pub fn load_hl_n(cpu: &mut Z80, n: u8, mem: &mut [u8]) {
    let addr = cpu.reg(Register::HL);
    mem[addr as usize] = n;
}

#[inline]
pub fn load_ix_n(cpu: &mut Z80, d: i8, n: u8, mem: &mut [u8]) {
    let addr = cpu.index_x as i32 + d as i32;
    mem[addr as usize] = n;
}

#[inline]
pub fn load_iy_n(cpu: &mut Z80, d: i8, n: u8, mem: &mut [u8]) {
    let addr = cpu.index_y as i32 + d as i32;
    mem[addr as usize] = n;
}

#[inline]
pub fn load_a_bc(cpu: &mut Z80, mem: &[u8]) {
    let addr = cpu.reg(Register::BC) as usize;
    let n = mem[addr] as u16;
    cpu.set_reg(Register::A, n);
}

#[inline]
pub fn load_a_de(cpu: &mut Z80, mem: &[u8]) {
    let addr = cpu.reg(Register::DE) as usize;
    let n = mem[addr] as u16;
    cpu.set_reg(Register::A, n);
}

#[inline]
pub fn load_a_nn(cpu: &mut Z80, addr: u16, mem: &[u8]) {
    let n = mem[addr as usize] as u16;
    cpu.set_reg(Register::A, n);
}

#[inline]
pub fn load_bc_a(cpu: &mut Z80, mem: &mut [u8]) {
    let addr = cpu.reg(Register::BC);
    mem[addr as usize] = cpu.reg(Register::A) as u8;
}

#[inline]
pub fn load_de_a(cpu: &mut Z80, mem: &mut [u8]) {
    let addr = cpu.reg(Register::DE);
    mem[addr as usize] = cpu.reg(Register::A) as u8;
}

#[inline]
pub fn load_nn_a(cpu: &mut Z80, nn: u16, mem: &mut [u8]) {
    mem[nn as usize] = cpu.reg(Register::A) as u8;
}

#[inline]
pub fn load_a_i(cpu: &mut Z80) {
    cpu.set_reg(Register::A, cpu.interrupt as u16);
}

#[inline]
pub fn load_a_r(cpu: &mut Z80) {
    cpu.set_reg(Register::A, cpu.refresh as u16);
}

#[inline]
pub fn load_i_a(cpu: &mut Z80) {
    cpu.interrupt = cpu.reg(Register::A) as u8;
}

#[inline]
pub fn load_r_a(cpu: &mut Z80) {
    cpu.refresh = cpu.reg(Register::A) as u8;
}