use super::Cpu;

impl Cpu {
    pub fn set_znhc(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.af.z = z;
        self.af.n = n;
        self.af.h = h;
        self.af.c = c;
    }

    pub fn reset_znhc(&mut self) {
        self.set_znhc(false, false, false, false);
    }
}
