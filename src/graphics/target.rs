use crate::prelude::*;
use std::marker::PhantomData;

use super::{
    display::{DRC, RenderTarget, TV},
    surface::{ColorBuffer, ColorBufferDescriptor, DepthBuffer, DepthBufferDescriptor},
};

pub struct Target<T: RenderTarget> {
    pub(crate) color: ColorBuffer,
    pub(crate) depth: DepthBuffer,
    _marker: PhantomData<T>,
}

impl<T: RenderTarget> Target<T> {
    pub fn new() -> Self {
        let (width, height) = T::size();

        Self {
            color: ColorBuffer::new(&ColorBufferDescriptor {
                width,
                height,
                format: T::COLOR_FORMAT,
                aa: T::AA,
            }),
            depth: DepthBuffer::new(&DepthBufferDescriptor {
                width,
                height,
                format: T::DEPTH_FORMAT,
                aa: T::AA,
            }),
            _marker: PhantomData,
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
