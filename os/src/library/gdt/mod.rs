mod tss;
use spin::Lazy;


// expose tss information
use tss::TSS;
pub use tss::{DOUBLE_FAULT_IST_INDEX, DEBUG_IST_INDEX, NON_MASKABLE_INTERRUPT_IST_INDEX};


use x86_64::{structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}, registers::segmentation::DS};

use crate::println;

use super::interrupt::handler_interrupt::Registers;

pub static GDT: Lazy<(GlobalDescriptorTable, Selectors)> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let kernel_cs = gdt.add_entry(Descriptor::kernel_code_segment());
    let kernel_ds = gdt.add_entry(Descriptor::kernel_data_segment());
    let user_ds = gdt.add_entry(Descriptor::user_data_segment());
    let user_cs = gdt.add_entry(Descriptor::user_code_segment());

    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
    (gdt, Selectors {kernel_cs, kernel_ds, user_cs, user_ds, tss_selector})
});

pub struct Selectors {
    pub kernel_cs: SegmentSelector,
    pub kernel_ds: SegmentSelector,
    pub user_cs: SegmentSelector,
    pub user_ds: SegmentSelector,
    pub tss_selector: SegmentSelector
}

pub fn init_gdt() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{SS, CS, Segment};

    GDT.0.load();
    unsafe {
        // SS::set_reg(SegmentSelector::new(0, GDT.1.code_selector.rpl()));
        // SS::set_reg(SegmentSelector::NULL);
        SS::set_reg(GDT.1.kernel_ds);
        CS::set_reg(GDT.1.kernel_cs);
        DS::set_reg(GDT.1.kernel_ds);
        load_tss(GDT.1.tss_selector);
    }
}

pub fn init_trap() {
    init_syscall_msr(&GDT.1);
}

fn init_syscall_msr(selector: &Selectors) {
    // current not work
    use x86_64::registers::model_specific::{Star, LStar, SFMask,  Efer, EferFlags};
    use x86_64::registers::rflags::RFlags;

    use crate::library::syscall::trap::trap_start;

    // serial_println!("kcs {:?}", selector.kernel_cs.0);
    // serial_println!("kds {:?}", selector.kernel_ds.0);
    // serial_println!("ucs {:?}", selector.user_cs.0);
    // serial_println!("uds {:?}", selector.user_ds.0);

    // let ss = selector.kernel_ds;
    unsafe {
        let eflag = Efer::read();
        Efer::write(eflag | EferFlags::SYSTEM_CALL_EXTENSIONS);
        
        Star::write(selector.user_cs, selector.user_ds, selector.kernel_cs, selector.kernel_ds).unwrap();
        
        // let syscall_ptr = syscall as *const u64 as *mut u64;
        
        // let syscall_ptr = syscall as *const fn() as u64;
        let syscall_ptr = trap_start as *const fn() as u64;
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

// #[naked]
// extern "C" fn syscall() {
//     // serial_println!("syscall");
//     unsafe {
//         core::arch::asm!(
//             "
//                 push rbp
//                 push rax
//                 push rbx
//                 push rcx
//                 push rdx
//                 push rsi
//                 push rdi
//                 push r8
//                 push r9
//                 push r10
//                 push r11
//                 push r12
//                 push r13
//                 push r14
//                 push r15
//                 mov rdi, rsp    // first arg: register list
//                 call {}
//                 pop r15
//                 pop r14
//                 pop r13
//                 pop r12 
//                 pop r11
//                 pop r10
//                 pop r9
//                 pop r8
//                 pop rdi
//                 pop rsi
//                 pop rdx
//                 pop rcx
//                 pop rbx
//                 pop rax
//                 pop rbp
//                 sysretq
//             ", 
//             sym syscall_handler,
//             options(noreturn));
//     }
// }

// extern "C" fn syscall_handler(regs: Registers) {
//     serial_println!("{:?}", regs);
//     if regs.rax == 0x10 {
//         super::qemu::exit_qemu(super::qemu::QemuExitCode::Success);
//     }
// }