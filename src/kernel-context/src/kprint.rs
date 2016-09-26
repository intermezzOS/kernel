/// Prints something to the screen, with a trailing newline.
///
/// # Examples
///
/// ```ignore
/// kprintln!("Hello, world!");
/// ```
#[macro_export]
macro_rules! kprintln {
    ($ctx:ident, $fmt:expr) => (kprint!($ctx, concat!($fmt, "\n")));
    ($ctx:ident, $fmt:expr, $($arg:tt)*) => (kprint!($ctx, concat!($fmt, "\n"), $($arg)*));
}

/// Prints something to the screen.
///
/// # Examples
///
/// ```ignore
/// kprint!("Hello, world!");
/// ```
#[macro_export]
macro_rules! kprint {
    ($ctx:ident, $($arg:tt)*) => ({
        use core::fmt::Write;
        let mut vga = $ctx.vga.lock();
        vga.write_fmt(format_args!($($arg)*)).unwrap();
        vga.flush();
    });
}
