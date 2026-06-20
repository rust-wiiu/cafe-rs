use crate::sys::gx2::mem;

pub use mem::Invalidate;

pub fn invalidate<T>(mode: mem::Invalidate, ptr: *const T, size: usize) {
    unsafe {
        mem::invalidate(mode, ptr.cast_mut().cast(), size as u32);
    }
}
