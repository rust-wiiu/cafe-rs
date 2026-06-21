use crate::prelude::*;
use std::marker::PhantomData;
use sys::gx2;

// use super::{
//     display::{DRC, RenderTarget, TV},
//     surface::{ColorBuffer, DepthBuffer},
// };

use super::{display::RenderTarget, pipeline::DirectPipeline, target::Target};

pub struct Context<T: RenderTarget> {
    ctx: Box<gx2::state::Context>,
    _marker: PhantomData<T>,
}

impl<T: RenderTarget> Context<T> {
    pub fn new() -> Self {
        Self {
            ctx: Box::new(gx2::state::Context::init(|ctx| unsafe {
                gx2::state::init_context(ctx, 1);
            })),
            _marker: PhantomData,
        }
    }

    pub fn direct_render<F: Fn(&mut DirectPipeline<T>)>(
        &mut self,
        target: &mut Target<T>,
        idx: gx2::state::RenderTarget,
        f: F,
    ) {
        let (width, height) = T::size();
        unsafe {
            gx2::state::set_context(self.as_raw());
            gx2::state::set_colorbuffer(target.color.as_raw(), idx);
            gx2::state::set_depthbuffer(target.depth.as_raw());
            gx2::state::set_viewport(0.0, 0.0, width as f32, height as f32, 0.0, 1.0);
            gx2::state::set_scissor(0, 0, width as u32, height as u32);

            match T::SCAN_TARGET {
                gx2::display::ScanTarget::Tv => {
                    gx2::state::set_tv_scale(width as u32, height as u32)
                }
                gx2::display::ScanTarget::Drc => {
                    gx2::state::set_drc_scale(width as u32, height as u32)
                }
            }
        }

        let mut pipeline = DirectPipeline { ctx: self, target };
        f(&mut pipeline);

        unsafe {
            gx2::display::copy_color_to_scan_buffer(target.color.as_raw(), T::SCAN_TARGET);
        }
    }

    pub fn as_raw(&self) -> &gx2::state::Context {
        &self.ctx
    }
}
