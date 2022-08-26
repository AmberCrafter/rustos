// current not work

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use rustos::library::memory;
use x86_64::VirtAddr;
use x86_64::structures::paging::Translate;
// use spin::Mutex;
use core::panic::PanicInfo;

use rustos;
#[allow(unused)]
use rustos::{print, println};
#[allow(unused)]
use rustos::{serial_print, serial_println};

entry_point!(main);
pub fn main(boot_info: &'static mut BootInfo) -> ! {
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    
    rustos::init(boot_info);
    serial_println!("Hello, this is tests::page");
    test_mapper(physical_memory_offset);
    
    test_main();
    rustos::hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::library::handler_panic::kernel_panic::panic_handler(info)
}

// test case
// #[test_case]
// fn test_level4_address() {
//     use rustos::library::memory::page::print_level4_address;
//     print_level4_address();
// }

// I can't figure out how to parse the args into test funtion
// fn test_show_p4_entries(physical_memory_offset: VirtAddr) {
//     let p4 = unsafe {
//         get_level_4_table(physical_memory_offset)
//     };

//     show_entries(p4);
// }

fn test_mapper(physical_memory_offset: VirtAddr) {
    let mapper = unsafe {
        memory::init(physical_memory_offset)
    };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        physical_memory_offset.as_u64(),
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { mapper.translate_addr(virt) };
        println!("{:?} -> {:?}", virt, phys);
        serial_println!("{:?} -> {:?}", virt, phys);
    }
}