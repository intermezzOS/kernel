#![feature(lang_items)]

#![no_std]
#![no_main]

extern crate rlibc;
extern crate spin;

extern crate console;
use console::Vga;

extern crate pic;

use spin::Mutex;

#[macro_use]
extern crate intermezzos;

#[cfg(not(test))]
pub mod panic;

static mut PRINT_REF: Option<&'static Mutex<Vga<&'static mut [u8]>>> = None;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let ctx = intermezzos::kernel::Context::new();

    // ... yeah this doesn't quite cut it. SO UNSAFE.
    unsafe {
        // so here, we use transmuate to elongate the lifetime to static.
        PRINT_REF = core::mem::transmute(Some(&ctx.vga));
    }

    pic::remap();
    ctx.idt.enable_interrupts();

    kprintln!(ctx, "Kernel initialized.");

    loop { }
}
