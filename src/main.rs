#![feature(lang_items)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]

#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;

extern crate rlibc;
extern crate spin;

extern crate console;

#[macro_use]
extern crate interrupts;
extern crate x86;
use x86::bits64::irq::IdtEntry;

use core::intrinsics;

extern crate keyboard;

extern crate pic;

#[cfg(not(test))]
pub mod panic;

#[macro_use]
extern crate intermezzos;

lazy_static! {
    static ref CONTEXT: intermezzos::kernel::Context = intermezzos::kernel::Context::new();
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    pic::remap();

    let gpf = make_idt_entry!(isr13, {
        panic!("omg GPF");
    });

    let timer = make_idt_entry!(isr32, {
        pic::eoi_for(32);
    });

    let keyboard = make_idt_entry!(isr33, {
        let scancode = unsafe { pic::inb(0x60) };

        if let Some(c) = keyboard::from_scancode(scancode as usize) {
            kprint!(CONTEXT, "{}", c);
        }

        pic::eoi_for(33);
    });

    CONTEXT.idt.set_handler(13, gpf);
    CONTEXT.idt.set_handler(32, timer);
    CONTEXT.idt.set_handler(33, keyboard);

    CONTEXT.idt.enable_interrupts();

    kprintln!(CONTEXT, "Kernel initialized.");

    loop { }
}
