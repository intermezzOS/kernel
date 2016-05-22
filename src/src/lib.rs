#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

extern crate rlibc;

#[macro_use]
extern crate vga;

extern crate interrupts;

pub mod support; // For Rust lang items

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    vga::clear_console();

    unsafe { interrupts::enable() };

    kprintln!("Hello from Rust world!");
    kprint!("Hello");
    kprintln!(" again!");

    let x = 5;
    let p = &x;

    kprintln!("Hello a final time: {:p}", p);

    loop { }
}


#[no_mangle]
pub extern "C" fn interrupt_handler(interrupt_number: isize, error_code: isize) {
    match interrupt_number {
        32 => {}, // timer
        _ => panic!("interrupt {} with error code 0x{:x}", interrupt_number, error_code),
    }
    unsafe{
        send_eoi(interrupt_number);
        interrupts::enable();
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
    kprintln!("Key code!: {}", key_code);
    unsafe{
        send_eoi(interrupt_number);
        interrupts::enable();
    }
    assert!(interrupt_number == 33);
}

unsafe fn send_eoi(interrupt_number: isize) {
    let master_port = Port::new(0x20);
    let slave_port = Port::new(0xA0);

    unsafe fn send_eoi(mut port: Port) {port.out8(0x20)}

    match interrupt_number {
        i if i >= 40 => {
            send_eoi(slave_port);
            send_eoi(master_port);
        },
        32...40 => send_eoi(master_port),
        _ => {},
    }
}

pub struct Port(u16);

impl Port {
    pub const fn new(port_address: u16) -> Port {
        Port(port_address)
    }

    pub unsafe fn out8(&mut self, data: u8) {
        asm!("outb %al, %dx" :: "{dx}"(self.0), "{al}"(data) :: "volatile");
    }

    pub unsafe fn out16(&mut self, data: u16) {
        asm!("outw %ax, %dx" :: "{dx}"(self.0), "{ax}"(data) :: "volatile");
    }

    pub unsafe fn out32(&mut self, data: u32) {
        asm!("outl %eax, %dx" :: "{dx}"(self.0), "{eax}"(data) :: "volatile");
    }

    pub unsafe fn in8(&self) -> u8 {
        let ret: u8;
        asm!("inb %dx, %al" : "={al}"(ret) : "{dx}"(self.0) :: "volatile");
        ret
    }

    pub unsafe fn in16(&self) -> u16 {
        let ret: u16;
        asm!("inw %dx, %ax" : "={ax}"(ret) : "{dx}"(self.0) :: "volatile");
        ret
    }

    pub unsafe fn in32(&self) -> u32 {
        let ret: u32;
        asm!("inl %dx, %eax" : "={eax}"(ret) : "{dx}"(self.0) :: "volatile");
        ret
    }
}
