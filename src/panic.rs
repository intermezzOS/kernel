use core::fmt;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: fmt::Arguments,
                               file: &'static str,
                               line: u32) -> ! {
    unsafe {
        use core::fmt::Write;
        let mut vga = ::PRINT_REF.unwrap().lock();
        vga.write_fmt(format_args!("PANIC: {} {} {}", msg, file, line)).unwrap();
        vga.flush();
    }
    loop {}
}
