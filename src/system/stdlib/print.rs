//! Implementations of the rust print!() and println!() family of macros

use core::fmt;
use crate::system::stdlib::text::color::{Color, ColorCode};

/// Print formatted text to the display
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::system::stdlib::print::_print(format_args!($($arg)*)));
}

/// Print formatted text to the display
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// print error in RED
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::system::stdlib::print::_eprint(format_args!($($arg)*)));
}

/// print error in RED
#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => {
        ($crate::eprint!("{}\n", format_args!($($arg)*)))
    };
}

/// print warning in YELLOW
#[macro_export]
macro_rules! wprint {
    ($($arg:tt)*) => ($crate::system::stdlib::print::_wprint(format_args!($($arg)*)));
}

/// print warning in YELLOW
#[macro_export]
macro_rules! wprintln {
    () => ($crate::wprint!("\n"));
    ($($arg:tt)*) => {
        ($crate::wprint!("{}\n", format_args!($($arg)*)))
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    super::display::DISPLAY.lock().write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut display = super::display::display().lock();

    let old_color = display.writer.color_code;
    display.writer.color_code = crate::config::color::ERROR;
    display.write_fmt(args).unwrap();
    display.writer.color_code = old_color;
}

#[doc(hidden)]
pub fn _wprint(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut display = super::display::display().lock();

    let old_color = display.writer.color_code;
    display.writer.color_code = crate::config::color::WARNING;
    display.write_fmt(args).unwrap();
    display.writer.color_code = old_color;
}