use x86_64::{structures::paging::{OffsetPageTable, Translate, PageTable}, VirtAddr};

use crate::PHYSICAL_MEMORY_OFFSET;

use super::pcb::ProcessControlBlock;

#[repr(C)]
pub struct  ProcessContext {
    r15: usize,
    r14: usize,
    r13: usize,
    r12: usize,
    r11: usize,
    rbx: usize,
    rbp: usize,
    rip: usize,
}

impl ProcessContext {
    pub fn return_from_trap() -> Self {
        use crate::library::syscall::trap::trap_return;
        Self {
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            r11: 0,
            rbx: 0,
            rbp: 0,
            rip: trap_return as usize,
        }
    }
}

#[naked]
pub extern "C" fn switch_to(current: *const usize, target: usize) {
    // rdi: current: **mut ProcessContext
    // rsi: target: *mut ProcessContext
    unsafe {
        core::arch::asm!("
            push rbp
            push rbx
            push r11
            push r12
            push r13
            push r14
            push r15
        
            mov [rdi], rsp
            mov rsp, rsi

            pop r15
            pop r14
            pop r13
            pop r12
            pop r11
            pop rbx
            pop rbp
            ret
        ", options(noreturn));
    }
}

#[naked]
pub extern "C" fn switch_mm(target: usize) {
    // rdi: *mut OffsetPageTable
    unsafe {
        core::arch::asm!("
            mov cr3, rdi
        ", options(noreturn));
    }
}

pub fn switch_page_table_ptr(current: &ProcessControlBlock, target: &ProcessControlBlock) -> usize {
    let physical_memory_offset = PHYSICAL_MEMORY_OFFSET.get().unwrap().clone();
    let translator = &mut current.inner_lock().memory_set.page_table;
    let target_page_table_ptr: *const PageTable = target.inner_lock().memory_set.page_table.level_4_table();
    let phys_addr = translator.translate_addr(VirtAddr::new(target_page_table_ptr as u64)).unwrap();
    (phys_addr.as_u64() + physical_memory_offset.as_u64()) as usize
}