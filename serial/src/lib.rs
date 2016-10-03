//! Dealing with the Serial Port
//!
//! References:
//!
//! * [OSDev wiki](http://wiki.osdev.org/Serial_Ports)
//! * [Wikipedia](https://en.wikipedia.org/wiki/Serial_port)
//! * [Little OS Book] (https://littleosbook.github.io/#the-serial-ports)

#![no_std]
extern crate cpuio;

use core::fmt;
use core::fmt::Write;

use cpuio::Port;

const PORT: u16 = 0x3F8; // COM1

pub struct Serial {
	data: Port<u8>,
	status: Port<u8>,
}

impl Serial {
	pub fn new() -> Serial {

		// Initialize
		unsafe {
			Port::new(PORT + 1u16).write(0x00u8);  // Disable interrupts
			Port::new(PORT + 3u16).write(0x80u8);  // Enable DLAB (to set baud rate divisor)
			Port::new(PORT + 0u16).write(0x03u8);  // Set divisor to 3 (lo byte) for 38400hz baud
			Port::new(PORT + 1u16).write(0x00u8);  //                  (hi byte)
			Port::new(PORT + 3u16).write(0x03u8);  // 8 bits, no parity, one stop bit
			Port::new(PORT + 2u16).write(0xC7u8);  // Enable FIFO, clear them, with 14b threshold
			Port::new(PORT + 4u16).write(0x0Bu8);  // IRQs enabled, RTS/DSR set
		}

		Serial {
			data: unsafe { Port::new(PORT) },
			status: unsafe { Port::new(PORT + 5u16) },
		}
	}


	pub fn write_byte(&mut self, byte: u8) {
		while self.status.read() & 0x20 == 0 { continue; }

		self.data.write(byte);
	}

	pub fn read_byte(&mut self) -> u8 {
		while self.status.read() & 0x1 == 0 { continue; }

		self.data.read()
	}
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for b in s.bytes() {
            self.write_byte(b);
        }

        Ok(())
    }
}
