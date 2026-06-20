use crate::prelude::*;
use sys::gx2;

use super::{
    context::Context,
    display::RenderTarget,
    shader::{BufferList, FormatList, ShaderGroup},
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

    pub fn use_shader_group<F, B>(&mut self, group: &ShaderGroup<F>, buffers: B)
    where
        F: FormatList,
        B: BufferList<Formats = F>,
    {
        unsafe {
            gx2::shader::set_fetch_shader(group.fetch.as_raw());
            gx2::shader::set_vertex_shader(group.vertex.as_raw());
            gx2::shader::set_pixel_shader(group.pixel.as_raw());
        }

        for (buffer, attr) in buffers.bindings(&group.attributes) {
            unsafe {
                gx2::shader::set_attribute_buffer(buffer, attr.slot, attr.stride, attr.offset);
            }
        }
    }

    pub fn draw(&mut self, mode: gx2::shader::PrimitiveMode, vertices: usize, instances: usize) {
        unsafe {
            gx2::shader::draw_ex(mode, vertices as u32, 0, instances as u32);
        }
    }
}
