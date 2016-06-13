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

extern {
    fn isr0();
    fn isr1();
    fn isr2();
    fn isr3();
    fn isr4();
    fn isr5();
    fn isr6();
    fn isr7();
    fn isr8();
    fn isr9();
    fn isr10();
    fn isr11();
    fn isr12();
    fn isr13();
    fn isr14();
    fn isr15();
    fn isr16();
    fn isr17();
    fn isr18();
    fn isr19();
    fn isr20();
    fn isr21();
    fn isr22();
    fn isr23();
    fn isr24();
    fn isr25();
    fn isr26();
    fn isr27();
    fn isr28();
    fn isr29();
    fn isr30();
    fn isr31();
    fn isr32();
}

macro_rules! define_isr {
    ($name:ident, $body:expr) => {
        #[no_mangle]
        pub extern "C" fn isr33handler(_interrupt_number: isize, _error_code: isize) {
            $body
        }

        #[naked]
        unsafe fn $name() {
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
                  movq rdi, 33
                  push rdi
                  
                  call isr33handler

                  add rsp, 16

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

                  iretq" :::: "volatile", "intel");
            intrinsics::unreachable();
        }
    }
}

define_isr!(isr33, {
    let scancode = unsafe { inb(0x60) };
    Keyboard.handle_keys(scancode as usize);

    pic::eoi_for(33);
    enable();
});

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

    fn set_isr(&mut self, num: u8, base: u64) {
        let new_isr = IdtEntry {
            base_low: base as u16,
            selector: 0x8,
            zero: 0,
            flags: 0x8e,
            base_mid: (base >> 16) as u16,
            base_high: (base >> 32) as u32,
            reserved: 0,
        };

        self.entries[num as usize] = new_isr;
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

        idt.set_isr(0, isr0 as u64);
        idt.set_isr(1, isr1 as u64);
        idt.set_isr(2, isr2 as u64);
        idt.set_isr(3, isr3 as u64);
        idt.set_isr(4, isr4 as u64);
        idt.set_isr(5, isr5 as u64);
        idt.set_isr(6, isr6 as u64);
        idt.set_isr(7, isr7 as u64);
        idt.set_isr(8, isr8 as u64);
        idt.set_isr(9, isr9 as u64);
        idt.set_isr(10, isr10 as u64);
        idt.set_isr(11, isr11 as u64);
        idt.set_isr(12, isr12 as u64);
        idt.set_isr(13, isr13 as u64);
        idt.set_isr(14, isr14 as u64);
        idt.set_isr(15, isr15 as u64);
        idt.set_isr(16, isr16 as u64);
        idt.set_isr(17, isr17 as u64);
        idt.set_isr(18, isr18 as u64);
        idt.set_isr(19, isr19 as u64);
        idt.set_isr(20, isr20 as u64);
        idt.set_isr(21, isr21 as u64);
        idt.set_isr(22, isr22 as u64);
        idt.set_isr(23, isr23 as u64);
        idt.set_isr(24, isr24 as u64);
        idt.set_isr(25, isr25 as u64);
        idt.set_isr(26, isr26 as u64);
        idt.set_isr(27, isr27 as u64);
        idt.set_isr(28, isr28 as u64);
        idt.set_isr(29, isr29 as u64);
        idt.set_isr(30, isr30 as u64);
        idt.set_isr(31, isr31 as u64);
        idt.set_isr(32, isr32 as u64);
        idt.set_isr(33, isr33 as u64);

        idt
    };
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

#[no_mangle]
pub extern "C" fn interrupt_handler(interrupt_number: isize, error_code: isize) {
    match interrupt_number {
        32 => {}, // timer
        33 => keyboard_handler(),
        _ => panic!("interrupt {} with error code {:x}", interrupt_number, error_code),
    }

    pic::eoi_for(interrupt_number);

    enable();
}

fn keyboard_handler() {
    let scancode = unsafe { inb(0x60) };
    Keyboard.handle_keys(scancode as usize);
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let ret : u8;
    asm!("inb $1, $0" : "={ax}"(ret) : "{dx}N"(port) : : "volatile");
    return ret;
}
