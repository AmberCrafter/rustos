#![no_std]
use bootloader::BootInfo;
#[macro_use]
pub mod library;

pub fn init(boot_info: &'static mut BootInfo) {
    library::renderer::init(boot_info);
}