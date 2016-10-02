use core;

use console::Vga;
use keyboard::Keyboard;

use interrupts::IdtRef;

use spin::Mutex;

#[macro_use]
mod kprint;

pub struct Context {
    pub vga: Mutex<Vga<&'static mut [u8]>>,
    pub idt: IdtRef,
    pub keyboard: Mutex<Keyboard>,
}

impl Context {
    pub fn new() -> Context {
        let slice = unsafe {
            core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000)
        };

        Context {
            vga: Mutex::new(Vga::new(slice)),
            idt: IdtRef::new(),
            keyboard: Mutex::new(Keyboard::new()),
        }
    }
}
