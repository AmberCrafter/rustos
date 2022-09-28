# Kernel space int 0x80
trap frame wrap syscall function
```rust
idt[0x80].set_handler_fn(syscall_handler_naked_wrap);

/// ref. https://github.com/xfoxfu/rust-xos/blob/main/kernel/src/interrupts/handlers.rs
/// rewarp calling convention with naked function

#[repr(align(8), C)]
#[derive(Debug)]
pub struct Registers {
    r15: usize,
    r14: usize,
    r13: usize,
    r12: usize,
    r11: usize,
    r10: usize,
    r9: usize,
    r8: usize,
    rdi: usize,
    rsi: usize,
    rdx: usize,
    rcx: usize,
    rbx: usize,
    rax: usize,
    rbp: usize,
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

```


# User space syscall
1. setup Star, LStar, SFMask, EFER
```rust
fn init_syscall_msr(selector: &Selectors) {
    // current not work
    use x86_64::registers::model_specific::{Star, LStar, SFMask,  Efer, EferFlags};
    use x86_64::registers::rflags::RFlags;

    unsafe {
        let eflag = Efer::read();
        Efer::write(eflag | EferFlags::SYSTEM_CALL_EXTENSIONS);
        
        Star::write(selector.user_cs, selector.user_ds, selector.kernel_cs, selector.kernel_ds).unwrap();
        
        let syscall_ptr = syscall as *const fn() as u64;
        let syscall_addr = x86_64::VirtAddr::new(syscall_ptr);
        LStar::write(syscall_addr);
        SFMask::write(
            RFlags::CARRY_FLAG | RFlags::PARITY_FLAG | RFlags::AUXILIARY_CARRY_FLAG |
            RFlags::ZERO_FLAG | RFlags::SIGN_FLAG | RFlags::TRAP_FLAG |
            RFlags::IOPL_LOW | RFlags::IOPL_HIGH |RFlags::NESTED_TASK | RFlags::RESUME_FLAG |
            RFlags::ALIGNMENT_CHECK | RFlags::ID
        )
    }
}

#[naked]
extern "C" fn syscall() {
    // serial_println!("syscall");
    unsafe {
        core::arch::asm!("sysretq", options(noreturn));
    }
}
```

2. use step
user space use syscall -> ring0
kernel space use sysretq -> ring3

Note. sysretq need naked funtion to ensure stack is correct
```rust
// user space
pub fn user_space_func() {
    unsafe {
        core::arch::asm!("nop", "nop", "nop");
        core::arch::asm!(
            "mov rax, 0x01", 
            "mov rdi, 0x02", 
            "syscall"
        );
        core::arch::asm!("mov rax, 0x01", "mov rbx, 0x01", "mov rcx, 0x01");
    }
}

// kernel space
#[naked]
extern "C" fn syscall() {
    // serial_println!("syscall");
    unsafe {
        core::arch::asm!("sysretq", options(noreturn));
    }
}
```