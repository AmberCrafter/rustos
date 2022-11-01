// We don't have seperated panic/should panic test currently, thus we still use harness wrap the test to use the handler_panic::kernel_panic::should_panic_handler for runner
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use rustos::library::qemu::{exit_qemu, QemuExitCode};
use spin::Lazy;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
// use spin::Mutex;
use core::panic::PanicInfo;

use rustos::{self, hlt_loop};
#[allow(unused)]
use rustos::{print, println};
#[allow(unused)]
use rustos::{serial_print, serial_println};

entry_point!(main);
pub fn main(boot_info: &'static mut BootInfo) -> ! {
    rustos::init(boot_info);
    init_test_idt();
    serial_println!("Hello, this is tests::stack_overflow");
    test_main();
    rustos::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::library::handler_panic::kernel_panic::should_panic_handler(info)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    rustos::library::handler_panic::kernel_panic::alloc_error_handler(layout)
}

static TEST_IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    unsafe {
        idt.double_fault
            .set_handler_fn(test_double_fault_handler)
            .set_stack_index(rustos::library::gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt
});

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[OK]");
    exit_qemu(QemuExitCode::Success);
    // hlt_loop()
}

fn init_test_idt() {
    TEST_IDT.load();
}

#[test_case]
fn test_stack_overflow() {
    #[allow(unconditional_recursion)]
    fn stack_overflow_trigger() {
        stack_overflow_trigger();
        volatile::Volatile::new(0).read_only(); // prevent tail recursion optimizations
    }

    stack_overflow_trigger();
    serial_println!("[Error] Never rich here.")
}
