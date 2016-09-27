extern crate console;
extern crate core;

use core::fmt::Write;
use console::Vga;

#[test]
fn create() {
    let mut mock_memory = vec![0u8; 25 * 80 * 2];

    Vga::new(&mut mock_memory);
}

fn check_write<T: Write>(_: T) { }

#[test]
fn write() {
    let mut mock_memory = vec![0u8; 25 * 80 * 2];
    let vga = Vga::new(&mut mock_memory);
    check_write(vga);
}

#[test]
fn flush() {
    let mut mock_memory = vec![0u8; 25 * 80 * 2];

    {
        let mut vga = Vga::new(&mut mock_memory);

        vga.write_str("hello").unwrap();

        vga.flush();
    }

    assert_eq!(mock_memory[0], 'h' as u8);
    assert_eq!(mock_memory[1], 0x02);
    assert_eq!(mock_memory[2], 'e' as u8);
    assert_eq!(mock_memory[3], 0x02);
    assert_eq!(mock_memory[4], 'l' as u8);
    assert_eq!(mock_memory[5], 0x02);
    assert_eq!(mock_memory[6], 'l' as u8);
    assert_eq!(mock_memory[7], 0x02);
    assert_eq!(mock_memory[8], 'o' as u8);
    assert_eq!(mock_memory[9], 0x02);
}
