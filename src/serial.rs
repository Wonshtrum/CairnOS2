use core::fmt;

use crate::arch::x86::ports::{inb, outb};

pub const PORT_COM1: u16 = 0x3F8;

#[derive(Debug)]
pub struct Console {
    port: u16,
}

impl Console {
    pub unsafe fn new_uninit(port: u16) -> Self {
        Self { port }
    }

    pub unsafe fn new(port: u16) -> Result<Self, ()> {
        outb(port + 1, 0x00); // disable all interrupts
        outb(port + 3, 0x80); // enable DLAB (set baud rate divisor)
        outb(port + 0, 0x03); // set divisor to 3 (lo byte) 38400 baud
        outb(port + 1, 0x00); //                  (hi byte)
        outb(port + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(port + 2, 0xC7); // enable FIFO, clear them, with 14-byte threshold
        outb(port + 4, 0x0B); // IRQs enabled, RTS/DSR set
        outb(port + 4, 0x1E); // set in loopback mode, test the serial chip
        outb(port + 0, 0xAE); // test serial chip (send byte 0xAE and check if serial returns same byte)

        // check if serial is faulty (i.e: not same byte as sent)
        if inb(port) != 0xAE {
            return Err(());
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        outb(port + 4, 0x0F);
        Ok(Self { port })
    }

    fn is_transmit_ready(&self) -> bool {
        unsafe { (inb(self.port + 5) & 0x20) != 0 }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if cfg!(feature = "serial-carriage") && byte == b'\n' {
            self.write_byte(b'\r');
        }
        while !self.is_transmit_ready() {}
        unsafe { outb(self.port, byte) }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            self.write_byte(*byte);
        }
        Ok(())
    }
}
