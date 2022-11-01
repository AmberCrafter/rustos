#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)]
#![feature(asm_const)]

extern crate alloc;

// use alloc::collections::BTreeMap;
use bootloader::{entry_point, BootInfo};
// use spin::Mutex;
use core::arch::asm;
use core::panic::PanicInfo;

use rustos;
#[allow(unused)]
use rustos::{print, println};
#[allow(unused)]
use rustos::{serial_print, serial_println};

entry_point!(main);
pub fn main(boot_info: &'static mut BootInfo) -> ! {
    rustos::init(boot_info);
    serial_println!("Hello, this is tests::idt_syscall");
    test_main();
    rustos::hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::library::handler_panic::kernel_panic::panic_handler(info)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    rustos::library::handler_panic::kernel_panic::alloc_error_handler(layout)
}

// test case
#[test_case]
fn test_interrupt_syscall() {
    let mut reg: u64 = 0;
    unsafe {
        asm!(
            "mov ecx, 0xC0000102",
            "rdmsr",
            "shl rdx, 32",
            "or rax, rdx",
            out("rax") reg
        )
    }

    serial_println!("\n>>>>>>>>>>>>\nrax: {:x?}", reg);

    // unsafe {
    //     asm!(
    //         "mov rax, 0x01",
    //         "mov rcx, 0x02",
    //         "mov rdx, 0x03",
    //         "int 0x80",
    //         // "sysenter",
    //         out("rax") _, out("rcx") _, out("rdx") _,
    //     );
    //     // x86_64::software_interrupt!(0x80);
    // }
    serial_println!("After invoke syscall interrupt");
}

// #[test_case]
// fn test_interrupt_breakpoint() {
//     // invoke a breakpoint exception
//     x86_64::instructions::interrupts::int3();
//     serial_println!("After invoke breakpoint interrupt");
// }
