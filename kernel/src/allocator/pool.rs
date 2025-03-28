use super::{align_up, LockedHeap};
use core::alloc::{GlobalAlloc, Layout};
use core::mem;
use core::ptr;

use crate::log_write;

/*
 * This allocator is built on the implementation from
 * https://os.phil-opp.com/allocator-designs/#linked-list-allocator
 * */

struct PoolNode {
    size: usize,
    next: Option<&'static mut PoolNode>,
}

impl PoolNode {
    const fn new(size: usize) -> Self {
        PoolNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct Allocator {
    head: PoolNode,
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            head: PoolNode::new(0),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        assert_eq!(align_up(addr, mem::align_of::<PoolNode>()), addr);
        assert!(size >= mem::size_of::<PoolNode>());

        let mut current = &mut self.head;
        let mut prev: Option<&'static mut PoolNode> = None;

        while let Some(ref mut region_upper) = current.next {
            let region_upper_ptr = region_upper.start_addr() as *mut PoolNode;
            if addr < region_upper.start_addr() {
                let diff_upper = region_upper.start_addr() - addr - size;
                if let Some(ref mut region_lower) = prev {
                    let diff_lower = addr - region_lower.end_addr();

                    if diff_upper < mem::size_of::<PoolNode>()
                        && diff_lower < mem::size_of::<PoolNode>()
                    {
                        let next = region_upper.next.take();

                        region_lower.size = region_upper.end_addr() - region_lower.start_addr();
                        region_lower.next = next;
                    } else if diff_upper < mem::size_of::<PoolNode>() {
                        let next = region_upper.next.take();

                        let mut node = PoolNode::new(region_upper.end_addr() - addr);
                        node.next = next;

                        let node_ptr = addr as *mut PoolNode;
                        node_ptr.write(node);

                        region_lower.next = Some(&mut *node_ptr);
                    } else if diff_lower < mem::size_of::<PoolNode>() {
                        region_lower.size = addr + size - region_lower.start_addr();
                    } else {
                        let mut node = PoolNode::new(size);
                        node.next = Some(&mut *region_upper_ptr);

                        let node_ptr = addr as *mut PoolNode;
                        node_ptr.write(node);

                        region_lower.next = Some(&mut *node_ptr);
                    }
                } else if diff_upper < mem::size_of::<PoolNode>() {
                    let next = region_upper.next.take();

                    let mut node = PoolNode::new(region_upper.end_addr() - addr);
                    node.next = next;

                    let node_ptr = addr as *mut PoolNode;
                    node_ptr.write(node);

                    current.next = Some(&mut *node_ptr);
                } else {
                    let mut node = PoolNode::new(size);
                    node.next = Some(&mut *region_upper_ptr);

                    let node_ptr = addr as *mut PoolNode;
                    node_ptr.write(node);

                    current.next = Some(&mut *node_ptr);
                }
                return;
            } else {
                current = current.next.as_mut().unwrap();
                prev = Some(&mut *region_upper_ptr);
            }
        }

        if let Some(region_lower) = prev {
            let diff_lower = addr - region_lower.end_addr();
            if diff_lower < mem::size_of::<PoolNode>() {
                region_lower.size = addr + size - region_lower.start_addr();
                return;
            }
        }

        let mut node = PoolNode::new(size);
        node.next = current.next.take();
        let node_ptr = addr as *mut PoolNode;
        node_ptr.write(node);
        current.next = Some(&mut *node_ptr)
    }

    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut PoolNode, usize)> {
        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
                let next = region.next.take();
                let ret = Some((current.next.take().unwrap(), alloc_start));
                current.next = next;
                return ret;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        None
    }

    fn alloc_from_region(region: &PoolNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;
        if excess_size > 0 && excess_size < mem::size_of::<PoolNode>() {
            return Err(());
        }

        Ok(alloc_start)
    }

    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<PoolNode>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(mem::size_of::<PoolNode>());
        (size, layout.align())
    }

    // TODO: Implement proper tests instead of using this for debugging
    pub fn display(&mut self) {
        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next {
            log_write!(" _______________________________________\n");
            log_write!("|size:\t\t{:<#8}\t\t|\n", region.size);
            log_write!("|start_addr:\t{:>#16x}\t|\n", region.start_addr());
            log_write!("|end_addr:\t{:>#16x}\t|\n", region.end_addr());
            log_write!("|_______________________________________|\n");
            current = current.next.as_mut().unwrap();
        }
    }
}

unsafe impl GlobalAlloc for LockedHeap<Allocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = Allocator::size_align(layout);
        let mut allocator = self.lock();

        if let Some((region, alloc_start)) = allocator.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).expect("overflow");
            let excess_size = region.end_addr() - alloc_end;
            if excess_size > 0 {
                allocator.add_free_region(alloc_end, excess_size);
            }
            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let (size, _) = Allocator::size_align(layout);
        self.lock().add_free_region(ptr as usize, size)
    }
}
