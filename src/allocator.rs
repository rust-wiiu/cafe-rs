//! Global Allocator
//!
//! Note: Because the `log` crate uses format! (which needs allocations) no logs can be made here

use crate::{std::alloc, sys};

struct Allocator;
unsafe impl alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align().max(4);

        match unsafe { sys::coreinit::mem::AllocFromDefaultHeapEx } {
            None => panic!("memory pools were not initialized"),
            Some(malloc) => {
                let ptr = unsafe { malloc(size as u32, align as i32) };

                if ptr.is_null() {
                    panic!("could not allocate {} bytes", size);
                }

                ptr.cast()
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: alloc::Layout) {
        match unsafe { sys::coreinit::mem::FreeToDefaultHeap } {
            None => (),
            Some(free) => unsafe { free(ptr.cast()) },
        }
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;
