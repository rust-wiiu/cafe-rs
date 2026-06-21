use crate::{alloc::Allocator, prelude::*};
use cafe::alloc::{FG, MEM1, MEM2};
use std::{alloc::Layout, ffi::c_void, ptr};
use sys::gx2;

pub mod buffer;
pub mod context;
pub mod display;
pub mod pipeline;
pub mod shader;
pub mod surface;
pub mod sync;
pub mod target;
pub mod types;

pub use gfx2::Gfx2;

pub use buffer::{Buffer, ScanBuffer, VertexBuffer};
pub use context::Context;
pub use display::{DRC, Display, TV};
pub use shader::{Attribute, ShaderGroup};
pub use target::Target;

pub fn init() {
    use gx2::surface::ResourceFlags as Flags;

    unsafe extern "C" fn alloc(flags: Flags, size: u32, align: u32) -> *mut c_void {
        if flags.contains(Flags::ScanBuffer) && !flags.contains(Flags::ForceMem1 | Flags::ForceMem2)
        {
            let layout = Layout::from_size_align(size as usize, 0x1000).unwrap();
            log::debug!("GX2 Allocate FG: {:?} {:?}", &layout, flags);
            FG.allocate(layout).unwrap().as_ptr().cast()
        } else if flags.intersects(Flags::ColorBuffer | Flags::DepthBuffer | Flags::ForceMem1)
            && !flags.contains(Flags::ForceMem2)
        {
            let layout = Layout::from_size_align(size as usize, align as usize).unwrap();
            log::debug!("GX2 Allocate MEM1: {:?} {:?}", &layout, flags);
            MEM1.allocate(layout).unwrap().as_ptr().cast()
        } else {
            let layout = Layout::from_size_align(size as usize, align as usize).unwrap();
            log::debug!("GX2 Allocate MEM2: {:?} {:?}", &layout, flags);
            MEM2.allocate(layout).unwrap().as_ptr().cast()
        }
    }

    unsafe extern "C" fn free(flags: Flags, ptr: *mut c_void) {
        let ptr = match ptr::NonNull::new(ptr) {
            Some(ptr) => ptr.cast(),
            None => return,
        };

        if flags.contains(Flags::ScanBuffer) && !flags.contains(Flags::ForceMem1 | Flags::ForceMem2)
        {
            log::debug!("GX2 Deallocate FG: {:?}", flags);
            unsafe {
                FG.deallocate(ptr);
            }
        } else if flags.intersects(Flags::ColorBuffer | Flags::DepthBuffer | Flags::ForceMem1)
            & !flags.contains(Flags::ForceMem2)
        {
            log::debug!("GX2 Deallocate MEM1: {:?}", flags);
            unsafe {
                MEM1.deallocate(ptr);
            }
        } else {
            log::debug!("GX2 Deallocate MEM2: {:?}", flags);
            unsafe {
                MEM2.deallocate(ptr);
            }
        }
    }

    unsafe {
        gx2::state::init(ptr::null_mut());

        gx2::mem::set_allocator(Some(alloc), Some(free));

        gx2::display::set_swap_interval(gx2::display::SwapInterval::VSync60Hz);
    }
}

pub fn deinit() {
    unsafe { gx2::state::deinit() }
}
