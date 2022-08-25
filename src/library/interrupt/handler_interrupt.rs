use x86_64::structures::idt::InterruptStackFrame;

use crate::print;
use crate::println;
use crate::serial_println;

use super::PICS;
use super::InterruptIndex;

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("[Interrupt] Exception: BREAKPOINT\n{:#?}\n", stack_frame);
    serial_println!("[Interrupt] Exception: BREAKPOINT\n{:#?}\n", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("\n[Interrupt] Exception: DOUBLE_FAULT\n{:#?}\n", stack_frame);
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    serial_print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

// https://wiki.osdev.org/%228042%22_PS/2_Controller
// only test on graphic mode
pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    let mut port = Port::new(0x60);
    let scancode:u8 = unsafe {
        port.read()
    };
    print!("Scancode: {:}", scancode);
    serial_print!("Scancode: {:}", scancode);

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}