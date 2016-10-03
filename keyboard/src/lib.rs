#![no_std]

/// Decode a code in the PS/2 scan code set 1 (legacy set).
///
/// Difference between set 1 and sets 2 & 3:
///   http://wiki.osdev.org/%228042%22_PS/2_Controller#Translation
///
/// Reference table:
///   http://www.computer-engineering.org/ps2keyboard/scancodes1.html
pub fn from_scancode(code: usize) -> Option<char> {
    let printable = match code {
        0x1e => 'a',
        0x30 => 'b',
        0x2e => 'c',
        0x20 => 'd',
        0x12 => 'e',
        0x21 => 'f',
        0x22 => 'g',
        0x23 => 'h',
        0x17 => 'i',
        0x24 => 'j',
        0x25 => 'k',
        0x26 => 'l',
        0x32 => 'm',
        0x31 => 'n',
        0x18 => 'o',
        0x19 => 'p',
        0x10 => 'q',
        0x13 => 'r',
        0x1f => 's',
        0x14 => 't',
        0x16 => 'u',
        0x2f => 'v',
        0x11 => 'w',
        0x2d => 'x',
        0x15 => 'y',
        0x2c => 'z',
        0x0b => '0',
        0x02 => '1',
        0x03 => '2',
        0x04 => '3',
        0x05 => '4',
        0x06 => '5',
        0x07 => '6',
        0x08 => '7',
        0x09 => '8',
        0x0a => '9',
        0x29 => '`',
        0x0c => '-',
        0x0d => '=',
        0x2b => '\\',
        0x39 => ' ',
        0x1a => '[',
        0x1b => ']',
        0x27 => ';',
        0x28 => '\'',
        0x33 => ',',
        0x34 => '.',
        0x35 => '/',
        0x37 => '*', // Keypad
        0x4a => '-', // Keypad
        0x4e => '+', // Keypad
        _ => return None,
    };

    Some(printable)
}
