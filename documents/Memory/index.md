# Memory Design

1. Segments
 Used to seperate memory region. Only used segment to divide memory usage.

 Convert logic address to linear address (Physical address).

 - logic address: segment:offset
 - linear address = $(segment<<4) + offset$

2. Discription table
 Used to manager memory block entry. Use a table to descript each memory block usage.

 Convert logic address to linear address (Physical address).
 
 - GDTR/GDT, IDTR/IDT, LDTR/LDT [ref](https://xem.github.io/minix86/manual/intel-x86-and-64-manual-vol3/o_fe12b1e2a880e0ce-74.html)
 - logic address: segment:offset
 - linear address = $memory base + offset$
 
 Work flow:
 1. segment choose:
    - code: CS
    - data: DS
    and it will show which discriptor table (GDT/LDT) is used

 2. if `CS=[LDT,TI=3]`,
    - Use GDTR to find GDT
    - Use LDTR(index of GDT) to find the GDT entry index of LDT
    - Use `CS: TI=3` to figure out entry in LDT
    - Use LDT entry to figure out memory base

 3. if `CS=[GDT,TI=3]`
    - Use GDTR to find GDT
    - Use `CS: TI=3` to figure out entry in GDT
    - Use GDT entry to figure out memory base

3. Paging table
 Use to manager the virtual memory space.

 Convert virtual memory to physical memory.
 Virtual address --Paging--> Linear address --Segment(segment=0)--> Physical address
 
 [ref](https://wiki.osdev.org/Paging)

 - Due to the lagacy privileges check, we still need to set GDT and segment registor in correct. Now GDT contains:
    - NULL descriptor
    - Kernel code descriptor
    - Kernel data descriptor

 - Flat model design
 




---
## Ref:

https://blog.csdn.net/Rong_Toa/article/details/118163743

https://intermezzos.github.io/book/first-edition/setting-up-a-gdt.html