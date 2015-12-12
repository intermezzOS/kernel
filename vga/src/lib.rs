#![no_std]

const CONSOLE_SIZE: isize = 4000;

struct VgaBuffer {
    location: *mut u8,
    position: usize,
}

impl VgaBuffer {
    fn new() -> VgaBuffer {
        VgaBuffer {
            location: 0xb8000 as *mut u8,
            position: 0,
        }
    }

    fn write_byte(&mut self, byte: u8, color: u8) {
        unsafe {
            let location = self.location.offset(self.position as isize);

            *location = byte;
            let location = location.offset(1);
            *location = color;

            self.position = self.position + 2;
        }
    }
}

/// Prints a string
pub fn kprintf(s: &str, color: u8) {
    let mut buffer = VgaBuffer::new();

    for byte in s.bytes() {
        buffer.write_byte(byte, color);
    }
}

/// Clears the console
pub fn clear_console() {
    let mut buffer = VgaBuffer::new();
    let space = ' ' as u8;
    let color = 0x0a;

    for _ in 0..CONSOLE_SIZE {
        buffer.write_byte(space, color);
    }
}

