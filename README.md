# rustos
Experimental OS base on blog_os

Goal:
1. Without all assembly and link file, depends on bootloader and x86_64 crate.
2. Filesystem
3. Network interface


TODO LIST:
1. Convert TSS table into implementation with memory allocator


> Note.
>
> 1. Leave the Qemu: ctrl+a x
---
# Future works
 - [ ] TextWriter: Support console like input
 - [x] Make cargo ktest work
 - [ ] Implement APIC (current use 8259 PIC)

---
# ChangeLog
[2022-08-23]
1. Setup interrupt handler: breakpoint, doubl_fault

[2022-08-22]
1. Modulize unittest, qemu, panic handler
2. Fix ktest error: https://github.com/rust-lang/cargo/issues/7359


[2022-08-21]
1. Change module name: render -> renderer
2. Make document for renderer
3. Append features: change fore/background color for TextWriter

[2022-08-20]
1. Due to make the screen output interface consistent on both bios and uefi, VGA_Buffer no longer to use.
2. Package the screen print interface from bootloader logger as library/render.

[2022-08-19] 
1. Initialization this project.
2. bootloader Config setting: https://docs.rs/bootloader/latest/bootloader/struct.Config.html#structfield.kernel_stack_size
