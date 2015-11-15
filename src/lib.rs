#![feature(lang_items)]
#![feature(start)]
#![feature(no_std)]
#![feature(asm)]
#![feature(core_str_ext)]
#![no_std]

extern crate rlibc;

pub mod support; // For Rust lang items

#[no_mangle]
pub extern fn kmain() {
    let vga = 0xb8000 as *mut u8;
    let color = 0x4f;
    let hello = "Hello from Rust world!";

    print_center_string(hello, color, vga);
}

const CONSOLE_SIZE : isize = 4000;

/// Prints a string in the center of the screen.
///
/// Only works for short strings, less than a full line.
fn print_center_string(s: &str, color: u8, location: *mut u8) {
    // we offset by the length because we need two bytes per letter
    let mut offset = CONSOLE_SIZE / 2;
    offset -= s.len() as isize;

    let location = unsafe { location.offset(offset) };

    print_string(s, color, location);
}

/// Prints a string
fn print_string(s: &str, color: u8, location: *mut u8) {
    for (i, c) in s.bytes().enumerate() {
        unsafe {
            let location = location.offset((i * 2) as isize);
            *location = c;

            let location = location.offset(1 as isize);
            *location = color;
        }
    }

}

