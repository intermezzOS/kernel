#![feature(lang_items)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]

#![no_std]
#![no_main]

extern crate rlibc;
extern crate spin;

extern crate console;
use console::Vga;

#[macro_use]
extern crate interrupts;
extern crate x86;
use x86::bits64::irq::IdtEntry;

use core::intrinsics;

extern crate pic;

use spin::Mutex;

#[macro_use]
extern crate intermezzos;

#[cfg(not(test))]
pub mod panic;

static mut PRINT_REF: Option<&'static Mutex<Vga<&'static mut [u8]>>> = None;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let mut ctx = intermezzos::kernel::Context::new();

    // ... yeah this doesn't quite cut it. SO UNSAFE.
    unsafe {
        // so here, we use transmuate to elongate the lifetime to static.
        PRINT_REF = core::mem::transmute(Some(&ctx.vga));
    }

    pic::remap();

	let gpf = make_idt_entry!(isr13, {
		panic!("omg GPF");
	});

    let timer = make_idt_entry!(isr32, {
        pic::eoi_for(32);

    });

    let keyboard = make_idt_entry!(isr33, {
        let scancode = unsafe { pic::inb(0x60) };

        unsafe {
            use core::fmt::Write;
            let mut vga = ::PRINT_REF.unwrap().lock();
            vga.write_fmt(format_args!("got keycode: {}", scancode)).unwrap();
            vga.flush();
        }

        pic::eoi_for(33);
    });

    ctx.idt.set_handler(13, gpf);
    ctx.idt.set_handler(32, timer);
    ctx.idt.set_handler(33, keyboard);

    ctx.idt.enable_interrupts();

    kprintln!(ctx, "Kernel initialized.");

    loop { }
}
