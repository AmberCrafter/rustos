mod tss;
use spin::Lazy;


// expose tss information
use tss::TSS;
pub use tss::DOUBLE_FAULT_IST_INDEX;


use x86_64::{structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}, registers::segmentation::DS};

use crate::println;

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
    tss_selector: SegmentSelector
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
    
    // init_syscall_msr(&GDT.1);
}

fn init_syscall_msr(selector: &Selectors) {
    // current not work
    use x86_64::registers::model_specific::{Star, LStar,SFMask, Efer};

    serial_println!("kcs {:?}", selector.kernel_cs.0);
    serial_println!("kds {:?}", selector.kernel_ds.0);
    serial_println!("ucs {:?}", selector.user_cs.0);
    serial_println!("uds {:?}", selector.user_ds.0);

    // let ss = selector.kernel_ds;
    unsafe {
        Star::write(selector.user_cs, selector.user_ds, selector.kernel_cs, selector.kernel_ds).unwrap();
        
        // let syscall_ptr = syscall as *const u64 as *mut u64;
        let syscall_ptr = syscall as *const fn() as u64;
        let syscall_addr = x86_64::VirtAddr::new(syscall_ptr);
        LStar::write(syscall_addr);
    }
}

fn syscall() {
    serial_println!("syscall");
}