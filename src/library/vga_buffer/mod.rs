pub mod color;
pub mod screen;
#[macro_use]
pub mod interface;

pub use color::{Color, ColorCode};

const VGA_BUFFER_ADDR: usize = 0xb8000;
const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_HEIGHT: usize = 25;
