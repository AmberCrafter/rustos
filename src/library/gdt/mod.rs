pub mod tss;
use spin::Lazy;


// expose tss information
pub use tss::TSS;
pub use tss::DOUBLE_FAULT_IST_INDEX;


use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};

static GDT: Lazy<(GlobalDescriptorTable, Selectors)> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
    (gdt, Selectors {code_selector, tss_selector})
});

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector
}

pub fn init_gdt() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    GDT.0.load();

    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}