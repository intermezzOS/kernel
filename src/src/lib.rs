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
        interrupts::reload_idt();
        interrupts::enable();
        interrupts::print_idt_info();
    };

    kprintln!("Setup complete.");
    loop { }
}
