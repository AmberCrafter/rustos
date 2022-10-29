// Physical memory: frame
// Virtual memory:  page

use crate::{println, library::memory::frame_allocator::FRAME_ALLOCATORL};
use alloc::boxed::Box;
use x86_64::{registers::control::Cr3, VirtAddr, structures::paging::{PageTable, OffsetPageTable, Page, Size4KiB, Translate, mapper::{TranslateResult, MappedFrame}, Mapper, PageTableFlags, PhysFrame, FrameAllocator, page}, PhysAddr};

use super::PAGEMAPPER;

pub const PAGE_SIZE: usize = 4096;


// pub(super) unsafe fn get_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
pub unsafe fn get_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (level4_table_frame, _) = Cr3::read();
    
    let phys_addr = level4_table_frame.start_address();
    let virt_addr = physical_memory_offset + phys_addr.as_u64();
    let level4_table_ptr:*mut PageTable = virt_addr.as_mut_ptr();

    &mut *level4_table_ptr
}

pub fn show_entries(pagetable: &PageTable) {
    for (i, entry) in pagetable.iter().enumerate() {
        if !entry.is_unused() {
            println!("Entry [{:}]: {:?}", i, entry);
            serial_println!("Entry [{:}] {:x?}: {:?}", i, entry.addr().as_u64(),entry);
        }
    }
}

// pub unsafe fn addr_virt2phys(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     // wrap safe operation
//     translate_map_physical_memory(addr, physical_memory_offset)
// }

// fn translate_map_physical_memory(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     let (level4_table_frame, _) = Cr3::read();
//     let table_indexes = [addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()];
//     let mut frame = level4_table_frame;

//     for index in table_indexes {
//         // covert physical to virtural
//         let virt = physical_memory_offset + frame.start_address().as_u64();
//         let table_ptr: *const PageTable =virt.as_ptr();
//         let table = unsafe {
//             &*table_ptr
//         }; //type casting

//         let entry = &table[index];
//         frame = match entry.frame() {
//             Ok(frame) => frame,
//             Err(FrameError::FrameNotPresent) => return None,
//             Err(FrameError::HugeFrame) => panic!("Huge page not support!")
//         };
//     }
//     Some(frame.start_address() + u64::from(addr.page_offset()))
// }

// Must call after initializing heap
/* 
Note:
Box::leak(A) will unpack the Box(A) wrapper and return a mutable reference (&'a mut A).
However, we need to wrap it into Box by Box::from_raw() (&mut A -> Box(A)) before we free it.
*/ 

pub fn empty_page_table() -> &'static mut PageTable {
    Box::leak(Box::new(PageTable::new()))
}

// generate a new pagetable used for different process
// Assume we use the offset table
// Page table description:
// active_page_table (active_frame): The first and only physical memory owner
//  -> process_page_table (active_frame): Process space memory, which 

pub fn kernel_mapped_new_page_table() -> OffsetPageTable<'static> {
    let phys_offset = physical_memory_offset();
    let new_page_table = empty_page_table();
    let mut new_offset_page_table = unsafe {
        OffsetPageTable::new(new_page_table, phys_offset)
    };
    // use translator mapping process's page area into actual memory frame
    let translator = unsafe {
        current_offset_page_table()
    };
    
    // bootloader page table & process kernel stack
    // page: 0x00 ~ 0x60_0000
    // kernel stack: 0x70_0000 ~ 0x7a_0000
    let page_range = {
        let start_addr = VirtAddr::new(0x0);
        let end_addr = VirtAddr::new(0x100_0000);
        let start_page = Page::containing_address(start_addr);
        let end_page = Page::containing_address(end_addr);
        Page::<Size4KiB>::range(start_page, end_page)
    };

    // bootloader kernel stack
    let kernel_stack_range = {
        let start_addr = VirtAddr::new(0x80_0000_0000);
        let end_addr = VirtAddr::new(0x80_0000_0000 + 0x000a_0000);
        let start_page = Page::containing_address(start_addr);
        let end_page = Page::containing_address(end_addr);
        Page::<Size4KiB>::range(start_page, end_page)
    };

    // bootloader memory_regions
    let bootloader_memory_regions = {
        let start_addr = VirtAddr::new(0x180_0000_0000);
        let end_addr = VirtAddr::new(0x180_0000_0000 + 0x000a_0000);
        let start_page = Page::containing_address(start_addr);
        let end_page = Page::containing_address(end_addr);
        Page::<Size4KiB>::range(start_page, end_page)
    };

    // kernel heap
    use crate::library::memory::allocator::{HEAP_START, HEAP_SIZE};
    let heap_range = {
        let start_addr = VirtAddr::new(HEAP_START as u64);
        let end_addr = VirtAddr::new((HEAP_START + HEAP_SIZE) as u64);
        let start_page = Page::containing_address(start_addr);
        let end_page = Page::containing_address(end_addr);
        Page::<Size4KiB>::range(start_page, end_page)
    };

    // kernel space of physical memory (OffsetTable)
    let offset_page_range = {
        const MEMORY_SIZE: u64 = 0x800_0000; // 128 MiB
        let start_addr = phys_offset.clone();
        let end_addr = phys_offset + MEMORY_SIZE;
        let start_page = Page::containing_address(start_addr);
        let end_page = Page::containing_address(end_addr);
        Page::<Size4KiB>::range(start_page, end_page)
    };

    let mut frame_allocator_lock = FRAME_ALLOCATORL.lock();
    let frame_allocator = frame_allocator_lock.get_mut();

    // Map kernel page
    for page in page_range {
        if let TranslateResult::Mapped { frame, offset, flags } = translator.translate(page.start_address()) {
            if let MappedFrame::Size4KiB(frame) = frame {
                unsafe {new_offset_page_table.map_to(page, frame, flags, frame_allocator)}
                    .expect("map kernel page failed")
                    .flush()
            }
        }
    }

    // Map kernel stack
    for page in kernel_stack_range {
        if let TranslateResult::Mapped { frame, offset, flags } = translator.translate(page.start_address()) {
            if let MappedFrame::Size4KiB(frame) = frame {
                unsafe {new_offset_page_table.map_to(page, frame, flags, frame_allocator)}
                    .expect("map kernel page failed")
                    .flush()
            }
        }
    }

    // Map bootloader memory regions
    for page in bootloader_memory_regions {
        if let TranslateResult::Mapped { frame, offset, flags } = translator.translate(page.start_address()) {
            if let MappedFrame::Size4KiB(frame) = frame {
                unsafe {new_offset_page_table.map_to(page, frame, flags, frame_allocator)}
                    .expect("map kernel page failed")
                    .flush()
            }
        }
    }
    
    // Map kernel heap
    for page in heap_range {
        if let TranslateResult::Mapped { frame, offset, flags } = translator.translate(page.start_address()) {
            if let MappedFrame::Size4KiB(frame) = frame {
                unsafe {new_offset_page_table.map_to(page, frame, flags, frame_allocator)}
                    .expect("map kernel heap failed")
                    .flush()
            }
        }
    }

    // Map offset memory
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    for page in offset_page_range {
        let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(
            page.start_address().as_u64() - phys_offset.as_u64()
        ));
        unsafe {new_offset_page_table.map_to(page, frame, flags, frame_allocator)}
            .expect("map offset memory failed")
            .flush()
    }
    new_offset_page_table
}

pub fn physical_memory_offset() -> VirtAddr {
    use crate::PHYSICAL_MEMORY_OFFSET;
    PHYSICAL_MEMORY_OFFSET.get().unwrap().clone()
}

pub unsafe fn current_offset_page_table() -> OffsetPageTable<'static> {
    let physical_memory_offset = physical_memory_offset();
    let level4_table = get_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level4_table, physical_memory_offset)
}

pub unsafe fn current_page_table_address() -> usize {
    let (frame, flags) = Cr3::read();
    frame.start_address().as_u64() as usize + physical_memory_offset().as_u64() as usize
}

pub const PROCESS_KERNEL_STACK_START: u64 = 0x80_0000;
pub const PROCESS_KERNEL_STACK_END:   u64 = 0x90_0000;
pub const PROCESS_KERNEL_STACK_SIZE:  u64 = PAGE_SIZE as u64 * 7; // 4096 * 7 Bytes => 28 KiB
pub const GUARD_SIZE: u64 = PAGE_SIZE as u64;

pub fn init_process_kernel_stack() {
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    let mut allocator_lock = FRAME_ALLOCATORL.lock();
    let mut frame_allocator = allocator_lock.get_mut();

    for start in (PROCESS_KERNEL_STACK_START..PROCESS_KERNEL_STACK_END).step_by((PROCESS_KERNEL_STACK_SIZE + GUARD_SIZE) as usize) {
        let page_range = Page::<Size4KiB>::range(
            Page::containing_address(VirtAddr::new(start + GUARD_SIZE)),
            Page::containing_address(VirtAddr::new(start + GUARD_SIZE + PROCESS_KERNEL_STACK_SIZE))
        );
        for page in page_range {
            let frame = frame_allocator.allocate_frame().expect("alloc frame failed");
            unsafe {PAGEMAPPER.lock().map_to(page, frame, flags, frame_allocator)}
                .expect("init process kernel failed")
                .flush();
        }
    }
}
