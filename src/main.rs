#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_assignments, unused_variables)
)]

mod cpu;
mod helpers;

use cpu::Cpu;

fn main() {
    helpers::print_memory_region(0xBABE);
    helpers::print_io_registers(0xFF07);
    let mut cpu = Cpu::default();
    cpu.load_8_rv(0, 0xDE);
    cpu.load_8_rv(1, 0xAD);
    cpu.load_16_rv(2, 3, 0xBE, 0xEF);
    cpu.load_16_rv(4, 5, 0xBA, 0xBA);
    println!("{}", cpu);
    cpu.and_8_rr(2, 5);
    println!("{}", cpu);
    cpu.swap_8_r(4);
    println!("{}", cpu);
}
