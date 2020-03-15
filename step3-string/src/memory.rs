use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

#[alloc_error_handler]
fn out_of_memory(_: Layout) -> ! {
    // panic!("Out-Of-Memory!");
    crate::debug::puts("Out-Of-Memory!");
    crate::debug::exit(-1)
}

extern crate spinning_top;
extern crate linked_list_allocator;
use spinning_top::Spinlock;
use linked_list_allocator::Heap;

static HEAP: Spinlock<Heap> = Spinlock::new(Heap::empty());

pub struct Allocator;

impl Allocator {
    unsafe fn init(&self, start: usize, size: usize) -> () {
        HEAP.lock().init(start, size);
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HEAP.lock()
            .allocate_first_fit(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        HEAP.lock()
            .deallocate(NonNull::new_unchecked(ptr), layout)
    }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

pub unsafe fn init(start: usize, size: usize) {
    ALLOCATOR.init(start, size)
}
