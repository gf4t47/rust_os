#![no_std] // don't link the Rust standard library
#![no_main]

extern crate bootloader_precompiled;
extern crate volatile;
extern crate spin;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("Some panic message");
    println!("Hello World{}", "!");

    loop {}
}
