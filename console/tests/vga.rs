extern crate console;
extern crate core;

use core::fmt::Write;
use console::Vga;

#[test]
fn create() {
    let mut mock_memory = vec![0u8; 25 * 80 * 2];

    unsafe {
        Vga::from_raw(&mut mock_memory)
    };
}

fn check_write<T: Write>(_: T) { }

#[test]
fn write() {
    let mut mock_memory = vec![0u8; 25 * 80 * 2];

    let vga = unsafe {
        Vga::from_raw(&mut mock_memory)
    };
    check_write(vga);
}
