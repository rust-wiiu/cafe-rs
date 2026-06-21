use crate::prelude::*;
use sys::gx2;

use super::{
    buffer::VertexBuffer,
    context::Context,
    display::RenderTarget,
    shader::{Attribute, ShaderGroup},
    target::Target,
    types::Color,
};

pub struct DirectPipeline<'a, T: RenderTarget> {
    pub(crate) ctx: &'a mut Context<T>,
    pub(crate) target: &'a mut Target<T>,
}

impl<T: RenderTarget> DirectPipeline<'_, T> {
    pub fn set_color(&mut self, color: impl Into<Color>) {
        let color = color.into();
        let (r, g, b, a) = color.into();
        unsafe {
            gx2::display::clear_color(self.target.color.as_raw_mut(), r, g, b, a);
            gx2::display::clear_depth_stencil_ex(
                self.target.depth.as_raw_mut(),
                self.target.depth.as_raw().clear_depth,
                self.target.depth.as_raw().clear_stencil as u8,
                gx2::display::ClearMode::Both,
            );
            gx2::state::set_context(self.ctx.as_raw());
        }
    }

    pub fn use_shader_group<A>(&mut self, group: &ShaderGroup<A>) {
        unsafe {
            gx2::shader::set_fetch_shader(group.fetch.as_raw());
            gx2::shader::set_vertex_shader(group.vertex.as_raw());
            gx2::shader::set_pixel_shader(group.pixel.as_raw());
        }
    }

    pub fn set_attribute_stream<B>(&mut self, attr: &Attribute, buffer: &VertexBuffer<B>) {
        unsafe {
            gx2::shader::set_attribute_buffer(
                buffer.as_raw(),
                attr.0.buffer,
                size_of::<B>() as u32,
                attr.0.offset,
            );
        }
    }

    pub fn draw(&mut self, mode: gx2::shader::PrimitiveMode, vertices: usize, instances: usize) {
        unsafe {
            gx2::shader::draw_ex(mode, vertices as u32, 0, instances as u32);
        }
    }
}
