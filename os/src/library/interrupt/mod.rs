pub mod handler_interrupt;
mod hardware_controller;

// expose hardware_controller
use hardware_controller::PICS;

use spin::Lazy;
use x86_64::{set_general_handler, structures::idt::InterruptDescriptorTable};

use hardware_controller::InterruptIndex;

use super::gdt;
pub use handler_interrupt::STDIN_BUFFER;

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    idt.divide_error
        .set_handler_fn(handler_interrupt::divide_error_handler);
    unsafe {
        idt.debug
            .set_handler_fn(handler_interrupt::debug_handler)
            .set_stack_index(gdt::DEBUG_IST_INDEX);
        idt.non_maskable_interrupt
            .set_handler_fn(handler_interrupt::non_maskable_interrupt_handler)
            .set_stack_index(gdt::NON_MASKABLE_INTERRUPT_IST_INDEX);
    }
    idt.breakpoint
        .set_handler_fn(handler_interrupt::breakpoint_handler);
    idt.overflow
        .set_handler_fn(handler_interrupt::overflow_handler);
    idt.bound_range_exceeded
        .set_handler_fn(handler_interrupt::bound_range_exceeded_handler);
    idt.invalid_opcode
        .set_handler_fn(handler_interrupt::invalid_opcode_handler);
    idt.device_not_available
        .set_handler_fn(handler_interrupt::device_not_available_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(handler_interrupt::double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt.invalid_tss
        .set_handler_fn(handler_interrupt::invalid_tss_handler);
    idt.segment_not_present
        .set_handler_fn(handler_interrupt::segment_not_present_handler);
    idt.stack_segment_fault
        .set_handler_fn(handler_interrupt::stack_segment_fault_handler);
    idt.general_protection_fault
        .set_handler_fn(handler_interrupt::general_protection_fault_handler);
    idt.page_fault
        .set_handler_fn(handler_interrupt::page_fault_handler);
    idt.x87_floating_point
        .set_handler_fn(handler_interrupt::x87_floating_point_handler);
    idt.alignment_check
        .set_handler_fn(handler_interrupt::alignment_check_handler);
    idt.machine_check
        .set_handler_fn(handler_interrupt::machine_check_handler);
    idt.simd_floating_point
        .set_handler_fn(handler_interrupt::simd_floating_point_handler);
    idt.security_exception
        .set_handler_fn(handler_interrupt::security_exception_handler);

    idt[InterruptIndex::Timer.as_usize()]
        .set_handler_fn(handler_interrupt::timer_interrupt_handler);
    idt[InterruptIndex::Keyboard.as_usize()]
        .set_handler_fn(handler_interrupt::keyboard_interrupt_handler);

    idt[0x80].set_handler_fn(handler_interrupt::syscall_handler_naked_wrap);

    idt
});

pub fn init_idt() {
    IDT.load();
}

pub fn init_pic() {
    unsafe { hardware_controller::PICS.lock().initialize() };
}

pub fn enable_hardware_interrupt() {
    x86_64::instructions::interrupts::enable();
}

pub fn disable_hardware_interrupt() {
    x86_64::instructions::interrupts::disable();
}

pub fn idt_ptr() -> x86_64::structures::DescriptorTablePointer {
    x86_64::instructions::tables::sidt()
}
