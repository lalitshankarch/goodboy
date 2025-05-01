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

    pub fn swap_8_m(&mut self, mem: &mut [u8; 1]) {
        let val = mem[0];
        let swapped = (val & 0x0F) << 4 | (val & 0xF0) >> 4;
        mem[0] = swapped;
        self.set_znhc(swapped == 0, false, false, false);
    }

    pub fn test_8_r(&mut self, reg_idx: usize, bit_idx: usize) {
        let test = self.regs[reg_idx] & (1 << bit_idx);
        self.set_znhc(test == 0, false, true, self.af.c);
    }

    pub fn test_8_m(&mut self, mem: &[u8; 1], bit_idx: usize) {
        let test = mem[0] & (1 << bit_idx);
        self.set_znhc(test == 0, false, true, self.af.c);
    }

    pub fn res_8_r(&mut self, reg_idx: usize, bit_idx: usize) {
        self.regs[reg_idx] &= !(1 << bit_idx);
    }

    pub fn res_8_m(&self, mem: &mut [u8; 1], bit_idx: usize) {
        mem[0] &= !(1 << bit_idx);
    }

    pub fn set_8_r(&mut self, reg_idx: usize, bit_idx: usize) {
        self.regs[reg_idx] |= 1 << bit_idx;
    }

    pub fn set_8_m(&self, mem: &mut [u8; 1], bit_idx: usize) {
        mem[0] |= 1 << bit_idx;
    }

    pub fn rotl_8_r(&mut self, reg_idx: usize) {
        let carry = (self.regs[reg_idx] & 0x80) >> 7;
        self.regs[reg_idx] = self.regs[reg_idx].rotate_left(1);
        self.set_znhc(self.regs[reg_idx] == 0, false, false, carry == 1);
    }

    pub fn rotl_8_m(&mut self, mem: &mut [u8; 1]) {
        let carry = (mem[0] & 0x80) >> 7;
        mem[0] = mem[0].rotate_left(1);
        self.set_znhc(mem[0] == 0, false, false, carry == 1);
    }

    pub fn rotr_8_r(&mut self, reg_idx: usize) {
        let carry = self.regs[reg_idx] & 0x1;
        self.regs[reg_idx] = self.regs[reg_idx].rotate_right(1);
        self.set_znhc(self.regs[reg_idx] == 0, false, false, carry == 1);
    }

    pub fn rotr_8_m(&mut self, mem: &mut [u8; 1]) {
        let carry = mem[0] & 0x1;
        mem[0] = mem[0].rotate_right(1);
        self.set_znhc(mem[0] == 0, false, false, carry == 1);
    }

    pub fn rotr_c_8_r(&mut self, reg_idx: usize) {
        let carry = self.regs[reg_idx] & 0x1;
        self.regs[reg_idx] = self.regs[reg_idx] >> 1 | ((self.af.c as u8) << 7);
        self.set_znhc(self.regs[reg_idx] == 0, false, false, carry == 1);
    }

    pub fn rotr_c_8_m(&mut self, mem: &mut [u8; 1]) {
        let carry = mem[0] & 0x1;
        mem[0] = mem[0] >> 1 | ((self.af.c as u8) << 7);
        self.set_znhc(mem[0] == 0, false, false, carry == 1);
    }

    pub fn rotl_c_8_r(&mut self, reg_idx: usize) {
        let carry = (self.regs[reg_idx] & 0x80) >> 7;
        self.regs[reg_idx] = self.regs[reg_idx] << 1 | (self.af.c as u8);
        self.set_znhc(self.regs[reg_idx] == 0, false, false, carry == 1);
    }

    pub fn rotl_c_8_m(&mut self, mem: &mut [u8; 1]) {
        let carry = (mem[0] & 0x80) >> 7;
        mem[0] = mem[0] << 1 | (self.af.c as u8);
        self.set_znhc(mem[0] == 0, false, false, carry == 1);
    }

    pub fn sal_8_r(&mut self, reg_idx: usize) {
        let carry = (self.regs[reg_idx] & 0x80) >> 7;
        self.regs[reg_idx] = self.regs[reg_idx] << 1;
        self.set_znhc(self.regs[reg_idx] == 0, false, false, carry == 1);
    }

    pub fn sal_8_m(&mut self, mem: &mut [u8; 1]) {
        let carry = (mem[0] & 0x80) >> 7;
        mem[0] = mem[0] << 1;
        self.set_znhc(mem[0] == 0, false, false, carry == 1);
    }

    pub fn sar_8_r(&mut self, reg_idx: usize) {
        let carry = self.regs[reg_idx] & 0x1;
        self.regs[reg_idx] = (self.regs[reg_idx] & 0xF0) | self.regs[reg_idx] >> 1;
        self.set_znhc(self.regs[reg_idx] == 0, false, false, carry == 1);
    }

    pub fn sar_8_m(&mut self, mem: &mut [u8; 1]) {
        let carry = mem[0] & 0x1;
        mem[0] = (mem[0] & 0xF0) | mem[0] >> 1;
        self.set_znhc(mem[0] == 0, false, false, carry == 1);
    }

    pub fn slr_8_r(&mut self, reg_idx: usize) {
        let carry = self.regs[reg_idx] & 0x1;
        self.regs[reg_idx] = self.regs[reg_idx] >> 1;
        self.set_znhc(self.regs[reg_idx] == 0, false, false, carry == 1);
    }

    pub fn slr_8_m(&mut self, mem: &mut [u8; 1]) {
        let carry = mem[0] & 0x1;
        mem[0] = mem[0] >> 1;
        self.set_znhc(mem[0] == 0, false, false, carry == 1);
    }
}
