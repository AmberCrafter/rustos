use bootloader::{BootInfo, boot_info::FrameBuffer};

pub use self::text_renderer::TEXTWRITER;

pub mod color;
pub mod text_renderer;
pub mod pc_keyboard_interface;

pub fn init(framebuffer: Option<&'static mut FrameBuffer>) {
    if framebuffer.is_none() {panic!("FrameBuffer initialize failed");}
    text_renderer::init(framebuffer);
}
