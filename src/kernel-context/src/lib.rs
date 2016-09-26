#![feature(const_fn)]
#![no_std]

extern crate vga;
extern crate spin;

use vga::Vga;
use spin::Mutex;

#[macro_use]
mod kprint;

pub struct KernelContext {
    pub vga: Mutex<Vga>,
}

impl KernelContext {
    pub const fn new() -> KernelContext {
        unsafe {
            KernelContext {
                vga: Mutex::new(Vga::new(0xb8000 as *mut u8)),
            }
        }
    }
}
