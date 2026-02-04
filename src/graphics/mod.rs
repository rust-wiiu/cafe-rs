pub mod buffer;
pub mod display;
pub mod gfd;
pub mod mem;
pub mod pipeline;

use crate::prelude::*;
use std::ptr;
use sys::gx2;

pub fn init() {
    unsafe {
        gx2::state::init(ptr::null_mut());
        gx2::display::set_swap_interval(gx2::display::SwapInterval::VSync60Hz);
    }
}
