use super::types::{FloatR32, Rect, TextureFormat, UnormR8G8B8A8};
use crate::prelude::*;
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};
use sys::gx2;

pub use gx2::mem::{
    AntiAliasing, Dimension, Format, ResourceFlags, SurfaceData as TextureData, TileMode,
};
pub use gx2::shader::{Component, ComponentSelection};

#[inline]
fn create_surface(
    width: usize,
    height: usize,
    format: Format,
    aa: AntiAliasing,
    tile_mode: TileMode,
    flags: ResourceFlags,
) -> gx2::mem::Surface {
    let mut surface = gx2::mem::Surface::init(|s| {
        s.dim = Dimension::D2;
        s.width = width as u32;
        s.height = height as u32;
        s.depth = 1;
        s.num_mips = 1;
        s.format = format;
        s.aa = aa;
        s.tile_mode = tile_mode;
        s.flags = flags;

        unsafe {
            gx2::mem::calc_size_alignment(s);
        }
    });

    if unsafe { gx2::mem::create_surface(&mut surface, flags) } == 0 {
        panic!("No available heap memory for allocating texture");
    } else {
        surface
    }
}

#[inline]
fn destroy_surface(surface: &mut gx2::mem::Surface) {
    unsafe {
        gx2::mem::destroy_surface(surface, ResourceFlags::empty());
    }
}

pub struct RenderBufferDescriptor {
    pub width: usize,
    pub height: usize,
    pub aa: AntiAliasing,
    pub tile_mode: TileMode,
}

pub struct ColorBuffer<T: TextureFormat = UnormR8G8B8A8> {
    raw: gx2::mem::ColorBuffer,
    _marker: PhantomData<T>,
}

impl<T: TextureFormat> ColorBuffer<T> {
    pub fn new(desc: &RenderBufferDescriptor) -> Self {
        let surface = create_surface(
            desc.width,
            desc.height,
            T::FORMAT,
            desc.aa,
            desc.tile_mode,
            ResourceFlags::Texture
                | ResourceFlags::ColorBuffer
                | ResourceFlags::Gpu
                | ResourceFlags::Cpu,
        );

        Self {
            raw: gx2::mem::ColorBuffer::init(|buf| {
                buf.surface = surface;
                buf.view_num_slices = 1;

                unsafe {
                    gx2::mem::init_colorbuffer_regs(buf);
                }
            }),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_raw(&self) -> &gx2::mem::ColorBuffer {
        &self.raw
    }

    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut gx2::mem::ColorBuffer {
        &mut self.raw
    }

    #[inline]
    pub fn view_2d(&mut self) -> View2D<'_, T> {
        View2D::new(&mut self.raw.surface, TextureData::Image)
    }
}

impl<T: TextureFormat> Drop for ColorBuffer<T> {
    fn drop(&mut self) {
        destroy_surface(&mut self.raw.surface);
    }
}

pub struct DepthBuffer<T: TextureFormat = FloatR32> {
    raw: gx2::mem::DepthBuffer,
    _marker: PhantomData<T>,
}

impl<T: TextureFormat> DepthBuffer<T> {
    pub fn new(desc: &RenderBufferDescriptor) -> Self {
        let surface = create_surface(
            desc.width,
            desc.height,
            T::FORMAT,
            desc.aa,
            desc.tile_mode,
            ResourceFlags::Texture | ResourceFlags::DepthBuffer | ResourceFlags::Gpu,
        );

        Self {
            raw: gx2::mem::DepthBuffer::init(|buf| {
                buf.surface = surface;
                buf.view_num_slices = 1;
                buf.clear_depth = 1.0;

                unsafe {
                    gx2::mem::init_depthbuffer_regs(buf);
                }
            }),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_raw(&self) -> &gx2::mem::DepthBuffer {
        &self.raw
    }

    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut gx2::mem::DepthBuffer {
        &mut self.raw
    }

    #[inline]
    pub fn view_2d(&mut self) -> View2D<'_, T> {
        View2D::new(&mut self.raw.surface, TextureData::Image)
    }
}

impl<T: TextureFormat> Drop for DepthBuffer<T> {
    fn drop(&mut self) {
        destroy_surface(&mut self.raw.surface);
    }
}

pub struct TextureDescriptor {
    pub width: usize,
    pub height: usize,
    pub aa: AntiAliasing,
    pub tile_mode: TileMode,
    pub comp: ComponentSelection,
}

impl Default for TextureDescriptor {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            aa: AntiAliasing::X1,
            tile_mode: TileMode::Default,
            comp: ComponentSelection::xyzw(),
        }
    }
}

pub struct Texture<T: TextureFormat> {
    raw: gx2::mem::Texture,
    _marker: PhantomData<T>,
}

impl<T: TextureFormat> Texture<T> {
    pub fn new(desc: &TextureDescriptor) -> Self {
        let surface = create_surface(
            desc.width,
            desc.height,
            T::FORMAT,
            desc.aa,
            desc.tile_mode,
            ResourceFlags::Texture | ResourceFlags::Gpu | ResourceFlags::Cpu,
        );

        Self {
            raw: gx2::mem::Texture::init(|tex| {
                tex.surface = surface;

                unsafe {
                    gx2::mem::init_texture_regs(tex);
                }
            }),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.raw.surface.height as usize
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.raw.surface.width as usize
    }

    #[inline]
    pub fn pitch(&self) -> usize {
        self.raw.surface.pitch as usize
    }

    #[inline]
    pub fn as_raw(&self) -> &gx2::mem::Texture {
        &self.raw
    }

    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut gx2::mem::Texture {
        &mut self.raw
    }

    #[inline]
    pub fn view_2d(&mut self, data: TextureData) -> View2D<'_, T> {
        View2D::new(&mut self.raw.surface, data)
    }
}

impl<T: TextureFormat> Drop for Texture<T> {
    fn drop(&mut self) {
        destroy_surface(&mut self.raw.surface);
    }
}

pub struct View2D<'a, T: TextureFormat> {
    surface: &'a mut gx2::mem::Surface,
    data: &'a mut [T::TYPE],
    what: TextureData,
    height: usize,
    width: usize,
    pitch: usize,
}

impl<'a, T: TextureFormat> View2D<'a, T> {
    fn new(surface: &'a mut gx2::mem::Surface, what: TextureData) -> Self {
        let ptr =
            unsafe { gx2::mem::lock_surface(surface, what, gx2::mem::ResourceFlags::empty()) };

        let (len, pitch, width, height) = match what {
            TextureData::Image => (
                surface.image_size as usize,
                surface.pitch as usize,
                surface.width as usize,
                surface.height as usize,
            ),
            TextureData::MipAll => panic!("dont know hot to implement yet"),
            // UNTESTED
            mip => {
                let level = mip.try_into().unwrap();
                unsafe {
                    (
                        match level {
                            gx2::mem::MipLevel::L0 => surface.image_size as usize,
                            gx2::mem::MipLevel::L1 => surface.mip_offset[1] as usize,
                            _ => {
                                if level as u32 == surface.num_mips - 1 {
                                    surface.mip_size as usize
                                        - surface.mip_offset[level as usize - 1] as usize
                                } else {
                                    surface.mip_offset[level as usize] as usize
                                        - surface.mip_offset[level as usize - 1] as usize
                                }
                            }
                        },
                        gx2::mem::surface_mip_pitch(surface, level) as usize,
                        (surface.width as usize >> mip as usize).max(1),
                        (surface.height as usize >> mip as usize).max(1),
                    )
                }
            }
        };

        let data = unsafe {
            std::slice::from_raw_parts_mut(ptr as *mut T::TYPE, len / size_of::<T::TYPE>())
        };

        Self {
            surface,
            data,
            what,
            width,
            height,
            pitch,
        }
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T::TYPE]> + '_ {
        let width = self.width;
        self.data
            .chunks_exact(self.pitch)
            .take(self.height)
            .map(move |row| &row[..width])
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T::TYPE]> + '_ {
        let width = self.width;
        self.data
            .chunks_exact_mut(self.pitch)
            .take(self.height)
            .map(move |row| &mut row[..width])
    }

    pub fn pixels(&self) -> impl Iterator<Item = (usize, usize, &T::TYPE)> + '_ {
        self.rows()
            .enumerate()
            .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, px)| (r, c, px)))
    }

    pub fn pixels_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T::TYPE)> + '_ {
        self.rows_mut()
            .enumerate()
            .flat_map(|(r, row)| row.iter_mut().enumerate().map(move |(c, px)| (r, c, px)))
    }

    /// Copy from 2D slice into view.
    ///
    /// The source is expected to be a row-major, continous slice.
    ///
    /// # Panic
    ///
    /// The function will panic if the source's length is smaller than `rect`.
    pub fn copy_from_slice_2d(&mut self, src: &[T::TYPE], rect: Rect) {
        assert!(src.len() >= rect.w * rect.h);

        for (dst, src) in self
            .rows_mut()
            .skip(rect.y)
            .take(rect.h)
            .zip(src.chunks_exact(rect.w))
        {
            dst.copy_from_slice(src);
        }
    }

    // copy_from_sub_slice_2d(&mut self, data, src, dst) ?
}

impl<T: TextureFormat> Drop for View2D<'_, T> {
    fn drop(&mut self) {
        unsafe {
            gx2::mem::unlock_surface(self.surface, self.what, gx2::mem::ResourceFlags::empty());
        }
    }
}

impl<T: TextureFormat> Deref for View2D<'_, T> {
    type Target = [T::TYPE];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: TextureFormat> DerefMut for View2D<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: TextureFormat> Index<(usize, usize)> for View2D<'_, T> {
    type Output = T::TYPE;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;

        &self.data[row * self.pitch + col]
    }
}

impl<T: TextureFormat> IndexMut<(usize, usize)> for View2D<'_, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;

        &mut self.data[row * self.pitch + col]
    }
}
