use spin::Lazy;
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// stack growth from top to down (large memory address to low memory address)
pub static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe {
            &STACK
        });
        let stack_end = stack_start + STACK_SIZE;
        stack_end
    };
    tss
});
