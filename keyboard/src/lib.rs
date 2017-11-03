#![no_std]

pub enum Action {
    Pressed,
    Released,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    // These are the numbers along the top row of the keyboard, *not* the ones
    // to the right on the keypad.
    Keyboard0,
    Keyboard1,
    Keyboard2,
    Keyboard3,
    Keyboard4,
    Keyboard5,
    Keyboard6,
    Keyboard7,
    Keyboard8,
    Keyboard9,
    Backtick,
    Dash,
    Equal,
    Tab,
    Backslash,
    Space,
    LeftSquareBracket,
    RightSquareBracket,
    Semicolon,
    Apostrophe,
    Comma,
    Period,
    Slash,
    KeypadAsterisk, // Keypad
    KeypadMinus, // Keypad
    KeypadPlus, // Keypad
    Backspace,
    Enter,
    LeftShift,
    RightShift,
}

impl Key {
    pub fn is_ascii(self) -> bool {
        self.ascii(false).is_some()
    }

    pub fn ascii(self, shift: bool) -> Option<char> {
        use Key::*;

        let character = match (self, shift) {
            (A, false) => 'a',          (A, true) => 'A',
            (B, false) => 'b',          (B, true) => 'B',
            (C, false) => 'c',          (C, true) => 'C',
            (D, false) => 'd',          (D, true) => 'D',
            (E, false) => 'e',          (E, true) => 'E',
            (F, false) => 'f',          (F, true) => 'F',
            (G, false) => 'g',          (G, true) => 'G',
            (H, false) => 'h',          (H, true) => 'H',
            (I, false) => 'i',          (I, true) => 'I',
            (J, false) => 'j',          (J, true) => 'J',
            (K, false) => 'k',          (K, true) => 'K',
            (L, false) => 'l',          (L, true) => 'L',
            (M, false) => 'm',          (M, true) => 'M',
            (N, false) => 'n',          (N, true) => 'N',
            (O, false) => 'o',          (O, true) => 'O',
            (P, false) => 'p',          (P, true) => 'P',
            (Q, false) => 'q',          (Q, true) => 'Q',
            (R, false) => 'r',          (R, true) => 'R',
            (S, false) => 's',          (S, true) => 'S',
            (T, false) => 't',          (T, true) => 'T',
            (U, false) => 'u',          (U, true) => 'U',
            (V, false) => 'v',          (V, true) => 'V',
            (W, false) => 'w',          (W, true) => 'W',
            (X, false) => 'x',          (X, true) => 'X',
            (Y, false) => 'y',          (Y, true) => 'Y',
            (Z, false) => 'z',          (Z, true) => 'Z',
            (Keyboard0, false) => '0',  (Keyboard0, true) => ')',
            (Keyboard1, false) => '1',  (Keyboard1, true) => '!',
            (Keyboard2, false) => '2',  (Keyboard2, true) => '@',
            (Keyboard3, false) => '3',  (Keyboard3, true) => '#',
            (Keyboard4, false) => '4',  (Keyboard4, true) => '$',
            (Keyboard5, false) => '5',  (Keyboard5, true) => '%',
            (Keyboard6, false) => '6',  (Keyboard6, true) => '^',
            (Keyboard7, false) => '7',  (Keyboard7, true) => '&',
            (Keyboard8, false) => '8',  (Keyboard8, true) => '*',
            (Keyboard9, false) => '9',  (Keyboard9, true) => '(',
            (Backslash, false) => '\\', (Backslash, true) => '|',
            (Backtick, false) => '`',   (Backtick, true) => '~',
            (Comma, false) => ',',      (Comma, true) => '<',
            (Period, false) => '.',     (Period, true) => '>',
            (Backspace, _) => '\x08',
            (Enter, _) => '\n',
            (Space, _) => ' ',
            (Tab, _) => '\t',
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
        0x00...0x58 => (code, Action::Pressed),
        0x81...0xD8 => (code - 0x80, Action::Released),
        _ => return None,
    };

    let key = match code {
        0x1e => A,
        0x30 => B,
        0x2e => C,
        0x20 => D,
        0x12 => E,
        0x21 => F,
        0x22 => G,
        0x23 => H,
        0x17 => I,
        0x24 => J,
        0x25 => K,
        0x26 => L,
        0x32 => M,
        0x31 => N,
        0x18 => O,
        0x19 => P,
        0x10 => Q,
        0x13 => R,
        0x1f => S,
        0x14 => T,
        0x16 => U,
        0x2f => V,
        0x11 => W,
        0x2d => X,
        0x15 => Y,
        0x2c => Z,
        0x0b => Keyboard0,
        0x02 => Keyboard1,
        0x03 => Keyboard2,
        0x04 => Keyboard3,
        0x05 => Keyboard4,
        0x06 => Keyboard5,
        0x07 => Keyboard6,
        0x08 => Keyboard7,
        0x09 => Keyboard8,
        0x0a => Keyboard9,
        0x29 => Backtick,
        0x0c => Dash,
        0x0d => Equal,
        0x0f => Tab,
        0x2b => Backslash,
        0x39 => Space,
        0x1a => LeftSquareBracket,
        0x1b => RightSquareBracket,
        0x27 => Semicolon,
        0x28 => Apostrophe,
        0x33 => Comma,
        0x34 => Period,
        0x35 => Slash,
        0x37 => KeypadAsterisk, // Keypad
        0x4a => KeypadMinus,    // Keypad
        0x4e => KeypadPlus,     // Keypad
        0x0e => Backspace,
        0x1c => Enter,
        0x2A => LeftShift,
        0x36 => RightShift,
        _ => return None,
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
                    return self.ascii();
                }
            }
            Action::Released => self.clear_key(key),
        }

        None
    }

    /// Add an active `Key` to the top of stack. This is normally in response
    /// to a key-pressed event from the keyboard.
    fn push_key(&mut self, key: Key) {
        // Don't repeat-register key presses
        if self.is_key_pressed(key) {
            return;
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
                continue;
            } else {
                // Shift everything after the value forwards over it and then
                // shrink the size
                for j in i..(self.size - 1) {
                    self.stack[j] = self.stack[j + 1];
                }
                self.stack[self.size - 1] = None;
                self.size -= 1;
                break;
            }
        }
    }

    /// Scans the stack to determine if the given `Key` is pressed.
    fn is_key_pressed(&self, key: Key) -> bool {
        for i in 0..self.size {
            match self.stack[i] {
                Some(k) => {
                    if k == key {
                        return true;
                    }
                }
                None => (),
            }
        }
        false
    }

    fn is_shift_pressed(&self) -> bool {
        for i in 0..self.size {
            match self.stack[i] {
                Some(Key::LeftShift) => return true,
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

        keyboard.push_key(Key::A);
        assert_eq!(keyboard.size, 1);
        assert_eq!(keyboard.stack[0], Some(Key::A));
        assert_eq!(keyboard.stack[1], None);

        // Doesn't double-push
        keyboard.push_key(Key::A);
        assert_eq!(keyboard.size, 1);
        assert_eq!(keyboard.stack[0], Some(Key::A));
        assert_eq!(keyboard.stack[1], None);

        keyboard.push_key(Key::B);
        assert_eq!(keyboard.size, 2);
        assert_eq!(keyboard.stack[0], Some(Key::A));
        assert_eq!(keyboard.stack[1], Some(Key::B));
        assert_eq!(keyboard.stack[2], None);
    }

    #[test]
    fn clears_key() {
        let mut keyboard = Keyboard::new();
        keyboard.push_key(Key::LeftShift);
        keyboard.push_key(Key::A);
        assert_eq!(keyboard.size, 2);
        assert_eq!(keyboard.stack[0], Some(Key::LeftShift));
        assert_eq!(keyboard.stack[1], Some(Key::A));
        assert_eq!(keyboard.stack[2], None);

        // Check that it removes it and shifts everything down
        keyboard.clear_key(Key::LeftShift);
        assert_eq!(keyboard.size, 1);
        assert_eq!(keyboard.stack[0], Some(Key::A));
        assert_eq!(keyboard.stack[1], None);
    }

    #[test]
    fn inputs_character_scancode() {
        let mut keyboard = Keyboard::new();

        let result = keyboard.input(0x1e); // "A" pressed
        assert_eq!(result, Some('a'));
        assert_eq!(keyboard.size, 1);
        assert_eq!(keyboard.stack[0], Some(Key::A));

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
        assert_eq!(keyboard.stack[0], Some(Key::LeftShift));

        // Check that the modifier is applied to the character
        let result = keyboard.input(0x1e); // "A" pressed
        assert_eq!(result, Some('A'));
        assert_eq!(keyboard.size, 2);
        assert_eq!(keyboard.stack[1], Some(Key::A));

        // And that it's still applied upon repeat ticks
        let result = keyboard.input(0x1e); // "A" pressed
        assert_eq!(result, Some('A'));
        assert_eq!(keyboard.size, 2);
        assert_eq!(keyboard.stack[1], Some(Key::A));

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
        assert_eq!(keyboard.stack[0], Some(Key::B));
    }
}
