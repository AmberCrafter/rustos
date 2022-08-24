## Origin

Double fault occur when gdt be setted and iterrupt trigger.

TL;DR
[https://github.com/rust-osdev/bootloader/issues/190#issuecomment-900299505](https://github.com/rust-osdev/bootloader/issues/190#issuecomment-900299505)


```
check_exception old: 0xffffffff new 0xd
1: v=0d e=0010 i=0 cpl=0 IP=0008:000000000020cf2e pc=000000000020cf2e SP=0010:000000800009ed88 env->regs[R_EAX]=000000800009ee40
RAX=000000800009ee40 RBX=0000020000000000 RCX=0000000000204c00 RDX=0000000000000001
RSI=0000000000000000 RDI=0000000000200270 RBP=000000000005a558 RSP=000000800009ed88
R8 =0000000000000000 R9 =0000000000000001 R10=00000000000003f8 R11=0000000000032e20
R12=0000000000007272 R13=0000000000000004 R14=000000000000766a R15=00000000000072ba
RIP=000000000020cf2e RFL=00000002 [-------] CPL=0 II=0 A20=1 SMM=0 HLT=0
ES =0010 0000000000000000 ffffffff 00cf9300 DPL=0 DS   [-WA]
CS =0008 0000000000000000 ffffffff 00af9b00 DPL=0 CS64 [-RA]
SS =0010 0000000000000000 ffffffff 00cf9300 DPL=0 DS   [-WA]
DS =0010 0000000000000000 ffffffff 00cf9300 DPL=0 DS   [-WA]
FS =0000 0000000000000000 0000ffff 00009300 DPL=0 DS   [-WA]
GS =0000 0000000000000000 0000ffff 00009300 DPL=0 DS   [-WA]
LDT=0000 0000000000000000 0000ffff 00008200 DPL=0 LDT
TR =0010 00000000002428a0 00000067 00008900 DPL=0 TSS64-avl
GDT=     0000000000242910 0000001f
IDT=     0000000000242990 00000fff
CR0=80010011 CR2=0000000000000000 CR3=000000000045e000 CR4=00000020
DR0=0000000000000000 DR1=0000000000000000 DR2=0000000000000000 DR3=0000000000000000 
DR6=00000000ffff0ff0 DR7=0000000000000400
CCS=00000000000000b0 CCD=000000800009ed40 CCO=ADDQ    
EFER=0000000000000d00

```
[Exception Table](https://wiki.osdev.org/Exceptions#Selector_Error_Code)
 - Exception vector number: 
    `v = 0d` => General Protection Fault
 - Exception code (Selector Error Code): 
    `e=0010` => 0b0001_0000 => Not external, GDT error, index 2


## Tracing
Trace by gdb

```
(gdb) file ./target/x86_64-rustos-none/debug/deps/interrupt-b5aa4371b9e2d1be
Reading symbols from ./target/x86_64-rustos-none/debug/deps/interrupt-b5aa4371b9e2d1be...
(gdb) target remote :1234
Remote debugging using :1234
0x000000000000fff0 in ?? ()
(gdb) b tests/interrupt.rs: 37
Breakpoint 1 at 0x2087c4: file tests/interrupt.rs, line 37.
(gdb) b tests/interrupt.rs: 38
Breakpoint 2 at 0x2087c9: file tests/interrupt.rs, line 38.
(gdb) b src/library/interrupt/handler_interrupt.rs: 11
Breakpoint 3 at 0x20ce52: file src/library/interrupt/handler_interrupt.rs, line 11.
```

1. Clear GDT
 - brackpoint before interrupt
    <details>

   ```
   (gdb) c
   Continuing.

   Breakpoint 1, interrupt::test_interrupt_breakpoint ()
      at tests/interrupt.rs:37
   37          x86_64::instructions::interrupts::int3();
   (gdb) i r
   rax            0x800009ee40        549756464704
   rbx            0x20000000000       2199023255552
   rcx            0x204a00            2116096
   rdx            0x1                 1
   rsi            0x0                 0
   rdi            0x200270            2097776
   rbp            0x56698             0x56698
   rsp            0x800009edc0        0x800009edc0
   r8             0x0                 0
   r9             0x1                 1
   r10            0x3f8               1016
   r11            0x32e20             208416
   r12            0x7272              29298
   r13            0x4                 4
   r14            0x766a              30314
   r15            0x72ba              29370
   rip            0x208594            0x208594 <interrupt::test_interrupt_breakpoint+4>
   eflags         0x6                 [ IOPL=0 PF ]
   cs             0x8                 8
   ss             0x10                16
   ds             0x10                16
   es             0x10                16
   fs             0x0                 0
   gs             0x0                 0
   fs_base        0x0                 0
   gs_base        0x0                 0
   k_gs_base      0x0                 0
   cr0            0x80010011          [ PG WP ET PE ]
   cr2            0x0                 0
   cr3            0x45a000            [ PDBR=0 PCID=0 ]
   cr4            0x20                [ PAE ]
   cr8            0x0                 0
   efer           0xd00               [ NXE LMA LME ]
   ```
   </details>

 - brackpoint in interrupt

    <details>
    ```
   (gdb) c
   Continuing.

   Breakpoint 3, rustos::library::interrupt::handler_interrupt::breakpoint_handler (stack_frame=...)
      at src/library/interrupt/handler_interrupt.rs:11
   11          println!("[Interrupt] Exception: BREAKPOINT\n{:#?}\n", stack_frame);
   (gdb) i r
   rax            0x800009ee40        549756464704
   rbx            0x20000000000       2199023255552
   rcx            0x204a00            2116096
   rdx            0x1                 1
   rsi            0x0                 0
   rdi            0x800009ed88        549756464520
   rbp            0x56698             0x56698
   rsp            0x800009ec90        0x800009ec90
   r8             0x0                 0
   r9             0x1                 1
   r10            0x3f8               1016
   r11            0x32e20             208416
   r12            0x7272              29298
   r13            0x4                 4
   r14            0x766a              30314
   r15            0x72ba              29370
   rip            0x20bec2            0x20bec2 <rustos::library::interrupt::handler_interrupt::breakpoint_handler+34>
   eflags         0x6                 [ IOPL=0 PF ]
   cs             0x8                 8
   ss             0x10                16
   ds             0x10                16
   es             0x10                16
   fs             0x0                 0
   gs             0x0                 0
   fs_base        0x0                 0
   gs_base        0x0                 0
   k_gs_base      0x0                 0
   cr0            0x80010011          [ PG WP ET PE ]
   --Type <RET> for more, q to quit, c to continue without paging--
   cr2            0x0                 0
   cr3            0x45a000            [ PDBR=0 PCID=0 ]
   cr4            0x20                [ PAE ]
   cr8            0x0                 0
   efer           0xd00               [ NXE LMA LME ]
    ```
    </details>

 - brackpoint after interrupt

    <details>
    ```
   (gdb) c
   Continuing.

   Breakpoint 2, interrupt::test_interrupt_breakpoint () at tests/interrupt.rs:38
   38          serial_println!("After invoke breakpoint interrupt");
   (gdb) i r
   rax            0x800009ee40        549756464704
   rbx            0x20000000000       2199023255552
   rcx            0x204a00            2116096
   rdx            0x1                 1
   rsi            0x0                 0
   rdi            0x200270            2097776
   rbp            0x56698             0x56698
   rsp            0x800009edc0        0x800009edc0
   r8             0x0                 0
   r9             0x1                 1
   r10            0x3f8               1016
   r11            0x32e20             208416
   r12            0x7272              29298
   r13            0x4                 4
   r14            0x766a              30314
   r15            0x72ba              29370
   rip            0x208599            0x208599 <interrupt::test_interrupt_breakpoint+9>
   eflags         0x6                 [ IOPL=0 PF ]
   cs             0x8                 8
   ss             0x10                16
   ds             0x10                16
   es             0x10                16
   fs             0x0                 0
   gs             0x0                 0
   fs_base        0x0                 0
   gs_base        0x0                 0
   k_gs_base      0x0                 0
   cr0            0x80010011          [ PG WP ET PE ]
   cr2            0x0                 0
   cr3            0x45a000            [ PDBR=0 PCID=0 ]
   cr4            0x20                [ PAE ]
   cr8            0x0                 0
   efer           0xd00               [ NXE LMA LME ]
    ```
    </details>




2. Set GDT
 - brackpoint before interrupt
    <details>

    ```
    (gdb) c
    Continuing.

    Breakpoint 1, interrupt::test_interrupt_breakpoint () at tests/interrupt.rs:37
    37          x86_64::instructions::interrupts::int3();

    (gdb) i r
    rax            0x800009ee40        549756464704
    rbx            0x20000000000       2199023255552
    rcx            0x204c00            2116608
    rdx            0x1                 1
    rsi            0x0                 0
    rdi            0x200270            2097776
    rbp            0x5a558             0x5a558
    rsp            0x800009edc0        0x800009edc0
    r8             0x0                 0
    r9             0x1                 1
    r10            0x3f8               1016
    r11            0x32e20             208416
    r12            0x7272              29298
    r13            0x4                 4
    r14            0x766a              30314
    r15            0x72ba              29370
    rip            0x2087c4            0x2087c4 <interrupt::test_interrupt_breakpoint+4>
    eflags         0x6                 [ IOPL=0 PF ]
    cs             0x8                 8
    ss             0x10                16
    ds             0x10                16
    es             0x10                16
    fs             0x0                 0
    gs             0x0                 0
    fs_base        0x0                 0
    gs_base        0x0                 0
    k_gs_base      0x0                 0
    cr0            0x80010011          [ PG WP ET PE ]
    cr2            0x0                 0
    cr3            0x45e000            [ PDBR=0 PCID=0 ]
    cr4            0x20                [ PAE ]
    cr8            0x0                 0
    efer           0xd00               [ NXE LMA LME ]
    ```
    </details>

 - brackpoint in interrupt

    <details>
    ```
    (gdb) c
    Continuing.

    Breakpoint 3, rustos::library::interrupt::handler_interrupt::breakpoint_handler (stack_frame=...)
        at src/library/interrupt/handler_interrupt.rs:11
    11          println!("[Interrupt] Exception: BREAKPOINT\n{:#?}\n", stack_frame);
    (gdb) i r
    rax            0x800009ee40        549756464704
    rbx            0x20000000000       2199023255552
    rcx            0x204c00            2116608
    rdx            0x1                 1
    rsi            0x0                 0
    rdi            0x800009ed88        549756464520
    rbp            0x5a558             0x5a558
    rsp            0x800009ec90        0x800009ec90
    r8             0x0                 0
    r9             0x1                 1
    r10            0x3f8               1016
    r11            0x32e20             208416
    r12            0x7272              29298
    r13            0x4                 4
    r14            0x766a              30314
    r15            0x72ba              29370
    rip            0x20ce52            0x20ce52 <rustos::library::interrupt::handler_interrupt::breakpoint_handler+34>
    eflags         0x6                 [ IOPL=0 PF ]
    cs             0x8                 8
    ss             0x10                16
    ds             0x10                16
    es             0x10                16
    fs             0x0                 0
    gs             0x0                 0
    fs_base        0x0                 0
    gs_base        0x0                 0
    k_gs_base      0x0                 0
    cr0            0x80010011          [ PG WP ET PE ]
    cr2            0x0                 0
    cr3            0x45e000            [ PDBR=0 PCID=0 ]
    cr4            0x20                [ PAE ]
    cr8            0x0                 0
    efer           0xd00               [ NXE LMA LME ]
    ```
    </details>

 - brackpoint after interrupt

    <details>
    ```
    (gdb) n
    ...
    (gdb) n
    rustos::library::interrupt::handler_interrupt::double_fault_handler (stack_frame=..., _error_code=0)
        at src/library/interrupt/handler_interrupt.rs:15
    15      pub extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    (gdb) layout asm
    (gdb) i r
    rax            0x800009ee40        549756464704
    rbx            0x20000000000       2199023255552
    rcx            0x204c00            2116608
    rdx            0x1                 1
    rsi            0x0                 0
    rdi            0x200270            2097776
    rbp            0x5a558             0x5a558
    rsp            0x248980            0x248980 <rustos::library::gdt::tss::TSS::{{closure}}::STACK+20432>
    r8             0x0                 0
    r9             0x1                 1
    r10            0x3f8               1016
    r11            0x32e20             208416
    r12            0x7272              29298
    r13            0x4                 4
    r14            0x766a              30314
    r15            0x72ba              29370
    rip            0x20cf30            0x20cf30 <rustos::library::interrupt::handler_interrupt::double_fault_handler>
    eflags         0x2                 [ IOPL=0 ]
    cs             0x8                 8
    ss             0x0                 0
    ds             0x10                16
    es             0x10                16
    fs             0x0                 0
    gs             0x0                 0
    fs_base        0x0                 0
    gs_base        0x0                 0
    k_gs_base      0x0                 0
    cr0            0x80010011          [ PG WP ET PE ]
    cr2            0x0                 0
    cr3            0x45e000            [ PDBR=0 PCID=0 ]
    cr4            0x20                [ PAE ]
    --Type <RET> for more, q to quit, c to continue without paging--
    cr8            0x0                 0
    efer           0xd00               [ NXE LMA LME ]
    ```
    </details>

In here, we can figure out 
     - brackpoint interrupt without change SS
     - it will trigger double fault exception after iretq
     - SS will be change to 0 in double fault exception

Thus, I'll guess segment will be used eventhough paging enable in the exception process.
Base on the [blog_os post](https://os.phil-opp.com/returning-from-exceptions/), we will know below work was done before jump into exception handle function.
 - Align stack
 - Push SS
 - Push RSP
 - Push RFLAGS
 - Push CS
 - Push RIP
 - Push Error Code
 - Alloc new stack area for handler funtion
 - Set new RSP
 - Execute handle funtion

When finished the exception handle funtion and need to go back to keep going running code, there are execute `iretq` which was reverse the privous process. And in [this book](https://0xax.gitbooks.io/linux-insides/content/Interrupts/linux-interrupts-1.html) mention that

> The iret instruction unconditionally pops the stack pointer (ss:rsp) to restore the stack of the interrupted process and does not depend on the cpl change.

This mean SS is still used in long-mode. Just like above gdb show results.

## Summery

When [paging](https://os.phil-opp.com/paging-introduction/) enable in the long mode, all of the segment will not used in the most of case. 

***Why our code was run well before the GDT set?***

I think the answer is that bootloader was set the GDT under `SS=10` in [stage_3.s](https://github.com/rust-osdev/bootloader/blob/v0.10.12/src/asm/stage_3.s), which is the valid gdt table with valid entry.

When the new GDT set by our rust code (under long mode with paging), `SS` is not used to address GDT, and we never set `SS` (still remain the old value 0x10).

Due to iretq is used (ss:rsp) to restore the stack, which will offset our virtural memory into invalid memory position.
$$SS:RPS = VirtualMemory + SS<<4$$

To solve this problem, we need to set `SS=0` or other valid value when set GDT by our rust code.

