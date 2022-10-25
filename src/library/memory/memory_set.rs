// wrap frame-page allocation method

use alloc::vec::Vec;
use x86_64::{structures::paging::{page::PageRangeInclusive, PageTableFlags, Page, OffsetPageTable, Mapper, Translate}, VirtAddr};
use x86_64::structures::paging::mapper::TranslateError::PageNotMapped;

use crate::library::memory::page::PAGE_SIZE;

pub struct MapArea {
    page_range: PageRangeInclusive,
    start_virt_addr: VirtAddr,
    end_virt_addr: VirtAddr,
    flags: PageTableFlags
}

impl MapArea {
    pub fn map_one(&mut self, page: Page, page_table: &mut OffsetPageTable) {
        // map one page into page table
        use crate::library::memory::FRAME_ALLOCATORL;
        use crate::library::memory::alloc_frame;

        match page_table.translate_page(page) {
            Err(PageNotMapped) => {
                let frame = alloc_frame().expect("Alloc new memory frame failed");
                unsafe {
                    page_table.map_to(page, frame, self.flags, FRAME_ALLOCATORL.lock().get_mut()).expect("Map page to frame failed.").flush();
                }
            }

            Ok(frame) => {
            }

            _ => {}
        }

        // match page_table.translate_page(page) {
        //     Err(PageNotMapped) => {
        //         let frame = alloc_frame().unwrap();
        //     }
        // }
    }
}


impl MapArea {
    pub fn new(start_virt_addr: VirtAddr, end_virt_addr: VirtAddr, flags: PageTableFlags) -> Self {
        let start = Page::containing_address(start_virt_addr);
        let end = Page::containing_address(end_virt_addr);
        Self {
            page_range: Page::range_inclusive(start, end),
            start_virt_addr,
            end_virt_addr,
            flags
        }
    }

    pub fn from(other: &MapArea) -> Self {
        // likes copy
        Self {
            page_range: other.page_range,
            start_virt_addr: other.start_virt_addr,
            end_virt_addr: other.end_virt_addr,
            flags: other.flags
        }
    }

    pub fn map(&mut self, page_table: &mut OffsetPageTable) {
        for page in self.page_range {
            self.map_one(page, page_table);
        }
    }

    pub fn unmap(&mut self, page_table: &mut OffsetPageTable) {
        for page in self.page_range {
            page_table.unmap(page);
        }
    }

    pub fn copy_data(&mut self, page_table: &mut OffsetPageTable, data: &[u8]) {
        use crate::library::memory::PHYSICAL_MEMORY_OFFSET;

        let len = data.len();
        let pysical_memory_offset = PHYSICAL_MEMORY_OFFSET.get().unwrap().clone().as_u64();
        let mut start_virt = self.start_virt_addr;
        let end_virt = self.end_virt_addr;
        for page in self.page_range {
            let start = page.start_address().as_u64() as usize
                - self.page_range.start.start_address().as_u64() as usize;
            let src = &data[start..len.min(start+PAGE_SIZE)];
            let dest = unsafe {
                let mut dst = page_table.translate_addr(start_virt).unwrap().as_u64();
                dst += pysical_memory_offset;
                core::slice::from_raw_parts_mut(dst as usize as *mut u8, src.len())
            };
            dest.copy_from_slice(src);
            start_virt += PAGE_SIZE;

            if start_virt >= end_virt {break;}
        }
    }
}

pub struct MemorySet {
    pub page_table: OffsetPageTable<'static>,
    pub areas: Vec<MapArea>,
}

impl MemorySet {
    // used to generate a new memory map for each process
    pub fn new() -> Self {
        todo!()
    }
}



