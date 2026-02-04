use crate::prelude::*;

use super::buffer::FrameBuffer;
use cafe_sys::gx2;
use std::{cell::UnsafeCell, ffi::c_void, ptr};
use sys::{
    gx2::{
        display::{
            AspectRatio, Buffering, DrcMode, ScanMode, ScanTarget, TvMode, aspect_ratio,
            drc_framebuffer_size, drc_mode, scan_mode, tv_framebuffer_size,
        },
        surface::{
            AntiAliasing, ColorBuffer, DepthBuffer, Dimension, Format, ResourceFlags, Surface,
        },
    },
    proc_ui,
};

struct SyncUnsafeCell<T>(UnsafeCell<T>);
unsafe impl<T> Sync for SyncUnsafeCell<T> {}

pub struct TV;

impl TV {
    /// Width x Height
    #[inline]
    pub fn size() -> (usize, usize) {
        match unsafe { scan_mode() } {
            ScanMode::NTSC | ScanMode::NTSCp => match unsafe { aspect_ratio() } {
                AspectRatio::Standard => (854, 480),
                AspectRatio::Widescreen => (640, 480),
            },
            ScanMode::PAL | ScanMode::HD => (1280, 720),
            ScanMode::FHD | ScanMode::FHDi => (1920, 1080),
        }
    }

    #[inline]
    pub fn mode() -> TvMode {
        match unsafe { scan_mode() } {
            ScanMode::NTSC | ScanMode::NTSCp => match unsafe { aspect_ratio() } {
                AspectRatio::Standard => TvMode::Wide480,
                AspectRatio::Widescreen => TvMode::Standard480,
            },
            ScanMode::PAL | ScanMode::HD => TvMode::Wide720,
            ScanMode::FHD | ScanMode::FHDi => TvMode::Wide1080,
        }
    }

    #[inline]
    pub fn enable(enable: bool) {
        unsafe {
            gx2::display::enable_tv(enable);
        }
    }

    pub fn init() {
        unsafe {
            proc_ui::register_callback(
                proc_ui::Message::Acquire,
                Some(Self::acquire),
                ptr::null_mut(),
                100,
            );

            proc_ui::register_callback(
                proc_ui::Message::Release,
                Some(Self::release),
                ptr::null_mut(),
                100,
            );

            Self::acquire(ptr::null_mut());
        }
    }

    unsafe extern "C" fn acquire(_context: *mut c_void) -> u32 {
        // This currently is modeled after the way its done in WUT.
        // Im not sure if this can race but I think it can.
        // I will take another look once the main thing is working.

        let (width, height) = Self::size();

        Self::color_buffer().insert(
            ColorBuffer::builder()
                .surface(
                    Surface::builder()
                        .flags(
                            ResourceFlags::ColorBuffer | ResourceFlags::Texture | ResourceFlags::Tv,
                        )
                        .dim(Dimension::D2)
                        .width(width as u32)
                        .height(height as u32)
                        .depth(1)
                        .format(Format::UnormR8G8B8A8)
                        .aa(AntiAliasing::X1)
                        .build(),
                )
                .build(),
        );

        Self::depth_buffer().insert(
            DepthBuffer::builder()
                .surface(
                    Surface::builder()
                        .flags(ResourceFlags::DepthBuffer | ResourceFlags::Texture)
                        .dim(Dimension::D2)
                        .width(width as u32)
                        .height(height as u32)
                        .depth(1)
                        .format(Format::UnormR8G8B8A8)
                        .aa(AntiAliasing::X1)
                        .build(),
                )
                .build(),
        );

        Self::frame_buffer().insert({
            let mut _unused = 0;
            let mut size = 0;

            unsafe {
                tv_framebuffer_size(
                    Self::mode(),
                    Format::UnormR8G8B8A8,
                    Buffering::Double,
                    &mut size,
                    &mut _unused,
                );
            }

            FrameBuffer::new(size as usize)
        });

        0
    }

    unsafe extern "C" fn release(_context: *mut c_void) -> u32 {
        let _ = Self::color_buffer().take();
        let _ = Self::depth_buffer().take();
        let _ = Self::frame_buffer().take();

        0
    }

    pub(crate) fn color_buffer() -> &'static mut Option<ColorBuffer> {
        static BUFFER: SyncUnsafeCell<Option<ColorBuffer>> = SyncUnsafeCell(UnsafeCell::new(None));

        unsafe { &mut (*BUFFER.0.get()) }
    }

    pub(crate) fn depth_buffer() -> &'static mut Option<DepthBuffer> {
        static BUFFER: SyncUnsafeCell<Option<DepthBuffer>> = SyncUnsafeCell(UnsafeCell::new(None));

        unsafe { &mut (*BUFFER.0.get()) }
    }

    pub(crate) fn frame_buffer() -> &'static mut Option<FrameBuffer> {
        static BUFFER: SyncUnsafeCell<Option<FrameBuffer>> = SyncUnsafeCell(UnsafeCell::new(None));

        unsafe { &mut (*BUFFER.0.get()) }
    }
}

pub struct DRC;

impl DRC {
    /// Width x Height
    #[inline]
    pub fn size() -> (usize, usize) {
        (854, 480)
    }

    #[inline]
    pub fn mode() -> DrcMode {
        unsafe { drc_mode() }
    }

    #[inline]
    pub fn enable(enable: bool) {
        unsafe {
            gx2::display::enable_drc(enable);
        }
    }

    pub fn init() {
        unsafe {
            proc_ui::register_callback(
                proc_ui::Message::Acquire,
                Some(Self::acquire),
                ptr::null_mut(),
                100,
            );

            proc_ui::register_callback(
                proc_ui::Message::Release,
                Some(Self::release),
                ptr::null_mut(),
                100,
            );

            Self::acquire(ptr::null_mut());
        }
    }

    unsafe extern "C" fn acquire(_context: *mut c_void) -> u32 {
        // This currently is modeled after the way its done in WUT.
        // Im not sure if this can race but I think it can.
        // I will take another look once the main thing is working.

        let (width, height) = Self::size();

        Self::color_buffer().insert(
            ColorBuffer::builder()
                .surface(
                    Surface::builder()
                        .flags(
                            ResourceFlags::ColorBuffer | ResourceFlags::Texture | ResourceFlags::Tv,
                        )
                        .dim(Dimension::D2)
                        .width(width as u32)
                        .height(height as u32)
                        .depth(1)
                        .format(Format::UnormR8G8B8A8)
                        .aa(AntiAliasing::X1)
                        .build(),
                )
                .build(),
        );

        Self::depth_buffer().insert(
            DepthBuffer::builder()
                .surface(
                    Surface::builder()
                        .flags(ResourceFlags::DepthBuffer | ResourceFlags::Texture)
                        .dim(Dimension::D2)
                        .width(width as u32)
                        .height(height as u32)
                        .depth(1)
                        .format(Format::UnormR8G8B8A8)
                        .aa(AntiAliasing::X1)
                        .build(),
                )
                .build(),
        );

        Self::frame_buffer().insert({
            let mut _unused = 0;
            let mut size = 0;

            unsafe {
                drc_framebuffer_size(
                    Self::mode(),
                    Format::UnormR8G8B8A8,
                    Buffering::Double,
                    &mut size,
                    &mut _unused,
                );
            }

            FrameBuffer::new(size as usize)
        });

        0
    }

    unsafe extern "C" fn release(_context: *mut c_void) -> u32 {
        let _ = Self::color_buffer().take();
        let _ = Self::depth_buffer().take();
        let _ = Self::frame_buffer().take();

        0
    }

    pub(crate) fn color_buffer() -> &'static mut Option<ColorBuffer> {
        static BUFFER: SyncUnsafeCell<Option<ColorBuffer>> = SyncUnsafeCell(UnsafeCell::new(None));

        unsafe { &mut (*BUFFER.0.get()) }
    }

    pub(crate) fn depth_buffer() -> &'static mut Option<DepthBuffer> {
        static BUFFER: SyncUnsafeCell<Option<DepthBuffer>> = SyncUnsafeCell(UnsafeCell::new(None));

        unsafe { &mut (*BUFFER.0.get()) }
    }

    pub(crate) fn frame_buffer() -> &'static mut Option<FrameBuffer> {
        static BUFFER: SyncUnsafeCell<Option<FrameBuffer>> = SyncUnsafeCell(UnsafeCell::new(None));

        unsafe { &mut (*BUFFER.0.get()) }
    }
}
