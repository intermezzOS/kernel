#![feature(lang_items)]
#![feature(start)]
#![feature(no_std)]
#![feature(asm)]
#![feature(core_str_ext)]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub extern fn kmain() {
    let vga = 0xb8000 as *mut u8;
    let color = 0x4f;
    let hello = "Hello from Rust world!";

    print_string(hello, color, vga);
}

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

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[lang = "begin_unwind"]
pub extern fn begin_unwind() {}
