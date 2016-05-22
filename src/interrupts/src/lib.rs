#![no_std]
#![feature(asm)]

pub unsafe fn enable() {
    asm!("sti" :::: "volatile");
}
