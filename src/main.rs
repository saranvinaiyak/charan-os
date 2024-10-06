#![no_main]
#![no_std]

mod vga_buffer;
use core::panic::PanicInfo;

//target cfg in .cargo/config.toml runs cargo build and passes qemu-system-x86_64 -drive format=raw,file=target/x86_64-charan_os_config/debug/bootimage-charan_os.bin
static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello again supersic!");
    println!("We'll be right back: {}", "!!!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
