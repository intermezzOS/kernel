#![no_std]
#![feature(nll)]

use core::fmt;
use core::fmt::Write;
use core::ptr;

mod character;

use character::Character;
pub use character::Color;

const ROWS: usize = 25;
const COLS: usize = 80;

pub struct Vga<T: AsMut<[u8]>> {
    slice: T,
    buffer: [Character; ROWS * COLS],
    position: usize,
    foreground_color: Color,
    background_color: Color,
}

impl<T: AsMut<[u8]>> Vga<T> {
    pub fn new(mut slice: T) -> Vga<T> {
        // we must have enough bytes of backing storage to make this work.
        assert_eq!(slice.as_mut().len(), ROWS * COLS * 2);

        let foreground_color = Color::Green;
        let background_color = Color::Black;

        let buffer = [Character::new(b' ', foreground_color, background_color); ROWS * COLS];

        Vga {
            slice,
            buffer,
            position: 0,
            foreground_color,
            background_color,
        }
    }

    pub fn set_foreground_color(&mut self, color: Color) {
        self.foreground_color = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn flush(&mut self) {
        // we need to use `write_volatile` here so that the writes aren't optimized out
        unsafe {
            let p = self.slice.as_mut();

            for (chunk, character) in p.chunks_mut(2).zip(self.buffer.iter()) {
                let (character, attribute) = character.as_bytes();

                let p = &mut chunk[0] as *mut u8;
                ptr::write_volatile(p, character);

                let p = &mut chunk[1] as *mut u8;
                ptr::write_volatile(p, attribute);
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        let i = self.position;

        if byte == '\n' as u8 {
            let current_line = self.position / COLS;
            self.position = (current_line + 1) * COLS;
        } else {
            self.buffer[i] = Character::new(byte, self.foreground_color, self.background_color);

            self.position += 1;
        }

        if self.position >= self.buffer.len() {
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        for row in 1..ROWS {
            for cb in 0..COLS {
                let prev_position = ((row - 1) * COLS) + cb;
                let current_position = (row * COLS) + cb;
                self.buffer[prev_position] = self.buffer[current_position];
            }
        }

        for cb in 0..COLS {
            self.buffer[((ROWS - 1) * COLS) + cb] =
                Character::new(b' ', self.foreground_color, self.background_color);
        }

        self.position = (ROWS - 1) * COLS;
    }
}

impl<T: AsMut<[u8]>> Write for Vga<T> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for b in s.bytes() {
            self.write_byte(b);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Write;
    use Vga;

    use COLS;
    use ROWS;

    #[test]
    fn write_a_letter() {
        let mut mock_memory = [0u8; ROWS * COLS * 2];
        let mut vga = Vga::new(&mut mock_memory[..]);

        vga.write_str("a").unwrap();
        vga.flush();

        assert_eq!(mock_memory[0], b'a');
        assert_eq!(mock_memory[1], 0x02);
    }

    #[test]
    fn write_a_word() {
        let mut mock_memory = [0u8; ROWS * COLS * 2];
        let mut vga = Vga::new(&mut mock_memory[..]);

        let word = "word";
        vga.write_str(word).unwrap();
        vga.flush();

        assert_eq!(mock_memory[0], b'w');
        assert_eq!(mock_memory[1], 0x02);
        assert_eq!(mock_memory[2], b'o');
        assert_eq!(mock_memory[3], 0x02);
        assert_eq!(mock_memory[4], b'r');
        assert_eq!(mock_memory[5], 0x02);
        assert_eq!(mock_memory[6], b'd');
        assert_eq!(mock_memory[7], 0x02);
    }

    #[test]
    fn write_multiple_words() {
        let mut mock_memory = [0u8; ROWS * COLS * 2];
        let mut vga = Vga::new(&mut mock_memory[..]);

        vga.write_str("hello ").unwrap();
        vga.write_str("world").unwrap();
        vga.flush();

        assert_eq!(mock_memory[0], b'h');
        assert_eq!(mock_memory[1], 0x02);
        assert_eq!(mock_memory[2], b'e');
        assert_eq!(mock_memory[3], 0x02);
        assert_eq!(mock_memory[4], b'l');
        assert_eq!(mock_memory[5], 0x02);
        assert_eq!(mock_memory[6], b'l');
        assert_eq!(mock_memory[7], 0x02);
        assert_eq!(mock_memory[8], b'o');
        assert_eq!(mock_memory[9], 0x02);
        assert_eq!(mock_memory[10], b' ');
        assert_eq!(mock_memory[11], 0x02);
        assert_eq!(mock_memory[12], b'w');
        assert_eq!(mock_memory[13], 0x02);
        assert_eq!(mock_memory[14], b'o');
        assert_eq!(mock_memory[15], 0x02);
        assert_eq!(mock_memory[16], b'r');
        assert_eq!(mock_memory[17], 0x02);
        assert_eq!(mock_memory[18], b'l');
        assert_eq!(mock_memory[19], 0x02);
        assert_eq!(mock_memory[20], b'd');
        assert_eq!(mock_memory[21], 0x02);
    }

    #[test]
    fn write_newline() {
        let mut mock_memory = [0u8; ROWS * COLS * 2];
        let mut vga = Vga::new(&mut mock_memory[..]);

        vga.write_str("hello\nworld\n!").unwrap();
        vga.flush();

        assert_eq!(mock_memory[0], b'h');
        assert_eq!(mock_memory[1], 0x02);
        assert_eq!(mock_memory[2], b'e');
        assert_eq!(mock_memory[3], 0x02);
        assert_eq!(mock_memory[4], b'l');
        assert_eq!(mock_memory[5], 0x02);
        assert_eq!(mock_memory[6], b'l');
        assert_eq!(mock_memory[7], 0x02);
        assert_eq!(mock_memory[8], b'o');
        assert_eq!(mock_memory[9], 0x02);
        assert_eq!(mock_memory[160], b'w');
        assert_eq!(mock_memory[161], 0x02);
        assert_eq!(mock_memory[162], b'o');
        assert_eq!(mock_memory[163], 0x02);
        assert_eq!(mock_memory[164], b'r');
        assert_eq!(mock_memory[165], 0x02);
        assert_eq!(mock_memory[166], b'l');
        assert_eq!(mock_memory[167], 0x02);
        assert_eq!(mock_memory[168], b'd');
        assert_eq!(mock_memory[169], 0x02);
        assert_eq!(mock_memory[320], b'!');
        assert_eq!(mock_memory[321], 0x02);
    }

    #[test]
    fn write_scroll() {
        let mut mock_memory = [0u8; ROWS * COLS * 2];
        let mut vga = Vga::new(&mut mock_memory[..]);

        for b in "abcdefghijklmnopqrstuvwxyz".bytes() {
            vga.write_byte(b);
            vga.write_byte('\n' as u8);
        }

        vga.flush();

        assert_eq!(mock_memory[0], b'c');

        for cb in 0..COLS {
            assert_eq!(mock_memory[(ROWS - 1) * COLS * 2 + (cb * 2)], b' ');
        }
    }
}
