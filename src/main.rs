#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::boot_info::FrameBuffer;
use bootloader::{entry_point, BootInfo};
// use spin::Mutex;
use core::panic::PanicInfo;

use rustos::library;
use rustos::library::render;
use rustos::{serial_print, serial_println};
use rustos::{print, println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // // init writer
    // let mut writer = if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
    //     let buffer_info = framebuffer.info();
    //     let mut buffer = framebuffer.buffer_mut();
    //     render::TEXTWRITER
    //         .get_or_init(move || Mutex::new(render::TextWriter::new(&mut buffer, buffer_info)))
    // } else {
    //     panic!("Screen renderer init failed");
    // };

    // use core::fmt::Write;
    // writeln!(writer.lock(), "Hello world").unwrap();
    println!("Hello world");

    // #[cfg(test)]
    // test_main();

    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }

    loop {}
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
