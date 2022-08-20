use core::ops::{Deref, DerefMut};

use volatile::Volatile;

use super::{color::ColorCode, VGA_BUFFER_WIDTH, VGA_BUFFER_HEIGHT};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode
}

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT]
}

impl Deref for ScreenChar {
    type Target = Self;
    fn deref(&self) -> &Self::Target {
        self
    }
}

impl DerefMut for ScreenChar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

impl ScreenChar {
    pub fn new(ascii_char: u8, color_code: ColorCode) -> Self {
        Self { ascii_char, color_code }
    }
}