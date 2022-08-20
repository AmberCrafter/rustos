pub mod color;
pub mod screen;
#[macro_use]
pub mod interface;

use bootloader::BootInfo;
pub use color::{Color, ColorCode};
use interface::Writer;
use screen::Buffer;

// use lazy_static::lazy_static;
use spin::Mutex;
use conquer_once::spin::OnceCell;

// const VGA_BUFFER_ADDR: usize = 0xb8000;
const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_HEIGHT: usize = 25;
static VGA_BUFFER: OnceCell<Mutex<Writer>> = OnceCell::uninit();


pub fn init(boot_info: &mut BootInfo) {
    VGA_BUFFER.try_init_once(|| {
        Mutex::new(interface::Writer::new(
            0,
            ColorCode::new(Color::LightGreen, Color::Black),
            unsafe {
                &mut *(boot_info.framebuffer.as_mut().unwrap() as *mut _ as *mut Buffer)
            }
        ))
    });
}