#![feature(lang_items)]
#![no_std]

extern crate rlibc;

#[macro_use]
extern crate vga;

pub mod support; // For Rust lang items

#[no_mangle]
pub extern "C" fn kmain() {
    vga::clear_console();

    kprintln!("Hello from Rust world!");
    kprint!("Hello");
    kprintln!(" again!");

    let x = 5;
    let p = &x;

    kprintln!("Hello a final time: {:p}", p);
}
