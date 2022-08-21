use bootloader::BootInfo;

pub use self::text_renderer::TEXTWRITER;

pub mod color;
pub mod text_renderer;

pub fn init(boot_info: &'static mut BootInfo) {
    text_renderer::init(boot_info);
}
