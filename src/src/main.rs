#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]

#![no_std]
#![no_main]

extern crate rlibc;
extern crate spin;
extern crate vga;

#[macro_use]
extern crate kernel_context;
use kernel_context::KernelContext;

// For Rust lang items
#[cfg(not(test))]
pub mod support;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let ctx = KernelContext::new();

    kprintln!(ctx, "Kernel initialized.");

    loop { }
}
