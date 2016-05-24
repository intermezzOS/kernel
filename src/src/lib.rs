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

    kprintln!("Hello from Rust world!");
    kprint!("Hello");
    kprintln!(" again!");

    let x = 5;
    let p = &x;

    kprintln!("Hello a final time: {:p}", p);

    loop { }
}
