use x86_64::{VirtAddr, structures::paging::OffsetPageTable};

pub mod page;

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level4_table = page::get_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level4_table, physical_memory_offset)
}