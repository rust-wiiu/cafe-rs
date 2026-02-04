use crate::prelude::*;
use cafe::{
    alloc::{Allocator, FG},
    graphics::mem::{Invalidate, invalidate},
};
use std::{
    alloc::Layout,
    marker::PhantomData,
    mem::size_of,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    slice,
};
use sys::gx2;

pub use sys::gx2::surface::{ColorBuffer, DepthBuffer};

#[derive(Debug)]
pub struct FrameBuffer {
    ptr: NonNull<[u8]>,
    // size: usize,
}

impl FrameBuffer {
    pub fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(size, 0x1000).unwrap();
        let ptr = FG.allocate(layout).unwrap();

        invalidate(Invalidate::Cpu, ptr.as_ptr() as *mut u8, size);

        Self { ptr }
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr() as *mut u8
    }

    pub fn len(&self) -> usize {
        unsafe { self.ptr.as_ref().len() }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            FG.deallocate(self.ptr.cast());
        }
    }
}

pub type Vec4<T> = (T, T, T, T);

pub trait Usage {
    const FLAGS: gx2::surface::ResourceFlags;
}

macro_rules! flags {
    ($($flag:ident),+ $(,)?) => {
        gx2::surface::ResourceFlags::from_bits_truncate(
            $(gx2::surface::ResourceFlags::$flag.bits())|+
        )
    };
}

pub struct Vertex;
pub type VertexBuffer<T> = Buffer<Vertex, T>;
impl Usage for Vertex {
    const FLAGS: gx2::surface::ResourceFlags = flags!(VertexBuffer, Cpu, GpuRead);
}

pub struct Program;
pub type ShaderProgram = Buffer<Program, u8>;
impl Usage for Program {
    const FLAGS: gx2::surface::ResourceFlags = flags!(ShaderProgram, Cpu, GpuRead);
}

pub struct Command;
pub type CommandBuffer = Buffer<Command, u8>;
impl Usage for Command {
    const FLAGS: gx2::surface::ResourceFlags = flags!(DisplayList, Cpu, GpuRead);
}

pub struct Buffer<U, T> {
    inner: gx2::mem::Buffer,
    _marker: PhantomData<(U, T)>,
}

impl<U: Usage, T: Copy> Buffer<U, T> {
    // pub type Lock<'a> = BufferLock<'a, U, T>;

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: gx2::mem::Buffer::builder()
                .flags(U::FLAGS)
                .element_size(size_of::<T>())
                .element_count(capacity)
                .build(),
            _marker: PhantomData,
        }
    }

    pub fn lock<'a>(&'a mut self) -> Result<BufferLock<'a, U, T>, ()> {
        BufferLock::new(self)
    }

    pub fn len(&self) -> usize {
        self.inner.element_count as usize
    }

    // pub fn resize(&mut self) {
    //     todo!()
    // }
}

impl<U: Usage, T: Copy, S: AsRef<[T]>> From<S> for Buffer<U, T> {
    fn from(value: S) -> Self {
        let value = value.as_ref();
        let mut buffer = Self::with_capacity(value.len());
        buffer
            .lock()
            .expect("Cannot lock GX2RBuffer but must be able to")
            .copy_from_slice(value);
        buffer
    }
}

pub struct BufferLock<'a, U, T> {
    buffer: &'a mut Buffer<U, T>,
    data: &'a mut [T],
}

impl<'a, U, T> BufferLock<'a, U, T> {
    fn new(buffer: &'a mut Buffer<U, T>) -> Result<Self, ()> {
        let ptr = unsafe {
            gx2::mem::lock_buffer_ex(&buffer.inner, gx2::surface::ResourceFlags::empty())
        };

        let len = buffer.inner.element_count as usize;
        match ptr.is_null() {
            true => Err(()),
            false => Ok(Self {
                buffer,
                data: unsafe { slice::from_raw_parts_mut(ptr.cast(), len) },
            }),
        }
    }
}

impl<'a, U, T> Deref for BufferLock<'a, U, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, U, T> DerefMut for BufferLock<'a, U, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<U, T> Drop for BufferLock<'_, U, T> {
    fn drop(&mut self) {
        unsafe {
            gx2::mem::unlock_buffer_ex(&self.buffer.inner, gx2::surface::ResourceFlags::empty());
        }
    }
}
