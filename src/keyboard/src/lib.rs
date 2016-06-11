#![no_std]

#[macro_use]
extern crate vga;

static KBDUS: [u8; 59] = *b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?";

pub struct Keyboard;

impl Keyboard {
	pub fn handle_keys(&self, scancode: usize) {
		if scancode >= 0 && scancode <= 59 {
			kprint!("{}", KBDUS[scancode] as char);
		}
	}
}