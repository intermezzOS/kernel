use console::Vga;

use interrupts::IdtRef;

use spin::Mutex;

#[macro_use]
mod kprint;

pub struct Context {
    pub vga: Mutex<Vga<&'static mut [u8]>>,
    pub idt: IdtRef,
}

impl Context {
    pub fn new() -> Context {
        Context {
            vga: Mutex::new(Vga::new()),
            idt: IdtRef::new(),
        }
    }
}
