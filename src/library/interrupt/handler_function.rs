use x86_64::structures::idt::InterruptStackFrame;

use crate::println;

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("[Interrupt] Exception: BREAKPOINT\n{:#?}\n", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("[Interrupt] Exception: DOUBLE_FAULT\n{:#?}\n", stack_frame);
}