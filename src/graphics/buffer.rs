use crate::prelude::*;
use gx2::mem::ResourceFlags;
use std::{marker::PhantomData, mem::ManuallyDrop, ptr};
use sys::gx2;

#[repr(transparent)]
pub struct Buffer<U, T> {
    raw: gx2::mem::Buffer,
    _marker: PhantomData<(U, T)>,
}

impl<U: Usage, T> Buffer<U, T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let raw = gx2::mem::Buffer::init(|buf| {
            buf.flags = U::FLAGS;
            buf.element_size = size_of::<T>() as u32;
            buf.element_count = capacity as u32;

            if unsafe { gx2::mem::create_buffer(buf) } == 0 {
                panic!("Cannot create buffer");
            }
        });

        Self {
            raw,
            _marker: PhantomData,
        }
    }
}

impl<U: Usage, T: Copy> From<Vec<T>> for Buffer<U, T> {
    fn from(mut value: Vec<T>) -> Self {
        let mut buffer = Self::with_capacity(value.len());
        buffer.lock().swap_with_slice(value.as_mut());
        buffer
    }
}

impl<U: Usage, T: Copy, const N: usize> From<[T; N]> for Buffer<U, T> {
    fn from(mut value: [T; N]) -> Self {
        let mut buffer = Self::with_capacity(N);
        buffer.lock().swap_with_slice(value.as_mut());
        buffer
    }
}

impl<U: Usage, T: Copy> From<&[T]> for Buffer<U, T> {
    fn from(value: &[T]) -> Self {
        let mut buffer = Self::with_capacity(value.len());
        buffer.lock().copy_from_slice(value);
        buffer
    }
}

impl<U, T> Buffer<U, T> {
    pub fn len(&self) -> usize {
        self.raw.element_count as usize
    }

    pub fn as_raw(&self) -> &gx2::mem::Buffer {
        &self.raw
    }

    pub fn as_raw_mut(&mut self) -> &mut gx2::mem::Buffer {
        &mut self.raw
    }

    pub fn into_raw(b: Buffer<U, T>) -> gx2::mem::Buffer {
        let b = ManuallyDrop::new(b);
        unsafe { ptr::read(&b.raw) }
    }

    pub unsafe fn from_raw(b: gx2::mem::Buffer) -> Buffer<U, T> {
        Self {
            raw: b,
            _marker: PhantomData,
        }
    }

    pub fn lock(&mut self) -> BufferLock<'_, U, T> {
        BufferLock::new(self)
    }

    pub fn invalidate(&self) {
        unsafe {
            gx2::mem::invalidate_buffer(self.as_raw(), ResourceFlags::empty());
        }
    }
}

impl<U, T> Drop for Buffer<U, T> {
    fn drop(&mut self) {
        unsafe {
            gx2::mem::destroy_buffer(&mut self.raw, ResourceFlags::empty());
        }
    }
}

pub struct BufferLock<'a, U, T> {
    buffer: &'a mut Buffer<U, T>,
    data: &'a mut [T],
}

impl<'a, U, T> BufferLock<'a, U, T> {
    pub fn new(buffer: &'a mut Buffer<U, T>) -> Self {
        let ptr = unsafe { gx2::mem::lock_buffer_ex(buffer.as_raw(), ResourceFlags::empty()) };

        let data = unsafe { std::slice::from_raw_parts_mut(ptr as *mut T, buffer.len()) };

        Self { buffer, data }
    }
}

impl<U, T> Drop for BufferLock<'_, U, T> {
    fn drop(&mut self) {
        unsafe {
            gx2::mem::unlock_buffer_ex(self.buffer.as_raw(), ResourceFlags::empty());
        }
    }
}

impl<U, T> std::ops::Deref for BufferLock<'_, U, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<U, T> std::ops::DerefMut for BufferLock<'_, U, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<U, T: Debug> std::fmt::Debug for BufferLock<'_, U, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

pub trait Usage {
    const FLAGS: ResourceFlags;
}

pub struct Shader;
impl Usage for Shader {
    const FLAGS: ResourceFlags = ResourceFlags::ShaderProgram
        .union(ResourceFlags::GpuRead)
        .union(ResourceFlags::Cpu);
}
pub type ShaderProgram = Buffer<Shader, u8>;

pub struct Vertex;
impl Usage for Vertex {
    const FLAGS: ResourceFlags = ResourceFlags::VertexBuffer
        .union(ResourceFlags::GpuRead)
        .union(ResourceFlags::Cpu);
}
pub type VertexBuffer<T> = Buffer<Vertex, T>;

pub struct Scan;
impl Usage for Scan {
    const FLAGS: ResourceFlags = ResourceFlags::ScanBuffer
        .union(ResourceFlags::Gpu)
        .union(ResourceFlags::Cpu);
}
pub type ScanBuffer = Buffer<Scan, u8>;
