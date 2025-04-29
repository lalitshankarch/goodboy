#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_assignments, unused_variables)
)]

mod cpu;
mod helpers;

fn main() {
    helpers::print_memory_region(0xBABE);
    helpers::print_io_registers(0xFF07);
    let cpu = cpu::Cpu::default();
}
