#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::library::test_runner)]
#![reexport_test_harness_main = "test_main"]

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

    pub fn main(boot_info: &'static mut BootInfo) -> ! {
        println!("Hello, this is main::kernel");
        rustos::init(boot_info);
        rustos::hlt_loop();
    }

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        rustos::library::handler::kernel_panic::panic_handler(info)
    }
}


#[cfg(test)]
entry_point!(tests::main);
#[cfg(test)]
mod tests {
    pub fn main(boot_info: &'static mut BootInfo) -> ! {
        println!("Hello, this is main::tests");
        rustos::init(boot_info);
        rustos::hlt_loop()
    }

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        rustos::library::handler::kernel_panic::panic_handler(info)
    }
}