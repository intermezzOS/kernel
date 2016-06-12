//! Dealing with the VGA cursor.
//!
//! The VGA cursor is handled through a system of 'indexed registers.' This means that you take a
//! location, `CRT_PORT` in this case, and write the index you'd like to access to it. You can then
//! access `CRT_PORT + 1` to read/write to that index. Within these functions, we've called these
//! two ports `crt_index` and `crt_io`, respectively. We create these ports within each function,
//! rather than trying to safe some kind of static value, because statics aren't safe, and it's not
//! like the `Port`s are expensive to create in the first place.
//!
//! This module has two functions, one of which initializes the cursor in the first place, and the
//! second which updates it to a particular location. `initialize()` must be called before you call
//! `set()` or `set()` can't do its job.
//!
//! References:
//!
//! * [OSDev wiki](http://wiki.osdev.org/Text_Mode_Cursor)
//! * [Wikipedia](http://wiki.osdev.org/VGA_Hardware)

use cpuio::Port;

const CRT_PORT: u16 = 0x3D4;

/// Initializes the cursor.
///
/// This function must be called in order to use the cursor, otherwise, it will not work.
pub fn initialize() {
    let mut crt_index: Port<u8> = unsafe { Port::new(CRT_PORT) };
    let mut crt_io: Port<u8> = unsafe { Port::new(CRT_PORT + 1) };

    // Set up cursor start register (0x0Ah)
    // Bits 0-4: Scanline start (where the cursor beings on the y axis)
    // Bit    5: Visibility status (0 = visible, 1 = invisible)
    crt_index.write(0b1010);
    crt_io.write(0b0000);

    // Set up cursor end register (0x0Bh)
    // Bits 0-4: Scanline end (where the cursor ends on the y axis)
    crt_index.write(0b1011);

    // Scanline 0x0-0xF creates 'block' cursor, 0xE-0xF creates underscore
    crt_io.write(0b1111);
}

/// Sets the cursor to a given position.
///
/// Instead of an X/Y coordinate, this position is a 0-(CONSOLE_ROWS * CONSOLE_COLS) position.
pub fn set(position: u16) {
    let mut crt_index: Port<u8> = unsafe { Port::new(CRT_PORT) };
    let mut crt_io: Port<u8> = unsafe { Port::new(CRT_PORT + 1) };

    // Set cursor low
    crt_index.write(0b1111);
    crt_io.write(position as u8);

    // Set cursor high
    crt_index.write(0b1110);
    crt_io.write((position >> 8) as u8);
}
