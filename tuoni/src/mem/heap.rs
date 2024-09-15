use core::mem;

type Next = Option<&'static mut HeapBlock>;

struct HeapBlock {
    next: Next,
    size: usize,
    free: usize
}

impl HeapBlock {
    pub fn empty() -> Self {
        HeapBlock {
            next: None,
            size: 0,
            free: 1,
        }
    }

    fn new(base: usize, size: usize, next: Option<&'static mut HeapBlock>) -> &mut HeapBlock {
        let p = base as *mut HeapBlock;
        unsafe {
            (*p).next = next;
            (*p).size = size;
            (*p).free = 0;
            &mut *p
        }
    }

    fn get_base(&self) -> usize {
        self as *const Self as usize
    }
}

struct Heap {
    head: HeapBlock,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            head: HeapBlock::empty(),
        }
    }

    fn align_to(value: usize, align: usize) -> usize {
        let m = align - 1;
        (value + m) & !m
    }

    pub unsafe fn init(&mut self, base: usize, size: usize) {
        assert!(Heap::align_to(base, mem::align_of::<HeapBlock>()) == base);
        self.head.next = Some(HeapBlock::new(base, size, None));
    }
}
