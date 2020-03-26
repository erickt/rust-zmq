use std::alloc::{GlobalAlloc, Layout, System};
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

struct Allocator;

static CHECK_PTR: AtomicPtr<u8> = AtomicPtr::new(ptr::null_mut());

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr == CHECK_PTR.load(Ordering::SeqCst) {
            assert_eq!(layout, Layout::new::<[u8; 42]>());
            CHECK_PTR.store(ptr::null_mut(), Ordering::SeqCst);
        }
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static A: Allocator = Allocator;

#[test]
fn message_from_boxed_slice() {
    let mut b: Box<[u8]> = Box::new([0u8; 42]);
    CHECK_PTR.store(b.as_mut_ptr() as *mut u8, Ordering::SeqCst);
    let _ = zmq::Message::from(b);
}
