use crate::prelude::*;
use sys::gx2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl From<Color> for (f32, f32, f32, f32) {
    fn from(value: Color) -> Self {
        (
            value.r as f32 / 255.0,
            value.g as f32 / 255.0,
            value.b as f32 / 255.0,
            value.a as f32 / 255.0,
        )
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
            a: 255,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    /// Row
    pub y: usize,
    /// Column
    pub x: usize,
    /// Height
    pub h: usize,
    /// Width
    pub w: usize,
}

impl Rect {
    pub const fn zero() -> Self {
        Self {
            y: 0,
            x: 0,
            h: 0,
            w: 0,
        }
    }

    pub const fn quad(y: usize, x: usize, s: usize) -> Self {
        Self { y, x, h: s, w: s }
    }

    pub const fn overlap(&self, other: Self) -> bool {
        self.x < other.x + other.w
            && self.y < other.y + other.h
            && self.x + self.w > other.x
            && self.y + self.h > other.y
    }
}

pub trait AttributeFormat {
    const FORMAT: gx2::shader::AttribFormat;
}

impl AttributeFormat for [f32; 4] {
    const FORMAT: gx2::shader::AttribFormat = gx2::shader::AttribFormat::Float32_32_32_32;
}

pub trait TextureFormat {
    type TYPE: Copy;
    const FORMAT: gx2::mem::Format;
}

pub struct UnormR8;

impl TextureFormat for UnormR8 {
    type TYPE = u8;
    const FORMAT: gx2::mem::Format = gx2::mem::Format::UnormR8;
}

pub struct UnormR8G8B8A8;

impl TextureFormat for UnormR8G8B8A8 {
    type TYPE = [u8; 4];
    const FORMAT: gx2::mem::Format = gx2::mem::Format::UnormR8G8B8A8;
}

pub struct FloatR32;

impl TextureFormat for FloatR32 {
    type TYPE = f32;
    const FORMAT: gx2::mem::Format = gx2::mem::Format::FloatR32;
}
