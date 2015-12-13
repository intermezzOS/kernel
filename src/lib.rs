#![feature(lang_items)]
#![no_std]

extern crate rlibc;
extern crate vga;

pub mod support; // For Rust lang items

#[no_mangle]
pub extern fn kmain() {
    let color = 0x0a;
    let hello = "Hello from Rust world!";

    vga::clear_console();
    vga::kprintf(hello, color);
    vga::kprintf("\n", color);
    vga::kprintf(hello, color);
}
