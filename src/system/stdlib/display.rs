use core::fmt;
use core::mem::MaybeUninit;

use lazy_static::lazy_static;
use spin::Mutex;

use super::{memory::vga_buffer_screen, text::{color::{Color, ColorCode}, screen::{Screen, Writer}, Writable}};

pub struct Display {
    pub screen: &'static mut Screen,
    pub writer: Writer,
}

impl Display {
    pub fn new() -> Self {
        Self {
            screen: vga_buffer_screen(),
            writer: Writer::new()
        }
    }

    /// Print colored text
    #[inline]
    pub fn print_colored(&mut self, text: &str, color: ColorCode) {
        let old_color = self.writer.color_code;
        self.writer.color_code = color;
        self.writer.write_str(text);
        self.writer.color_code = old_color;
    }

    pub fn show_logo(&mut self, logo: &str) {
        self.print_colored(logo, ColorCode::new(Color::LightCyan, Color::Black));
        self.writer.newline();
    }

    #[inline(always)]
    pub fn newline(&mut self) {
        self.writer.newline();
    }

    pub fn println(&mut self, text: &str) {
        self.writer.write_str(text);
        self.writer.newline();
    }
    /// Print a message to the screen from the system, useful for debugging
    pub fn msg(&mut self, text: &str) {
        let old_color = self.writer.color_code;

        self.writer.color_code = ColorCode::new(Color::LightGreen, Color::Black);
        self.writer.write_str("[SYSTEM]");
        self.writer.color_code = old_color;
        self.writer.write_byte(b' ');

        self.writer.write_str(text);
        self.writer.newline();
    }
}

impl fmt::Write for Display {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer.write_str(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref DISPLAY: Mutex<Display> = Mutex::new(Display::new());
}

pub fn display() -> &'static Mutex<Display> {
    &DISPLAY
}