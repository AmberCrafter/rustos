pub mod handler_interrupt;
mod hardware_controller;

// expose hardware_controller
use hardware_controller::PICS;


use spin::Lazy;
use x86_64::{structures::idt::InterruptDescriptorTable, set_general_handler};

use hardware_controller::InterruptIndex;

use super::gdt;

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(handler_interrupt::breakpoint_handler);
    unsafe {
        idt.double_fault.set_handler_fn(handler_interrupt::double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt.page_fault.set_handler_fn(handler_interrupt::page_fault_handler);
    idt.general_protection_fault.set_handler_fn(handler_interrupt::general_protection_fault_handler);
    idt.stack_segment_fault.set_handler_fn(handler_interrupt::stack__segment_fault_handler);

    idt[InterruptIndex::Timer.as_usize()].set_handler_fn(handler_interrupt::timer_interrupt_handler);
    idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(handler_interrupt::keyboard_interrupt_handler);


    idt[0x80].set_handler_fn(handler_interrupt::syscall_handler);

    // unsafe {
    //     idt[0x80]
    //         .set_handler_fn(unsafe {
    //             core::mem::transmute(handler_interrupt::syscall_handler_naked_wrap as *mut fn())
    //         })
    //         .set_stack_index(0);
    // }
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