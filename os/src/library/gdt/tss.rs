use spin::Lazy;
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const DEBUG_IST_INDEX: u16 = 1;
pub const NON_MASKABLE_INTERRUPT_IST_INDEX: u16 = 2;

// stack growth from top to down (large memory address to low memory address)
pub static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    // guard stack frame
    tss.privilege_stack_table[0] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
        let stack_end = stack_start + STACK_SIZE;
        stack_end
    };
    
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe {
            &STACK
        });
        let stack_end = stack_start + STACK_SIZE;
        stack_end
    };
    tss.interrupt_stack_table[DEBUG_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe {
            &STACK
        });
        let stack_end = stack_start + STACK_SIZE;
        stack_end
    };
    tss.interrupt_stack_table[NON_MASKABLE_INTERRUPT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe {
            &STACK
        });
        let stack_end = stack_start + STACK_SIZE;
        stack_end
    };

    // tss.privilege_stack_table[0] = {
    //     const STACK_SIZE: usize = 4096 * 5;
    //     static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
    //     let stack_start = VirtAddr::from_ptr( unsafe {
    //         &STACK
    //     });
    //     let stack_end = stack_start + STACK_SIZE;
    //     stack_end
    // };

    // tss.privilege_stack_table[1] = {
    //     const STACK_SIZE: usize = 4096 * 5;
    //     static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
    //     let stack_start = VirtAddr::from_ptr( unsafe {
    //         &STACK
    //     });
    //     let stack_end = stack_start + STACK_SIZE;
    //     stack_end
    // };

    // tss.privilege_stack_table[2] = {
    //     const STACK_SIZE: usize = 4096 * 5;
    //     static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
    //     let stack_start = VirtAddr::from_ptr( unsafe {
    //         &STACK
    //     });
    //     let stack_end = stack_start + STACK_SIZE;
    //     stack_end
    // };
    
    tss
});
