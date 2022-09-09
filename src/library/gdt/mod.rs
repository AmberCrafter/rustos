mod tss;
use spin::Lazy;


// expose tss information
use tss::TSS;
pub use tss::DOUBLE_FAULT_IST_INDEX;


use x86_64::{structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}, registers::segmentation::DS};

static GDT: Lazy<(GlobalDescriptorTable, Selectors)> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
    (gdt, Selectors {code_selector, data_selector, tss_selector})
});

struct Selectors {
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    tss_selector: SegmentSelector
}

pub fn init_gdt() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{SS, CS, Segment};

    GDT.0.load();
    unsafe {
        // SS::set_reg(SegmentSelector::new(0, GDT.1.code_selector.rpl()));
        SS::set_reg(SegmentSelector::NULL);
        CS::set_reg(GDT.1.code_selector);
        DS::set_reg(GDT.1.data_selector);
        load_tss(GDT.1.tss_selector);
    }
}