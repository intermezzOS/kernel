#![no_std]

static NULL: char = '\0';

#[derive(Clone, Copy, PartialEq)]
pub struct Key {
    code: u8,
}

impl Key {
    /// Decode a code in the PS/2 scan code set 1 (legacy set).
    ///
    /// Difference between set 1 and sets 2 & 3:
    ///   http://wiki.osdev.org/%228042%22_PS/2_Controller#Translation
    ///
    /// Reference table:
    ///   http://www.computer-engineering.org/ps2keyboard/scancodes1.html
    pub fn to_char(&self) -> Option<char> {
        let printable = match self.code {
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
            _ => NULL,
        };

        if printable != NULL {
            Some(printable)
        } else {
            None
        }
    }
}

const MAX_CODES: usize = 256;

/// PS/2 keyboard driver: manages receiving signals from keyboard port,
/// updating its internal state according to those signals, and providing
/// relevant output (if any) based on its state.
pub struct Keyboard {
    pressed_keys: [Option<Key>; MAX_CODES],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            pressed_keys: [None; MAX_CODES],
        }
    }

    // Update the driver's internal state in response to a scanned key code
    pub fn update(&mut self, scancode: u8) {
        // FIXME: We want to properly track release codes rather than
        //   resetting our state every time
        for i in 0..self.pressed_keys.len() {
            self.pressed_keys[i] = None;
        }

        if self.pressed_keys[scancode as usize] == None {
            self.pressed_keys[scancode as usize] = Some(Key { code: scancode })
        }
    }

    pub fn printable(&self) -> Option<char> {
        for key in self.pressed_keys.iter() {
            if let &Some(pressed_key) = key {
                match pressed_key.to_char() {
                    Some(c) => return Some(c),
                    _ => ()
                }
            }
        }

        None
    }
}
