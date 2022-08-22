#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::library::`test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
// use spin::Mutex;
use core::panic::PanicInfo;

use rustos;
use rustos::library::qemu::{exit_qemu, QemuExitCode};
#[allow(unused)]
use rustos::{serial_print, serial_println};
#[allow(unused)]
use rustos::{print, println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    use rustos::library::renderer::color::{ColorRGB, Color};
    rustos::init(boot_info);
    println!("1 Hello world");
    if let Some(writer) = rustos::library::renderer::TEXTWRITER.get() {
        writer.lock().set_background_color(ColorRGB::vga(Color::White));
    }
    println!("2 Hello world");
    if let Some(writer) = rustos::library::renderer::TEXTWRITER.get() {
        writer.lock().set_background_color(ColorRGB::vga(Color::Black));
    }
    println!("3 Hello world");
    if let Some(writer) = rustos::library::renderer::TEXTWRITER.get() {
        writer.lock().set_foreground_color(ColorRGB::vga(Color::Red));
    }
    println!("4 Hello world");
    println!("5 Hello world");
    if let Some(writer) = rustos::library::renderer::TEXTWRITER.get() {
        writer.lock().shift_frame(1);
    }
    println!("6 Hello world");
    println!("7 Hello world");
    if let Some(writer) = rustos::library::renderer::TEXTWRITER.get() {
        writer.lock().set_foreground_color(ColorRGB::vga(Color::Black));
        writer.lock().set_background_color(ColorRGB::vga(Color::Pink));
    }
    println!("8 Hello world");

    // #[cfg(test)]
    // test_main();

    loop {}
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


#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
