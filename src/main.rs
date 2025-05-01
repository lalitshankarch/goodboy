#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_assignments, unused_variables)
)]

mod cpu;
mod helpers;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::default();
    cpu.load_8_rv(0, 0x96);
    println!("{}", cpu);
    cpu.rotl_c_8_r(0);
    cpu.rotr_c_8_r(0);
    println!("{}", cpu);
    cpu.rotr_c_8_r(0);
    println!("{}", cpu);
    cpu.rotl_c_8_r(0);
    println!("{}", cpu);
    cpu.rotl_c_8_r(0);
    println!("{}", cpu);
    cpu.rotl_c_8_r(0);
    println!("{}", cpu);
    cpu.set_8_r(0, 2);
    println!("{}", cpu);
    cpu.res_8_r(0, 2);
    println!("{}", cpu);
}
