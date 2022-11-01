use alloc::{collections::VecDeque, sync::Arc};
use spin::{Lazy, Mutex};

use super::pcb::ProcessControlBlock;

pub struct ProcessManager {
    ready_queue: VecDeque<Arc<ProcessControlBlock>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    pub fn add(&mut self, process: Arc<ProcessControlBlock>) {
        self.ready_queue.push_back(process);
    }
    pub fn fetch(&mut self) -> Option<Arc<ProcessControlBlock>> {
        self.ready_queue.pop_front()
    }
}

pub static PROCESS_MANAGER: Lazy<Mutex<ProcessManager>> =
    Lazy::new(|| Mutex::new(ProcessManager::new()));

// public method
pub fn add_process(process: Arc<ProcessControlBlock>) {
    PROCESS_MANAGER.lock().add(process)
}
pub fn fetch_process() -> Option<Arc<ProcessControlBlock>> {
    PROCESS_MANAGER.lock().fetch()
}
