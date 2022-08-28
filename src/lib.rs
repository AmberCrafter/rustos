#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]
// #![feature(alloc_error_handler)]

// extern crate alloc;


use bootloader::boot_info::MemoryRegions;
#[allow(unused)]
use bootloader::{BootInfo, entry_point};
use library::memory::frame_allocator::bootinfo_allocator::BootInfoFrameAllocator;
#[macro_use]
pub mod library;

pub fn init(boot_info: &'static mut BootInfo) {
    let framebuffer = boot_info.framebuffer.as_mut().take();

    library::renderer::init(framebuffer);
    library::gdt::init_gdt();
    library::interrupt::init_idt();
    library::interrupt::init_pic();
    library::interrupt::enable_hardware_interrupt(); // enable pic
    
}

// unsafe fn init_memory_map(memory_region: &'static mut MemoryRegions) {
//     // unsafe: need valid memory_region
//     let mut frame_allocator = library::memory::frame_allocator::bootinfo_allocator::BootInfoFrameAllocator::init(&boot_info.memory_regions);


// }

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
    use super::serial_println;

    pub fn main(boot_info: &'static mut BootInfo) -> ! {
        super::init(boot_info);
        serial_println!("Hello, this is lib::tests");
        super::test_main();
        super::hlt_loop()
    }


    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        crate::library::handler_panic::kernel_panic::panic_handler(info)
    }

    // #[alloc_error_handler]
    // fn alloc_error_handler(layout: alloc::alloc::Layout) ->! {
    //     rustos::library::handler_panic::kernel_panic::alloc_error_handler(layout)
    // }
}