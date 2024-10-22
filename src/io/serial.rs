use core::fmt;

use crate::arch::ports::Port;
use crate::io::WriteBytes;

pub const PORT_COM1: u16 = 0x3F8;

#[derive(Debug)]
pub struct Console {
    port: Port,
}

#[allow(dead_code)]
impl Console {
    pub unsafe fn new_uninit(port: u16) -> Self {
        Self {
            port: Port::new(port),
        }
    }

    pub fn try_new(port: u16) -> Result<Self, ()> {
        let port = Port::new(port);
        port.add(1).out_u8(0x00); // disable all interrupts
        port.add(3).out_u8(0x80); // enable DLAB (set baud rate divisor)
        port.add(0).out_u8(0x03); // set divisor to 3 (lo byte) 38400 baud
        port.add(1).out_u8(0x00); //                  (hi byte)
        port.add(3).out_u8(0x03); // 8 bits, no parity, one stop bit
        port.add(2).out_u8(0xC7); // enable FIFO, clear them, with 14-byte threshold
        port.add(4).out_u8(0x0B); // IRQs enabled, RTS/DSR set
        port.add(4).out_u8(0x1E); // set in loopback mode, test the serial chip
        port.add(0).out_u8(0xAE); // test serial chip (send byte 0xAE and check if serial returns same byte)

        // check if serial is faulty (i.e: not same byte as sent)
        if port.in_u8() != 0xAE {
            return Err(());
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        port.add(4).out_u8(0x0F);
        Ok(Self { port })
    }

    fn is_transmit_ready(&self) -> bool {
        self.port.add(5).in_u8() & 0x20 != 0
    }
}

impl WriteBytes for Console {
    fn write_byte(&mut self, byte: u8) {
        if cfg!(feature = "serial-carriage") && byte == b'\n' {
            self.write_byte(b'\r');
        }
        while !self.is_transmit_ready() {}
        self.port.out_u8(byte)
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
