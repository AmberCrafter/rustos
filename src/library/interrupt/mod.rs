pub mod handler_function;

use spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;


static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(handler_function::breakpoint_handler);
    idt.double_fault.set_handler_fn(handler_function::double_fault_handler);
    idt
});

pub fn init_idt() {
    IDT.load();
}