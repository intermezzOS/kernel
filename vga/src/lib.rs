#![no_std]

const CONSOLE_SIZE: isize = 4000;

/// Prints a string
pub fn kprintf(s: &str, color: u8) {
    let location = 0xb8000 as *mut u8;
    for (i, c) in s.bytes().enumerate() {
        unsafe {
            let location = location.offset((i * 2) as isize);
            *location = c;

            let location = location.offset(1 as isize);
            *location = color;
        }
    }
}

/// Clears the console
pub fn clear_console() {
    let location = 0xb8000 as *mut u8;
    let color = 0x0a;
    let c = ' ' as u8;
    for i in 0..CONSOLE_SIZE {
        unsafe {
            let location = location.offset((i * 2) as isize);
            *location = c;

            let location = location.offset(1 as isize);
            *location = color;
        }
    }
}

