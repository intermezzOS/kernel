#![no_std]

#[macro_export]
macro_rules! kprintln {
    ($ctx:ident, $fmt:expr) => (kprint!($ctx, concat!($fmt, "\n")));
    ($ctx:ident, $fmt:expr, $($arg:tt)*) => (kprint!($ctx, concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! kprint {
    ($ctx:ident, $($arg:tt)*) => ({
        use core::fmt::Write;
        $ctx.write_fmt(format_args!($($arg)*)).unwrap();
        $ctx.flush();
    });
}
