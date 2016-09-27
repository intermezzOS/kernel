#![feature(lang_items)]

#![no_std]
#![no_main]

extern crate rlibc;

#[macro_use]
extern crate intermezzos;

// For Rust lang items
#[cfg(not(test))]
pub mod support;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let ctx = intermezzos::kernel::Context::new();

    kprintln!(ctx, "Kernel initialized.");

    loop { }
}
