#![allow(dead_code)]

use crate::utils::bits::u2;
use core::arch::asm;

#[derive(Debug)]
#[repr(C, packed(2))]
struct Descriptor {
    size: u16,
    offset: u32,
}
const_assert!(@size Descriptor == 6);

pub fn load(entries: &[Entry]) {
    let descriptor = Descriptor {
        size: core::mem::size_of_val(entries) as u16,
        offset: entries.as_ptr() as u32,
    };
    unsafe {
        asm!(
            "lgdt [{}]",
            in(reg) &descriptor,
            options(nostack, preserves_flags)
        )
    };
}

pub fn reload_cs(selector: u16) {
    unsafe {
        asm!(
            "push {:e}",
            "push $1f",
            "lret",
            "1:",
            in(reg) selector,
            options(att_syntax, nostack, preserves_flags)
        )
    };
}

pub fn reload_ds(selector: u16) {
    unsafe { asm!("mov ds, ax", in("ax") selector) };
}
pub fn reload_ss(selector: u16) {
    unsafe { asm!("mov ss, ax", in("ax") selector) };
}
pub fn reload_es(selector: u16) {
    unsafe { asm!("mov es, ax", in("ax") selector) };
}
pub fn reload_fs(selector: u16) {
    unsafe { asm!("mov fs, ax", in("ax") selector) };
}
pub fn reload_gs(selector: u16) {
    unsafe { asm!("mov gs, ax", in("ax") selector) };
}

pub fn selector(index: u16, ti: bool, dpl: u2) -> u16 {
    (dpl as u16) | ((ti as u16) << 2) | ((index & 0x1FFF) << 3)
}

#[derive(Clone, Copy, Debug)]
pub struct Entry(u64);

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum SystemSegmentType {
    Ldt = 0x2,
    TssAvailable = 0x9,
    TssBusy = 0xB,
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

    pub fn set_limit(self, limit: u32) -> Self {
        Self(
            (self.0 & !0x000F0000_0000FFFF)
                | (limit as u64 & 0xF0000) << 32
                | (limit as u64 & 0x0FFFF),
        )
    }
    pub fn set_base(self, base: u32) -> Self {
        Self(
            (self.0 & !0xFF0000FF_FFFF0000)
                | (base as u64 & 0xFF000000) << 32
                | (base as u64 & 0x00FFFFFF) << 16,
        )
    }

    pub fn set_p(self, bit: bool) -> Self {
        self.set_bit(47, bit)
    }
    pub fn set_dpl(self, dpl: u2) -> Self {
        self.set_bits(45, dpl as u64, 2)
    }
    pub fn set_s(self, bit: bool) -> Self {
        self.set_bit(44, bit)
    }
    pub fn set_e(self, bit: bool) -> Self {
        self.set_bit(43, bit)
    }
    pub fn set_dc(self, bit: bool) -> Self {
        self.set_bit(42, bit)
    }
    pub fn set_rw(self, bit: bool) -> Self {
        self.set_bit(41, bit)
    }
    pub fn set_a(self, bit: bool) -> Self {
        self.set_bit(40, bit)
    }
    pub fn set_type(self, typ: SystemSegmentType) -> Self {
        self.set_bits(43, typ as u64, 4)
    }

    pub fn set_g(self, bit: bool) -> Self {
        self.set_bit(55, bit)
    }
    pub fn set_db(self, bit: bool) -> Self {
        self.set_bit(54, bit)
    }
    pub fn set_l(self, bit: bool) -> Self {
        self.set_bit(43, bit)
    }

    pub fn get_flags(&self) -> u8 {
        ((self.0 >> 52) & 0x0F) as u8
    }
    pub fn get_access_byte(&self) -> u8 {
        ((self.0 >> 40) & 0xFF) as u8
    }
}

pub fn default_segments() -> [Entry; 5] {
    let null = Entry::new();
    let kernel_code = Entry::new()
        .set_base(0)
        .set_limit(0x80000 - 1)
        .set_p(true)
        .set_dpl(u2::V00)
        .set_s(true)
        .set_e(true)
        .set_dc(false)
        .set_rw(true)
        .set_a(true)
        .set_g(true)
        .set_db(true);
    let kernel_data = Entry::new()
        .set_base(0)
        .set_limit(0x80000 - 1)
        .set_p(true)
        .set_dpl(u2::V00)
        .set_s(true)
        .set_e(false)
        .set_dc(false)
        .set_rw(true)
        .set_a(true)
        .set_g(true)
        .set_db(true);
    let user_code = Entry::new()
        .set_base(0x80000 - 1)
        .set_limit(0x80000)
        .set_p(true)
        .set_dpl(u2::V11)
        .set_s(true)
        .set_e(false)
        .set_dc(false)
        .set_rw(true)
        .set_a(true)
        .set_g(true)
        .set_db(true);
    let user_data = Entry::new()
        .set_base(0x80000 - 1)
        .set_limit(0x80000)
        .set_p(true)
        .set_dpl(u2::V11)
        .set_s(true)
        .set_e(false)
        .set_dc(false)
        .set_rw(true)
        .set_a(true)
        .set_g(true)
        .set_db(true);
    [null, kernel_code, kernel_data, user_code, user_data]
}
