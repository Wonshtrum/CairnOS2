pub mod serial;
pub mod vga;

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => {
        let _ = write!($crate::STDOUT.get_mut(), $($args)+);
    };
}

#[macro_export]
macro_rules! println {
    ($($args:tt)+) => {
        let _ = writeln!($crate::STDOUT.get_mut(), $($args)+);
    };
}

#[macro_export]
macro_rules! eprint {
    ($($args:tt)+) => {
        let _ = write!($crate::SERIAL.get_mut(), $($args)+);
    };
}

#[macro_export]
macro_rules! eprintln {
    ($($args:tt)+) => {
        let _ = writeln!($crate::SERIAL.get_mut(), $($args)+);
    };
}

pub trait WriteBytes {
    fn write_byte(&mut self, byte: u8);

    fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
    }
}
