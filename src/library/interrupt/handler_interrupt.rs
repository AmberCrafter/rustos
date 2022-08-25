use pc_keyboard::Keyboard;
use spin::Lazy;
use spin::Mutex;
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
// need to read out the buffer, otherwise keyboard interrupt will be stuck
// Use the pc-keyboard to decode it 
pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{ layouts::Us104Key, ScancodeSet1, HandleControl::Ignore, DecodedKey };
    use x86_64::instructions::port::Port;

    static KEYBOARD: Lazy<Mutex<Keyboard<Us104Key, ScancodeSet1>>> = Lazy::new(|| {
        let mut keyboard = pc_keyboard::Keyboard::new(
            Us104Key, ScancodeSet1, Ignore
        );
        Mutex::new(keyboard) 
    });

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode:u8 = unsafe {
        port.read()
    };
    
    if let Ok(Some(event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(envet) {
            match key {
                DecodedKey::RawKey(key) => {
                    print!("{:?}", key);
                    serial_print!("{:?}", key);
                },
                DecodedKey::Unicode(charactor) => {
                    print!("{:}", charactor);
                    serial_print!("{:}", charactor);
                }
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}