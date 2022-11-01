#[repr(usize)]
pub enum SystemCall {
    SysRead = 1,
    SysWrite,
    SysExit,
    SysYield,
    SysFork,
    SysExec,
    SysWaitPid,
}

impl SystemCall {
    pub fn as_u64(self) -> u64 {
        self as u64
    }
    pub fn as_usize(self) -> usize {
        self as usize
    }
}

core::arch::global_asm!(
    "\
.global system_call
system_call:
    mov rax, rdi
    mov rdi, rsi
    mov rsi, rdx
    mov rdx, rcx
    lea rcx, [rip]
    syscall
    ret
"
);

extern "C" {
    fn system_call(syscall_id: SystemCall, arg0: usize, arg1: usize, arg2: usize) -> isize;
}

pub fn sys_read(buffer: &mut [u8]) -> isize {
    unsafe {
        system_call(
            SystemCall::SysRead,
            buffer.as_mut_ptr() as usize,
            buffer.len(),
            0,
        )
    }
}

pub fn sys_write(buffer: &[u8]) -> isize {
    unsafe {
        system_call(
            SystemCall::SysWrite,
            buffer.as_ptr() as usize,
            buffer.len(),
            0,
        )
    }
}

pub fn sys_exit(exit_code: i32) -> isize {
    unsafe { system_call(SystemCall::SysExit, exit_code as usize, 0, 0) }
}

pub fn sys_yield() -> isize {
    unsafe { system_call(SystemCall::SysYield, 0, 0, 0) }
}

pub fn sys_fork() -> isize {
    unsafe { system_call(SystemCall::SysFork, 0, 0, 0) }
}

pub fn sys_exec(path: &str) -> isize {
    unsafe { system_call(SystemCall::SysExec, path.as_ptr() as usize, 0, 0) }
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut isize) -> isize {
    unsafe {
        system_call(
            SystemCall::SysWaitPid,
            pid as usize,
            exit_code_ptr as usize,
            0,
        )
    }
}
