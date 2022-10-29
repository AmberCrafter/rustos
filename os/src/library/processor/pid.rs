use alloc::vec::Vec;
use spin::{Lazy, Mutex};


static PID_ALLOCATOR: Lazy<Mutex<PidAllocator>> = Lazy::new(|| {
    Mutex::new(PidAllocator::new())
});

#[derive(Debug, Clone)]
pub struct PidHandle(pub usize);

struct PidAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl PidAllocator {
    pub fn new() -> Self {
        Self {
            current: 0,
            recycled: Vec::new(),
        }
    }
    pub fn alloc(&mut self) -> PidHandle {
        if let Some(pid) = self.recycled.pop() {
            PidHandle(pid)
        } else {
            self.current += 1;
            PidHandle(self.current - 1)
        }
    }
    pub fn dealloc(&mut self, pid: usize) {
        assert!(pid<self.current);
        assert!(
            self.recycled.iter().find(|&&re_pid| re_pid==pid).is_none(),
            "pid {} has been deallocated",
            pid
        );
        self.recycled.push(pid);
    }
}

pub fn alloc_pid() -> PidHandle {
    PID_ALLOCATOR.lock().alloc()
}

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.lock().dealloc(self.0)
    }
}