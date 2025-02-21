use crate::sync;

#[cfg(feature = "bump")]
use bump::Allocator;
#[cfg(not(feature = "bump"))]
use pool::Allocator;

#[cfg(feature = "bump")]
mod bump;
#[cfg(not(feature = "bump"))]
mod pool;

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

pub struct LockedHeap<T>(sync::mutex::Mutex<T>);

impl<T> LockedHeap<T> {
    pub const fn new(heap: T) -> Self {
        LockedHeap(sync::mutex::Mutex::new(heap))
    }

    pub fn lock(&self) -> sync::mutex::MutexGuard<T> {
        self.0.lock()
    }
}

#[global_allocator]
pub static ALLOCATOR: LockedHeap<Allocator> = LockedHeap::new(Allocator::new());

pub fn init(base: usize, size: usize) {
    unsafe { ALLOCATOR.lock().init(base, size) }
}

#[allow(dead_code)]
pub fn display() {
    ALLOCATOR.lock().display()
}
