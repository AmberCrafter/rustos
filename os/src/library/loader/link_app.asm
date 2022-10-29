    .align 4
    .section .data
    .global _num_app
_num_app:
    .quad 3
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_2_end

    .global _app_names
_app_names:
    .string "hello_world"
    .string "initproc"
    .string "user_shell"

    .section .data
    .global app_0_start
    .global app_0_end
    .align 4
app_0_start:
    .incbin "../user/target/x86_64-rustos-none/release/hello_world"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
    .align 4
app_1_start:
    .incbin "../user/target/x86_64-rustos-none/release/initproc"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
    .align 4
app_2_start:
    .incbin "../user/target/x86_64-rustos-none/release/user_shell"
app_2_end: