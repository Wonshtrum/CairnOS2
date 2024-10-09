#![no_std]
#![no_main]

use core::fmt::Write;

mod arch;
mod builtins;
mod multiboot;
mod serial;
mod vga;

extern "C" {
    fn kernel_hlt() -> !;
}

#[panic_handler]
fn kernel_panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { kernel_hlt() }
}

const WALLPAPER: &[u8] = include_bytes!("../py-shenanigans/wallpaper.vga");

#[no_mangle]
pub extern "C" fn kernel_main(info: &multiboot::Info, magic: u32) {
    assert!(magic == multiboot::MAGIC);
    //let mut console = unsafe { serial::Console::new(serial::PORT_COM1) };
    let mut console = vga::Console::new(
        info.get_framebuffer().unwrap(),
        vga::Color::LightGrey,
        vga::Color::Black,
    );
    //console.clear(vga::Color::Red);
    console.wallpaper(WALLPAPER);
    writeln!(console, "Hello from CairnOS!");
    writeln!(console, "{:b}", info.get_flags());
    writeln!(console, "{:?}", info.get_mem());
    writeln!(console, "{:?}", info.get_boot_device());
    writeln!(console, "{:?}", info.get_cmdline());
    writeln!(console, "{:?}", info.get_mods());
    writeln!(console, "{:?}", info.get_syms());
    writeln!(console, "{:?}", info.get_mmap());
    writeln!(console, "{:?}", info.get_drives());
    writeln!(console, "{:?}", info.get_config_table());
    writeln!(console, "{:?}", info.get_boot_loader_name());
    writeln!(console, "{:?}", info.get_apm_table());
    writeln!(console, "{:?}", info.get_vbe());
    writeln!(console, "{:?}", info.get_framebuffer());
    //writeln!(console, "{info:#?}");
    writeln!(console, "Bye!");
}
