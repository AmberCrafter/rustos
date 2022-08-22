// current not work

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
// use spin::Mutex;
use core::panic::PanicInfo;

use rustos;
#[allow(unused)]
use rustos::{print, println};
#[allow(unused)]
use rustos::{serial_print, serial_println};

entry_point!(main);
pub fn main(boot_info: &'static mut BootInfo) -> ! {
    rustos::init(boot_info);
    println!("Hello, this is tests::framebuffer");
    test_main();
    rustos::hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::library::handler::kernel_panic::panic_handler(info)
}

// test case
#[test_case]
fn framebuffer_print() {
    print!("Hello");
    println!(" World!");

    println!("int: {}, float: {}, char: {}, str: {}", 1, 1.0/3.0, 'c', "words");
    assert_eq!(0,0);
}
