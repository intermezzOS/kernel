#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[lang = "begin_unwind"]
pub extern fn begin_unwind() {}
