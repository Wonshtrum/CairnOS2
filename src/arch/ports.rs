use core::arch::asm;

#[derive(Debug)]
pub struct Port(u16);

impl Port {
    pub const fn new(port: u16) -> Self {
        Self(port)
    }

    pub const fn add(&self, offset: u16) -> Self {
        Self(self.0 + offset)
    }

    pub fn out_u8(&self, val: u8) {
        unsafe { asm!("out dx, al", in("dx") self.0, in("al") val) };
    }

    pub fn slow_out_u8(&self, val: u8) {
        self.out_u8(val);
        wait();
    }

    pub fn in_u8(&self) -> u8 {
        let ret;
        unsafe { asm!("in al, dx", in("dx") self.0, out("al") ret) };
        ret
    }
}

pub fn wait() {
    // port 0x80 is always unused after boot
    Port::new(0x80).out_u8(0);
}
