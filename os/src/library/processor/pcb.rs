use alloc::sync::{Weak, Arc};
use spin::{Mutex, MutexGuard};
use alloc::vec::Vec;

use crate::library::{memory::memory_set::MemorySet, syscall::trap::TrapFrame};

use super::{kernel_stack::KernelStack, pid::{PidHandle, alloc_pid}, switch::ProcessContext};

pub struct ProcessControlBlock {
    pub pid: PidHandle,
    pub kernel_stack: KernelStack,
    inner: Mutex<ProcessControlBlockInner>
}

pub struct ProcessControlBlockInner {
    pub memory_set: MemorySet,
    pub process_status: ProcessStatus,
    pub process_context_ptr: usize,
    pub parent: Option<Weak<ProcessControlBlock>>,
    pub children: Vec<Arc<ProcessControlBlock>>,
    pub exit_code: isize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProcessStatus {
    Ready,
    Running,
    Zombie
}

impl ProcessControlBlockInner {
    pub fn process_status(&self) -> ProcessStatus {
        self.process_status
    }
    pub fn get_process_context_ptr2(&self) -> *const usize {
        &self.process_context_ptr as *const usize
    }
    pub fn is_zombie(&self) -> bool {
        self.process_status==ProcessStatus::Zombie
    }
}

impl ProcessControlBlock {
    pub fn new(elf_data: &[u8]) -> Self {
        let (memory_set, user_stack, entry_point) = MemorySet::from_elf(elf_data);
        // serial_println!("entry_point: {:x?}", entry_point);
        let pid = alloc_pid();
        let kernel_stack = KernelStack::new(&pid);
        let mut trap_frame = TrapFrame::new();
        trap_frame.rsp = user_stack as u64;
        trap_frame.rcx = entry_point as u64;
        trap_frame.r11 = 0x203; //RFlags
        kernel_stack.push_to_top(trap_frame, 0);
        // Push process context
        let process_context_ptr = kernel_stack.push_to_top(
            ProcessContext::return_from_trap(), 
            core::mem::size_of::<TrapFrame>()
        );
        // serial_println!("process_context_ptr: {:?}", process_context_ptr as usize);
        let task_control_block = Self {
            pid,
            kernel_stack,
            inner: Mutex::new(ProcessControlBlockInner { 
                memory_set, 
                process_status: ProcessStatus::Ready, 
                process_context_ptr: process_context_ptr as usize, 
                parent: None, 
                children: Vec::new(), 
                exit_code: 0 })
        };
        task_control_block
    }
    pub fn exec(&self, elf_data: &[u8]) {
        let mut inner = self.inner_lock();
        inner.memory_set.remove_all_areas();
        let (user_stack, entry_point) = inner.memory_set.read_elf(elf_data);
        let trap_frame = self.get_trap_frame();
        trap_frame.rsp = user_stack as u64;
        trap_frame.rcx = entry_point as u64;
        trap_frame.r11 = 0x203;
    }
    pub fn fork(self: &Arc<ProcessControlBlock>) -> Arc<ProcessControlBlock> {
        let mut parent_inner = self.inner_lock();
        let memory_set = MemorySet::from(&parent_inner.memory_set);
        let pid = alloc_pid();
        let kernel_stack = KernelStack::new(&pid);
        let trap_frame_size = core::mem::size_of::<TrapFrame>();
        let process_context_ptr = kernel_stack.push_to_top(
            ProcessContext::return_from_trap(), 
            trap_frame_size);
        let parent_trap_frame = self.get_trap_frame();
        kernel_stack.push_to_top(parent_trap_frame.clone(), 0);
        // serial_println!("parent TrapFrame:{:?}", parent_trap_frame as *const TrapFrame as usize);
        // serial_println!("parent TrapFrame:\n{:?}", parent_trap_frame);
        // serial_println!("kernel stack: {:?}", kernel_stack.get_top());
        let process_control_block = Arc::new(ProcessControlBlock {
            pid,
            kernel_stack,
            inner: Mutex::new(ProcessControlBlockInner { 
                memory_set, 
                process_status: ProcessStatus::Ready, 
                process_context_ptr: process_context_ptr as usize, 
                parent: Some(Arc::downgrade(self)), 
                children: Vec::new(), 
                exit_code: 0 })
        });
        parent_inner.children.push(process_control_block.clone());
        process_control_block
    }
    pub fn get_trap_frame(&self) -> &'static mut TrapFrame {
        // serial_println!("get_trap_frame: {:?}", self.kernel_stack.get_top());
        unsafe {
            &mut *((self.kernel_stack.get_top() - core::mem::size_of::<TrapFrame>())
                as *mut TrapFrame)
        }
    }
    pub fn inner_lock(&self) -> MutexGuard<ProcessControlBlockInner> {
        self.inner.lock()
    }
    pub fn getpid(&self) -> usize {
        self.pid.0
    }
}