#![feature(lang_items)]
#![feature(asm)]
#![no_std]

extern crate rlibc;

#[macro_use]
extern crate vga;

extern crate interrupts;

pub mod support; // For Rust lang items

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    vga::clear_console();

    unsafe {
        interrupts::install();
        interrupts::enable();
    };

    kprintln!("Kernel initialized.");

    loop { }
}
