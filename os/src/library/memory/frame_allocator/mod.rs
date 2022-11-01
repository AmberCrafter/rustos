// use spin::{Mutex, MutexGuard};

pub mod bootinfo_allocator;
pub mod empty_allocator;

// use conquer_once::spin::OnceCell;
use bootinfo_allocator::BootInfoFrameAllocator;
use bootloader::boot_info::MemoryRegions;
use spin::Lazy;
use spin::Mutex;
use x86_64::structures::paging::FrameAllocator;
use x86_64::structures::paging::PhysFrame;

pub static FRAME_ALLOCATORL: Lazy<Mutex<BootInfoFrameAllocator>> =
    Lazy::new(|| Mutex::new(BootInfoFrameAllocator::new()));

pub unsafe fn init(memory_regions: &'static MemoryRegions) {
    FRAME_ALLOCATORL.lock().init(memory_regions);
}

pub fn alloc_frame() -> Option<PhysFrame> {
    unsafe { FRAME_ALLOCATORL.lock().allocate_frame() }
}

// pub struct Locked<A> {
//     inner: Mutex<A>
// }

// impl<A> Locked<A> {
//     pub const fn new(inner: A) -> Self {
//         Self {
//             inner: Mutex::new(inner)
//         }
//     }

//     pub fn lock(&self) -> MutexGuard<A> {
//         self.inner.lock()
//     }
// }
