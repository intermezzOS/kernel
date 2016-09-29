use core::fmt;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    loop {}
}
