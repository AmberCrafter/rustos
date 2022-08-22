pub mod handler;

use spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;


static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(handler::breakpoint_handler);
    idt
});

pub fn init_idt() {
    IDT.load();
}