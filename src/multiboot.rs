use core::ffi::CStr;

use crate::mem::Mmap;
use crate::vga;

pub const MAGIC: u32 = 0x2BADB002;

#[derive(Debug)]
#[repr(C, packed)]
pub struct Info {
    flags: u32,
    mem_upper: u32,
    mem_lower: u32,
    boot_device: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    syms: [u32; 4],
    mmap_length: u32,
    mmpa_addr: u32,
    drives_length: u32,
    drives_addr: u32,
    config_table: u32,
    boot_loader_name: u32,
    apm_table: u32,
    vbe_control_info: u32,
    vbe_mode_info: u32,
    vbe_mode: u16,
    vbe_interface_seq: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,
    framebuffer_addr: u64,
    framebuffer_pitch: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_bpp: u8,
    framebuffer_type: u8,
    color_info: [u8; 5],
}
const_assert!(@size Info == 115);

#[derive(Debug)]
pub enum Symbols {
    AOut {
        tabsize: u32,
        strsize: u32,
        addr: u32,
    },
    Elf {
        num: u32,
        size: u32,
        addr: u32,
        shndx: u32,
    },
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct ApmTable {
    version: u16,
    cseg: u16,
    offset: u32,
    cseg_16: u16,
    dseg: u16,
    flags: u16,
    cseg_len: u16,
    cseg_16_len: u16,
    dseg_len: u16,
}
const_assert!(@size ApmTable == 20);

#[derive(Debug)]
pub struct Vbe;

impl Info {
    pub fn is_flag_set(&self, bit: u32) -> bool {
        self.flags & (1 << bit) != 0
    }

    pub fn get_flags(&self) -> u32 {
        self.flags
    }

    pub fn get_mem(&self) -> Option<(u32, u32)> {
        self.is_flag_set(0)
            .then_some((self.mem_lower, self.mem_upper))
    }

    pub fn get_boot_device(&self) -> Option<u32> {
        self.is_flag_set(1).then_some(self.boot_device)
    }

    pub fn get_cmdline(&self) -> Option<&CStr> {
        self.is_flag_set(2)
            .then_some(unsafe { CStr::from_ptr(self.cmdline as _) })
    }

    pub fn get_mods(&self) -> Option<(u32, u32)> {
        self.is_flag_set(3)
            .then_some((self.mods_count, self.mods_addr))
    }

    pub fn get_syms(&self) -> Option<Symbols> {
        match (self.is_flag_set(4), self.is_flag_set(5)) {
            (true, false) => Some(Symbols::AOut {
                tabsize: self.syms[0],
                strsize: self.syms[1],
                addr: self.syms[2],
            }),
            (false, true) => Some(Symbols::Elf {
                num: self.syms[0],
                size: self.syms[1],
                addr: self.syms[2],
                shndx: self.syms[3],
            }),
            _ => None,
        }
    }

    pub fn get_mmaps(&self) -> Option<&[Mmap]> {
        if self.is_flag_set(6) {
            let ptr = self.mmpa_addr as *const Mmap;
            let len = self.mmap_length as usize / core::mem::size_of::<Mmap>();
            // TODO: Apparently the Mmap pairs can have different sizes
            Some(unsafe { core::slice::from_raw_parts(ptr, len) })
        } else {
            None
        }
    }

    pub fn get_drives(&self) -> Option<(u32, u32)> {
        self.is_flag_set(7)
            .then_some((self.drives_length, self.drives_addr))
    }

    pub fn get_config_table(&self) -> Option<u32> {
        self.is_flag_set(8).then_some(self.config_table)
    }

    pub fn get_boot_loader_name(&self) -> Option<&CStr> {
        self.is_flag_set(9)
            .then_some(unsafe { CStr::from_ptr(self.boot_loader_name as _) })
    }

    pub fn get_apm_table(&self) -> Option<&ApmTable> {
        self.is_flag_set(10)
            .then_some(unsafe { &*(self.apm_table as *const ApmTable) })
    }

    pub fn get_vbe(&self) -> Option<Vbe> {
        // TODO
        self.is_flag_set(11).then_some(Vbe)
    }

    pub fn get_framebuffer(&self) -> Option<vga::FrameBuffer> {
        // TODO: bpp, type and color_info
        if self.is_flag_set(12) && self.framebuffer_addr < u32::MAX as u64 {
            Some(vga::FrameBuffer::new(
                self.framebuffer_width as usize,
                self.framebuffer_height as usize,
                self.framebuffer_pitch as usize >> 1,
                self.framebuffer_addr as u32 as *mut u16,
            ))
        } else {
            None
        }
    }
}
