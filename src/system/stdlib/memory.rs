use super::text::screen::Screen;

pub mod addr {
    /// Address to the VGA buffer, used for text writing
    pub const VGA_BUFFER: usize = 0xb8000;
}

/// Obtain mutable pointer to VGA buffer, used for text writing
#[no_mangle]
pub const fn vga_buffer() -> *mut u8 {
    addr::VGA_BUFFER as *mut u8
}

/// Get a mutable pointer to VGA buffer as a `Screen` type, useful for advanced writing.
#[no_mangle]
pub fn vga_buffer_screen() -> &'static mut Screen {
    unsafe {
        &mut *(addr::VGA_BUFFER as *mut Screen)
    }
}