#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

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
    serial_println!("Hello, this is tests::interrupt");
    test_main();
    rustos::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::library::handler_panic::kernel_panic::panic_handler(info)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) ->! {
    rustos::library::handler_panic::kernel_panic::alloc_error_handler(layout)
}


// #[test_case]
// fn test_interrupt_breakpoint() {
//     // invoke a breakpoint exception
//     x86_64::instructions::interrupts::int3();
//     serial_println!("After invoke breakpoint interrupt");
// }

// #[test_case]
// fn test_interrupt_double_fault() {
//     // invoke a double_fault exception
//     // trigger a page fault
//     unsafe {
//         *(0xdeadbeef as *mut u64) = 42;
//     };
//     serial_println!("After invoke double_fault interrupt");
// }

#[test_case]
fn test_interrupt_page_fault() {
    // invoke a page_fault exception
    // trigger a page fault

    let ptr = 0xdeadbeef as *mut u64;
    unsafe {
        *ptr = 42;
    };
    serial_println!("After invoke page_fault interrupt");
}

