#![no_std]

static CODES: [u8; 59] = *b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?";

pub fn from_scancode(code: usize) -> Option<char> {
    if code <= 59 { 
        Some(CODES[code] as char)
    } else {
        None
    }
}
