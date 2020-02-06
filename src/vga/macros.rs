use crate::VGA_WRITER;
use core::fmt::{self, Write};

// Basically just a copy+paste of the std `print!` macro, but it calls the `_print` function
// defined below.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::macros::_print(format_args!($($arg)*)));
}

// See previous, but I've replaced the path in the std definition with the proper one for the above
// macro.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::vga::macros::print!("{}\n", format_args!($($arg)*)));
}

// Just calls a provided method of `Write` on the VGA writer.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    VGA_WRITER.lock().write_fmt(args).unwrap();
}