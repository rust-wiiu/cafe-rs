pub mod buffer;
pub mod display;
pub mod gfd;
pub mod mem;
pub mod pipeline;

use crate::prelude::*;
use std::{marker::PhantomData, ptr};
use sys::gx2;

pub fn init() {
    unsafe {
        gx2::state::init(ptr::null_mut());
        gx2::display::set_swap_interval(gx2::display::SwapInterval::VSync60Hz);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Foo")]
    Foo,
}

pub struct TV;
pub struct DRC;
// pub struct Virtual<T: Target>(T);

pub trait Target {}

impl Target for TV {}
impl Target for DRC {}
// impl<T: Target> Target for Virtual<T> {}

pub struct Context<T: Target> {
    // color buffer
    // depth buffer
    _marker: PhantomData<T>,
}

impl<T: Target> Context<T> {
    pub fn enable(&self) {}
}

pub struct Shader<'a> {
    // fetch
    // vertex
    // pixel
    _marker: PhantomData<&'a ()>,
}

pub trait Usage {}

pub struct Vertex;

impl Usage for Vertex {}

pub type VertexBuffer<T> = Buffer<Vertex, T>;

pub struct Buffer<U: Usage, T> {
    _marker: PhantomData<(U, T)>,
}

impl<U: Usage, T> Buffer<U, T> {
    pub fn map<'a>(&'a self) -> Result<MappedBuffer<'a, U, T>, Error> {
        MappedBuffer::map(self)
    }
}

pub struct MappedBuffer<'a, U: Usage, T> {
    buffer: &'a Buffer<U, T>,
    data: &'a mut [T],
}

impl<'a, U: Usage, T> MappedBuffer<'a, U, T> {
    fn map(buffer: &'a Buffer<U, T>) -> Result<Self, Error> {
        todo!()
    }

    pub fn unmap(self) {}
}

impl<'a, U: Usage, T> ::core::ops::Deref for MappedBuffer<'a, U, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, U: Usage, T> ::core::ops::DerefMut for MappedBuffer<'a, U, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'a, U: Usage, T> Drop for MappedBuffer<'a, U, T> {
    fn drop(&mut self) {
        todo!()
    }
}

pub struct Pipeline<'a> {
    _marker: PhantomData<&'a ()>,
}

pub struct PipelineBuilder<'a>(Pipeline<'a>);

impl<'a> PipelineBuilder<'a> {
    pub fn clear_color(&self) {}
    pub fn clear_depth(&self) {}
    pub fn set_shader(&self, shader: &'a Shader) {}
    pub fn set_attribute<T>(&self, buffer: &'a Buffer<Vertex, T>) {}
    pub fn draw(&self) {}
}

#[derive(Debug, Default)]
pub struct Instance {
    // framebuffer
}

impl Instance {
    pub fn create_context<T: Target>(&self) -> Context<T> {
        todo!()
    }

    pub fn create_shader<'a, V, P>(&self, vertex: &'a V, pixel: &'a P) -> Shader<'a> {
        todo!()
    }

    pub fn create_buffer<U: Usage, T>(&self, context: impl AsRef<[T]>) -> Buffer<U, T> {
        todo!()
    }

    pub fn create_pipeline<'a, F: FnOnce(&mut PipelineBuilder<'a>)>(
        &self,
        builder: F,
    ) -> Pipeline<'a> {
        todo!()
    }

    pub fn vsync(&self) -> bool {
        todo!()
    }

    pub fn render<T: Target>(&self, context: &Context<T>, pipeline: &Pipeline<'_>) {}

    pub fn swap(&self, tv: Option<&Context<TV>>, drc: Option<&Context<DRC>>) {}
}
