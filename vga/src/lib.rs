#![feature(const_fn)]
#![feature(lang_items)]
#![no_std]

extern crate spin;

use spin::Mutex;
use core::fmt;

pub const DEFAULT_COLOR: ColorCode = ColorCode::new(Color::LightGreen, Color::Black);
const CONSOLE_COLS: isize = 80;
const CONSOLE_ROWS: isize = 25;

#[repr(u8)]
pub enum Color {
   Black = 0,
   Blue = 1,
   Green = 2,
   Cyan = 3,
   Red = 4,
   Magenta = 5,
   Brown = 6,
   LightGray = 7,
   DarkGray = 8,
   LightBlue = 9,
   LightGreen = 10,
   LightCyan = 11,
   LightRed = 12,
   LightMagenta = 13,
   Yellow = 14,
   White = 15,
}

#[derive(Copy, Clone)]
pub struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Copy,Clone)]
#[repr(C)]
struct VgaCell {
    character: u8,
    color: ColorCode,
}

pub static BUFFER: Mutex<VgaBuffer> = Mutex::new(VgaBuffer {
    buffer: [VgaCell {
        character: ' ' as u8,
        color: DEFAULT_COLOR,
    }; (CONSOLE_ROWS * CONSOLE_COLS * 2) as usize],
    position: 0,
});

pub struct VgaBuffer {
    buffer: [VgaCell; (CONSOLE_ROWS * CONSOLE_COLS * 2) as usize],
    position: usize,
}

impl VgaBuffer {
    fn write_byte(&mut self, byte: u8, color: ColorCode) {
        if byte == ('\n' as u8) {
            // to get the current line, we divide by the length of a line
            let current_line = (self.position as isize) / CONSOLE_COLS;

            let next_line = if current_line + 1 > CONSOLE_ROWS {

                let end = CONSOLE_ROWS * CONSOLE_COLS;

                for i in CONSOLE_COLS..(end) {
                    let prev = i - CONSOLE_COLS;
                    self.buffer[prev as usize] = self.buffer[i as usize];

                }

                // blank out the last row
                for i in (end - CONSOLE_COLS)..(end) {
                    let cell = &mut self.buffer[i as usize];
                    *cell = VgaCell {
                        character: ' ' as u8,
                        color: DEFAULT_COLOR,
                    };
                }

                CONSOLE_ROWS - 1
            } else {
                current_line + 1
            };

            self.position = (next_line * CONSOLE_COLS) as usize;
        } else {
            let cell = &mut self.buffer[self.position];

            *cell = VgaCell {
                character: byte,
                color: color,
            };

            self.position += 1;
        }
    }

    fn reset_position(&mut self) {
        self.position = 0;
    }

    pub fn flush(&self) {
        unsafe {
            let vga = 0xb8000 as *mut u8;
            let length = self.buffer.len() * 2;
            let buffer: *const u8 = core::mem::transmute(&self.buffer);
            core::intrinsics::copy_nonoverlapping(buffer, vga, length);
        }
    }

    fn clear(&mut self) {
        for i in 0..(CONSOLE_ROWS * CONSOLE_COLS * 2) {
            let cell = &mut self.buffer[i as usize];
            *cell = VgaCell {
                character: ' ' as u8,
                color: DEFAULT_COLOR,
            };
        }

        self.reset_position();

        self.flush();
    }
}

impl fmt::Write for VgaBuffer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        let color = DEFAULT_COLOR;
        for byte in s.bytes() {
            self.write_byte(byte, color)
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! kprintln {
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut b = $crate::BUFFER.lock();
        b.write_fmt(format_args!($($arg)*)).unwrap();
        b.flush();
    });
}

/// Clears the console
pub fn clear_console() {
    let mut b = BUFFER.lock();
    b.clear();
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    panic!("lol");
}
