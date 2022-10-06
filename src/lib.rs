#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(asm_sym)]
#![feature(naked_functions)]
#![feature(thread_local)]

extern crate alloc;

use bootloader::boot_info::MemoryRegions;
#[allow(unused)]
use bootloader::{entry_point, BootInfo};
use conquer_once::spin::OnceCell;
use library::memory::{
    self, allocator,
    frame_allocator::{self, bootinfo_allocator::BootInfoFrameAllocator},
};
use x86_64::VirtAddr;
#[macro_use]
pub mod library;
pub mod user;

static PHYSICAL_MEMORY_OFFSET: OnceCell<VirtAddr> = OnceCell::uninit();

pub fn init(boot_info: &'static mut BootInfo) {
    serial_println!("Start init");
    PHYSICAL_MEMORY_OFFSET
        .init_once(|| VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap()));
    // let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let framebuffer = boot_info.framebuffer.as_mut().take();

    library::renderer::init(framebuffer);
    library::gdt::init_gdt();
    library::interrupt::init_idt();
    library::interrupt::init_pic();
    library::interrupt::enable_hardware_interrupt(); // enable pic

    unsafe {
        init_memory_map(&mut boot_info.memory_regions);
    }
    // library::task::init();

    // library::context::init();

    // library::filesystem::vfs::init();

    serial_println!("Finished init");

    // set_user_mode();
    // user_space();

    serial_println!("Finished init");
}

// #[naked]
// extern "C" fn set_user_mode() {
//     unsafe {
//         core::arch::asm!(
//             "
//                 cli
//                 // set ds
//                 mov ax, (4 * 8) | 3
//                 mov ds, ax
//                 mov es, ax
//                 mov fs, ax
//                 mov gs, ax

//                 // setup the iret stack frame
//                 mov rax, rsp
//                 lea rbx, {}

//                 push (4 * 8) | 3
//                 push rax
//                 pushf

//                 // enable interrupt
//                 pop rax
//                 or rax, 0x200
//                 push rax

//                 push (3 * 8) | 3
//                 push rbx
//                 iretq
//             ",
//             sym user_space,
//             options(noreturn)
//         )
//     }
// }

// #[naked]
// fn user_space() {
//     unsafe {
//         core::arch::asm!(
//             "
//                 hlt
//             ", options(noreturn)
//         )
//     }
//     // hlt_loop();
// }

unsafe fn init_memory_map(
    // physical_memory_offset: VirtAddr,
    memory_regions: &'static mut MemoryRegions,
) {
    // unsafe: need valid physical_memory_offset
    if let Some(physical_memory_offset) = PHYSICAL_MEMORY_OFFSET.get() {
        // unsafe: need valid memory_region
        // let mut frame_allocator = BootInfoFrameAllocator::init(memory_regions);
        frame_allocator::init(memory_regions);
        
        // let mut mapper = memory::init();
        // let mut mapper = library::memory::PAGEMAPPER.lock();
        // safe
        allocator::init_heap().expect("Heap initialize failed");
        // unsafe
        crate::user::user_init()
            .expect("User space initialisze failed");
    }
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
    use super::serial_println;
    use super::BootInfo;
    use core::panic::PanicInfo;

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

    #[alloc_error_handler]
    fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
        rustos::library::handler_panic::kernel_panic::alloc_error_handler(layout)
    }
}
