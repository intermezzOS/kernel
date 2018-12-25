#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate vga;
#[macro_use]
extern crate intermezzos;
extern crate bootloader;

#[cfg(not(test))]
pub mod panic;

use vga::Vga;

#[no_mangle]
pub fn _start() -> ! {
    let slice = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };

    let mut vga = Vga::new(slice);

    kprintln!(vga, "hello world");

    loop {}
}
