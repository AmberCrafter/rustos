#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]


#[allow(unused)]
use bootloader::{BootInfo, entry_point};
#[macro_use]
pub mod library;

pub fn init(boot_info: &'static mut BootInfo) {
    library::renderer::init(boot_info);
    library::interrupt::init_idt();
    library::gdt::init_gdt();
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
    use super::BootInfo;
    use super::println;

    pub fn main(boot_info: &'static mut BootInfo) -> ! {
        super::init(boot_info);
        println!("Hello, this is lib::tests");
        super::test_main();
        super::hlt_loop()
    }


    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        crate::library::handler_panic::kernel_panic::panic_handler(info)
    }
}