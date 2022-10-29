pub mod kernel_stack;
pub mod manager;
mod pcb;
mod pid;
mod switch;
use core::cell::RefCell;

use alloc::sync::Arc;
use spin::Lazy;
use x86_64::{
    structures::paging::{OffsetPageTable, PageTable, PhysFrame, Size4KiB, Translate},
    VirtAddr,
};

use crate::library::processor::{manager::fetch_process, pcb::ProcessStatus};

use self::{manager::add_process, pcb::ProcessControlBlock, switch::switch_to};

use super::{loader::get_app_data_by_name, memory::page::current_offset_page_table};

pub static PROCESSOR: Lazy<Processor> = Lazy::new(|| Processor::new());
pub static INITPROC: Lazy<Arc<ProcessControlBlock>> = Lazy::new(|| {
    // serial_println!("INITPROC: ");
    Arc::new(ProcessControlBlock::new(
        get_app_data_by_name("initproc").unwrap(),
    ))
});

pub struct Processor {
    inner: RefCell<ProcessorInner>,
}

pub struct ProcessorInner {
    current: Option<Arc<ProcessControlBlock>>,
    idle_process_context_ptr: usize,
    idle_page_table: OffsetPageTable<'static>,
}

unsafe impl Sync for Processor {}

impl Processor {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(ProcessorInner {
                current: None,
                idle_process_context_ptr: 0,
                idle_page_table: unsafe { current_offset_page_table() },
            }),
        }
    }
    fn get_idle_process_context_ptr(&self) -> usize {
        self.inner.borrow().idle_process_context_ptr
    }
    fn get_idle_process_context_ptr2(&self) -> *const usize {
        let inner = self.inner.borrow();
        &inner.idle_process_context_ptr as *const usize
    }
    pub fn run(&self) {
        loop {
            if let Some(process) = fetch_process() {
                // serial_println!("PID: {:?}", process.getpid());
                let idle_task_cx_ptr2 = self.get_idle_process_context_ptr2();
                let mut process_inner = process.inner_lock();
                let next_process_context = process_inner.process_context_ptr;

                // next process page table
                let page_table: PhysFrame<Size4KiB> = {
                    let page_table = &mut process_inner.memory_set.page_table;
                    let page_talbe_virt =
                        VirtAddr::new(page_table.level_4_table() as *mut PageTable as u64);
                    let page_table_phys = page_table.translate_addr(page_talbe_virt).unwrap();
                    PhysFrame::containing_address(page_table_phys)
                };
                process_inner.process_status = ProcessStatus::Running;
                drop(process_inner);
                self.inner.borrow_mut().current.replace(process);

                // serial_println!("[debug] {:?}", page_table);
                // serial_println!("[debug] idle_task_cx: {:?}", self.get_idle_process_context_ptr2() as usize);
                // serial_println!("[debug] idle_task_cx: {:?}", self.get_idle_process_context_ptr());
                // serial_println!("[debug] next_process_context: {:?}", next_process_context);
                // serial_println!("[debug] context switch");
                unsafe {
                    use x86_64::registers::control::Cr3;
                    let (_, flags) = Cr3::read();
                    Cr3::write(page_table, flags);
                    // switch_mm(page_table as usize);
                    // serial_println!("checker");
                    switch_to(idle_task_cx_ptr2, next_process_context);
                }
            }
        }
    }
    pub fn current(&self) -> Option<Arc<ProcessControlBlock>> {
        self.inner.borrow().current.as_ref().cloned()
    }
    pub fn take_current(&self) -> Option<Arc<ProcessControlBlock>> {
        self.inner.borrow_mut().current.take()
    }
}

pub fn add_initproc() {
    add_process(INITPROC.clone());
}

pub fn run_processes() {
    PROCESSOR.run()
}

pub fn current_process() -> Option<Arc<ProcessControlBlock>> {
    PROCESSOR.current()
}

pub fn current_kernel_stack() -> usize {
    let c = PROCESSOR.current().unwrap();
    // serial_println!("[debug] pid: {:?}", c.getpid());
    // serial_println!("[debug] stack_top: {:?}", c.kernel_stack.get_top());
    c.kernel_stack.get_top()
}

pub fn take_current_process() -> Option<Arc<ProcessControlBlock>> {
    PROCESSOR.take_current()
}

pub fn schedule(swtiched_process_context_ptr2: *const usize) {
    let idle_process_context_ptr = PROCESSOR.get_idle_process_context_ptr();
    unsafe {
        switch_to(swtiched_process_context_ptr2, idle_process_context_ptr);
    }
}

pub fn suspend_current_and_run_next() {
    let process = take_current_process().unwrap();
    let mut inner = process.inner_lock();
    let task_context_ptr2 = inner.get_process_context_ptr2();
    inner.process_status = ProcessStatus::Ready;
    drop(inner);
    add_process(process);
    schedule(task_context_ptr2);
}

pub fn exit_current_and_run_next(exit_code: isize) {
    let process = take_current_process().unwrap();
    let mut inner = process.inner_lock();
    inner.process_status = ProcessStatus::Zombie;
    inner.exit_code = exit_code;
    {
        let mut initproc_inner = INITPROC.inner_lock();
        for child in inner.children.iter() {
            child.inner_lock().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    inner.children.clear();
    drop(inner);
    drop(process);
    let _unused: usize = 0;
    schedule(&_unused as *const _);
}

#[naked]
extern "C" fn test_syscall() {
    unsafe {
        core::arch::asm!(
            "
            syscall
        ",
            options(noreturn)
        );
    }
}
