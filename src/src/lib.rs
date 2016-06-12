#![feature(lang_items)]
#![feature(asm)]
#![no_std]

extern crate rlibc;

#[macro_use]
extern crate vga;

extern crate interrupts;
extern crate keyboard;
extern crate pic;

// For Rust lang items
#[cfg(not(test))]
pub mod support;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    pic::remap();

    vga::initialize();

    interrupts::install();
    interrupts::enable();

    vga::clear_console();
    kprintln!("Kernel initialized.");

    loop { }
}
