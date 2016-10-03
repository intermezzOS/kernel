/// Writes something to the serial port, with a trailing newline.
///
/// # Examples
///
/// ```ignore
/// kdebug!("Hello, World!");
/// ```
#[macro_export]
macro_rules! kdebugln {
    ($ctx:ident, $fmt:expr) => (kdebug!($ctx, concat!($fmt, "\n")));
    ($ctx:ident, $fmt:expr, $($arg:tt)*) => (kdebug!($ctx, concat!($fmt, "\n"), $($arg)*));
}

/// Writes something to the serial port.
///
/// # Examples
///
/// ```ignore
/// kprint!("Hello, World!");
/// ```
#[macro_export]
macro_rules! kdebug {
    ($ctx:ident, $($arg:tt)*) => ({
        use core::fmt::Write;
        let mut serial = $ctx.serial.lock();
        serial.write_fmt(format_args!($($arg)*)).unwrap();
    });
}
