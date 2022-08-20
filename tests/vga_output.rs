#![no_std]
#![no_main]
#![feature(lang_items)]

use rustos::{library, println, print};

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // turn the screen gray
    print!("Hello, ");
    println!("World!");
    println!("Hello world");
    println!("1 + 1 = {}", 2);
    println!("1 / 3 = {}", 1/3);

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}
