use x86_64::PhysAddr;
use x86_64::structures::paging::{Page, OffsetPageTable, FrameAllocator, Size4KiB, PhysFrame, Mapper};
use x86_64::structures::paging::PageTableFlags as Flags;


pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>
) {
    // framebuffer start at PhysAddr: 0x20
    let frame = PhysFrame::containing_address(PhysAddr::new(0x20));
    let flags = Flags::WRITABLE | Flags::PRESENT;

    let map_to_result = unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("failed to map").flush();
}