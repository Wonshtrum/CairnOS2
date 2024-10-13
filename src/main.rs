#![no_std]
#![no_main]
#![allow(clippy::identity_op)]

mod arch;
mod builtins;
mod io;
mod lazy;
mod mem;
mod multiboot;

use core::fmt::Write;
use core::panic::PanicInfo;

use io::serial;
use io::vga::{self, Border, Color};
use io::WriteBytes;
use lazy::{Lazy, LazyMut};

struct Context {
    info: &'static multiboot::Info,
    mmaps: &'static [mem::Mmap],
}

const WALLPAPER: &[u8] = include_bytes!("../assets/wallpaper.vga");

static SERIAL: LazyMut<serial::Console> = LazyMut::new();
static SCREEN: LazyMut<vga::Console> = LazyMut::new();
static STDOUT: LazyMut<&mut dyn Write> = LazyMut::new();
static CONTEXT: Lazy<Context> = Lazy::new();

#[no_mangle]
pub extern "C" fn kernel_main(info: &'static multiboot::Info, magic: u32) {
    if magic != multiboot::MAGIC {
        panic!("Wrong multiboot magic number");
    }

    if let Ok(console) = serial::Console::try_new(serial::PORT_COM1) {
        unsafe { SERIAL.init(console) };
        eprintln!("\nSerial console initialized");
    } else {
        panic!("Could not initialize serial console")
    }

    eprintln!("Multiboot flags: {:013b}", info.get_flags());
    eprintln!("Multiboot infos: {info:#?}");

    if let Some(framebuffer) = info.get_framebuffer() {
        let mut console = vga::Console::new(framebuffer, Color::LightGrey, Color::Black);
        eprintln!("VGA console initialized");
        // console.enable_cursor(0, 15); // full cursor
        console.wallpaper(WALLPAPER);
        unsafe { SCREEN.init(console) };
        unsafe { STDOUT.init(SCREEN.get_mut()) };
    } else {
        unsafe { STDOUT.init(SERIAL.get_mut()) };
    }

    println!("Hello from CairnOS!");
    println!("{:b}", info.get_flags());
    println!("{:?}", info.get_mem());
    println!("{:?}", info.get_boot_device());
    println!("{:?}", info.get_cmdline());
    println!("{:?}", info.get_mods());
    println!("{:?}", info.get_syms());
    println!("{:#?}", info.get_mmaps());
    println!("{:?}", info.get_drives());
    println!("{:?}", info.get_config_table());
    println!("{:?}", info.get_boot_loader_name());
    println!("{:?}", info.get_apm_table());
    println!("{:?}", info.get_vbe());
    println!("{:?}", info.get_framebuffer());

    if let Some(mmaps) = info.get_mmaps() {
        unsafe { CONTEXT.init(Context { info, mmaps }) };
    } else {
        panic!("Could not get memory map entries");
    }

    println!("Bye!");
}

#[panic_handler]
fn kernel_collapse(info: &PanicInfo) -> ! {
    extern "C" {
        fn kernel_hlt() -> !;
    }

    fn console_message<W: Write + WriteBytes>(console: &mut W, info: &PanicInfo, peter: &[u8]) {
        console.write_bytes(peter);
        if let Some(location) = info.location() {
            let _ = writeln!(console, "CairnOS collapsed at {location}:");
        } else {
            let _ = writeln!(console, "CairnOS collapsed:");
        }
        let _ = write!(console, "{}", info.message());
    }

    if let Some(console) = SERIAL.try_get_mut() {
        const PETER: &[u8] = include_bytes!("../assets/Peter");
        console_message(console, info, PETER);
        let _ = console.write_str("\n\n");
    }
    if let Some(console) = SCREEN.try_get_mut() {
        const PETER: &[u8] = include_bytes!("../assets/Peter.vga");
        console.set_color(Color::White, Color::Blue);
        console.clear();
        console.border(Border::Double);
        let mut console = console.sub_surface(2, 1, -2, -1);
        console_message(&mut console, info, PETER);
    }

    unsafe { kernel_hlt() }
}
