pub mod handler_interrupt;
mod hardware_controller;

// expose hardware_controller
use hardware_controller::PICS;


use spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;

use hardware_controller::InterruptIndex;

use super::gdt;

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(handler_interrupt::breakpoint_handler);
    unsafe {
        idt.double_fault.set_handler_fn(handler_interrupt::double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    // idt[InterruptIndex::Timer.as_usize()].set_handler_fn(handler_interrupt::timer_interrupt_handler);

    idt
});

pub fn init_idt() {
    IDT.load();
}

pub fn init_pic() {
    unsafe{hardware_controller::PICS.lock().initialize()};
}

pub fn enable_hardware_interrupt() {
    x86_64::instructions::interrupts::enable();
}