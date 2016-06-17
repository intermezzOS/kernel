#![no_std]
#![feature(const_fn)]

#[macro_use]
extern crate vga;
extern crate spin;

extern crate ringbuffer;

use spin::Mutex;
use ringbuffer::RingBuffer;

static KBDUS: [u8; 59] = *b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?";
static KBDUS_SHIFT: [u8; 59] = *b"??!@#$%^&*()_+??QWERTYUIOP{}\n?ASDFGHJKL:\"~?|ZXCVBNM<>??*? ?";

// State of Modifier keys
pub static STATE: Mutex<Modifiers> = Mutex::new(Modifiers {
	shift: false,
	ctrl: false,
	alt: false,
	caps: false,
});

static mut BUFFER: RingBuffer<u8> = RingBuffer::new();

pub struct Modifiers {
    shift: bool,
    ctrl: bool,
    alt: bool,
    caps: bool,
}

impl Modifiers {
	pub fn update_state(&mut self, scancode: u8) {
		// Check if keydown
		match scancode {
			0x2A | 0x36 => self.shift = true,
			0x1D		=> self.ctrl = true,
			0x38 		=> self.alt = true,
			0x3A 		=> self.caps = !self.caps,
			0xAA | 0xB6 => self.shift = false,
			0x9D 		=> self.ctrl = false,
			0xB8 		=> self.alt = false,
			_ => {}
		}
	}
}

pub struct Keyboard;

impl Keyboard {
    pub fn handle_keypress(&self, scancode: u8) {
        unsafe { BUFFER.push(scancode); }
    }

	pub fn handle_input(&self) {
        unsafe {
            if let Some(scancode) = BUFFER.pop() {
                if scancode <= 59 {
                    let state = STATE.lock();
                    if state.shift ^ state.caps {
                        kprint!("{}", KBDUS_SHIFT[scancode as usize] as char);
                    } else {
                        kprint!("{}", KBDUS[scancode as usize] as char);
                    }
                }
            }
        }
	}
}
