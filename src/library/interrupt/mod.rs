pub mod handler_function;

use spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;

use super::gdt;


static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(handler_function::breakpoint_handler);
    unsafe {
        idt.double_fault.set_handler_fn(handler_function::double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt
});

pub fn init_idt() {
    IDT.load();
}