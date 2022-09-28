# Prepare

0. memory
 - physic and virtual memory mapping
    - Physical memory area [frame]: mmu (hardware) -> bootloader wrap (memory_regin) -> FrameAllocator<Sized> :: allocate_frame
    - Virtual memory area [page]: PageTable -> wrap OffsetPageTable<Mapper<Sized>> -> Mapper<Sized> :: map_to
    > mapper.map_to(Page, PhysFrame, flags, frame_allocator)

1. setup gdt of user space segments
```rust
pub static GDT: Lazy<(GlobalDescriptorTable, Selectors)> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let kernel_cs = gdt.add_entry(Descriptor::kernel_code_segment());
    let kernel_ds = gdt.add_entry(Descriptor::kernel_data_segment());
    let user_ds = gdt.add_entry(Descriptor::user_data_segment());
    let user_cs = gdt.add_entry(Descriptor::user_code_segment());

    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
    (gdt, Selectors {kernel_cs, kernel_ds, user_cs, user_ds, tss_selector})
});
```

2. setup user space
    - stack
        ```rust
        pub static mut USER_STACK: [u8; 4096*8] = [0; 4096*8];
        pub fn create_user_stack_map(
            page_range: PageRangeInclusive,
            frame_range: PhysFrameRangeInclusive,
            mapper: &mut OffsetPageTable,
            frame_allocator: &mut impl FrameAllocator<Size4KiB>
        ) -> Result<(), MapToError<Size4KiB>> {
            use x86_64::structures::paging::PageTableFlags;

            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
            for (page, frame) in page_range.zip(frame_range) {
                unsafe {
                    mapper.map_to(page, frame, flags, frame_allocator)?.flush();
                };
            }
            Ok(())
        }

        pub fn init_user_stack(mapper: &mut OffsetPageTable, frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> VirtAddr {
            let (start_frame, end_frame, offset) = {
                let start = VirtAddr::new(unsafe {
                    USER_STACK.as_ptr() as u64
                });
                let end = start + unsafe {
                    USER_STACK.len()
                };
                let start_phys = mapper.translate_addr(start).expect("User stack error");
                let end_phys = mapper.translate_addr(end).expect("User stack error");
                let start_frame = PhysFrame::containing_address(start_phys);
                let end_frame = PhysFrame::containing_address(end_phys);
                // serial_println!("stack: {:?} -> {:?}", start, start_phys);
                (
                    start_frame, end_frame, start_phys-start_frame.start_address()
                )
            };
            let (start_page, end_page) = {
                let start_virt = VirtAddr::new(0x3000_0000_0000);
                let end_virt = start_virt + unsafe {
                    USER_STACK.len()
                };
                (
                    Page::containing_address(start_virt),
                    Page::containing_address(end_virt),
                )
            };
            let page_range = Page::<Size4KiB>::range_inclusive(start_page, end_page);
            let frame_range = PhysFrame::<Size4KiB>::range_inclusive(start_frame, end_frame);
            create_user_stack_map(page_range, frame_range, mapper, frame_allocator);
            start_page.start_address() + offset + unsafe {
                USER_STACK.len()
            }
        }
        ```

    - entry_point
        mapping user code frame on user space memory page
        note. the user code only used one page, the best way is mapping an elf file
        ```rust
        pub fn user_entry_point(mapper: &mut OffsetPageTable, frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<VirtAddr, MapToError<Size4KiB>> {
            let virt = VirtAddr::new(user_space_func as u64);
            let phys = mapper.translate_addr(virt).expect("No user entry point.");
            // serial_println!("entry: {:?} -> {:?}", virt, phys);

            let page = Page::<Size4KiB>::containing_address(VirtAddr::new(0x3000_4000_0000));
            let frame = PhysFrame::<Size4KiB>::containing_address(phys);
            let offset = phys - frame.start_address();

            use x86_64::structures::paging::PageTableFlags;
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
            unsafe {
                mapper.map_to(page, frame, flags, frame_allocator)?.flush();
            }
            Ok(page.start_address() + offset)
        }

        pub fn user_space_func() {
            unsafe {
                core::arch::asm!("nop", "nop", "nop", "int 3");
                core::arch::asm!("mov rax, 0x01", "mov rbx, 0x01", "mov rcx, 0x01");
            }
        }
        ```

    - jump to user space
        here use iretq method which is more easy
        note. due to not support user space interrupt currently, we need to clear interrupt flag (IF, cli)
        ```rust
        pub fn jump_to_user_space(user_stack: VirtAddr, user_entry_point: VirtAddr) {
            use crate::library::gdt::GDT;
            use x86_64::instructions::segmentation::{DS, ES, Segment};

            let user_stack = user_stack.as_u64();
            let user_entry_point = user_entry_point.as_u64();

            let user_cs = GDT.1.user_cs;
            let user_ds = GDT.1.user_ds;

            unsafe {
                interrupts::disable();
                DS::set_reg(user_ds);
                ES::set_reg(user_ds);

                core::arch::asm!(
                    "
                        push (3 * 8) | 3
                        push rsi

                        ;pushf
                        ;// enable interrupt
                        ;pop rax
                        ;or rax, 0x200
                        ;push rax

                        push (4 * 8) | 3
                        push rdi
                        iretq
                    ",
                    in("rdi") user_entry_point,
                    in("rsi") user_stack,
                )
            }
        }
        ```

    - main scripts
        ```rust
        pub fn user_init(mapper: &mut OffsetPageTable, frame_allocator: &mut impl FrameAllocator<Size4KiB>, physical_memory_offset: VirtAddr) -> Result<(), MapToError<Size4KiB>> {
            let user_stack = init_user_stack(mapper, frame_allocator);
            let user_entry_point = user_entry_point(mapper, frame_allocator)?;
            let stack = mapper.translate_addr(user_stack);
            let entry = mapper.translate_addr(user_entry_point);
            jump_to_user_space(user_stack, user_entry_point);
            Ok(())
        }
        ```