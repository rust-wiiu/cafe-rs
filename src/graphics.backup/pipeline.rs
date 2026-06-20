use crate::prelude::*;
use cafe::graphics::{
    buffer::{BufferLock, Command, CommandBuffer},
    display::{DRC, TV},
};
use std::{boxed::Box, marker::PhantomData};
use sys::gx2::{self, shader};

pub struct Context<DISPLAY> {
    ctx: Box<gx2::state::Context>,
    _marker: PhantomData<DISPLAY>,
}

impl<DISPLAY> Context<DISPLAY> {
    pub fn activate(&self) {
        unsafe {
            gx2::state::set_context(self.ctx.as_ref());
        }
    }
}

impl Context<TV> {
    pub fn tv() -> Self {
        let ctx = unsafe {
            let mut ctx = Box::new_zeroed();
            gx2::state::init_context(ctx.as_mut_ptr(), 0);
            ctx.assume_init()
        };

        let (width, height) = TV::size();

        unsafe {
            gx2::state::set_context(ctx.as_ref());
            gx2::state::set_colorbuffer(
                TV::color_buffer()
                    .as_mut()
                    .expect("graphics::init must be called beforehands")
                    .as_ref(),
                gx2::state::RenderTarget::T0,
            );
            gx2::state::set_depthbuffer(
                TV::depth_buffer()
                    .as_mut()
                    .expect("graphics::init must be called beforehands")
                    .as_ref(),
            );
            gx2::state::set_viewport(0.0, 0.0, width as f32, height as f32, 0.0, 1.0);
            gx2::state::set_scissor(0, 0, width as u32, height as u32);
            gx2::state::set_tv_scale(width as u32, height as u32);

            let fb = TV::frame_buffer()
                .as_ref()
                .expect("graphics::init must be called beforehands");

            gx2::display::set_tv_buffer(
                fb.as_ptr().cast(),
                fb.len() as u32,
                TV::mode(),
                gx2::surface::Format::UnormR8G8B8A8,
                gx2::display::Buffering::Double,
            );
        }

        Self {
            ctx,
            _marker: PhantomData,
        }
    }

    pub fn copy_render_to_framebuffer(&self) {
        unsafe {
            gx2::display::copy_color_to_scan_buffer(
                TV::color_buffer()
                    .as_mut()
                    .expect("graphics::init must be called beforehands")
                    .as_ref(),
                gx2::display::ScanTarget::Tv,
            );
        }
    }
}

impl Context<DRC> {
    pub fn drc() -> Self {
        let ctx = unsafe {
            let mut ctx = Box::new_zeroed();
            gx2::state::init_context(ctx.as_mut_ptr(), 0);
            ctx.assume_init()
        };

        let (width, height) = DRC::size();

        unsafe {
            gx2::state::set_context(ctx.as_ref());
            gx2::state::set_colorbuffer(
                DRC::color_buffer()
                    .as_mut()
                    .expect("graphics::init must be called beforehands")
                    .as_ref(),
                gx2::state::RenderTarget::T0,
            );
            gx2::state::set_depthbuffer(
                DRC::depth_buffer()
                    .as_mut()
                    .expect("graphics::init must be called beforehands")
                    .as_ref(),
            );
            gx2::state::set_viewport(0.0, 0.0, width as f32, height as f32, 0.0, 1.0);
            gx2::state::set_scissor(0, 0, width as u32, height as u32);
            gx2::state::set_drc_scale(width as u32, height as u32);

            let fb = DRC::frame_buffer()
                .as_ref()
                .expect("graphics::init must be called beforehands");

            gx2::display::set_drc_buffer(
                fb.as_ptr().cast(),
                fb.len() as u32,
                DRC::mode(),
                gx2::surface::Format::UnormR8G8B8A8,
                gx2::display::Buffering::Double,
            );
        }

        Self {
            ctx,
            _marker: PhantomData,
        }
    }

    pub fn copy_render_to_framebuffer(&self) {
        unsafe {
            gx2::display::copy_color_to_scan_buffer(
                DRC::color_buffer()
                    .as_mut()
                    .expect("graphics::init must be called beforehands")
                    .as_ref(),
                gx2::display::ScanTarget::Drc,
            );
        }
    }
}

pub struct Shader {
    pub fetch: shader::FetchShader,
    pub vertex: shader::VertexShader,
    pub pixel: shader::PixelShader,
}

impl Shader {
    pub fn new(vertex: shader::VertexShader, pixel: shader::PixelShader) -> Self {
        todo!()
    }
}

impl From<()> for Shader {
    fn from(value: ()) -> Self {
        todo!()
    }
}

pub struct Pipeline {
    buf: CommandBuffer,
}

impl Pipeline {
    const DEFAULT_CAPACITY: usize = 0x2000;

    pub fn new<'a>(capacity: usize) -> PipelineBuilder<'a> {
        PipelineBuilder::new(Self {
            buf: CommandBuffer::with_capacity(capacity),
        })
    }

    pub fn default<'a>() -> PipelineBuilder<'a> {
        Self::new(Self::DEFAULT_CAPACITY)
    }

    pub fn run(&self) {
        todo!()
    }
}

pub struct PipelineBuilder<'a> {
    pipeline: Pipeline,
    lock: BufferLock<'a, Command, u8>,
}

impl<'a> PipelineBuilder<'a> {
    fn new(pipeline: Pipeline) -> Self {
        // let lock = pipeline.buf.lock().unwrap();

        // Self { pipeline, lock }

        todo!()
    }

    pub fn build(self) -> Pipeline {
        // self.0
        todo!()
    }

    pub fn clear_color(self, color: ()) -> Self {
        todo!()
    }

    pub fn clear_depth_stencil(self) -> Self {
        todo!()
    }

    pub fn set_shader(self, shader: ()) -> Self {
        todo!()
    }

    pub fn set_attribute_buffer(self, buffer: ()) -> Self {
        todo!()
    }

    pub fn draw(self) -> Self {
        todo!()
    }
}

pub fn render<'a>(
    tv: Option<(&Context<TV>, impl AsRef<[&'a Pipeline]>)>,
    drc: Option<(&Context<DRC>, impl AsRef<[&'a Pipeline]>)>,
) {
    if let Some((ctx, pipelines)) = tv {
        ctx.activate();

        for pipeline in pipelines.as_ref().iter() {
            pipeline.run();
        }

        ctx.copy_render_to_framebuffer();
    }

    if let Some((ctx, pipelines)) = drc {
        ctx.activate();

        for pipeline in pipelines.as_ref().iter() {
            pipeline.run();
        }

        ctx.copy_render_to_framebuffer();
    }

    unsafe {
        gx2::display::swap_scan_buffers();
    }
}

/// Returns `true` after VSync done
#[inline]
pub fn vsync() -> bool {
    unsafe {
        gx2::display::wait_for_vsync();
    }
    true
}
