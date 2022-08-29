// Note.
// frame_allocator: Interface of mmap provided by bootloader (MMU, Hardware)
// allocator: Interface of solfware allocation provided by compiler (rustc, Software, Paging -> Frame)

use x86_64::{VirtAddr, structures::paging::OffsetPageTable};

pub mod page;

pub mod example_mapping;
pub mod frame_allocator;
pub mod allocator;

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level4_table = page::get_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level4_table, physical_memory_offset)
}