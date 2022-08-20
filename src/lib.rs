#![no_std]

use bootloader::BootInfo;
#[macro_use]
pub mod library;

pub fn init(boot_info: &mut BootInfo) {
    library::vga_buffer::init(boot_info);
}