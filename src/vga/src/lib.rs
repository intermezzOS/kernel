#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![no_std]

extern crate spin;
extern crate cpuio;

use core::fmt;
use spin::Mutex;

mod cursor;

pub fn initialize() {
    clear_console();
    cursor::initialize();
}

/// Clears the console.
///
/// This will reset the entire console to the background color, and move the cursor back to the
/// upper-left part of the screen.
///
/// # Exampes
///
/// Basic usage:
///
/// ```
/// vga::clear_console();
/// ```
pub fn clear_console() {
    let mut b = BUFFER.lock();
    b.clear();
    b.flush();
}

/// A VGA color.
///
/// There are sixteen possible colors in VGA's text mode, and this enum represents them all.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let black = Color::Black;
/// ```
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

/// A combination foreground/background color.
///
/// Each location on the screen has a foreground color and a background color, and representing
/// that is `ColorCode`'s job.
///
/// The default color is a `Color::LightGreen` foreground on a `Color::Black` background.
///
/// # Examples
///
/// ```
/// let green_on_black = ColorCode::new(Color::LightGreen, Color::Black);
///
/// let yellow_on_red = ColorCode::new(Color::Yellow, Color::Red);
/// ```
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColorCode(u8);

const DEFAULT_COLOR: ColorCode = ColorCode::new(Color::LightGreen, Color::Black);

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

const CONSOLE_COLS: isize = 80;
const CONSOLE_ROWS: isize = 25;

/// The public `VgaBuffer`.
///
/// This holds the canonical `VgaBuffer` for all to use.
///
/// You generally shouldn't interact with `BUFFER` on your own; use the `kprintln!` and `kprint!`
/// macros instead.
pub static BUFFER: Mutex<VgaBuffer> = Mutex::new(VgaBuffer {
    buffer: [VgaCell {
        character: b' ',
        color: DEFAULT_COLOR,
    }; (CONSOLE_ROWS * CONSOLE_COLS) as usize],
    position: 0,
});

/// The global VGA buffer.
///
/// This buffer contains a representation of the screen's memory. Since VGA is a memory-mapped
/// device, it's already memory. The reason we're doing this is so that we can make multiple
/// changes and then blit them all to the screen at once with `flush()`. This should produce
/// smoother scrolling, at the cost of a very small amount of memory.
///
/// You generally shouldn't make more `VgaBuffer`s.
pub struct VgaBuffer {
    buffer: [VgaCell; (CONSOLE_ROWS * CONSOLE_COLS) as usize],
    position: usize,
}

impl VgaBuffer {
    /// Writes the current values of the buffer out to the screen.
    ///
    /// Call this method when you're done updating the screen, and are ready for it to update.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// // unlock the buffer
    /// let mut b = BUFFER.lock();
    ///
    /// // clear the screen
    /// b.clear();
    ///
    /// // update the screen with our changes
    /// b.flush();
    /// ```
    pub fn flush(&self) {
        unsafe {
            let vga = 0xb8000 as *mut u8;

            let length = self.buffer.len() * 2;
            let buffer = self.buffer.as_ptr() as *const u8;

            core::ptr::copy_nonoverlapping(buffer, vga, length);
        }
    }

    pub fn backsp(&mut self) {
        {
            let cell = &mut self.buffer[self.position - 1];
            *cell = VgaCell {
                character: b' ',
                color: DEFAULT_COLOR,
            };
            self.position -= 1;
            cursor::set(self.position as u16);
        }
        self.flush();
    }

    fn clear(&mut self) {
        for i in 0..(CONSOLE_ROWS * CONSOLE_COLS) {
            let cell = &mut self.buffer[i as usize];

            *cell = VgaCell {
                character: b' ',
                color: DEFAULT_COLOR,
            };
        }

        self.reset_position();
        self.flush();
    }

    fn write_byte(&mut self, byte: u8, color: ColorCode) {
        if byte == (b'\n') {
            // to get the current line, we divide by the length of a line
            let current_line = (self.position as isize) / CONSOLE_COLS;
            self.position = ((current_line + 1) * CONSOLE_COLS) as usize;
        } else {
            let cell = &mut self.buffer[self.position];

            *cell = VgaCell {
                character: byte,
                color: color,
            };
            
            self.position += 1;
        }

        if self.position >= self.buffer.len() {
            self.scroll_up();
        }

        cursor::set(self.position as u16);
    }

    fn scroll_up(&mut self) {
        let end = CONSOLE_ROWS * CONSOLE_COLS;

        for i in CONSOLE_COLS..(end) {
            let prev = i - CONSOLE_COLS;
            self.buffer[prev as usize] = self.buffer[i as usize];
        }

        // blank out the last row
        for i in (end - CONSOLE_COLS)..(end) {
            let cell = &mut self.buffer[i as usize];
            *cell = VgaCell {
                character: b' ',
                color: DEFAULT_COLOR,
            };
        }

        self.position = (end - CONSOLE_COLS) as usize;
    }

    fn reset_position(&mut self) {
        self.position = 0;
        cursor::set(0);
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

/// Prints something to the screen, with a trailing newline.
///
/// # Examples
///
/// ```
/// kprintln!("Hello, world!");
/// ```
#[macro_export]
macro_rules! kprintln {
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

/// Prints something to the screen.
///
/// # Examples
///
/// ```
/// kprint!("Hello, world!");
/// ```
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut b = $crate::BUFFER.lock();
        b.write_fmt(format_args!($($arg)*)).unwrap();
        b.flush();
    });
}
