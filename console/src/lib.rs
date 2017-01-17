#![no_std]

use core::fmt;
use core::fmt::Write;

mod color;
use color::Color;

const ROWS: usize = 25;
const COLS: usize = 80;
const COL_BYTES: usize = COLS * 2;

pub struct Vga<'a> {
    slice: &'a mut [u8],
    position: usize,
}

impl Vga<'static> {
    pub fn new() -> Vga<'static> {
        unsafe {
            Vga::from_raw(core::slice::from_raw_parts_mut(0xb8000 as *mut u8, ROWS * COL_BYTES))
        }
    }
}

impl<'a> Vga<'a> {
    // for testing
    pub unsafe fn from_raw(slice: &mut [u8]) -> Vga {
        // we must have enough bytes of backing storage to make this work.
        assert_eq!(slice.len(), ROWS * COL_BYTES);

        slice.clone_from_slice(&[0; ROWS * COL_BYTES][..]);

        Vga {
            slice: slice,
            position: 0,
        }
    }

    fn write_byte(&mut self, byte: u8) {
        let i = self.position;

        if byte == '\n' as u8 {
            let current_line = self.position / (COL_BYTES);
            self.position = (current_line + 1) * COL_BYTES;
        } else {
            self.slice[i] = byte;
            self.slice[i + 1] = color::colorcode(Color::Green, Color::Black);

            self.position += 2;
        }

        if self.position >= self.slice.len() {
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        for row in 1..ROWS {
            for cb in 0..COL_BYTES {
                let prev_position = ((row - 1) * COL_BYTES) + cb;
                let current_position = (row * COL_BYTES) + cb;
                self.slice[prev_position] = self.slice[current_position];
            }
        }
         
        for cb in 0..COL_BYTES/2 {
            self.slice[((ROWS - 1) * COL_BYTES) + (cb * 2)] = ' ' as u8;
        }

        self.position = (ROWS - 1) * COL_BYTES;
    }
}

impl<'a> Write for Vga<'a> {
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

        let mut vga = unsafe {
            Vga::from_raw(&mut mock_memory[..])
        };

        vga.write_str("a").unwrap();

        assert_eq!(vga.slice[0], 'a' as u8);
        assert_eq!(vga.slice[1], 0x02);
    }

    #[test]
    fn write_a_word() {
        let mut mock_memory = [0u8; ROWS * COL_BYTES];

        let mut vga = unsafe {
            Vga::from_raw(&mut mock_memory[..])
        };

        let word = "word";
        vga.write_str(word).unwrap();

        assert_eq!(vga.slice[0], 'w' as u8);
        assert_eq!(vga.slice[1], 0x02);
        assert_eq!(vga.slice[2], 'o' as u8);
        assert_eq!(vga.slice[3], 0x02);
        assert_eq!(vga.slice[4], 'r' as u8);
        assert_eq!(vga.slice[5], 0x02);
        assert_eq!(vga.slice[6], 'd' as u8);
        assert_eq!(vga.slice[7], 0x02);
    }

    #[test]
    fn write_multiple_words() {
        let mut mock_memory = [0u8; ROWS * COL_BYTES];
        let mut vga = unsafe {
            Vga::from_raw(&mut mock_memory[..])
        };

        vga.write_str("hello ").unwrap();
        vga.write_str("world").unwrap();

        assert_eq!(vga.slice[0], 'h' as u8);
        assert_eq!(vga.slice[1], 0x02);
        assert_eq!(vga.slice[2], 'e' as u8);
        assert_eq!(vga.slice[3], 0x02);
        assert_eq!(vga.slice[4], 'l' as u8);
        assert_eq!(vga.slice[5], 0x02);
        assert_eq!(vga.slice[6], 'l' as u8);
        assert_eq!(vga.slice[7], 0x02);
        assert_eq!(vga.slice[8], 'o' as u8);
        assert_eq!(vga.slice[9], 0x02);
        assert_eq!(vga.slice[10], ' ' as u8);
        assert_eq!(vga.slice[11], 0x02);
        assert_eq!(vga.slice[12], 'w' as u8);
        assert_eq!(vga.slice[13], 0x02);
        assert_eq!(vga.slice[14], 'o' as u8);
        assert_eq!(vga.slice[15], 0x02);
        assert_eq!(vga.slice[16], 'r' as u8);
        assert_eq!(vga.slice[17], 0x02);
        assert_eq!(vga.slice[18], 'l' as u8);
        assert_eq!(vga.slice[19], 0x02);
        assert_eq!(vga.slice[20], 'd' as u8);
        assert_eq!(vga.slice[21], 0x02);
    }

    #[test]
    fn write_newline() {
        let mut mock_memory = [0u8; ROWS * COL_BYTES];
        let mut vga = unsafe {
            Vga::from_raw(&mut mock_memory[..])
        };

        vga.write_str("hello\nworld\n!").unwrap();

        assert_eq!(vga.slice[0], 'h' as u8);
        assert_eq!(vga.slice[1], 0x02);
        assert_eq!(vga.slice[2], 'e' as u8);
        assert_eq!(vga.slice[3], 0x02);
        assert_eq!(vga.slice[4], 'l' as u8);
        assert_eq!(vga.slice[5], 0x02);
        assert_eq!(vga.slice[6], 'l' as u8);
        assert_eq!(vga.slice[7], 0x02);
        assert_eq!(vga.slice[8], 'o' as u8);
        assert_eq!(vga.slice[9], 0x02);
        assert_eq!(vga.slice[160], 'w' as u8);
        assert_eq!(vga.slice[161], 0x02);
        assert_eq!(vga.slice[162], 'o' as u8);
        assert_eq!(vga.slice[163], 0x02);
        assert_eq!(vga.slice[164], 'r' as u8);
        assert_eq!(vga.slice[165], 0x02);
        assert_eq!(vga.slice[166], 'l' as u8);
        assert_eq!(vga.slice[167], 0x02);
        assert_eq!(vga.slice[168], 'd' as u8);
        assert_eq!(vga.slice[169], 0x02);
        assert_eq!(vga.slice[320], '!' as u8);
        assert_eq!(vga.slice[321], 0x02);
    }

    #[test]
    fn write_scroll() {
        let mut mock_memory = [0u8; ROWS * COL_BYTES];
        let mut vga = unsafe {
            Vga::from_raw(&mut mock_memory[..])
        };

        for b in "abcdefghijklmnopqrstuvwxyz".bytes() {
            vga.write_byte(b);
            vga.write_byte('\n' as u8);
        }

        assert_eq!(vga.slice[0], 'c' as u8);
        for cb in 0..COL_BYTES/2 {
            assert_eq!(vga.slice[(ROWS - 1) * COL_BYTES + (cb * 2)], ' ' as u8);
        }
    }

}
