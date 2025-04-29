use super::Cpu;

impl Cpu {
    pub fn load_8_rr(&mut self, reg1_idx: usize, reg2_idx: usize) {
        self.regs[reg1_idx] = self.regs[reg2_idx];
    }

    pub fn load_8_rv(&mut self, reg_idx: usize, val: u8) {
        self.regs[reg_idx] = val;
    }

    pub fn load_8_mr(&self, mem: &mut [u8; 1], reg_idx: usize) {
        mem[0] = self.regs[reg_idx];
    }

    pub fn load_8_ma(&self, mem: &mut [u8; 1]) {
        mem[0] = self.af.a;
    }

    pub fn load_8_mv(&self, mem: &mut [u8; 1], val: u8) {
        mem[0] = val;
    }

    pub fn load_8_rm(&mut self, reg_idx: usize, mem: &[u8; 1]) {
        self.regs[reg_idx] = mem[0];
    }

    pub fn load_8_am(&mut self, mem: &[u8; 1]) {
        self.af.a = mem[0];
    }

    pub fn load_16_rv(&mut self, reg_hi_idx: usize, reg_lo_idx: usize, val_hi: u8, val_lo: u8) {
        self.regs[reg_hi_idx] = val_hi;
        self.regs[reg_lo_idx] = val_lo;
    }
}
