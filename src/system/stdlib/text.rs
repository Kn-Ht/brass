use super::memory;
use volatile::Volatile;

pub trait Writable {
    fn write_byte(&mut self, b: u8);
    fn write_str(&mut self, s: &str);
    fn newline(&mut self);
    fn clear_row(&mut self, row: usize);
}

pub mod color {

    /// Color for printing colored text to VGA
    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Color {
        Black = 0,
        Blue = 1,
        Green = 2,
        Cyan = 3,
        Red = 4,
        Magenta = 5,
        Brown = 6,
        LightGray = 7,
        DarkGray = 8,
        LightBlue = 9,
        LightGreen = 10,
        LightCyan = 11,
        LightRed = 12,
        Pink = 13,
        Yellow = 14,
        White = 15,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ColorCode(u8);

    impl ColorCode {
        pub const fn new(fg: Color, bg: Color) -> Self {
            Self((bg as u8) << 4 | (fg as u8))
        }
    }
}

pub mod screen {
    use self::{color::Color, memory::{vga_buffer, vga_buffer_screen}};

    use super::{*, color::ColorCode};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(C)]
    pub struct Char {
        char: u8,
        color: ColorCode,
    }

    impl Char {
        pub const fn new(char: u8, color: ColorCode) -> Self {
            Self {char, color}
        }
    }

    pub const SCREEN_W: usize = 80;
    pub const SCREEN_H: usize = 25;

    /// The screen buffer which holds the characters
    #[repr(transparent)]
    pub struct Screen {
        chars: [[Volatile<Char>; SCREEN_W]; SCREEN_H],
    }

    /// Writer will write to the last line and is able to shift up or down the screen
    pub struct Writer {
        pub column: usize,
        pub color_code: ColorCode,
        pub screen: &'static mut Screen,
    }

    impl Writer {
        pub fn new() -> Self {
            Self {
                column: 0,
                color_code: crate::config::color::NORMAL,
                screen: vga_buffer_screen()
            }
        }
    }

    impl Writable for Writer {
        fn write_byte(&mut self, byte: u8) {
            match byte {
                b'\n' => self.newline(),
                byte => {
                    if self.column >= SCREEN_W {
                        self.newline();
                    }

                    let row = SCREEN_H - 1;
                    let col = self.column;

                    let color_code = self.color_code;
                    self.screen.chars[row][col].write(Char::new(byte, color_code));
                    self.column += 1;
                }
            }
        }
        fn newline(&mut self) {
            for row in 1..SCREEN_H {
                for col in 0..SCREEN_W {
                    let character = self.screen.chars[row][col].read();
                    self.screen.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(SCREEN_H - 1);
            self.column = 0;
        }
        fn clear_row(&mut self, row: usize) {
            let blank = Char::new(b' ', self.color_code);
            for col in 0..SCREEN_W {
                self.screen.chars[row][col].write(blank);
            }
        }
        fn write_str(&mut self, s: &str) {
            for byte in s.bytes() {
                match byte {
                    // printable ASCII byte or newline
                    0x20..=0x7e | b'\n' => self.write_byte(byte),
                    // not part of printable ASCII range
                    _ => self.write_byte(0xfe),
                }
    
            }
        }
    }
}

#[macro_export]
macro_rules! debug {
    () => {
        
    };
}