#![feature(lang_items)]
#![feature(start)]
#![feature(no_std)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![no_std]

#[no_mangle]
pub extern fn kmain() {

    let lol = 0x914c914F914c;
    let vga = 0xb8000 as *mut usize;

    unsafe {
        *vga = lol;
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
