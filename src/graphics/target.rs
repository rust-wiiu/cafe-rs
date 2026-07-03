use cafe_sys::gx2;

use crate::prelude::*;
use std::marker::PhantomData;

use super::{
    display::{DRC, RenderTarget, TV},
    texture::{ColorBuffer, DepthBuffer, RenderBufferDescriptor},
};

pub struct Target<T: RenderTarget> {
    pub color: ColorBuffer,
    pub depth: DepthBuffer,
    _marker: PhantomData<T>,
}

impl<T: RenderTarget> Target<T> {
    pub fn new() -> Self {
        let (width, height) = T::size();

        Self {
            color: ColorBuffer::new(&RenderBufferDescriptor {
                width,
                height,
                aa: T::AA,
                tile_mode: gx2::mem::TileMode::LinearAligned,
            }),
            depth: DepthBuffer::new(&RenderBufferDescriptor {
                width,
                height,
                aa: T::AA,
                tile_mode: gx2::mem::TileMode::Default,
            }),
            _marker: PhantomData,
        }
    }

    pub fn copy_to_framebuffer(&self) {
        unsafe {
            gx2::display::copy_color_to_scan_buffer(self.color.as_raw(), T::SCAN_TARGET);
        }
    }
}

impl Target<TV> {
    pub fn tv() -> Self {
        Self::new()
    }
}

impl Target<DRC> {
    pub fn drc() -> Self {
        Self::new()
    }
}
