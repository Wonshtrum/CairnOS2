#![allow(dead_code)]

use core::arch::asm;
use core::fmt;

use crate::arch::tables::Descriptor;
use crate::arch::ports::Port;
use crate::utils::bits::u2;

const PIC1_CMD: Port = Port::new(0x20);
const PIC1_DATA: Port = Port::new(0x21);
const PIC2_CMD: Port = Port::new(0xA0);
const PIC2_DATA: Port = Port::new(0xA1);

pub fn init(offset1: u8, offset2: u8) {
    const INIT: u8 = /*  */ 0x10;
    const ICW4_NEEDED: u8 = 0x01;
    const PIC2_IRQ: u8 = 0x2;
    const MODE_8086: u8 = 0x1;

    let mask1 = PIC1_DATA.in_u8();
    let mask2 = PIC2_DATA.in_u8();

    // ICW1
    PIC1_CMD.slow_out_u8(INIT | ICW4_NEEDED);
    PIC2_CMD.slow_out_u8(INIT | ICW4_NEEDED);
    // ICW2
    PIC1_DATA.slow_out_u8(offset1);
    PIC2_DATA.slow_out_u8(offset2);
    // ICW3
    PIC1_DATA.slow_out_u8(PIC2_IRQ << 1);
    PIC2_DATA.slow_out_u8(PIC2_IRQ);
    // ICW4
    PIC1_DATA.slow_out_u8(MODE_8086);
    PIC2_DATA.slow_out_u8(MODE_8086);

    PIC1_DATA.out_u8(mask1);
    PIC2_DATA.out_u8(mask2);
}

pub fn load(entries: &[Entry]) {
    let descriptor = Descriptor {
        size: core::mem::size_of_val(entries) as u16 - 1,
        offset: entries.as_ptr() as u32,
    };
    unsafe {
        asm!(
            "lidt [{}]",
            "sti",
            "int 13",
            in(reg) &descriptor,
            options(nostack, preserves_flags)
        )
    };
}

#[derive(Clone, Copy)]
pub struct Entry(u64);

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum GateType {
    Task = 0x5,
    Interrupt = 0xE,
    Trap = 0xF,
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Gate(")?;
        self.0.fmt(f)?;
        f.write_str(")")
    }
}

impl Entry {
    pub fn new() -> Self {
        Self(0)
    }

    fn set_bit(self, offset: u64, bit: bool) -> Self {
        let mask = 1 << offset;
        if bit {
            Self(self.0 | mask)
        } else {
            Self(self.0 & !mask)
        }
    }
    fn set_bits(self, offset: u64, bits: u64, len: u64) -> Self {
        let mask = ((1 << len) - 1) << offset;
        Self((self.0 & !mask) | (bits << offset))
    }

    pub fn set_offset(self, offset: u32) -> Self {
        Self(
            (self.0 & !0xFFFF0000_0000FFFF)
                | (offset as u64 & 0xFFFF0000) << 32
                | (offset as u64 & 0x0000FFFF),
        )
    }
    pub fn set_selector(self, selector: u16) -> Self {
        Self((self.0 & !0x00000000_FFFF0000) | (selector as u64) << 16)
    }

    pub fn set_p(self, bit: bool) -> Self {
        self.set_bit(47, bit)
    }
    pub fn set_dpl(self, dpl: u2) -> Self {
        self.set_bits(45, dpl as u64, 2)
    }
    pub fn set_type(self, typ: GateType) -> Self {
        self.set_bits(40, typ as u64, 4)
    }
}

extern "x86-interrupt" fn general_fault_handler() {
    crate::println!("GENERAL_FAULT_TRIGGERED");
}

pub fn default_gates(code_selector: u16) -> [Entry; 256] {
    let mut gates = [Entry::new(); 256];
    gates[13] = Entry::new()
        .set_offset(general_fault_handler as u32)
        .set_selector(code_selector)
        .set_p(true)
        .set_dpl(u2::V00)
        .set_type(GateType::Interrupt);
    gates
}
