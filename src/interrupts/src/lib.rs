#![feature(const_fn)]
#![feature(asm)]
#![no_std]

#[macro_use]
extern crate vga;

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
    fn isr33();
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

static mut IDT: Idt = Idt::new();

#[no_mangle]
pub static mut IDT_POINTER: IdtPointer = IdtPointer { limit: 0, base: 0 };

pub fn install() {
    unsafe {
        IDT.set_isr(0, isr0 as u64);
        IDT.set_isr(1, isr1 as u64);
        IDT.set_isr(2, isr2 as u64);
        IDT.set_isr(3, isr3 as u64);
        IDT.set_isr(4, isr4 as u64);
        IDT.set_isr(5, isr5 as u64);
        IDT.set_isr(6, isr6 as u64);
        IDT.set_isr(7, isr7 as u64);
        IDT.set_isr(8, isr8 as u64);
        IDT.set_isr(9, isr9 as u64);
        IDT.set_isr(10, isr10 as u64);
        IDT.set_isr(11, isr11 as u64);
        IDT.set_isr(12, isr12 as u64);
        IDT.set_isr(13, isr13 as u64);
        IDT.set_isr(14, isr14 as u64);
        IDT.set_isr(15, isr15 as u64);
        IDT.set_isr(16, isr16 as u64);
        IDT.set_isr(17, isr17 as u64);
        IDT.set_isr(18, isr18 as u64);
        IDT.set_isr(19, isr19 as u64);
        IDT.set_isr(20, isr20 as u64);
        IDT.set_isr(21, isr21 as u64);
        IDT.set_isr(22, isr22 as u64);
        IDT.set_isr(23, isr23 as u64);
        IDT.set_isr(24, isr24 as u64);
        IDT.set_isr(25, isr25 as u64);
        IDT.set_isr(26, isr26 as u64);
        IDT.set_isr(27, isr27 as u64);
        IDT.set_isr(28, isr28 as u64);
        IDT.set_isr(29, isr29 as u64);
        IDT.set_isr(30, isr30 as u64);
        IDT.set_isr(31, isr31 as u64);
        IDT.set_isr(32, isr32 as u64);
        IDT.set_isr(33, isr33 as u64);

        IDT_POINTER.limit = core::mem::size_of::<Idt>() as u16;
        IDT_POINTER.base = &IDT as *const _ as u64;

        load_idt();
    }
}

pub unsafe fn enable() {
    asm!("sti" :::: "volatile");
}

unsafe fn load_idt() {
    asm!("lidt $0"::"*m"(&IDT_POINTER)::"volatile");
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
