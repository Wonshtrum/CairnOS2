use core::fmt;
use core::ops::Deref;

use crate::arch::ports::Port;
use crate::io::WriteBytes;

const CRTC_INDEX: Port = Port::new(0x3D4);
const CRTC_DATA: Port = Port::new(0x3D5);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Border {
    Simple,
    Double,
    Thick,
    Custom {
        corners: u8,
        v: u8,
        h: u8,
    },
    FullCustom {
        no: u8,
        ne: u8,
        so: u8,
        se: u8,
        n: u8,
        s: u8,
        o: u8,
        e: u8,
    },
}

#[derive(Debug)]
pub struct FrameBuffer {
    width: usize,
    height: usize,
    pitch: usize,
    addr: *mut u16,
}

#[derive(Debug)]
pub struct Console {
    buffer: FrameBuffer,
    pub x: usize,
    pub y: usize,
    color: u8,
    root_width: usize,
    root_offset: usize,
}

const fn entry_color(fg: Color, bg: Color) -> u8 {
    fg as u8 | (bg as u8) << 4
}
const fn entry(byte: u8, color: u8) -> u16 {
    byte as u16 | (color as u16) << 8
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize, pitch: usize, addr: *mut u16) -> Self {
        Self {
            width,
            height,
            pitch,
            addr,
        }
    }
}

impl Deref for Console {
    type Target = FrameBuffer;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

#[allow(dead_code)]
impl Console {
    pub fn new(buffer: FrameBuffer, fg: Color, bg: Color) -> Self {
        Self {
            x: 0,
            y: 0,
            color: entry_color(fg, bg),
            root_width: buffer.width,
            root_offset: 0,
            buffer,
        }
    }

    pub fn sub_surface(&self, x: usize, y: usize, width: isize, height: isize) -> Self {
        Self {
            buffer: FrameBuffer {
                width: if width > 0 {
                    width as usize
                } else {
                    self.width - x - (-width as usize)
                },
                height: if height > 0 {
                    height as usize
                } else {
                    self.height - y - (-height as usize)
                },
                pitch: self.pitch,
                addr: unsafe { self.addr.add(y * self.pitch + x) },
            },
            x: 0,
            y: 0,
            color: self.color,
            root_width: self.root_width,
            root_offset: y * self.root_width + x + self.root_offset,
        }
    }

    pub fn enable_cursor(&mut self, start: u8, end: u8) {
        CRTC_INDEX.out_u8(0x0A);
        CRTC_DATA.out_u8((CRTC_DATA.in_u8() & 0xC0) | start);
        CRTC_INDEX.out_u8(0x0B);
        CRTC_DATA.out_u8((CRTC_DATA.in_u8() & 0xE0) | end);
    }

    pub fn disable_cursor(&mut self) {
        CRTC_INDEX.out_u8(0x0A);
        CRTC_DATA.out_u8(0x20);
    }

    pub fn get_cursor(&self) -> usize {
        self.y * self.root_width + self.x + self.root_offset
    }

    pub fn set_cursor(&mut self, pos: usize) {
        CRTC_INDEX.out_u8(0x0F);
        CRTC_DATA.out_u8((pos & 0xFF) as u8);
        CRTC_INDEX.out_u8(0x0E);
        CRTC_DATA.out_u8(((pos >> 8) & 0xFF) as u8);
    }

    pub fn update_cursor(&mut self) {
        self.set_cursor(self.get_cursor())
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = entry_color(fg, bg);
    }

    pub fn write_at(&mut self, idx: usize, byte: u8, color: u8) {
        unsafe { *self.addr.add(idx) = entry(byte, color) };
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.write_at(y * self.pitch + x, b' ', self.color);
            }
        }
    }

    pub fn border(&mut self, style: Border) {
        if self.width < 2 || self.height < 2 {
            return;
        }
        #[rustfmt::skip]
        let (no, ne, so, se, n, s, o, e) = match style {
            Border::Simple => (218, 191, 192, 217, 196, 196, 179, 179),
            Border::Double => (201, 187, 200, 188, 205, 205, 186, 186),
            Border::Thick => (219, 219, 219, 219, 223, 220, 219, 219),
            Border::Custom { corners: c, v, h } => (c, c, c, c, h, h, v, v),
            Border::FullCustom { no, ne, so, se, n, s, o, e} => (no, ne, so, se, n, s, o, e),
        };
        for i in 1..self.width - 1 {
            self.write_at(i, n, self.color);
            self.write_at(i + (self.height - 1) * self.pitch, s, self.color);
        }
        for i in 1..self.height - 1 {
            self.write_at(i * self.pitch, o, self.color);
            self.write_at(i * self.pitch + self.width - 1, e, self.color);
        }
        self.write_at(0, no, self.color);
        self.write_at(self.width - 1, ne, self.color);
        self.write_at((self.height - 1) * self.pitch, so, self.color);
        self.write_at(
            (self.height - 1) * self.pitch + self.width - 1,
            se,
            self.color,
        );
    }

    pub fn wallpaper(&mut self, img: &[u8]) {
        let len = self.width * self.height;
        if img.len() < len {
            return;
        }
        for y in 0..self.height {
            for x in 0..self.width {
                self.write_at(y * self.pitch + x, b' ', img[y * self.width + x]);
            }
        }
    }
}

impl WriteBytes for Console {
    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.x = 0;
            self.y += 1;
        } else {
            self.write_at(self.pitch * self.y + self.x, byte, self.color);
            self.x += 1;
            if self.x >= self.width {
                self.x = 0;
                self.y += 1;
            }
        }
        if self.y >= self.height {
            // TODO: scroll
            self.y = 0;
        }
        self.update_cursor();
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
