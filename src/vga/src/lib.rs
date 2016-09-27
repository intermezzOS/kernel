#![no_std]

use core::fmt;
use core::fmt::Write;

mod color;
use color::Color;

const ROWS: usize = 25;
const COLS: usize = 80;
const COL_BYTES: usize = COLS * 2;

pub struct Vga<T: AsMut<[u8]>> {
    slice: T,
    buffer: [u8; ROWS * COL_BYTES],
    position: usize,
}

impl<T: AsMut<[u8]>> Vga<T> {
    pub fn new(mut slice: T) -> Vga<T> {
        // we must have enough bytes of backing storage to make this work.
        assert_eq!(slice.as_mut().len(), ROWS * COL_BYTES);

        Vga {
            slice: slice,
            buffer: [0; ROWS * COL_BYTES],
            position: 0,
        }
    }

    pub fn flush(&mut self) {
        self.slice.as_mut().clone_from_slice(&self.buffer);
    }

    fn write_byte(&mut self, byte: u8) {
        let i = self.position;

        if byte == '\n' as u8 {
            let current_line = self.position / (COL_BYTES);
            self.position = (current_line + 1) * COL_BYTES;
        } else {
            self.buffer[i] = byte;
            self.buffer[i + 1] = color::colorcode(Color::Green, Color::Black);

            self.position += 2;
        }

        if self.position >= self.buffer.len() {
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        for row in 1..ROWS {
            for cb in 0..COL_BYTES {
                let prev_position = ((row - 1) * COL_BYTES) + cb;
                let current_position = (row * COL_BYTES) + cb;
                self.buffer[prev_position] = self.buffer[current_position];
            }
        }
         
        for cb in 0..COL_BYTES/2 {
            self.buffer[((ROWS - 1) * COL_BYTES) + (cb * 2)] = ' ' as u8;
        }

        self.position = (ROWS - 1) * COL_BYTES;
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
    use Vga;
    use core::fmt::Write;

    use ROWS;
    use COL_BYTES;

    #[test]
    fn write_a_letter() {
        let mut mock_memory = [0u8; ROWS * COL_BYTES];

        let mut vga = Vga::new(&mut mock_memory[..]);

        vga.write_str("a").unwrap();

        assert_eq!(vga.buffer[0], 'a' as u8);
        assert_eq!(vga.buffer[1], 0x02);
    }

    #[test]
    fn write_a_word() {
        let mut mock_memory = [0u8; ROWS * COL_BYTES];
        let mut vga = Vga::new(&mut mock_memory[..]);

        let word = "word";
        vga.write_str(word).unwrap();

        assert_eq!(vga.buffer[0], 'w' as u8);
        assert_eq!(vga.buffer[1], 0x02);
        assert_eq!(vga.buffer[2], 'o' as u8);
        assert_eq!(vga.buffer[3], 0x02);
        assert_eq!(vga.buffer[4], 'r' as u8);
        assert_eq!(vga.buffer[5], 0x02);
        assert_eq!(vga.buffer[6], 'd' as u8);
        assert_eq!(vga.buffer[7], 0x02);
    }

    #[test]
    fn write_multiple_words() {
        let mut mock_memory = [0u8; ROWS * COL_BYTES];
        let mut vga = Vga::new(&mut mock_memory[..]);

        vga.write_str("hello ").unwrap();
        vga.write_str("world").unwrap();

        assert_eq!(vga.buffer[0], 'h' as u8);
        assert_eq!(vga.buffer[1], 0x02);
        assert_eq!(vga.buffer[2], 'e' as u8);
        assert_eq!(vga.buffer[3], 0x02);
        assert_eq!(vga.buffer[4], 'l' as u8);
        assert_eq!(vga.buffer[5], 0x02);
        assert_eq!(vga.buffer[6], 'l' as u8);
        assert_eq!(vga.buffer[7], 0x02);
        assert_eq!(vga.buffer[8], 'o' as u8);
        assert_eq!(vga.buffer[9], 0x02);
        assert_eq!(vga.buffer[10], ' ' as u8);
        assert_eq!(vga.buffer[11], 0x02);
        assert_eq!(vga.buffer[12], 'w' as u8);
        assert_eq!(vga.buffer[13], 0x02);
        assert_eq!(vga.buffer[14], 'o' as u8);
        assert_eq!(vga.buffer[15], 0x02);
        assert_eq!(vga.buffer[16], 'r' as u8);
        assert_eq!(vga.buffer[17], 0x02);
        assert_eq!(vga.buffer[18], 'l' as u8);
        assert_eq!(vga.buffer[19], 0x02);
        assert_eq!(vga.buffer[20], 'd' as u8);
        assert_eq!(vga.buffer[21], 0x02);
    }

    #[test]
    fn write_newline() {
        let mut mock_memory = [0u8; ROWS * COL_BYTES];
        let mut vga = Vga::new(&mut mock_memory[..]);

        vga.write_str("hello\nworld\n!").unwrap();

        assert_eq!(vga.buffer[0], 'h' as u8);
        assert_eq!(vga.buffer[1], 0x02);
        assert_eq!(vga.buffer[2], 'e' as u8);
        assert_eq!(vga.buffer[3], 0x02);
        assert_eq!(vga.buffer[4], 'l' as u8);
        assert_eq!(vga.buffer[5], 0x02);
        assert_eq!(vga.buffer[6], 'l' as u8);
        assert_eq!(vga.buffer[7], 0x02);
        assert_eq!(vga.buffer[8], 'o' as u8);
        assert_eq!(vga.buffer[9], 0x02);
        assert_eq!(vga.buffer[160], 'w' as u8);
        assert_eq!(vga.buffer[161], 0x02);
        assert_eq!(vga.buffer[162], 'o' as u8);
        assert_eq!(vga.buffer[163], 0x02);
        assert_eq!(vga.buffer[164], 'r' as u8);
        assert_eq!(vga.buffer[165], 0x02);
        assert_eq!(vga.buffer[166], 'l' as u8);
        assert_eq!(vga.buffer[167], 0x02);
        assert_eq!(vga.buffer[168], 'd' as u8);
        assert_eq!(vga.buffer[169], 0x02);
        assert_eq!(vga.buffer[320], '!' as u8);
        assert_eq!(vga.buffer[321], 0x02);
    }

    #[test]
    fn write_scroll() {
        let mut mock_memory = [0u8; ROWS * COL_BYTES];
        let mut vga = Vga::new(&mut mock_memory[..]);

        for b in "abcdefghijklmnopqrstuvwxyz".bytes() {
            vga.write_byte(b);
            vga.write_byte('\n' as u8);
        }

        assert_eq!(vga.buffer[0], 'c' as u8);
        for cb in 0..COL_BYTES/2 {
            assert_eq!(vga.buffer[(ROWS - 1) * COL_BYTES + (cb * 2)], ' ' as u8);
        }
    }

}
