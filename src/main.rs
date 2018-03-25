#![feature(lang_items)]
#![no_std]
#![no_main]

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32,
) -> ! {
    loop {}
}

#[no_mangle]
pub fn _start() -> ! {
    loop {}
}
