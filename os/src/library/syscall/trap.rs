use core::arch::asm;
use super::syscall_handler;

use crate::library::processor::current_kernel_stack;

#[repr(align(8), C)]
#[derive(Debug,Clone)]
pub struct TrapFrame {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rsp: u64,
}

impl TrapFrame {
    pub fn new() -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rbp: 0,
            rsi: 0,
            rdi: 0,
            r8:  0,
            r9:  0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rsp: 0,
        }
    }
}

/// trap function
#[naked]
pub extern "C" fn trap_start() {
    unsafe {
        asm!("
            // save all registers
            push r15
            push r14
            push r13
            push r12
            push r11
            push r10
            push r9
            push r8
            push rdi
            push rsi
            push rbp
            push rdx
            push rcx
            push rbx
            push rax

            mov rbx, rsp            // save old stack pointer
            call {}                 // get current kernel stack: return `rax`
            mov rsp, rax            // set rsp to new stack address
                                    // setup `trap_frame` in new stack

            lea rbx, [rbx + 0x78]
            push rbx               // rsp
            
            sub rbx, 0x08
            push [rbx]             // r15
            sub rbx, 0x08
            push [rbx]             // r14
            sub rbx, 0x08
            push [rbx]             // r13
            sub rbx, 0x08
            push [rbx]             // r12
            sub rbx, 0x08
            push [rbx]             // r11
            sub rbx, 0x08
            push [rbx]             // r10
            sub rbx, 0x08
            push [rbx]             // r9
            sub rbx, 0x08
            push [rbx]             // r8
            sub rbx, 0x08
            push [rbx]             // rdi
            sub rbx, 0x08
            push [rbx]             // rsi
            sub rbx, 0x08
            push [rbx]             // rbp
            sub rbx, 0x08
            push [rbx]             // rdx
            sub rbx, 0x08
            push [rbx]             // rcx
            sub rbx, 0x08
            push [rbx]             // rdx
            sub rbx, 0x08
            push [rbx]             // rax

            mov rdi, rsp           // set 1st arg (TrapFrame)
            call {}                // call trap_handler
        ",
        // sym save_registers,
        sym current_kernel_stack,
        sym trap_handler,
        options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn trap_return() {
    unsafe {
        asm!("
            // restore all registers
            pop rax
            pop rbx
            pop rcx
            pop rdx
            pop rbp
            pop rsi
            pop rdi
            pop r8
            pop r9
            pop r10
            pop r11
            pop r12
            pop r13
            pop r14
            pop r15
            
            pop rsp             // restore stack pointer
            sysretq             // syscall return (32-bit: sysret; 64-bit: sysretq)
        ",
        // sym restore_registers,
        options(noreturn)
        )
    }
}

fn printer(reg: usize) {
    serial_println!("reg: {:?}", reg);
}

#[naked]
pub extern "C" fn save_registers() {
    unsafe {
        asm!("
            push r15
            push r14
            push r13
            push r12
            push r11
            push r10
            push r9
            push r8
            push rdi
            push rsi
            push rbp
            push rdx
            push rcx
            push rbx
            push rax
            ret
        ", options(noreturn));
    }
}

#[naked]
pub extern "C" fn restore_registers() {
    unsafe {
        asm!("
            pop rax
            pop rbx
            pop rcx
            pop rdx
            pop rbp
            pop rsi
            pop r8
            pop r9
            pop r10
            pop r11
            pop rdi
            pop r12
            pop r13
            pop r14
            pop r15
            ret
        ", options(noreturn));
    }
}

#[naked]
pub extern "C" fn trap_handler() {
    unsafe {
        asm!("
            call {}         // call syscall_hanlder; 
                            //          input: rdi 
                            //          output: rax
            add rsp, 0x10   // stack:        -->    // stack:          
                            //    ...               //  > ...          
                            //    [rax]             //    [rax]        
                            //  > [return addr]     //    [return addr]
            push rax
            jmp {}
        ",
        sym syscall_handler,
        sym trap_return,
        options(noreturn)
        )
    }
}