#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};
// use spin::Mutex;
use core::panic::PanicInfo;

use rustos;
#[allow(unused)]
use rustos::{serial_print, serial_println};
#[allow(unused)]
use rustos::{print, println};


entry_point!(main);
pub fn main(boot_info: &'static mut BootInfo) -> ! {
    rustos::init(boot_info);
    serial_println!("Hello, this is tests::provoke_deadlock");
    test_main();
    rustos::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::library::handler_panic::kernel_panic::panic_handler(info)
}


#[test_case]
fn test_deadlock() {
    // running time interrupt in the background
    loop {
        serial_print!("-");
    }
}

