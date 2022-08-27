// current not work

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(alloc_error_handler)]

extern crate alloc;

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
    serial_println!("Hello, this is tests::framebuffer");
    test_main();
    rustos::hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::library::handler_panic::kernel_panic::panic_handler(info)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) ->! {
    rustos::library::handler_panic::kernel_panic::alloc_error_handler(layout)
}

// test case
#[test_case]
fn test_framebuffer_print() {
    serial_print!("Hello");
    serial_println!(" World!");

    serial_println!("int: {}, float: {}, char: {}, str: {}", 1, 1.0/3.0, 'c', "words");
    assert_eq!(0,0);
}
