# rustos
Experimental OS base on blog_os

Target:
1. Without all assembly and link file, depends on bootloader and x86_64 crate.
2. Filesystem
3. Network interface

---
# ChangeLog
[2022-08-20]
1. Due to make the screen output interface consistent on both bios and uefi, VGA_Buffer no longer to use.
2. Package the screen print interface from bootloader logger as library/render.

[2022-08-19] 
1. Initialization this project.
2. bootloader Config setting: https://docs.rs/bootloader/latest/bootloader/struct.Config.html#structfield.kernel_stack_size
