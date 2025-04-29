use std::fmt::{self, Display};

mod bitwise;
mod helpers;
mod load;
mod misc;

#[derive(Default)]
pub struct Accumulator {
    a: u8,
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl Accumulator {
    pub fn to_u16(&self) -> u16 {
        ((self.a as u16) << 8)
            | ((self.z as u16) * 1 << 7)
            | ((self.n as u16) * 1 << 6)
            | ((self.h as u16) * 1 << 5)
            | ((self.c as u16) * 1 << 4)
    }
}

#[derive(Default)]
pub struct Cpu {
    pub af: Accumulator,
    pub sp: u16,
    pub pc: u16,
    pub regs: [u8; 6], // b, c, d, e, h and l
}

impl Cpu {
    pub fn bc(&self) -> u16 {
        ((self.regs[0] as u16) << 8) | self.regs[1] as u16
    }
    pub fn de(&self) -> u16 {
        ((self.regs[2] as u16) << 8) | self.regs[3] as u16
    }
    pub fn hl(&self) -> u16 {
        ((self.regs[4] as u16) << 8) | self.regs[5] as u16
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BC {:#06x}\tDE {:#06x}\tHL {:#06x}\n\
            AF {:#04x}  \tSP {:#06x}\tPC {:#06x}\n\
            Z {}\tN {}\tH {}\tC {}",
            self.bc(),
            self.de(),
            self.hl(),
            self.af.a,
            self.sp,
            self.pc,
            self.af.z as u32,
            self.af.n as u32,
            self.af.h as u32,
            self.af.c as u32,
        )
    }
}
