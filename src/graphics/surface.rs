use crate::prelude::*;
use std::{mem::ManuallyDrop, ptr};
use sys::gx2;

#[repr(transparent)]
pub struct Surface {
    raw: gx2::surface::Surface,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceDescriptor {
    pub dim: gx2::surface::Dimension,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub mip_levels: usize,
    pub format: gx2::surface::Format,
    pub aa: gx2::surface::AntiAliasing,
    pub tile_mode: gx2::surface::TileMode,
    pub flags: gx2::surface::ResourceFlags,
}

impl Default for SurfaceDescriptor {
    fn default() -> Self {
        Self {
            dim: gx2::surface::Dimension::D2,
            width: 0,
            height: 0,
            depth: 1,
            mip_levels: 1,
            format: gx2::surface::Format::UnormR8G8B8A8,
            aa: gx2::surface::AntiAliasing::X1,
            tile_mode: gx2::surface::TileMode::Default,
            flags: gx2::surface::ResourceFlags::Texture,
        }
    }
}

impl Surface {
    pub fn new(desc: &SurfaceDescriptor) -> Self {
        let raw = gx2::surface::Surface::init(|s| {
            s.dim = desc.dim;
            s.width = desc.width as u32;
            s.height = desc.height as u32;
            s.depth = desc.depth as u32;
            s.num_mips = desc.mip_levels as u32;
            s.format = desc.format;
            s.aa = desc.aa;
            s.flags = desc.flags;
            s.tile_mode = desc.tile_mode;

            unsafe {
                gx2::surface::calc_size_alignment(s);

                if gx2::surface::create_surface(s, s.flags) == 0 {
                    panic!("Cannot create surface");
                }
            }
        });

        Self { raw }
    }
}

impl Surface {
    pub fn as_raw(&self) -> &gx2::surface::Surface {
        &self.raw
    }

    pub fn as_raw_mut(&mut self) -> &mut gx2::surface::Surface {
        &mut self.raw
    }

    pub fn into_raw(s: Surface) -> gx2::surface::Surface {
        let s = ManuallyDrop::new(s);
        unsafe { ptr::read(&s.raw) }
    }

    pub unsafe fn from_raw(s: gx2::surface::Surface) -> Surface {
        Surface { raw: s }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            gx2::surface::destroy_surface(&mut self.raw, gx2::surface::ResourceFlags::empty());
        }
    }
}

pub struct ColorBuffer(gx2::surface::ColorBuffer);

pub struct ColorBufferDescriptor {
    pub width: usize,
    pub height: usize,
    pub format: gx2::surface::Format,
    pub aa: gx2::surface::AntiAliasing,
}

impl ColorBuffer {
    pub fn new(desc: &ColorBufferDescriptor) -> Self {
        let surface = Surface::new(&SurfaceDescriptor {
            width: desc.width,
            height: desc.height,
            format: desc.format,
            aa: desc.aa,
            flags: gx2::surface::ResourceFlags::Texture
                | gx2::surface::ResourceFlags::ColorBuffer
                | gx2::surface::ResourceFlags::Gpu
                | gx2::surface::ResourceFlags::Tv,
            ..Default::default()
        });

        let raw = gx2::surface::ColorBuffer::init(|buf| {
            buf.surface = Surface::into_raw(surface);
            buf.view_num_slices = 1;

            unsafe {
                gx2::surface::init_colorbuffer_regs(buf);
            }
        });

        Self(raw)
    }

    pub fn as_raw(&self) -> &gx2::surface::ColorBuffer {
        &self.0
    }

    pub fn as_raw_mut(&mut self) -> &mut gx2::surface::ColorBuffer {
        &mut self.0
    }
}

impl Drop for ColorBuffer {
    fn drop(&mut self) {
        unsafe {
            let surface = ptr::read(&self.0.surface);
            let _ = Surface::from_raw(surface);
        }
    }
}

pub struct DepthBuffer(gx2::surface::DepthBuffer);

pub struct DepthBufferDescriptor {
    pub width: usize,
    pub height: usize,
    pub format: gx2::surface::Format,
    pub aa: gx2::surface::AntiAliasing,
}

impl DepthBuffer {
    pub fn new(desc: &DepthBufferDescriptor) -> Self {
        let surface = Surface::new(&SurfaceDescriptor {
            width: desc.width,
            height: desc.height,
            format: desc.format,
            aa: desc.aa,
            flags: gx2::surface::ResourceFlags::Texture
                | gx2::surface::ResourceFlags::DepthBuffer
                | gx2::surface::ResourceFlags::Gpu,
            ..Default::default()
        });

        let raw = gx2::surface::DepthBuffer::init(|buf| {
            buf.surface = Surface::into_raw(surface);
            buf.view_num_slices = 1;
            buf.clear_depth = 1.0;

            unsafe {
                gx2::surface::init_depthbuffer_regs(buf);
            }
        });

        Self(raw)
    }

    pub fn as_raw(&self) -> &gx2::surface::DepthBuffer {
        &self.0
    }

    pub fn as_raw_mut(&mut self) -> &mut gx2::surface::DepthBuffer {
        &mut self.0
    }
}

impl Drop for DepthBuffer {
    fn drop(&mut self) {
        unsafe {
            let surface = ptr::read(&self.0.surface);
            let _ = Surface::from_raw(surface);
        }
    }
}
