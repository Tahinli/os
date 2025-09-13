#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello from TahinliOS!");

    loop {
        panic!("Panic or not, it's the whole thing.");
    }
}
