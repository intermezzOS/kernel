#![feature(asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![no_std]

#[macro_use]
extern crate vga;
#[macro_use]
extern crate lazy_static;
extern crate keyboard;
extern crate pic;

use keyboard::Keyboard;
use core::intrinsics;

macro_rules! make_idt_entry {
    ($number:expr) => {{
        unsafe extern fn handler() {
            kprintln!("unhandled interrupt #{}", $number);
        }

        IdtEntry::new(handler)
    }};
    ($name:ident, $body:expr) => {{
        fn body() {
            $body
        }

        #[naked]
        unsafe extern fn $name() {
            asm!("push rbp
                  push r15
                  push r14
                  push r13
                  push r12
                  push r11
                  push r10
                  push r9
                  push r8
                  push rsi
                  push rdi
                  push rdx
                  push rcx
                  push rbx
                  push rax

                  mov rsi, rsp
                  push rsi
                  
                  call $0

                  add rsp, 8

                  pop rax
                  pop rbx
                  pop rcx
                  pop rdx
                  pop rdi
                  pop rsi
                  pop r8
                  pop r9
                  pop r10
                  pop r11
                  pop r12
                  pop r13
                  pop r14
                  pop r15
                  pop rbp

                  iretq" :: "s"(body as fn()) :: "volatile", "intel");
            intrinsics::unreachable();
        }

        IdtEntry::new($name)
    }};
}

#[derive(Copy,Clone,Debug)]
#[repr(packed,C)]
struct IdtEntry {
    base_low: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    base_mid: u16,
    base_high: u32,
    reserved: u32,
}

impl IdtEntry {
    fn new(f: unsafe extern fn()) -> IdtEntry {
        let base = f as u64;

        IdtEntry {
            base_low: base as u16,
            selector: 0x8,
            zero: 0,
            flags: 0x8e,
            base_mid: (base >> 16) as u16,
            base_high: (base >> 32) as u32,
            reserved: 0,
        }
    }
}

#[derive(Debug)]
#[repr(packed,C)]
pub struct IdtPointer {
    limit: u16,
    base: u64,
}

#[repr(packed,C)]
struct Idt {
    entries: [IdtEntry; 256],
}

impl Idt {
    fn install(&'static self) {
        let idt_pointer = IdtPointer {
            limit: core::mem::size_of::<Idt>() as u16,
            base: self as *const _ as u64,
        };

        unsafe {
            load_idt(&idt_pointer);
        }
    }

    fn set_isr(&mut self, num: u8, entry: IdtEntry) {
        self.entries[num as usize] = entry;
    }
}

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt {
            entries: [IdtEntry {
                base_low: 0,
                selector: 0,
                zero: 0,
                flags: 0,
                base_mid: 0,
                base_high: 0,
                reserved: 0,
            }; 256]
        };

        idt.set_isr(0, make_idt_entry!(0));
        idt.set_isr(1, make_idt_entry!(1));
        idt.set_isr(2, make_idt_entry!(2));
        idt.set_isr(3, make_idt_entry!(3));
        idt.set_isr(4, make_idt_entry!(4));
        idt.set_isr(5, make_idt_entry!(5));
        idt.set_isr(6, make_idt_entry!(6));
        idt.set_isr(7, make_idt_entry!(7));
        idt.set_isr(8, make_idt_entry!(8));
        idt.set_isr(9, make_idt_entry!(9));
        idt.set_isr(10, make_idt_entry!(10));
        idt.set_isr(11, make_idt_entry!(11));
        idt.set_isr(12, make_idt_entry!(12));
        idt.set_isr(13, make_idt_entry!(13));
        idt.set_isr(14, make_idt_entry!(14));
        idt.set_isr(15, make_idt_entry!(15));
        idt.set_isr(16, make_idt_entry!(16));
        idt.set_isr(17, make_idt_entry!(17));
        idt.set_isr(18, make_idt_entry!(18));
        idt.set_isr(19, make_idt_entry!(19));
        idt.set_isr(20, make_idt_entry!(20));
        idt.set_isr(21, make_idt_entry!(21));
        idt.set_isr(22, make_idt_entry!(22));
        idt.set_isr(23, make_idt_entry!(23));
        idt.set_isr(24, make_idt_entry!(24));
        idt.set_isr(25, make_idt_entry!(25));
        idt.set_isr(26, make_idt_entry!(26));
        idt.set_isr(27, make_idt_entry!(27));
        idt.set_isr(28, make_idt_entry!(28));
        idt.set_isr(29, make_idt_entry!(29));
        idt.set_isr(30, make_idt_entry!(30));
        idt.set_isr(31, make_idt_entry!(31));

        idt.set_isr(32, make_idt_entry!(isr32, {
            // timer, do nothing for now
            pic::eoi_for(32);
            enable();
        }));

        idt.set_isr(33, make_idt_entry!(isr33, {
            let scancode = unsafe { inb(0x60) };
            Keyboard.handle_keys(scancode as usize);

            send_eoi_for(33);
            enable();
        }));

        idt
    };
}

/// This function is needed because the macros can't handle it, but can handle this. For some
/// reason.
fn send_eoi_for(interrupt: isize) {
    pic::eoi_for(interrupt);
}

pub fn install() {
    IDT.install();
}

pub fn enable() {
    unsafe {
        asm!("sti" :::: "volatile");
    }
}

unsafe fn load_idt(ptr: &IdtPointer) {
    asm!("lidt $0"::"*m"(ptr)::"volatile");
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let ret : u8;
    asm!("inb $1, $0" : "={ax}"(ret) : "{dx}N"(port) : : "volatile");
    return ret;
}
