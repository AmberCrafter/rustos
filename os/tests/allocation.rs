#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::collections::BTreeMap;
use bootloader::{entry_point, BootInfo};
// use spin::Mutex;
use core::panic::PanicInfo;

use rustos;
#[allow(unused)]
use rustos::{print, println};
#[allow(unused)]
use rustos::{serial_print, serial_println};

entry_point!(main);
pub fn main(boot_info: &'static mut BootInfo) -> ! {
    rustos::init(boot_info);
    serial_println!("Hello, this is tests::allocation");
    test_main();
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

// test case
#[test_case]
fn test_alloc_box() {
    use alloc::boxed::Box;
    use alloc::rc::Rc;
    use alloc::vec;
    use alloc::vec::Vec;

    let x = Box::new(10);
    serial_println!("Box ptr: {:p}\tvalue: {:?}", x, x);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    serial_println!("Vec ptr: {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    serial_println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    serial_println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    serial_println!("Test BTreeMap");
    let mut map = BTreeMap::new();
    map.insert(1, "One");
    map.insert(2, "Two");
    map.insert(3, "Three");

    for key in map.keys() {
        serial_println!("key: {:?}, value: {:?}", key, map.get(key));
    }
}
