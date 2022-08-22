use core::panic::PanicInfo;

#[allow(unused)]
use crate::{print, println, library::qemu::{exit_qemu, QemuExitCode}, hlt_loop};

#[allow(unused)]
pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("[Failed]");
    println!("Error: {}\n", info);

    exit_qemu(QemuExitCode::Failed);
    hlt_loop()
}

#[allow(unused)]
pub fn should_panic_handler(_info: &PanicInfo) -> ! {
    println!("[Ok]");
    exit_qemu(QemuExitCode::Success);
    hlt_loop()
}