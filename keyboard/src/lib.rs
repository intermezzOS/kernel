#![no_std]

pub enum Action {
    Pressed,
    Released,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Key {
    KA,
    KB,
    KC,
    KD,
    KE,
    KF,
    KG,
    KH,
    KI,
    KJ,
    KK,
    KL,
    KM,
    KN,
    KO,
    KP,
    KQ,
    KR,
    KS,
    KT,
    KU,
    KV,
    KW,
    KX,
    KY,
    KZ,
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    KBacktick,
    KDash,
    KEqual,
    KTab,
    KBackslash,
    KSpace,
    KLeftSquareBracket,
    KRightSquareBracket,
    KSemicolon,
    KApostrophe,
    KComma,
    KPeriod,
    KSlash,
    KKeypadAsterisk, // Keypad
    KKeypadMinus,    // Keypad
    KKeypadPlus,     // Keypad
    KBackspace,
    KEnter,
    KLeftShift,
    KRightShift,
}

impl Key {
    pub fn is_ascii(self) -> bool {
        self.ascii(false).is_some()
    }

    pub fn ascii(self, shift: bool) -> Option<char> {
        use Key::*;

        let character = match (self, shift) {
                   (KA, false) => 'a',          (KA, true) => 'A',
                   (KB, false) => 'b',          (KB, true) => 'B',
                   (KC, false) => 'c',          (KC, true) => 'C',
                   (KD, false) => 'd',          (KD, true) => 'D',
                   (KE, false) => 'e',          (KE, true) => 'E',
                   (KF, false) => 'f',          (KF, true) => 'F',
                   (KG, false) => 'g',          (KG, true) => 'G',
                   (KH, false) => 'h',          (KH, true) => 'H',
                   (KI, false) => 'i',          (KI, true) => 'I',
                   (KJ, false) => 'j',          (KJ, true) => 'J',
                   (KK, false) => 'k',          (KK, true) => 'K',
                   (KL, false) => 'l',          (KL, true) => 'L',
                   (KM, false) => 'm',          (KM, true) => 'M',
                   (KN, false) => 'n',          (KN, true) => 'N',
                   (KO, false) => 'o',          (KO, true) => 'O',
                   (KP, false) => 'p',          (KP, true) => 'P',
                   (KQ, false) => 'q',          (KQ, true) => 'Q',
                   (KR, false) => 'r',          (KR, true) => 'R',
                   (KS, false) => 's',          (KS, true) => 'S',
                   (KT, false) => 't',          (KT, true) => 'T',
                   (KU, false) => 'u',          (KU, true) => 'U',
                   (KV, false) => 'v',          (KV, true) => 'V',
                   (KW, false) => 'w',          (KW, true) => 'W',
                   (KX, false) => 'x',          (KX, true) => 'X',
                   (KY, false) => 'y',          (KY, true) => 'Y',
                   (KZ, false) => 'z',          (KZ, true) => 'Z',
                   (K0, false) => '0',          (K0, true) => ')',
                   (K1, false) => '1',          (K1, true) => '!',
                   (K2, false) => '2',          (K2, true) => '@',
                   (K3, false) => '3',          (K3, true) => '#',
                   (K4, false) => '4',          (K4, true) => '$',
                   (K5, false) => '5',          (K5, true) => '%',
                   (K6, false) => '6',          (K6, true) => '^',
                   (K7, false) => '7',          (K7, true) => '&',
                   (K8, false) => '8',          (K8, true) => '*',
                   (K9, false) => '9',          (K9, true) => '(',
           (KBackslash, false) => '\\', (KBackslash, true) => '|',
            (KBacktick, false) => '`',   (KBacktick, true) => '~',
               (KComma, false) => ',',      (KComma, true) => '<',
              (KPeriod, false) => '.',     (KPeriod, true) => '>',
               (KBackspace, _) => '\x08',
                   (KEnter, _) => '\n',
                   (KSpace, _) => ' ',
                     (KTab, _) => '\t',
                             _ => return None,
        };

        Some(character)
    }
}

/// Decode a code in the PS/2 scan code set 1 (legacy set).
///
/// Difference between set 1 and sets 2 & 3:
///   http://wiki.osdev.org/%228042%22_PS/2_Controller#Translation
///
/// Reference table:
///   http://www.computer-engineering.org/ps2keyboard/scancodes1.html
pub fn from_scancode(code: u8) -> Option<(Key, Action)> {
    use Key::*;

    // Set 1 has the release codes for the basic keys offset by 0x80 from the
    // press codes for the key
    let (code, action) = match code {
        0x00...0x58 => (code,        Action::Pressed),
        0x81...0xD8 => (code - 0x80, Action::Released),
        _ => return None,
    };

    let key = match code {
        0x1e => KA,
        0x30 => KB,
        0x2e => KC,
        0x20 => KD,
        0x12 => KE,
        0x21 => KF,
        0x22 => KG,
        0x23 => KH,
        0x17 => KI,
        0x24 => KJ,
        0x25 => KK,
        0x26 => KL,
        0x32 => KM,
        0x31 => KN,
        0x18 => KO,
        0x19 => KP,
        0x10 => KQ,
        0x13 => KR,
        0x1f => KS,
        0x14 => KT,
        0x16 => KU,
        0x2f => KV,
        0x11 => KW,
        0x2d => KX,
        0x15 => KY,
        0x2c => KZ,
        0x0b => K0,
        0x02 => K1,
        0x03 => K2,
        0x04 => K3,
        0x05 => K4,
        0x06 => K5,
        0x07 => K6,
        0x08 => K7,
        0x09 => K8,
        0x0a => K9,
        0x29 => KBacktick,
        0x0c => KDash,
        0x0d => KEqual,
        0x0f => KTab,
        0x2b => KBackslash,
        0x39 => KSpace,
        0x1a => KLeftSquareBracket,
        0x1b => KRightSquareBracket,
        0x27 => KSemicolon,
        0x28 => KApostrophe,
        0x33 => KComma,
        0x34 => KPeriod,
        0x35 => KSlash,
        0x37 => KKeypadAsterisk, // Keypad
        0x4a => KKeypadMinus,    // Keypad
        0x4e => KKeypadPlus,     // Keypad
        0x0e => KBackspace,
        0x1c => KEnter,
        0x2A => KLeftShift,
        0x36 => KRightShift,
        _    => return None,
    };

    Some((key, action))
}

/// Size of the buffer used for storing the state of the keyboard
const KEY_BUFFER: usize = 256;

/// PS/2 keyboard driver: manages receiving signals from keyboard port,
/// updating its internal state according to those signals, and providing
/// relevant output (if any) based on its state.
pub struct Keyboard {
    pub stack: [Option<Key>; KEY_BUFFER],
    pub size: usize,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            stack: [None; KEY_BUFFER],
            size: 0,
        }
    }

    /// Returns the `Key` at the top of the stack, or `None` if no key is
    /// currently pressed.
    fn stack_top(&self) -> Option<Key> {
        if self.size == 0 {
            None
        } else {
            self.stack[self.size - 1]
        }
    }

    // Update the driver's internal state in response to a scanned key code
    pub fn input(&mut self, scancode: u8) -> Option<char> {
        let key_action = match from_scancode(scancode) {
            Some(k) => k,
            None => return None,
        };

        let (key, action) = key_action;

        match action {
            Action::Pressed => {
                self.push_key(key);

                if key.is_ascii() {
                    return self.ascii()
                }
            },
            Action::Released => {
                self.clear_key(key);
            },
        }

        None
    }

    /// Add an active `Key` to the top of stack. This is normally in response
    /// to a key-pressed event from the keyboard.
    fn push_key(&mut self, key: Key) {
        // Don't repeat-register key presses
        if self.is_key_pressed(key) {
            return
        }

        // Guard against overflow
        if self.size == KEY_BUFFER {
            panic!("Keyboard buffer full (key: {:?})", key)
        }

        // Push the key onto the stack
        self.stack[self.size] = Some(key);
        self.size += 1;
    }

    /// Remove an active/pressed `Key` from the stack.
    fn clear_key(&mut self, key: Key) {
        for i in 0..self.size {
            let k = self.stack[i].unwrap();

            if k != key {
                continue
            } else {
                // Shift everything after the value forwards over it and then
                // shrink the size
                for j in i..(self.size - 1) {
                    self.stack[j] = self.stack[j + 1];
                }
                self.stack[self.size - 1] = None;
                self.size -= 1;
                break
            }
        }
    }

    /// Scans the stack to determine if the given `Key` is pressed.
    fn is_key_pressed(&self, key: Key) -> bool {
        for i in 0..self.size {
            match self.stack[i] {
                Some(k) => {
                    if k == key {
                        return true
                    }
                },
                None => (),
            }
        }
        false
    }

    fn is_shift_pressed(&self) -> bool {
        for i in 0..self.size {
            match self.stack[i] {
                Some(Key::KLeftShift) => return true,
                _ => (),
            }
        }
        false
    }

    /// Returns the ASCII code for the top (most recent) key on the stack.
    /// Takes active/pressed modifier keys into account.
    pub fn ascii(&self) -> Option<char> {
        if let Some(key) = self.stack_top() {
            let shift = self.is_shift_pressed();
            key.ascii(shift)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use Key;
    use Keyboard;

    #[test]
    fn pushes_keys() {
        let mut keyboard = Keyboard::new();
        assert_eq!(keyboard.size, 0);
        assert_eq!(keyboard.stack[0], None);

        keyboard.push_key(Key::KA);
        assert_eq!(keyboard.size, 1);
        assert_eq!(keyboard.stack[0], Some(Key::KA));
        assert_eq!(keyboard.stack[1], None);

        // Doesn't double-push
        keyboard.push_key(Key::KA);
        assert_eq!(keyboard.size, 1);
        assert_eq!(keyboard.stack[0], Some(Key::KA));
        assert_eq!(keyboard.stack[1], None);

        keyboard.push_key(Key::KB);
        assert_eq!(keyboard.size, 2);
        assert_eq!(keyboard.stack[0], Some(Key::KA));
        assert_eq!(keyboard.stack[1], Some(Key::KB));
        assert_eq!(keyboard.stack[2], None);
    }

    #[test]
    fn clears_key() {
        let mut keyboard = Keyboard::new();
        keyboard.push_key(Key::KLeftShift);
        keyboard.push_key(Key::KA);
        assert_eq!(keyboard.size, 2);
        assert_eq!(keyboard.stack[0], Some(Key::KLeftShift));
        assert_eq!(keyboard.stack[1], Some(Key::KA));
        assert_eq!(keyboard.stack[2], None);

        // Check that it removes it and shifts everything down
        keyboard.clear_key(Key::KLeftShift);
        assert_eq!(keyboard.size, 1);
        assert_eq!(keyboard.stack[0], Some(Key::KA));
        assert_eq!(keyboard.stack[1], None);
    }

    #[test]
    fn inputs_character_scancode() {
        let mut keyboard = Keyboard::new();

        let result = keyboard.input(0x1e); // "A" pressed
        assert_eq!(result, Some('a'));
        assert_eq!(keyboard.size, 1);
        assert_eq!(keyboard.stack[0], Some(Key::KA));

        let result = keyboard.input(0x9e); // "A" released
        assert_eq!(result, None);
        assert_eq!(keyboard.size, 0);
    }

    #[test]
    fn inputs_character_and_modifier_scancodes() {
        let mut keyboard = Keyboard::new();

        let result = keyboard.input(0x2a); // Left shift pressed
        assert_eq!(result, None);
        assert_eq!(keyboard.size, 1);
        assert_eq!(keyboard.stack[0], Some(Key::KLeftShift));

        // Check that the modifier is applied to the character
        let result = keyboard.input(0x1e); // "A" pressed
        assert_eq!(result, Some('A'));
        assert_eq!(keyboard.size, 2);
        assert_eq!(keyboard.stack[1], Some(Key::KA));

        // And that it's still applied upon repeat ticks
        let result = keyboard.input(0x1e); // "A" pressed
        assert_eq!(result, Some('A'));
        assert_eq!(keyboard.size, 2);
        assert_eq!(keyboard.stack[1], Some(Key::KA));

        let result = keyboard.input(0x9e); // "A" released
        assert_eq!(result, None);
        assert_eq!(keyboard.size, 1);

        let result = keyboard.input(0xaa); // Left shift released
        assert_eq!(result, None);
        assert_eq!(keyboard.size, 0);

        // Check that the modifier is no longer applied
        let result = keyboard.input(0x30); // "B" pressed
        assert_eq!(result, Some('b'));
        assert_eq!(keyboard.size, 1);
        assert_eq!(keyboard.stack[0], Some(Key::KB));
    }
}
