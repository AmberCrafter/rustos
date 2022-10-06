// use spin::{Mutex, MutexGuard};

pub mod empty_allocator;
pub mod bootinfo_allocator;

// use conquer_once::spin::OnceCell;
use spin::Mutex;
use spin::Lazy;
use bootinfo_allocator::BootInfoFrameAllocator;
use bootloader::boot_info::MemoryRegions;

pub static FRAMEALLOCATORL: Lazy<Mutex<BootInfoFrameAllocator>> = Lazy::new(|| {
    Mutex::new(BootInfoFrameAllocator::new())
});

pub unsafe fn init(memory_regions: &'static MemoryRegions) {
    FRAMEALLOCATORL.lock().init(memory_regions);
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