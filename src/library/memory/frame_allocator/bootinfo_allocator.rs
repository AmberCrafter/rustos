use bootloader::boot_info::MemoryRegions;
use bootloader::boot_info::MemoryRegionKind;
// use conquer_once::spin::OnceCell;
// use spin::Mutex;
use x86_64::PhysAddr;
use x86_64::structures::paging::FrameAllocator;
use x86_64::structures::paging::PhysFrame;
use x86_64::structures::paging::Size4KiB;

// pub static FRAMEALLOCATORL: OnceCell<Mutex<BootInfoFrameAllocator>> = OnceCell::uninit();

// pub unsafe fn init(memory_regions: &'static MemoryRegions) {
//     FRAMEALLOCATORL.get_or_init(move || {
//         Mutex::new(BootInfoFrameAllocator::init(memory_regions))
//     });            
// }


pub struct BootInfoFrameAllocator {
    memory_regions: &'static MemoryRegions,
    next: usize
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_regions: &'static MemoryRegions) -> Self {
        Self {
            memory_regions,
            next: 0  
        }
    }

    fn usable_frame(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_regions.iter();
        let usable_regions = regions.filter(|r| r.kind==MemoryRegionKind::Usable);
        let addr_ranges = usable_regions.map(|r| r.start..r.end);
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));  // 4KiB
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frame().nth(self.next);
        self.next += 1;
        frame
    }
}

unsafe impl Send for BootInfoFrameAllocator {}
unsafe impl Sync for BootInfoFrameAllocator {}