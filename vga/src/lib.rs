#![feature(const_fn)]
#![feature(lang_items)]
#![no_std]

extern crate spin;
extern crate rlibc;

use spin::Mutex;

const CONSOLE_SIZE: isize = 4000;
const CONSOLE_COLS: isize = 80;
//const CONSOLE_LINES: isize = 24;

static mut buffer: Mutex<VgaBuffer> = Mutex::new(VgaBuffer {
    location: 0xb8000 as *mut u8,
    position: 0,
});

struct VgaBuffer {
    location: *mut u8,
    position: usize,
}

impl VgaBuffer {
    fn write_byte(&mut self, byte: u8, color: u8) {
        if byte == ('\n' as u8) {
            // two bytes per logical position
            let line_length = CONSOLE_COLS * 2;

            // to get the current line, we divide by the length of a line
            let current_line = (self.position as isize) / line_length;

            let next_line = current_line + 1;

            self.position = (next_line * line_length) as usize;
        } else {
            unsafe {
                let location = self.location.offset(self.position as isize);

                *location = byte;
                let location = location.offset(1);
                *location = color;

                self.position = self.position + 2;
            }
        }
    }

    fn reset_position(&mut self) {
        self.position = 0;
    }
}

/// Prints a string
pub fn kprintf(s: &str, color: u8) {
    unsafe {
        let mut b = buffer.lock();
        for byte in s.bytes() {
            b.write_byte(byte, color);
        }
    }
}

/// Clears the console
pub fn clear_console() {
    let space = ' ' as u8;
    let color = 0x0a;

    unsafe {
        let mut b = buffer.lock();
        for _ in 0..CONSOLE_SIZE {
            b.write_byte(space, color);
        }

        b.reset_position();
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() -> ! { panic!("lol"); }
