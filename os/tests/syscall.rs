#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)]

extern crate alloc;

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
    serial_println!("Hello, this is tests::syscall");
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
fn test_syscall_syscallno() {
    use rustos::library::syscall;
    assert_eq!(syscall::Syscall::Read, 0_usize.try_into().unwrap());
    assert_eq!(syscall::Syscall::Write, 1_usize.try_into().unwrap());
    assert_eq!(syscall::Syscall::Getpid, 39_usize.try_into().unwrap());
    assert_eq!(
        syscall::Syscall::GetThreadArea,
        211_usize.try_into().unwrap()
    );
    assert_eq!(syscall::Syscall::FinitModule, 313_usize.try_into().unwrap());

    assert!(TryInto::<syscall::Syscall>::try_into(314_usize).is_err());
    assert!(TryInto::<syscall::Syscall>::try_into(999_usize).is_err());
}
