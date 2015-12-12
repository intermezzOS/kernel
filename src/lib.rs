#![feature(lang_items)]
#![no_std]

extern crate rlibc;
extern crate vga;

pub mod support; // For Rust lang items

#[no_mangle]
pub extern fn kmain() {
    let vga = 0xb8000 as *mut u8;
    let color = 0x0a;
    let hello = "Hello from Rust world!";

    clear_console(vga);
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

    if s.len() % 2 != 0 {
        offset -= 1;
    }

    let location = unsafe { location.offset(offset) };

    vga::kprintf(s, color, location);
}

/// Clears the console
fn clear_console(location: *mut u8) {
    let color = 0x0a;
    let c = ' ' as u8;
    for i in 0..CONSOLE_SIZE {
        unsafe {
            let location = location.offset((i * 2) as isize);
            *location = c;

            let location = location.offset(1 as isize);
            *location = color;
        }
    }
}

