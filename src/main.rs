#![no_std]
#![no_main]

use core::fmt::Write;

mod arch;
mod builtins;
mod serial;
mod vga;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const WALLPAPER: &[u8] = include_bytes!("../py-shenanigans/wallpaper.vga");

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    //let mut console = unsafe { serial::Console::new(serial::PORT_COM1).unwrap_unchecked() };
    let mut console = vga::Console::new(80, 25, vga::Color::LightGrey, vga::Color::Black);
    //console.clear(vga::Color::Red);
    console.wallpaper(WALLPAPER);
    let test = vga::Console::new(80, 25, vga::Color::White, vga::Color::Black);
    write!(console, "Hello from CairnOS!\n");
    write!(console, "{test:#?}\n");
    write!(console, "Bye!\n");
    loop {}
}
