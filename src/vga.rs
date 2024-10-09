use core::fmt;

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
}

const fn entry_color(fg: Color, bg: Color) -> u8 {
    fg as u8 | (bg as u8) << 4
}
const fn entry(c: u8, color: u8) -> u16 {
    c as u16 | (color as u16) << 8
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

impl Console {
    pub fn new(buffer: FrameBuffer, fg: Color, bg: Color) -> Self {
        Self {
            buffer,
            x: 0,
            y: 0,
            color: entry_color(fg, bg),
        }
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = entry_color(fg, bg);
    }

    pub fn write_at(&mut self, idx: usize, byte: u8, color: u8) {
        unsafe { *self.buffer.addr.add(idx) = entry(byte, color) };
    }

    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' || self.x >= self.buffer.width {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= self.buffer.height {
            self.y = 0;
        }
        if byte == b'\n' {
            return;
        }
        self.write_at(self.buffer.pitch * self.y + self.x, byte, self.color);
        self.x += 1;
    }

    pub fn clear(&mut self, bg: Color) {
        let color = entry_color(bg, bg);
        for y in 0..self.buffer.height {
            for x in 0..self.buffer.width {
                self.write_at(y * self.buffer.pitch + x, b' ', color);
            }
        }
    }

    pub fn wallpaper(&mut self, img: &[u8]) {
        let len = self.buffer.width * self.buffer.height;
        if img.len() < len {
            return;
        }
        for y in 0..self.buffer.height {
            for x in 0..self.buffer.width {
                self.write_at(
                    y * self.buffer.pitch + x,
                    b' ',
                    img[y * self.buffer.width + x],
                );
            }
        }
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
