#![allow(dead_code)]

pub const LOGO: &str =
r#"/$$$$$$$  /$$$$$$$   /$$$$$$   /$$$$$$   /$$$$$$ 
| $$__  $$| $$__  $$ /$$__  $$ /$$__  $$ /$$__  $$
| $$  \ $$| $$  \ $$| $$  \ $$| $$  \__/| $$  \__/
| $$$$$$$ | $$$$$$$/| $$$$$$$$|  $$$$$$ |  $$$$$$ 
| $$__  $$| $$__  $$| $$__  $$ \____  $$ \____  $$
| $$  \ $$| $$  \ $$| $$  | $$ /$$  \ $$ /$$  \ $$
| $$$$$$$/| $$  | $$| $$  | $$|  $$$$$$/|  $$$$$$/
|_______/ |__/  |__/|__/  |__/ \______/  \______/ "#;

pub mod color {
    use crate::system::stdlib::text::color::{Color, ColorCode};

    pub const ERROR: ColorCode = ColorCode::new(Color::LightRed, Color::Black);
    pub const WARNING: ColorCode = ColorCode::new(Color::Yellow, Color::Black);
    pub const NORMAL: ColorCode = ColorCode::new(Color::White, Color::Black);
}

