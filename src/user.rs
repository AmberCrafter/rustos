#[naked]
fn user_space() {
    core::arch::asm!(
        "mov rax, 0x01",
        options(noreturn)
    )
}