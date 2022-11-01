use x86_64::{structures::paging::PageTableFlags, VirtAddr};

use crate::library::memory::{
    memory_set::KERNEL_SPACE,
    page::{GUARD_SIZE, PROCESS_KERNEL_STACK_END, PROCESS_KERNEL_STACK_SIZE},
};

use super::pid::PidHandle;

#[derive(Debug, Clone)]
pub struct KernelStack {
    pid: usize,
}

fn kernel_stack_address(app_id: usize) -> (u64, u64) {
    let top = PROCESS_KERNEL_STACK_END - (app_id as u64) * (PROCESS_KERNEL_STACK_SIZE + GUARD_SIZE);
    let bottom = top - PROCESS_KERNEL_STACK_SIZE;
    (bottom, top)
}

impl KernelStack {
    pub fn new(pid_handle: &PidHandle) -> Self {
        let pid = pid_handle.0;
        let (bottom, top) = kernel_stack_address(pid);
        KERNEL_SPACE.lock().insert(
            VirtAddr::new(bottom),
            VirtAddr::new(top),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            None,
        );
        Self { pid }
    }
    pub fn get_top(&self) -> usize {
        // return physical address
        let (_, top) = kernel_stack_address(self.pid);
        top as usize
    }
    pub fn push_to_top<T>(&self, data: T, offset: usize) -> *mut T
    where
        T: Sized,
    {
        let top = self.get_top() - offset;
        // serial_println!("[Debug] kerenl stack top: {:?}", top);
        let ptr = (top - core::mem::size_of::<T>()) as *mut T;
        unsafe {
            *ptr = data;
        }
        ptr
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let (bottom, _) = kernel_stack_address(self.pid);
        KERNEL_SPACE
            .lock()
            .remove_area_with_start_addr(VirtAddr::new(bottom));
    }
}
