use x86_64::instructions::interrupts;
use x86_64::structures::paging::frame::PhysFrameRangeInclusive;
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::page::PageRangeInclusive;
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PhysFrame, Size4KiB, Translate,
};
use x86_64::VirtAddr;

use crate::library::memory::frame_allocator::FRAME_ALLOCATORL;
use crate::library::memory::PAGEMAPPER;
#[allow(unused)]
use crate::{print, println};
#[allow(unused)]
use crate::{serial_print, serial_println};

pub static mut USER_STACK: [u8; 4096 * 8] = [0; 4096 * 8];
pub fn create_user_stack_map(
    page_range: PageRangeInclusive,
    frame_range: PhysFrameRangeInclusive,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    use x86_64::structures::paging::PageTableFlags;

    let flags =
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
    for (page, frame) in page_range.zip(frame_range) {
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        };
    }
    Ok(())
}

pub fn init_user_stack(
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> VirtAddr {
    let (start_frame, end_frame, offset) = {
        let start = VirtAddr::new(unsafe { USER_STACK.as_ptr() as u64 });
        let end = start + unsafe { USER_STACK.len() };
        let start_phys = mapper.translate_addr(start).expect("User stack error");
        let end_phys = mapper.translate_addr(end).expect("User stack error");
        let start_frame = PhysFrame::containing_address(start_phys);
        let end_frame = PhysFrame::containing_address(end_phys);
        // serial_println!("stack: {:?} -> {:?}", start, start_phys);
        (
            start_frame,
            end_frame,
            start_phys - start_frame.start_address(),
        )
    };
    let (start_page, end_page) = {
        let start_virt = VirtAddr::new(0x3000_0000_0000);
        let end_virt = start_virt + unsafe { USER_STACK.len() };
        (
            Page::containing_address(start_virt),
            Page::containing_address(end_virt),
        )
    };
    let page_range = Page::<Size4KiB>::range_inclusive(start_page, end_page);
    let frame_range = PhysFrame::<Size4KiB>::range_inclusive(start_frame, end_frame);
    create_user_stack_map(page_range, frame_range, mapper, frame_allocator);
    start_page.start_address() + offset + unsafe { USER_STACK.len() }
}

pub fn user_entry_point(
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<VirtAddr, MapToError<Size4KiB>> {
    let virt = VirtAddr::new(user_space_func as u64);
    let phys = mapper.translate_addr(virt).expect("No user entry point.");
    // serial_println!("entry: {:?} -> {:?}", virt, phys);

    let page = Page::<Size4KiB>::containing_address(VirtAddr::new(0x3000_4000_0000));
    let frame = PhysFrame::<Size4KiB>::containing_address(phys);
    let offset = phys - frame.start_address();

    use x86_64::structures::paging::PageTableFlags;
    let flags =
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)?.flush();
    }
    Ok(page.start_address() + offset)
}

pub fn user_init(// mapper: &mut OffsetPageTable,
    // frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    // physical_memory_offset: VirtAddr,
) -> Result<(), MapToError<Size4KiB>> {
    let mut frame_allocator_guard = FRAME_ALLOCATORL.lock();
    let mut mapper = PAGEMAPPER.lock();
    if let frame_allocator = frame_allocator_guard.get_mut() {
        let user_stack_addr = init_user_stack(&mut mapper, frame_allocator);
        let user_entry_point = user_entry_point(&mut mapper, frame_allocator)?;
        let stack = mapper.translate_addr(user_stack_addr);
        let entry = mapper.translate_addr(user_entry_point);

        // serial_println!("user stack: {:?}", user_stack_addr);
        // serial_println!("user entry: {:?}", user_entry_point);
        // serial_println!("user stack: {:?}", stack);
        // serial_println!("user entry: {:?}", entry);

        // use x86_64::structures::paging::PageTable;
        // use crate::PHYSICAL_MEMORY_OFFSET;
        // let physical_memory_offset = PHYSICAL_MEMORY_OFFSET.get().unwrap().clone();

        // let p4 = unsafe {crate::library::memory::page::get_level_4_table(physical_memory_offset)};
        // crate::library::memory::page::show_entries(p4);

        // let p3:&mut PageTable = unsafe {&mut *(physical_memory_offset + p4[96].addr().as_u64()).as_mut_ptr()};
        // crate::library::memory::page::show_entries(p3);

        // let p2:&mut PageTable = unsafe {&mut *(physical_memory_offset + p3[1].addr().as_u64()).as_mut_ptr()};
        // crate::library::memory::page::show_entries(p2);

        // let p1:&mut PageTable = unsafe {&mut *(physical_memory_offset + p2[0].addr().as_u64()).as_mut_ptr()};
        // crate::library::memory::page::show_entries(p1);

        // let vaddr = user_entry_point;
        // serial_println!("l4: {:?}", vaddr.p4_index());
        // serial_println!("l3: {:?}", vaddr.p3_index());
        // serial_println!("l2: {:?}", vaddr.p2_index());
        // serial_println!("l1: {:?}", vaddr.p1_index());

        // let p4 = unsafe {get_level_4_table(physical_memory_offset)};
        // serial_println!("p4[96]: {:?}", p4[96]);
        // let p3: &mut PageTable = unsafe { &mut *(physical_memory_offset + p4[0].addr().as_u64()).as_mut_ptr() };
        // serial_println!("p3[0]: {:?}", p3[0]);
        // let p2: &mut PageTable = unsafe { &mut *(physical_memory_offset + p3[0].addr().as_u64()).as_mut_ptr() };
        // serial_println!("p2[1]: {:?}", p2[1]);
        // let p1: &mut PageTable = unsafe { &mut *(physical_memory_offset + p2[1].addr().as_u64()).as_mut_ptr() };
        // serial_println!("p1[1]: {:?}", p1[1]);

        jump_to_user_space(user_stack_addr, user_entry_point);
    }
    Ok(())
}

pub fn jump_to_user_space(user_stack_addr: VirtAddr, user_entry_point: VirtAddr) {
    use crate::library::gdt::GDT;
    use x86_64::instructions::segmentation::{Segment, DS, ES};

    // serial_println!("user_stack_addr: {:?}", user_stack_addr);
    // serial_println!("user_entry_point: {:?}", user_entry_point);

    // let (user_stack_addr, user_entry_point) = (
    //     unsafe {
    //         USER_STACK.as_ptr() as u64 + USER_STACK.len() as u64 - 4
    //     },
    //     user_space_func as u64
    // );

    let user_stack_addr = user_stack_addr.as_u64();
    let user_entry_point = user_entry_point.as_u64();

    // serial_println!("user_stack_addr: {:?}", user_stack_addr);
    // serial_println!("user_entry_point: {:?}", user_entry_point);

    let user_cs = GDT.1.user_cs;
    let user_ds = GDT.1.user_ds;

    // serial_println!("user_ds: {:?}\t{:?}", user_ds, user_ds.0);
    // serial_println!("user_cs: {:?}\t{:?}", user_cs, user_cs.0);

    unsafe {
        interrupts::disable();
        DS::set_reg(user_ds);
        ES::set_reg(user_ds);

        core::arch::asm!(
            "
                push (3 * 8) | 3
                push rsi

                pushf
                // enable interrupt
                pop rax
                or rax, 0x200
                push rax

                push (4 * 8) | 3
                push rdi
                iretq
            ",
            in("rdi") user_entry_point,
            in("rsi") user_stack_addr,
        )
    }
}

pub fn user_space_func() {
    unsafe {
        core::arch::asm!("nop", "nop", "nop");
        core::arch::asm!("mov rax, 0x10", "mov rdi, 0x02", "syscall");
        core::arch::asm!("mov rax, 0x01", "mov rbx, 0x01", "mov rcx, 0x01");
    }
}
