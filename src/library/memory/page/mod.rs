// Physical memory: frame
// Virtual memory:  page

use crate::println;
use x86_64::{registers::control::Cr3, VirtAddr, structures::paging::{PageTable, OffsetPageTable}};

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
            serial_println!("Entry [{:}] {:?}: {:?}", i, entry.addr().as_u64(),entry);
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