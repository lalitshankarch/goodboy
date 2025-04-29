use super::Cpu;

impl Cpu {
    pub fn and_8_rr(&mut self, reg1_lo_idx: usize, reg2_idx: usize) {
        self.regs[reg1_lo_idx] &= self.regs[reg2_idx];
        self.set_znhc(self.regs[reg1_lo_idx] == 0, false, true, false);
    }

    pub fn and_8_rm(&mut self, reg_lo_idx: usize, mem: &[u8; 1]) {
        self.regs[reg_lo_idx] &= mem[0];
        self.set_znhc(self.regs[reg_lo_idx] == 0, false, true, false);
    }

    pub fn and_8_rv(&mut self, reg_lo_idx: usize, val: u8) {
        self.regs[reg_lo_idx] &= val;
        self.set_znhc(self.regs[reg_lo_idx] == 0, false, true, false);
    }

    pub fn or_8_rr(&mut self, reg1_lo_idx: usize, reg2_idx: usize) {
        self.regs[reg1_lo_idx] |= self.regs[reg2_idx];
        self.set_znhc(self.regs[reg1_lo_idx] == 0, false, false, false);
    }

    pub fn or_8_rm(&mut self, reg_lo_idx: usize, mem: &[u8; 1]) {
        self.regs[reg_lo_idx] |= mem[0];
        self.set_znhc(self.regs[reg_lo_idx] == 0, false, false, false);
    }

    pub fn or_8_rv(&mut self, reg_lo_idx: usize, val: u8) {
        self.regs[reg_lo_idx] |= val;
        self.set_znhc(self.regs[reg_lo_idx] == 0, false, false, false);
    }

    pub fn xor_8_rr(&mut self, reg1_lo_idx: usize, reg2_idx: usize) {
        self.regs[reg1_lo_idx] ^= self.regs[reg2_idx];
        self.set_znhc(self.regs[reg1_lo_idx] == 0, false, false, false);
    }

    pub fn xor_8_rm(&mut self, reg_lo_idx: usize, mem: &[u8; 1]) {
        self.regs[reg_lo_idx] ^= mem[0];
        self.set_znhc(self.regs[reg_lo_idx] == 0, false, false, false);
    }

    pub fn xor_8_rv(&mut self, reg_lo_idx: usize, val: u8) {
        self.regs[reg_lo_idx] ^= val;
        self.set_znhc(self.regs[reg_lo_idx] == 0, false, false, false);
    }

    pub fn swap_8_r(&mut self, reg_idx: usize) {
        let val = self.regs[reg_idx];
        let swapped = (val & 0x0F) << 4 | (val & 0xF0) >> 4;
        self.regs[reg_idx] = swapped;
        self.set_znhc(swapped == 0, false, false, false);
    }

    pub fn swap_8_m(&mut self, mem: &mut [u8]) {
        let val = mem[self.hl() as usize];
        let swapped = (val & 0x0F) << 4 | (val & 0xF0) >> 4;
        mem[self.hl() as usize] = swapped;
        self.set_znhc(swapped == 0, false, false, false);
    }
}
