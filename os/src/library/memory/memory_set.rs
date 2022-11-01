// wrap frame-page allocation method

use alloc::vec::Vec;
use spin::{Lazy, Mutex};
use x86_64::structures::paging::mapper::TranslateError::PageNotMapped;
use x86_64::{
    structures::paging::{
        page::PageRangeInclusive, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags,
        Translate,
    },
    PhysAddr, VirtAddr,
};

use crate::{
    library::memory::{page::PAGE_SIZE, USER_STACK_SIZE},
    PHYSICAL_MEMORY_OFFSET,
};

pub static KERNEL_SPACE: Lazy<Mutex<MemorySet>> = Lazy::new(|| Mutex::new(MemorySet::new()));

pub struct MapArea {
    page_range: PageRangeInclusive,
    start_virt_addr: VirtAddr,
    end_virt_addr: VirtAddr,
    flags: PageTableFlags,
}

impl MapArea {
    pub fn map_one(&mut self, page: Page, page_table: &mut OffsetPageTable) {
        // map one page into page table
        use crate::library::memory::alloc_frame;
        use crate::library::memory::FRAME_ALLOCATORL;

        match page_table.translate_page(page) {
            Err(PageNotMapped) => {
                let frame = alloc_frame().expect("Alloc new memory frame failed");
                unsafe {
                    page_table
                        .map_to(page, frame, self.flags, FRAME_ALLOCATORL.lock().get_mut())
                        .expect("Map page to frame failed.")
                        .flush();
                }
            }

            Ok(frame) => {}

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
            flags,
        }
    }

    pub fn from(other: &MapArea) -> Self {
        // likes copy
        Self {
            page_range: other.page_range,
            start_virt_addr: other.start_virt_addr,
            end_virt_addr: other.end_virt_addr,
            flags: other.flags,
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
        let len = data.len();
        let pysical_memory_offset = PHYSICAL_MEMORY_OFFSET.get().unwrap().clone().as_u64();
        let mut start_virt = self.start_virt_addr;
        let end_virt = self.end_virt_addr;
        for page in self.page_range {
            let start = page.start_address().as_u64() as usize
                - self.page_range.start.start_address().as_u64() as usize;
            let src = &data[start..len.min(start + PAGE_SIZE)];
            let dest = unsafe {
                let mut dst = page_table.translate_addr(start_virt).unwrap().as_u64();
                dst += pysical_memory_offset;
                core::slice::from_raw_parts_mut(dst as usize as *mut u8, src.len())
            };
            dest.copy_from_slice(src);
            start_virt += PAGE_SIZE;

            if start_virt >= end_virt {
                break;
            }
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
        use super::page::kernel_mapped_new_page_table;
        Self {
            page_table: kernel_mapped_new_page_table(),
            areas: Vec::new(),
        }
    }
    fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&mut self.page_table, data)
        }
        self.areas.push(map_area);
    }
    pub fn insert(
        &mut self,
        start_virt_addr: VirtAddr,
        end_virt_addr: VirtAddr,
        flags: PageTableFlags,
        data: Option<&[u8]>,
    ) {
        self.push(MapArea::new(start_virt_addr, end_virt_addr, flags), data)
    }
    pub fn page_table_address(&mut self, translator: &OffsetPageTable) -> PhysAddr {
        let table_addr: *const PageTable = self.page_table.level_4_table();
        translator
            .translate_addr(VirtAddr::new(table_addr as u64))
            .unwrap()
    }
    pub fn remove_all_areas(&mut self) {
        let page_table = &mut self.page_table;
        self.areas
            .iter_mut()
            .rev()
            .for_each(|area| area.unmap(page_table));
        self.areas.clear();
    }
    pub fn remove_area_with_start_addr(&mut self, start_addr: VirtAddr) {
        if let Some((i, area)) = self
            .areas
            .iter_mut()
            .enumerate()
            .find(|(i, area)| area.page_range.start.start_address() == start_addr)
        {
            area.unmap(&mut self.page_table);
            self.areas.remove(i);
        }
    }
}

impl Drop for MemorySet {
    // release allocated page table
    fn drop(&mut self) {
        use alloc::boxed::Box;
        unsafe {
            Box::from_raw(self.page_table.level_4_table() as *mut PageTable);
        }
        // `self.page_table` deallocate after here
    }
}

// User space & elf file memory set
impl MemorySet {
    pub fn from(user_space: &MemorySet) -> Self {
        // create a new memory_set defined by input

        // create a new memory_set with new page table
        let mut memory_set = Self::new();
        let physical_memory_offset = PHYSICAL_MEMORY_OFFSET.get().unwrap().clone().as_u64();

        // copy areas from input memory_set
        for area in user_space.areas.iter() {
            // create a new map_area, just like clone
            let mut new_area = MapArea::from(area);
            memory_set.push(new_area, None);

            // copy memory data
            for page in area.page_range {
                // data src
                let src = {
                    let mut src = user_space
                        .page_table
                        .translate_page(page)
                        .unwrap()
                        .start_address()
                        .as_u64();
                    src += physical_memory_offset;
                    let len = 4096.min(area.end_virt_addr - page.start_address()) as usize;
                    // generate src data slince
                    // <virt_addr u64> -> <virt_addr usize> -> <pointer *u8> -> <[u8]>
                    unsafe { core::slice::from_raw_parts(src as usize as *const u8, len) }
                };
                // copy dest
                let dest = {
                    let mut dest = memory_set
                        .page_table
                        .translate_page(page)
                        .unwrap()
                        .start_address()
                        .as_u64();
                    dest += physical_memory_offset;
                    unsafe { core::slice::from_raw_parts_mut(dest as usize as *mut u8, src.len()) }
                };
                dest.copy_from_slice(src);
            }
        }
        memory_set
    }

    pub fn read_elf(&mut self, elf_data: &[u8]) -> (usize, usize) {
        // return
        //  - stack top
        //  - enrty point
        let elf = xmas_elf::ElfFile::new(elf_data).expect("Invalid elf data");
        let elf_header = elf.header;
        // check file type
        assert_eq!(
            elf.header.pt1.magic,
            [0x7f, 0x45, 0x4c, 0x46],
            "Invalid elf"
        );
        let ph_count = elf.header.pt2.ph_count();
        let mut max_page = Page::containing_address(VirtAddr::new(0));
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_virt_addr = VirtAddr::new(ph.virtual_addr());
                let end_virt_addr = VirtAddr::new(ph.virtual_addr() + ph.mem_size());
                // setup memory privileges
                let mut flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;
                let ph_flag = ph.flags();
                if !ph_flag.is_execute() {
                    flags |= PageTableFlags::NO_EXECUTE;
                }
                if ph_flag.is_write() {
                    flags |= PageTableFlags::WRITABLE;
                }
                let map_area = MapArea::new(start_virt_addr, end_virt_addr, flags);
                max_page = map_area.page_range.end;
                // serial_println!("Elf range: {:?} ~ {:?}", start_virt_addr, end_virt_addr);
                self.push(
                    map_area,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                );
            }
        }

        // memory layout
        // ------------------- lower  0x00
        // ...
        // [elf data]
        // [elf data]
        // [elf data]
        // [elf data end page] [guard page]
        // [user_stack_bottom]
        //   USER_STACK_SIZE
        // [user_stack_top]
        // ...
        // ------------------- upper

        let mut user_stack_bottom = max_page.start_address();
        user_stack_bottom += PAGE_SIZE as u64;
        let user_stack_top = user_stack_bottom + USER_STACK_SIZE;
        self.push(
            MapArea::new(
                user_stack_bottom,
                user_stack_top,
                PageTableFlags::PRESENT
                    | PageTableFlags::WRITABLE
                    | PageTableFlags::USER_ACCESSIBLE,
            ),
            None,
        );
        (
            user_stack_top.as_u64() as usize,
            elf.header.pt2.entry_point() as usize,
        )
    }
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        let mut memory_set = Self::new();
        let (user_stack_top, entry_point) = memory_set.read_elf(elf_data);
        (memory_set, user_stack_top, entry_point)
    }
}
