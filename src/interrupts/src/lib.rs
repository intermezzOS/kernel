#![feature(const_fn)]
#![feature(asm)]
#![no_std]

#[macro_use]
extern crate vga;

extern {
    fn load_idt();
    fn isr33();
    static idt: u64;
    static idt_pointer: IdtPointer;
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
    const fn new() -> Idt {
        Idt {
            entries: [IdtEntry {
                base_low: 0,
                selector: 0,
                zero: 0,
                flags: 0,
                base_mid: 0,
                base_high: 0,
                reserved: 0,
            }; 256],
        }
    }

    fn set_isr(&mut self, num: u8, base: u64) {
        let base_low = (base - 0x100000) as u16;

        let new_isr = IdtEntry {
            base_low: base_low,
            selector: 0x8,
            zero: 0,
            flags: 0x8e,
            base_mid: 0x10,
            base_high: 0,
            reserved: 0,
        };

        self.entries[num as usize] = new_isr;
    }
}

static mut IDT: Idt = Idt::new();

#[no_mangle]
pub static mut IDT_POINTER: IdtPointer = IdtPointer { limit: 0, base: 0 };

pub fn install() {
    unsafe {
        let my_idt = &idt as *const _ as *mut Idt;

        let isr33_base_low = (isr33 as u64 - 0x100000) as u16;
     
        let keyboard_isr = (*my_idt).entries[33];

        if isr33_base_low == keyboard_isr.base_low {
            kprintln!("ERROR: keyboard isr already installed");
        }

        (*my_idt).set_isr(33, isr33 as u64);

        let keyboard_isr = (*my_idt).entries[33];

        if isr33_base_low != keyboard_isr.base_low {
            kprintln!("ERROR: keyboard isr not properly installed");
        }

        IDT_POINTER.limit = 4096;
        IDT_POINTER.base = my_idt as u64;

        load_idt();
    }
}

pub unsafe fn enable() {
    asm!("sti" :::: "volatile");
}

#[no_mangle]
pub extern "C" fn interrupt_handler(interrupt_number: isize, error_code: isize) {
    match interrupt_number {
        32 => {}, // timer
        _ => panic!("interrupt {} with error code 0x{:x}", interrupt_number, error_code),
    }
    unsafe{
        send_eoi(interrupt_number);
        enable();
    };
}

#[no_mangle]
pub extern fn pagefault_handler(address: usize, error_code: isize) {
    panic!("pagefault at 0x{:x} with error code {}", address, error_code)
}

#[no_mangle]
pub extern fn general_protection_fault_handler(address: usize, error_code: isize) {
    panic!("general protection fault at 0x{:x} with error code {}", address, error_code)
}

#[no_mangle]
pub extern fn keyboard_handler(interrupt_number: isize, key_code: usize) {
    assert!(interrupt_number == 33);
    kprintln!("Key code!: {}", key_code);
    unsafe{
        send_eoi(interrupt_number);
        enable();
    }
}

unsafe fn send_eoi(interrupt_number: isize) {
    match interrupt_number {
        i if i >= 40 => {
            asm!("outb %al, %dx" :: "{dx}"(0x20), "{al}"(0x20) :: "volatile");
            asm!("outb %al, %dx" :: "{dx}"(0xA0), "{al}"(0x20) :: "volatile");
        },
        32...40 => asm!("outb %al, %dx" :: "{dx}"(0x20), "{al}"(0x20) :: "volatile"),
        _ => {},
    }
}
