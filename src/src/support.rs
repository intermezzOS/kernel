#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}

#[lang = "panic_fmt"]
#[unwind]
pub extern fn rust_begin_panic() -> ! {
    loop {}
}
