#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_assignments, unused_variables)
)]

fn print_memory_region(addr: u16) {
    match addr {
        0x0000..0x4000 => eprintln!("16 KiB ROM bank 00"),
        0x4000..0x8000 => eprintln!("16 KiB ROM bank 01 - NN"),
        0x8000..0xA000 => eprintln!("8 KiB Video RAM (VRAM)"),
        0xA000..0xC000 => eprintln!("8 KiB External RAM"),
        0xC000..0xD000 => eprintln!("4 KiB Work RAM (WRAM)"),
        0xD000..0xE000 => eprintln!("4 KiB Work RAM (WRAM)"),
        0xE000..0xFE00 => eprintln!("Echo RAM (mirrors C000–DDFF)"),
        0xFE00..0xFEA0 => eprintln!("Object attribute memory"),
        0xFEA0..0xFF00 => eprintln!("Not usable"),
        0xFF00..0xFF80 => eprintln!("I/O Registers"),
        0xFF80..0xFFFF => eprintln!("High RAM"),
        0xFFFF => eprintln!("Interrupt Enable register"),
    }
}

fn print_io_type(addr: u16) {
    match addr {
        0xFF00 => eprintln!("Joypad input"),
        0xFF01..=0xFF02 => eprintln!("Serial transfer"),
        0xFF04..=0xFF07 => eprintln!("Timer and divider"),
        0xFF0F => eprintln!("Interrupts"),
        0xFF10..=0xFF26 => eprintln!("Audio"),
        0xFF30..=0xFF3F => eprintln!("Wave pattern"),
        0xFF40..=0xFF4B => eprintln!("LCD Control, Status, Position, Scrolling, and Palettes"),
        0xFF4F => eprintln!("VRAM Bank Select"),
        0xFF50 => eprintln!("Set to non-zero to disable boot ROM"),
        0xFF51..=0xFF55 => eprintln!("VRAM DMA"),
        0xFF68..=0xFF6B => eprintln!("BG / OBJ Palettes"),
        0xFF70 => eprintln!("WRAM Bank Select"),
        _ => eprintln!("Invalid IO type"),
    }
}

fn main() {
    print_memory_region(0xBFDF);
    eprintln!("Hello, world!");
}
