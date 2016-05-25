#![feature(const_fn)]
#![feature(asm)]
#![no_std]

#[macro_use]
extern crate vga;

extern {
    fn load_idt(idt: *const IdtPointer);
    fn isr0();
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

        self.entries[33] = new_isr;
    }
}

static mut IDT: Idt = Idt::new();

#[no_mangle]
pub static mut IDT_POINTER: IdtPointer = IdtPointer { limit: 0, base: 0 };

pub fn print_idt_info() {
    kprintln!("IDT INFO");
    kprintln!("--------");
    kprintln!("idt: {:?}", idt);
    kprintln!("idt_pointer: {:#?}", idt_pointer);
    kprintln!("equal: {}", idt_pointer.base == &idt as *const _ as u64);
    let my_idt = &idt as *const _ as *mut Idt;
 

    let keyboard_isr = unsafe { (*my_idt).entries[33] };
    let base = isr33 as u64;
    let base_low = (base - 0x100000) as u16;
    kprintln!("base_low: {}", base_low);
    kprintln!("equal: {}", base_low == keyboard_isr.base_low);

    kprintln!("OVERWRITING KEYBOARD ISR");
    unsafe {
        (*my_idt).set_isr(33, isr33 as u64);
    };
    kprintln!("OVERWRITTEN!!!!!");
    let keyboard_isr = unsafe { (*my_idt).entries[33] };
    let base = isr33 as u64;
    let base_low = (base - 0x100000) as u16;
    kprintln!("equal: {}", base_low == keyboard_isr.base_low);
}

pub fn install() {
    // Modifying static muts are very unsafe. But we have no concurrency here, so we're not worried
    // about data races.
    unsafe {

        IDT_POINTER.limit = 4096; // (core::mem::size_of::<IdtEntry>() as u16 * 256) - 1;
        IDT_POINTER.base = idt; // &IDT as *const Idt as u64;

        // 0x8E for flags means: present, ring0, and lower 5 bits are 14, which is required
        //IDT.set_gate(0, isr0 as u64, 0x08, 0x8E);

        //load_idt(&IDT_POINTER as *const IdtPointer);
    } }

// TODO: mark this unsafe?
pub fn reload_idt() {
	unsafe {
        load_idt(&IDT_POINTER as *const IdtPointer);
    }
}

// #[repr(C)]
// #[repr(packed)]
// pub struct Regs {
//     gs: u16,
//     fs: u16,
//     es: u16,
//     ds: u16,
//     
//     edi: u16,
//     esi: u16,
//     ebp: u16,
//     esp: u16,
//     ebx: u16,
//     edx: u16,
//     ecx: u16,
//     eax: u16,
// 
//     int_no: u16,
//     err_code: u16,
// 
//     eip: u16,
//     cs: u16,
//     eflags: u16,
//     useresp: u16,
//     ss: u16,
// }
// 
// #[no_mangle]
// pub extern fn fault_handler(_regs: *const Regs) -> ! {
//     kprintln!("fault handler called");
//     loop {};
//    // unsafe {
//    //     if (*regs).int_no < 32 {
//    //         kprintln!("fault handler called with number: {}", (*regs).int_no);
//    //         loop {};
//    //     }
//    // }
// }


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
