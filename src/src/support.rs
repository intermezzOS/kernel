#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}
#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}

#[lang = "begin_unwind"]
pub extern "C" fn begin_unwind() {
}
