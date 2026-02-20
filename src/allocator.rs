//! Global Allocator
//!
//! Note: Because the `log` crate uses format! (which needs allocations) no logs can be made here

use crate::prelude::*;
use std::alloc::{GlobalAlloc, Layout};
use sys::coreinit::mem;

struct Allocator;
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align().max(4);

        match unsafe { mem::AllocFromDefaultHeapEx } {
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

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        match unsafe { mem::FreeToDefaultHeap } {
            None => (),
            Some(free) => unsafe { free(ptr.cast()) },
        }
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;
