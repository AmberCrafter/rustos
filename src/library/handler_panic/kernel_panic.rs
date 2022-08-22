use core::panic::PanicInfo;

#[allow(unused)]
use crate::{serial_print, serial_println};
#[allow(unused)]
use crate::{print, println, library::qemu::{exit_qemu, QemuExitCode}, hlt_loop};

#[allow(unused)]
pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("[Panic]");
    println!("Error: {}\n", info);

    serial_println!("[Failed]");
    serial_println!("Error: {}\n", info);

    exit_qemu(QemuExitCode::Failed);
    hlt_loop()
}

#[allow(unused)]
pub fn should_panic_handler(_info: &PanicInfo) -> ! {
    println!("[Ok]");

    serial_println!("[Ok]");
    exit_qemu(QemuExitCode::Success);
    hlt_loop()
}