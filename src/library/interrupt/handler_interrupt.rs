use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;
use core::arch::asm;

use crate::hlt_loop;
use crate::library::task;

#[allow(unused)]
use crate::{print, println};
#[allow(unused)]
use crate::{serial_print, serial_println};

use super::InterruptIndex;
use super::PICS;

// inspired by https://github.com/AmberCrafter/arrayJY_os/blob/master/os/src/interrupts/exception_handlers.rs
macro_rules! def_handler_func {
    ($name: tt, $info: expr) => {
        pub extern "x86-interrupt" fn $name(stack_frame: InterruptStackFrame) -> ! {
            panic!("\nEXCEPTION: {}\n{:#?}\n", $info, stack_frame);
        }
    };
}

macro_rules! def_handler_func_with_errorcode {
    ($name: tt, $info: expr) => {
        pub extern "x86-interrupt" fn $name(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
            panic!("\nEXCEPTION: {}\nErrorCode: {:x}\n{:#?}\n", $info, error_code, stack_frame);
        }
    };
}


def_handler_func!(divide_error_handler, "Divide Error");
def_handler_func!(debug_handler, "Debug");
def_handler_func!(overflow_handler, "Overflow");
def_handler_func!(device_not_available_handler, "Device Not Available");
def_handler_func!(non_maskable_interrupt_handler, "Non Maskable Interrupt");
def_handler_func!(bound_range_exceeded_handler, "Bound Range Exceeded");
def_handler_func!(invalid_opcode_handler, "Invalid Opcode");
def_handler_func!(x87_floating_point_handler, "x87 Floating Point");
def_handler_func!(machine_check_handler, "Machine Check");
def_handler_func!(simd_floating_point_handler, "Simd Floating Point");
def_handler_func!(virtualization_handler, "Virtualization");

def_handler_func_with_errorcode!(double_fault_handler, "Double Fault");
def_handler_func_with_errorcode!(page_fault_handler, "Page Fault");
def_handler_func_with_errorcode!(invalid_tss_handler, "Invalid TSS");
def_handler_func_with_errorcode!(alignment_check_handler, "Alignment Check");
def_handler_func_with_errorcode!(segment_not_present_handler, "Segment Not Present");
def_handler_func_with_errorcode!(stack_segment_fault_handler, "Stack Segment Fault");
def_handler_func_with_errorcode!(security_exception_handler, "Security Exception");
def_handler_func_with_errorcode!(general_protection_fault_handler, "General Protection Fault");

// pub extern "x86-interrupt" fn double_fault_handler(
//     stack_frame: InterruptStackFrame,
//     _error_code: u64,
// ) -> ! {
//     panic!(
//         "\n[Interrupt] Exception: DOUBLE_FAULT\n{:#?}\n",
//         stack_frame
//     );
// }


// pub extern "x86-interrupt" fn page_fault_handler(
//     stack_frame: InterruptStackFrame,
//     error_code: PageFaultErrorCode,
// ) {
//     //The CR2 register is automatically set by the CPU on a page fault and contains the accessed virtual address that caused the page fault.
//     use x86_64::registers::control::Cr2;
//     println!("\n[Interrupt] Exception: PAGE FAULT");
//     println!("Accessed Address: {:?}", Cr2::read());
//     println!("Error Code: {:?}", error_code);
//     println!("{:#?}", stack_frame);

//     serial_println!("\n[Interrupt] Exception: PAGE FAULT");
//     serial_println!("Accessed Address: {:?}", Cr2::read());
//     serial_println!("Error Code: {:?}", error_code);
//     serial_println!("{:#?}", stack_frame);

//     hlt_loop()
// }

// pub extern "x86-interrupt" fn general_protection_fault_handler(
//     stack_frame: InterruptStackFrame,
//     error_code: u64,
// ) {
//     println!("\n[Interrupt] Exception: GENERAL PROTECTION FAULT");
//     println!("Error Code: {:?}", error_code);
//     println!("index: {}", (error_code >> 3) & ((1 << 14) - 1));
//     println!("tbl: {}", (error_code >> 1) & 0b11);
//     println!("e: {}", error_code & 1);
//     println!("{:#?}", stack_frame);
    
//     serial_println!("\n[Interrupt] Exception: GENERAL PROTECTION FAULT");
//     serial_println!("Error Code: {:?}", error_code);
//     serial_println!("index: {}", (error_code >> 3) & ((1 << 14) - 1));
//     serial_println!("tbl: {}", (error_code >> 1) & 0b11);
//     serial_println!("e: {}", error_code & 1);
//     serial_println!("{:#?}", stack_frame);
//     hlt_loop()
// }

// pub extern "x86-interrupt" fn stack_segment_fault_handler(
//     stack_frame: InterruptStackFrame,
//     error_code: u64,
// ) {
//     println!("\n[Interrupt] Exception: STACK SEGMENT FAULT");
//     println!("Error Code: {:?}", error_code);
//     println!("index: {}", (error_code >> 3) & ((1 << 14) - 1));
//     println!("tbl: {}", (error_code >> 1) & 0b11);
//     println!("e: {}", error_code & 1);
//     println!("{:#?}", stack_frame);
    
//     serial_println!("\n[Interrupt] Exception: STACK SEGMENT FAULT");
//     serial_println!("Error Code: {:?}", error_code);
//     serial_println!("index: {}", (error_code >> 3) & ((1 << 14) - 1));
//     serial_println!("tbl: {}", (error_code >> 1) & 0b11);
//     serial_println!("e: {}", error_code & 1);
//     serial_println!("{:#?}", stack_frame);
//     hlt_loop()
// }


pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("\n[Interrupt] Exception: BREAKPOINT\n{:#?}\n", stack_frame);
    serial_println!("\n[Interrupt] Exception: BREAKPOINT\n{:#?}\n", stack_frame);
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");
    // serial_print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
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
    let scancode: u8 = unsafe { port.read() };
    // println!("Scancode: {:?}", scancode);
    // pc_keyboard_interface::execute(scancode);
    task::keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

/// ref. https://github.com/xfoxfu/rust-xos/blob/main/kernel/src/interrupts/handlers.rs
/// rewarp calling convention with naked function

#[repr(align(8), C)]
#[derive(Debug)]
pub struct Registers {
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rdi: usize,
    pub rsi: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rbx: usize,
    pub rax: usize,
    pub rbp: usize,
}

#[naked]
pub extern "x86-interrupt" fn syscall_handler_naked_wrap(stack_frame: InterruptStackFrame) {
    unsafe {
        core::arch::asm!(
            "
                push rbp
                push rax
                push rbx
                push rcx
                push rdx
                push rsi
                push rdi
                push r8
                push r9
                push r10
                push r11
                push r12
                push r13
                push r14
                push r15
                mov rsi, rsp    // second arg: register list
                mov rdi, rsp
                add rdi, 15*8   // first arg: interrupt frame
                call {}
                pop r15
                pop r14
                pop r13
                pop r12 
                pop r11
                pop r10
                pop r9
                pop r8
                pop rdi
                pop rsi
                pop rdx
                pop rcx
                pop rbx
                pop rax
                pop rbp
                iretq
            ",
            sym syscall_handler,
            options(noreturn)
        );
    }
}

pub extern "C" fn syscall_handler(sf: &mut InterruptStackFrame, regs: &mut Registers) {
    // here can invoke syscall function
    // need to ensure enclosure with x86_64::instructions::interrupts::without_interrupts
    serial_println!(
        "
            rax: {:?}\n
            rdi: {:?}\n
            rsi: {:?}\n
            rdx: {:?}
        ",
        regs.rax,
        regs.rdi,
        regs.rsi,
        regs.rdx
    );
    serial_println!("syscall finished!");
}
