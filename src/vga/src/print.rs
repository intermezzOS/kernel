/// Prints something to the screen, with a trailing newline.
///
/// # Examples
///
/// ```ignore
/// kprintln!("Hello, world!");
/// ```
#[macro_export]
macro_rules! kprintln {
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
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
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut b = $crate::BUFFER.lock();
        b.write_fmt(format_args!($($arg)*)).unwrap();
        b.flush();
    });
}
