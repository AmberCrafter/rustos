// Note.
// frame_allocator: Interface of mmap provided by bootloader (MMU, Hardware)
// allocator: Interface of solfware allocation provided by compiler (rustc, Software, Paging -> Frame)

use x86_64::{VirtAddr, structures::paging::OffsetPageTable};
use spin::{Lazy, Mutex};

pub mod page;
pub mod memory_set;

pub mod example_mapping;
pub mod frame_allocator;
pub mod allocator;

pub use x86_64::structures::paging::FrameAllocator;

pub use frame_allocator::FRAME_ALLOCATORL;
pub use frame_allocator::alloc_frame;
use crate::PHYSICAL_MEMORY_OFFSET;


const KERNEL_START_PHYS_ADDR: usize = 0; 
const USER_START_PHYS_ADDR: usize = 0x800_0000; 
const USER_STACK_SIZE: usize = 1024*1024; // 1MB


pub static PAGEMAPPER: Lazy<Mutex<OffsetPageTable<'static>>> = Lazy::new(|| {
    unsafe {
        let physical_memory_offset = PHYSICAL_MEMORY_OFFSET.get().unwrap().clone();
        let level4_table = page::get_level_4_table(physical_memory_offset);
        Mutex::new(OffsetPageTable::new(level4_table, physical_memory_offset))
    }
});

// pub unsafe fn init() -> OffsetPageTable<'static> {
//     let physical_memory_offset = PHYSICAL_MEMORY_OFFSET.get().unwrap().clone();
//     let level4_table = page::get_level_4_table(physical_memory_offset);
//     OffsetPageTable::new(level4_table, physical_memory_offset)
// }


