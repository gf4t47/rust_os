#![no_std] // don't link the Rust standard library
#![feature(abi_x86_interrupt)]

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
extern crate uart_16550;
extern crate x86_64;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

// NEW: We need to add `pub` here to make them accessible from the outside
#[macro_use]
pub mod vga_buffer;
pub mod serial;
pub mod interrupts;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}
