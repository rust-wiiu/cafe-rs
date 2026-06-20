use crate::prelude::*;
use std::time::SystemTime;
use sys::gx2;

#[inline]
pub fn vsync() {
    unsafe {
        gx2::display::wait_for_vsync();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SwapStatus {
    /// Number of times a buffer swap was requested
    pub swaps: u32,
    /// Number of times a buffer swap was done
    pub flips: u32,
    /// Timestamp of last buffer swap
    pub last_flip: SystemTime,
    /// Timestamp of last vsync period
    pub last_vsync: SystemTime,
}

#[inline]
pub fn swap_status() -> SwapStatus {
    let mut status = SwapStatus {
        swaps: 0,
        flips: 0,
        last_flip: SystemTime(0),
        last_vsync: SystemTime(0),
    };

    unsafe {
        gx2::display::swap_status(
            &mut status.swaps,
            &mut status.flips,
            &mut status.last_flip.0,
            &mut status.last_vsync.0,
        );
    }

    status
}
