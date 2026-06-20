use crate::prelude::*;
use bon::bon;
use cafe::{
    alloc::{Allocator, FG},
    graphics::mem::{Invalidate, invalidate},
};
use std::{
    alloc::Layout,
    ffi,
    marker::PhantomData,
    mem::{ManuallyDrop, size_of},
    ops::{Deref, DerefMut},
    ptr, slice,
};
use sys::gx2;

pub use sys::gx2::surface::{AntiAliasing, Dimension, Format, ResourceFlags, TileMode};

/// Framebuffer holds the final rendered image which is displayed on screen.
///
///
#[derive(Debug)]
pub struct FrameBuffer {
    ptr: ptr::NonNull<[u8]>,
    // size: usize,
}

impl FrameBuffer {
    pub fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(size, 0x1000).unwrap();
        let ptr = FG.allocate(layout).unwrap();

        invalidate(Invalidate::Cpu, ptr.as_ptr() as *mut u8, size);

        Self { ptr }
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr() as *mut u8
    }

    pub fn len(&self) -> usize {
        unsafe { self.ptr.as_ref().len() }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            FG.deallocate(self.ptr.cast());
        }
    }
}

/// Surface
///
///
#[repr(transparent)]
pub struct Surface(gx2::surface::Surface);

impl Surface {
    pub fn into_raw(self) -> gx2::surface::Surface {
        let this = ManuallyDrop::new(self);
        unsafe { ptr::read(&this.0) }
    }

    /// # Safety
    ///
    /// The caller must ensure that the [Surface][gx2::surface::Surface] is valid and that they have unique ownership of it.
    pub unsafe fn from_raw(raw: gx2::surface::Surface) -> Self {
        Self(raw)
    }

    pub fn lock_image<'a>(&'a mut self) -> Result<ImageLock<'a>, ()> {
        ImageLock::new(self)
    }
}

#[bon]
impl Surface {
    #[builder]
    pub fn new(
        dim: Dimension,
        width: u32,
        height: u32,
        depth: u32,
        #[builder(default = 0)] num_mips: u32,
        format: Format,
        aa: AntiAliasing,
        flags: ResourceFlags,
        #[builder(default = TileMode::Default)] tile_mode: TileMode,
    ) -> Self {
        let mut s = Self(gx2::surface::Surface {
            dim,
            width,
            height,
            depth,
            num_mips,
            format,
            aa,
            flags,
            image_size: 0,
            image: ptr::null_mut(),
            mip_size: 0,
            mip: ptr::null_mut(),
            tile_mode,
            swizzle: 0,
            alignment: 0,
            pitch: 0,
            mip_offset: [0; 13],
        });

        if unsafe {
            gx2::surface::calc_size_alignment(&mut s.0);
            gx2::surface::create_surface(&mut s.0, s.0.flags)
        } == 0
        {
            crate::OOM!();
        }

        s
    }
}

impl Clone for Surface {
    fn clone(&self) -> Self {
        let other = Self::builder()
            .dim(self.0.dim)
            .width(self.0.width)
            .height(self.0.height)
            .depth(self.0.depth)
            .num_mips(self.0.num_mips)
            .format(self.0.format)
            .aa(self.0.aa)
            .flags(self.0.flags)
            .tile_mode(self.0.tile_mode)
            .build();

        debug_assert_eq!(self.0.image_size, other.0.image_size);
        debug_assert_eq!(self.0.mip_size, other.0.mip_size);
        debug_assert_eq!(self.0.swizzle, other.0.swizzle);
        debug_assert_eq!(self.0.alignment, other.0.alignment);
        debug_assert_eq!(self.0.pitch, other.0.pitch);
        debug_assert_eq!(self.0.mip_offset, other.0.mip_offset);

        debug_assert_ne!(other.0.image, ptr::null_mut());

        if !self.0.image.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(self.0.image, other.0.image, self.0.image_size as usize);
            }
        }

        if !self.0.mip.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(self.0.mip, other.0.mip, self.0.mip_size as usize);
            }
        }

        other
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            gx2::surface::destroy_surface(&mut self.0, gx2::surface::ResourceFlags::empty());
        }
    }
}

impl AsRef<gx2::surface::Surface> for Surface {
    fn as_ref(&self) -> &gx2::surface::Surface {
        &self.0
    }
}

impl AsMut<gx2::surface::Surface> for Surface {
    fn as_mut(&mut self) -> &mut gx2::surface::Surface {
        &mut self.0
    }
}

pub struct ImageLock<'a> {
    surface: &'a mut Surface,
    data: &'a mut [u8],
}

impl<'a> ImageLock<'a> {
    fn new(surface: &'a mut Surface) -> Result<Self, ()> {
        let ptr = unsafe {
            gx2::surface::lock_surface(
                surface.as_ref(),
                gx2::surface::SurfaceData::Image,
                gx2::surface::ResourceFlags::empty(),
            )
        };

        let len = surface.0.image_size as usize;
        match ptr.is_null() {
            true => Err(()),
            false => Ok(Self {
                surface,
                data: unsafe { slice::from_raw_parts_mut(ptr.cast(), len) },
            }),
        }
    }
}

impl<'a> Deref for ImageLock<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a> DerefMut for ImageLock<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl Drop for ImageLock<'_> {
    fn drop(&mut self) {
        unsafe {
            gx2::surface::unlock_surface(
                self.surface.as_ref(),
                gx2::surface::SurfaceData::Image,
                gx2::surface::ResourceFlags::empty(),
            );
        }
    }
}

/*

I ignore this for now as its likely not very important rn

pub struct MipLock

*/

/// ColorBuffer
///
///
#[repr(transparent)]
pub struct ColorBuffer(gx2::surface::ColorBuffer);

#[bon]
impl ColorBuffer {
    #[builder]
    pub fn new(
        surface: Surface,
        #[builder(default = 0)] view_mip: u32,
        #[builder(default = 0)] view_first_slice: u32,
        #[builder(default = 1)] view_num_slices: u32,
        #[builder(default = (ptr::null_mut(), 0))] aa: (*mut ffi::c_void, usize),
    ) -> Self {
        let mut s = Self(gx2::surface::ColorBuffer {
            surface: surface.into_raw(),
            view_mip,
            view_first_slice,
            view_num_slices,
            aa_ptr: aa.0,
            aa_size: aa.1 as u32,
            _regs: [0; 5],
        });

        unsafe {
            gx2::surface::init_colorbuffer_regs(&mut s.0);
        }

        s
    }
}

impl Drop for ColorBuffer {
    fn drop(&mut self) {
        // SAFETY: Exclusive access + drop afterwards
        let surface = unsafe { ptr::read(&self.0.surface) };
        let _ = unsafe { Surface::from_raw(surface) };
    }
}

impl AsRef<gx2::surface::ColorBuffer> for ColorBuffer {
    fn as_ref(&self) -> &gx2::surface::ColorBuffer {
        &self.0
    }
}

impl AsMut<gx2::surface::ColorBuffer> for ColorBuffer {
    fn as_mut(&mut self) -> &mut gx2::surface::ColorBuffer {
        &mut self.0
    }
}

/// DepthBuffer
///
///
#[repr(transparent)]
pub struct DepthBuffer(gx2::surface::DepthBuffer);

#[bon]
impl DepthBuffer {
    #[builder]
    pub fn new(
        surface: Surface,
        #[builder(default = 0)] view_mip: u32,
        #[builder(default = 0)] view_first_slice: u32,
        #[builder(default = 1)] view_num_slices: u32,
        #[builder(default = 1.0)] clear_depth: f32,
        #[builder(default = 0)] clear_stencil: u32,
        #[builder(default = (ptr::null_mut(), 0))] z: (*mut ffi::c_void, usize),
    ) -> Self {
        let mut s = Self(gx2::surface::DepthBuffer {
            surface: surface.into_raw(),
            view_mip,
            view_first_slice,
            view_num_slices,
            z_ptr: z.0,
            z_size: z.1 as u32,
            clear_depth,
            clear_stencil,
            _regs: [0; 7],
        });

        unsafe {
            gx2::surface::init_depthbuffer_regs(&mut s.0);
        }

        s
    }
}

impl Drop for DepthBuffer {
    fn drop(&mut self) {
        // SAFETY: Exclusive access + drop afterwards
        let surface = unsafe { ptr::read(&self.0.surface) };
        let _ = unsafe { Surface::from_raw(surface) };
    }
}

impl AsRef<gx2::surface::DepthBuffer> for DepthBuffer {
    fn as_ref(&self) -> &gx2::surface::DepthBuffer {
        &self.0
    }
}

impl AsMut<gx2::surface::DepthBuffer> for DepthBuffer {
    fn as_mut(&mut self) -> &mut gx2::surface::DepthBuffer {
        &mut self.0
    }
}

//
// Buffer
//

pub type Vec4<T> = (T, T, T, T);

pub trait Usage {
    const FLAGS: gx2::surface::ResourceFlags;
}

macro_rules! flags {
    ($($flag:ident),+ $(,)?) => {
        gx2::surface::ResourceFlags::from_bits_truncate(
            $(gx2::surface::ResourceFlags::$flag.bits())|+
        )
    };
}

pub struct Vertex;
pub type VertexBuffer<T> = Buffer<Vertex, T>;
impl Usage for Vertex {
    const FLAGS: gx2::surface::ResourceFlags = flags!(VertexBuffer, Cpu, GpuRead);
}

pub struct Program;
pub type ShaderProgram = Buffer<Program, u8>;
impl Usage for Program {
    const FLAGS: gx2::surface::ResourceFlags = flags!(ShaderProgram, Cpu, GpuRead);
}

pub struct Command;
pub type CommandBuffer = Buffer<Command, u8>;
impl Usage for Command {
    const FLAGS: gx2::surface::ResourceFlags = flags!(DisplayList, Cpu, GpuRead);
}

pub struct Buffer<U, T> {
    inner: gx2::mem::Buffer,
    _marker: PhantomData<(U, T)>,
}

impl<U: Usage, T: Copy> Buffer<U, T> {
    // pub type Lock<'a> = BufferLock<'a, U, T>;

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            // inner: gx2::mem::Buffer::builder()
            //     .flags(U::FLAGS)
            //     .element_size(size_of::<T>())
            //     .element_count(capacity)
            //     .build(),
            inner: gx2::mem::Buffer::init(|buf| {
                buf.flags = U::FLAGS;
                buf.element_size = size_of::<T>() as u32;
                buf.element_count = capacity as u32;

                if unsafe { gx2::mem::create_buffer(buf) } == 0 {
                    panic!("OOM");
                }
            }),
            _marker: PhantomData,
        }
    }

    pub fn lock<'a>(&'a mut self) -> Result<BufferLock<'a, U, T>, ()> {
        BufferLock::new(self)
    }

    pub fn len(&self) -> usize {
        self.inner.element_count as usize
    }

    // pub fn resize(&mut self) {
    //     todo!()
    // }
}

impl<U: Usage, T: Copy, S: AsRef<[T]>> From<S> for Buffer<U, T> {
    fn from(value: S) -> Self {
        let value = value.as_ref();
        let mut buffer = Self::with_capacity(value.len());
        buffer
            .lock()
            .expect("Cannot lock GX2RBuffer but must be able to")
            .copy_from_slice(value);
        buffer
    }
}

pub struct BufferLock<'a, U, T> {
    buffer: &'a mut Buffer<U, T>,
    data: &'a mut [T],
}

impl<'a, U, T> BufferLock<'a, U, T> {
    fn new(buffer: &'a mut Buffer<U, T>) -> Result<Self, ()> {
        let ptr = unsafe {
            gx2::mem::lock_buffer_ex(&buffer.inner, gx2::surface::ResourceFlags::empty())
        };

        let len = buffer.inner.element_count as usize;
        match ptr.is_null() {
            true => Err(()),
            false => Ok(Self {
                buffer,
                data: unsafe { slice::from_raw_parts_mut(ptr.cast(), len) },
            }),
        }
    }
}

impl<'a, U, T> Deref for BufferLock<'a, U, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, U, T> DerefMut for BufferLock<'a, U, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<U, T> Drop for BufferLock<'_, U, T> {
    fn drop(&mut self) {
        unsafe {
            gx2::mem::unlock_buffer_ex(&self.buffer.inner, gx2::surface::ResourceFlags::empty());
        }
    }
}
