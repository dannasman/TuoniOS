use core::alloc::Layout;

mod heap;

static ALLOCATOR: heap::LockedHeap = heap::LockedHeap::new(heap::Heap::new());

pub fn init(base: usize, size: usize) {
    unsafe { ALLOCATOR.lock().init(base, size) }
}

pub fn alloc(layout: Layout) -> *mut u8 {
    ALLOCATOR.lock().alloc(layout)
}
