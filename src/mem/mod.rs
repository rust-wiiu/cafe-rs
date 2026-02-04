pub mod heap;

use crate::std;

pub struct AlignedBuf<const ALIGN: usize> {
    ptr: std::ptr::NonNull<u8>,
    len: usize,
}

impl<const ALIGN: usize> AlignedBuf<ALIGN> {
    pub fn new(len: usize) -> Self {
        assert!(ALIGN.is_power_of_two(), "Alignment must be a power of two");
        let layout = std::alloc::Layout::from_size_align(len, ALIGN).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        Self {
            ptr: std::ptr::NonNull::new(ptr).unwrap(),
            len,
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr().cast_const()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<const ALIGN: usize> Drop for AlignedBuf<ALIGN> {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::from_size_align(self.len, ALIGN).unwrap();
        unsafe {
            std::alloc::dealloc(self.ptr.as_ptr(), layout);
        }
    }
}
