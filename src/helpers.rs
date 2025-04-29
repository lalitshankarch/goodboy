
pub fn print_memory_region(addr: u16) {
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

pub fn print_io_registers(addr: u16) {
    match addr {
        0xFF00 => eprintln!("P1/JOYP Joypad Mixed All"),
        0xFF01 => eprintln!("SB Serial transfer data R/W All"),
        0xFF02 => eprintln!("SC Serial transfer control R/W Mixed"),
        0xFF04 => eprintln!("DIV Divider register R/W All"),
        0xFF05 => eprintln!("TIMA Timer counter R/W All"),
        0xFF06 => eprintln!("TMA Timer modulo R/W All"),
        0xFF07 => eprintln!("TAC Timer control R/W All"),
        0xFF0F => eprintln!("IF Interrupt flag R/W All"),
        0xFF10 => eprintln!("NR10 Sound channel 1 sweep R/W All"),
        0xFF11 => eprintln!("NR11 Sound channel 1 length timer & duty cycle Mixed All"),
        0xFF12 => eprintln!("NR12 Sound channel 1 volume & envelope R/W All"),
        0xFF13 => eprintln!("NR13 Sound channel 1 period low W All"),
        0xFF14 => eprintln!("NR14 Sound channel 1 period high & control Mixed All"),
        0xFF16 => eprintln!("NR21 Sound channel 2 length timer & duty cycle Mixed All"),
        0xFF17 => eprintln!("NR22 Sound channel 2 volume & envelope R/W All"),
        0xFF18 => eprintln!("NR23 Sound channel 2 period low W All"),
        0xFF19 => eprintln!("NR24 Sound channel 2 period high & control Mixed All"),
        0xFF1A => eprintln!("NR30 Sound channel 3 DAC enable R/W All"),
        0xFF1B => eprintln!("NR31 Sound channel 3 length timer W All"),
        0xFF1C => eprintln!("NR32 Sound channel 3 output level R/W All"),
        0xFF1D => eprintln!("NR33 Sound channel 3 period low W All"),
        0xFF1E => eprintln!("NR34 Sound channel 3 period high & control Mixed All"),
        0xFF20 => eprintln!("NR41 Sound channel 4 length timer W All"),
        0xFF21 => eprintln!("NR42 Sound channel 4 volume & envelope R/W All"),
        0xFF22 => eprintln!("NR43 Sound channel 4 frequency & randomness R/W All"),
        0xFF23 => eprintln!("NR44 Sound channel 4 control Mixed All"),
        0xFF24 => eprintln!("NR50 Master volume & VIN panning R/W All"),
        0xFF25 => eprintln!("NR51 Sound panning R/W All"),
        0xFF26 => eprintln!("NR52 Sound on/off Mixed All"),
        0xFF30..=0xFF3F => {
            eprintln!("Wave RAM Storage for one of the sound channels’ waveform R/W All")
        }
        0xFF40 => eprintln!("LCDC LCD control R/W All"),
        0xFF41 => eprintln!("STAT LCD status Mixed All"),
        0xFF42 => eprintln!("SCY Viewport Y position R/W All"),
        0xFF43 => eprintln!("SCX Viewport X position R/W All"),
        0xFF44 => eprintln!("LY LCD Y coordinate R All"),
        0xFF45 => eprintln!("LYC LY compare R/W All"),
        0xFF46 => eprintln!("DMA OAM DMA source address & start R/W All"),
        0xFF47 => eprintln!("BGP BG palette data R/W DMG"),
        0xFF48 => eprintln!("OBP0 OBJ palette 0 data R/W DMG"),
        0xFF49 => eprintln!("OBP1 OBJ palette 1 data R/W DMG"),
        0xFF4A => eprintln!("WY Window Y position R/W All"),
        0xFF4B => eprintln!("WX Window X position plus 7 R/W All"),
        0xFF4D => eprintln!("KEY1 Prepare speed switch Mixed CGB"),
        0xFF4F => eprintln!("VBK VRAM bank R/W CGB"),
        0xFF51 => eprintln!("HDMA1 VRAM DMA source high W CGB"),
        0xFF52 => eprintln!("HDMA2 VRAM DMA source low W CGB"),
        0xFF53 => eprintln!("HDMA3 VRAM DMA destination high W CGB"),
        0xFF54 => eprintln!("HDMA4 VRAM DMA destination low W CGB"),
        0xFF55 => eprintln!("HDMA5 VRAM DMA length/mode/start R/W CGB"),
        0xFF56 => eprintln!("RP Infrared communications port Mixed CGB"),
        0xFF68 => eprintln!(
            "BCPS/BGPI Background color palette specification / Background palette index R/W CGB"
        ),
        0xFF69 => {
            eprintln!("BCPD/BGPD Background color palette data / Background palette data R/W CGB")
        }
        0xFF6A => {
            eprintln!("OCPS/OBPI OBJ color palette specification / OBJ palette index R/W CGB")
        }
        0xFF6B => eprintln!("OCPD/OBPD OBJ color palette data / OBJ palette data R/W CGB"),
        0xFF6C => eprintln!("OPRI Object priority mode R/W CGB"),
        0xFF70 => eprintln!("SVBK WRAM bank R/W CGB"),
        0xFF76 => eprintln!("PCM12 Audio digital outputs 1 & 2 R CGB"),
        0xFF77 => eprintln!("PCM34 Audio digital outputs 3 & 4 R CGB"),
        0xFFFF => eprintln!("IE Interrupt enable R/W All"),
        _ => eprintln!("Invalid IO type"),
    }
}
