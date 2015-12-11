#![no_std]

/// Prints a string
pub fn kprintf(s: &str, color: u8, location: *mut u8) {
    for (i, c) in s.bytes().enumerate() {
        unsafe {
            let location = location.offset((i * 2) as isize);
            *location = c;

            let location = location.offset(1 as isize);
            *location = color;
        }
    }
}
