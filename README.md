# rustos
Experimental OS base on [blog_os](https://github.com/phil-opp/blog_os)
Waiting for new version of [osdev/bootloader](https://github.com/rust-osdev/bootloader)

Goal:
1. Without all assembly and link file, depends on bootloader and x86_64 crate.
2. Filesystem
3. Network interface

TODO LIST:
1. - [ ] ~~Convert TSS table into implementation with memory allocator~~
2. - [x] Fix render overflow
3. - [x] Make textrenderer support cursor
4. - [ ] Package TextWriter as editor and console
        - [ ] Document it!
        - [ ] Editor
        - [ ] Console
        - [ ] Support Del?
5. - [ ] Setup kernel process

> Note.
>
> 1. Leave the Qemu: ctrl+a x (-serial mon:stdio)
> 2. keyboard interrupt only work on graphic mode currently

---
# Future works
 - [x] TextWriter: Support console like input
 - [x] Make cargo ktest work
 - [ ] Implement APIC (current use 8259 PIC)
 - [ ] Learn E820 (memory controller?)


---
# ChangeLog
[2022-10-29]
1. Implement process and context switch
2. BUG: STDIN

[2022-10-27]
1. Error: syscall jmp to 0x0H in user mode

[2022-09-28]
1. enter user mode
2. syscall/sysretq

[2022-09-26]
1. Setup syscall software interrupt
2. Learn on ext2 filesystem

[2022-08-31]
1. Finished async implement

[2022-08-25]
1. Finished keyboard interrupt
2. Update TextEditor: Support cursor, Bugfix

[2022-08-24]
1. Solve ireqt double fault exception

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
