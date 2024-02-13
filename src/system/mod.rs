pub mod stdlib;

use stdlib::display::display;
use crate::config::*;
use stdlib::text::color::{Color, ColorCode};

pub fn init() -> ! {
    // Print startup message
    {
        let mut display = display().lock();

        display.show_logo(LOGO);
        display.print_colored(concat!("Brass OS version ", env!("CARGO_PKG_VERSION")), ColorCode::new(Color::Yellow, Color::Black));
        display.newline();
        display.print_colored("--------------------------------------------------", ColorCode::new(Color::LightGray, Color::Black));
        display.newline();
    }

    loop {}
}