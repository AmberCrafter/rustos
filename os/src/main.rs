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

#[cfg(not(test))]
entry_point!(kernel::main);

#[cfg(not(test))]
mod kernel {

    use super::PanicInfo;
    use super::BootInfo;
    use super::println;
    use super::serial_println;

    pub fn main(boot_info: &'static mut BootInfo) -> ! {
        rustos::init(boot_info);
        println!("Hello, this is main::kernel");
        serial_println!("Hello, this is main::kernel");
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
}


#[cfg(test)]
entry_point!(tests::main);
#[cfg(test)]
mod tests {
    use super::BootInfo;
    use super::PanicInfo;
    use super::println;

    pub fn main(boot_info: &'static mut BootInfo) -> ! {
        rustos::init(boot_info);
        println!("Hello, this is main::tests");
        super::test_main();
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
}