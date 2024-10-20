use core::arch::asm;

pub fn outb(port: u16, val: u8) {
    unsafe { asm!("out dx, al", in("dx") port, in("al") val) };
}

pub fn inb(port: u16) -> u8 {
    let ret: u8;
    unsafe { asm!("in al, dx", in("dx") port, out("al") ret) };
    ret
}
