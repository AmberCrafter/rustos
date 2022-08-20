#![no_std]
#![no_main]
#![feature(lang_items)]

use rustos;
use rustos::library;
use rustos::println;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    rustos::init(boot_info);
    // turn the screen gray
    // if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
    //     for byte in framebuffer.buffer_mut() {
    //         *byte = 0x90;
    //     }
    // }

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}
