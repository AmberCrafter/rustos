// wrap frame-page allocation method

use x86_64::{structures::paging::{page::PageRangeInclusive, PageTableFlags, Page, OffsetPageTable, Mapper}, VirtAddr};
use x86_64::structures::paging::mapper::TranslateError::PageNotMapped;

pub struct MapArea {
    page_range: PageRangeInclusive,
    start_virt_addr: VirtAddr,
    end_virt_addr: VirtAddr,
    flags: PageTableFlags
}

impl MapArea {
    pub fn map_one(&mut self, page: Page, page_table: &mut OffsetPageTable) {
        // map one page into page table


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
}