use spin::Mutex;
use vga::Vga;

#[macro_use]
mod kprint;

pub struct Context {
    pub vga: Mutex<Vga>,
}

impl Context {
    pub const fn new() -> Context {
        unsafe {
            Context {
                vga: Mutex::new(Vga::new(0xb8000 as *mut u8)),
            }
        }
    }
}
