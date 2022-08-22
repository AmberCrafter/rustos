#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[allow(unused)]
use bootloader::{BootInfo, entry_point};
#[macro_use]
pub mod library;

pub fn init(boot_info: &'static mut BootInfo) {
    library::renderer::init(boot_info);
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
entry_point!(tests::main);

#[cfg(test)]
mod tests {
    use core::panic::PanicInfo;
    use super::{print, println};


    pub fn main(boot_info: &'static mut BootInfo) -> ! {
        super::init(boot_info);
        super::test_main();
        super::hlt_loop()
    }


    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        crate::library::handler::kernel_panic::panic_handler(info)
    }
}