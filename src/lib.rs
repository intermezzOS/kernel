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

    let color: u8 = 0x4f;

    let hello = "Hello from Rust world!";

    for (i, c) in hello.bytes().enumerate() {
        unsafe {
            let vga = vga.offset((i * 2) as isize);
            *vga = c;

            let vga = vga.offset(1 as isize);
            *vga = color;
        }
    }

}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[lang = "begin_unwind"]
pub extern fn begin_unwind() {}
