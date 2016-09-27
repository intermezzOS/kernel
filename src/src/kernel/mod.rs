use core;
use vga::Vga;
use spin::Mutex;

#[macro_use]
mod kprint;

pub struct Context {
    pub vga: Mutex<Vga<'static>>,
}

impl Context {
    pub fn new() -> Context {
        let slice = unsafe {
            core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000)
        };

        Context {
            vga: Mutex::new(Vga::new(slice)),
        }
    }
}

