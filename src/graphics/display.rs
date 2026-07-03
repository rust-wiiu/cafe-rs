use cafe_sys::gx2;

use crate::prelude::*;
use std::marker::PhantomData;

use super::buffer::ScanBuffer;

pub trait RenderTarget {
    type Mode;
    const SCAN_FORMAT: gx2::mem::Format;
    const BUFFERING: gx2::display::Buffering;
    const AA: gx2::mem::AntiAliasing;
    const SCAN_TARGET: gx2::display::ScanTarget;

    fn size() -> (usize, usize);
    fn mode() -> Self::Mode;
}

pub struct TV;
impl RenderTarget for TV {
    type Mode = gx2::display::TvMode;
    const SCAN_FORMAT: gx2::mem::Format = gx2::mem::Format::UnormR8G8B8A8;

    const BUFFERING: gx2::display::Buffering = gx2::display::Buffering::Double;
    const AA: gx2::mem::AntiAliasing = gx2::mem::AntiAliasing::X1;
    const SCAN_TARGET: gx2::display::ScanTarget = gx2::display::ScanTarget::Tv;

    fn size() -> (usize, usize) {
        use gx2::display::{AspectRatio, ScanMode};
        match unsafe { gx2::display::scan_mode() } {
            ScanMode::NTSC | ScanMode::NTSCp => match unsafe { gx2::display::aspect_ratio() } {
                AspectRatio::Standard => (640, 480),
                AspectRatio::Widescreen => (854, 480),
            },
            ScanMode::PAL | ScanMode::HD => (1280, 720),
            ScanMode::FHD | ScanMode::FHDi => (1920, 1080),
        }
    }

    #[inline]
    fn mode() -> Self::Mode {
        use gx2::display::{AspectRatio, ScanMode, TvMode};
        match unsafe { gx2::display::scan_mode() } {
            ScanMode::NTSC | ScanMode::NTSCp => match unsafe { gx2::display::aspect_ratio() } {
                AspectRatio::Standard => TvMode::Standard480,
                AspectRatio::Widescreen => TvMode::Wide480,
            },
            ScanMode::PAL | ScanMode::HD => TvMode::Wide720,
            ScanMode::FHD | ScanMode::FHDi => TvMode::Wide1080,
        }
    }
}

pub struct DRC;
impl RenderTarget for DRC {
    type Mode = gx2::display::DrcMode;
    const SCAN_FORMAT: gx2::mem::Format = gx2::mem::Format::UnormR8G8B8A8;

    const BUFFERING: gx2::display::Buffering = gx2::display::Buffering::Double;
    const AA: gx2::mem::AntiAliasing = gx2::mem::AntiAliasing::X1;
    const SCAN_TARGET: gx2::display::ScanTarget = gx2::display::ScanTarget::Drc;

    #[inline]
    fn size() -> (usize, usize) {
        (854, 480)
    }

    #[inline]
    fn mode() -> Self::Mode {
        unsafe { gx2::display::drc_mode() }
    }
}

pub struct Display<T: RenderTarget> {
    pub scan: ScanBuffer,
    _marker: PhantomData<T>,
}

impl Display<TV> {
    pub fn tv() -> Self {
        let mut size = 0;
        let mut scale_needed = 0;
        unsafe {
            gx2::display::tv_framebuffer_size(
                TV::mode(),
                TV::SCAN_FORMAT,
                TV::BUFFERING,
                &mut size,
                &mut scale_needed,
            );
        };
        assert_ne!(size, 0);

        unsafe {
            gx2::display::enable_tv(true);
        }

        let scan = ScanBuffer::with_capacity(size as usize);

        unsafe {
            gx2::display::set_tv_buffer(
                scan.as_raw().ptr,
                size,
                TV::mode(),
                TV::SCAN_FORMAT,
                TV::BUFFERING,
            );
        }

        Self {
            scan,
            _marker: PhantomData,
        }
    }
}

impl Display<DRC> {
    pub fn drc() -> Self {
        let mut size = 0;
        let mut scale_needed = 0;
        unsafe {
            gx2::display::drc_framebuffer_size(
                DRC::mode(),
                DRC::SCAN_FORMAT,
                DRC::BUFFERING,
                &mut size,
                &mut scale_needed,
            );
        };
        assert_ne!(size, 0);

        unsafe {
            gx2::display::enable_drc(true);
        }

        let scan = ScanBuffer::with_capacity(size as usize);

        unsafe {
            gx2::display::set_drc_buffer(
                scan.as_raw().ptr,
                size,
                DRC::mode(),
                DRC::SCAN_FORMAT,
                DRC::BUFFERING,
            );
        }

        Self {
            scan,
            _marker: PhantomData,
        }
    }
}

#[inline]
pub fn request_swap() {
    unsafe {
        gx2::display::swap_scan_buffers();
        gx2::display::draw_done();
    }
}
