#[derive(Default)]
struct Accumulator {
    a: u8,
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

#[derive(Default)]
pub struct Cpu {
    af: Accumulator,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Cpu {
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }
    pub fn load_8_rr(&self, left: &mut u8, right: &mut u8) {
        *left = *right;
    }
    pub fn load_8_rv(&self, left: &mut u8, value: u8) {
        *left = value;
    }
    pub fn load_8_mr(&self, memory: &mut [u8], right: &mut u8) {
        memory[self.hl() as usize] = *right;
    }
    pub fn load_8_mv(&self, memory: &mut [u8], value: u8) {
        memory[self.hl() as usize] = value;
    }
    pub fn load_8_rm(&self, left: &mut u8, memory: &mut [u8]) {
        *left = memory[self.hl() as usize];
    }
}
