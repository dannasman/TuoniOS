use core::alloc::Layout;
use core::mem;
use core::ptr::null_mut;

use crate::sync;

// TODO: Correctly handle alignments not power of two?
fn align_down_size(size: usize, align: usize) -> usize {
    if align == 0 {
        size
    } else {
        size & !(align - 1)
    }
}

fn align_up_size(size: usize, align: usize) -> usize {
    align_down_size(size + align - 1, align)
}

pub fn align_up(addr: *mut u8, align: usize) -> *mut u8 {
    let offset = addr.align_offset(align);
    addr.wrapping_add(offset)
}

pub struct Heap {
    base: *mut u8,
    size: usize,
}

impl Heap {
    pub const fn new() -> Heap {
        Heap {
            base: null_mut(),
            size: 0,
        }
    }

    pub unsafe fn init(&mut self, base: usize, size: usize) {
        assert_eq!(self.size, 0);
        self.base = base as *mut u8;
        self.size = size;
    }

    pub fn base(&self) -> usize {
        self.base as usize
    }

    pub fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        let start = align_up(self.base, align);
        unsafe { self.base = start.wrapping_add(size) };
        start
    }
}

unsafe impl Send for Heap {}
unsafe impl Sync for Heap {}

pub struct LockedHeap(sync::mutex::Mutex<Heap>);

impl LockedHeap {
    pub const fn new(heap: Heap) -> Self {
        LockedHeap(sync::mutex::Mutex::new(heap))
    }

    pub fn lock(&self) -> sync::mutex::MutexGuard<Heap> {
        self.0.lock()
    }
}
