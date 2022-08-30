use pc_keyboard::KeyEvent;
use pc_keyboard::Keyboard;
use spin::Lazy;
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;

use crate::hlt_loop;
use crate::library::renderer::TEXTWRITER;
use crate::library::task;
use crate::print;
use crate::println;
use crate::serial_println;

use super::PICS;
use super::InterruptIndex;

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("\n[Interrupt] Exception: BREAKPOINT\n{:#?}\n", stack_frame);
    serial_println!("\n[Interrupt] Exception: BREAKPOINT\n{:#?}\n", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("\n[Interrupt] Exception: DOUBLE_FAULT\n{:#?}\n", stack_frame);
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");
    // serial_print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

// https://wiki.osdev.org/%228042%22_PS/2_Controller
// only test on graphic mode
// need to read out the buffer, otherwise keyboard interrupt will be stuck
// Use the pc-keyboard to decode it 
pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    // use crate::library::renderer::pc_keyboard_interface;

    let mut port = Port::new(0x60);
    let scancode:u8 = unsafe {
        port.read()
    };
    // println!("Scancode: {:?}", scancode);
    // pc_keyboard_interface::execute(scancode);
    task::keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}


pub extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    //The CR2 register is automatically set by the CPU on a page fault and contains the accessed virtual address that caused the page fault.
    use x86_64::registers::control::Cr2;
    println!("\n[Interrupt] Exception: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);

    serial_println!("\n[Interrupt] Exception: PAGE FAULT");
    serial_println!("Accessed Address: {:?}", Cr2::read());
    serial_println!("Error Code: {:?}", error_code);
    serial_println!("{:#?}", stack_frame);

    hlt_loop()
}