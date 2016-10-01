#![feature(asm)]
#![no_std]

#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let ret : u8;
    asm!("inb $1, $0" : "={ax}"(ret) : "{dx}N"(port) : : "volatile");
    return ret;
}

#[inline]
unsafe fn outb(port: u16, val: u8) {
    asm!("outb $1, $0" : : "{dx}N"(port), "{al}"(val) : : "volatile");
}

pub fn remap() {
    unsafe {
        let pic1_mask = inb(0x21);
        let pic2_mask = inb(0xA1);

        // initialize both PICs
        outb(0x20, 0x11);
        outb(0xA0, 0x11);

        // set vector offset of pic1 to 0x20
        outb(0x21, 0x20);
        // set vector offset of pic2 to 0x28
        outb(0xA1, 0x28);

        // tell PIC1 about PIC2 at IRQ2 (0000 0100)
        outb(0x21, 4);

        // tell PIC2 its cascade identity (0000 0010)
        outb(0xA1, 2);

        // set both PICs to 8086 mode
        outb(0x21, 0x1);
        outb(0xA1, 0x1);

        // restore masks
        outb(0x21, pic1_mask);
        outb(0xA1, pic2_mask);
    }
}

pub fn eoi_for(interrupt_number: isize) {
    unsafe {
        match interrupt_number {
            i if i >= 40 => {
                outb(0xA0, 0x20);
                outb(0x20, 0x20);
            },
            32...40 => outb(0x20, 0x20),
            _ => {},
        }
    }
}

