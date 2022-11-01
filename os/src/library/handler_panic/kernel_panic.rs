use core::panic::PanicInfo;

#[allow(unused)]
use crate::{
    hlt_loop,
    library::qemu::{exit_qemu, QemuExitCode},
    print, println,
};
#[allow(unused)]
use crate::{serial_print, serial_println};

#[allow(unused)]
pub fn panic_handler(info: &PanicInfo) -> ! {
    // println!("[Panic]");
    // println!("Error: {}\n", info);

    serial_println!("[Panic]");
    serial_println!("Error: {}\n", info);

    exit_qemu(QemuExitCode::Failed);
    hlt_loop()
}

#[allow(unused)]
pub fn should_panic_handler(info: &PanicInfo) -> ! {
    println!("[Ok]");

    serial_println!("[Ok]");
    serial_println!("Panic info:\n{:#?}\n", info);

    exit_qemu(QemuExitCode::Success);
    hlt_loop()
}

#[allow(unused)]
pub fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("alloc error: {:?}", layout)
}
