use crate::prelude::*;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::time::{Duration, SystemTime};
use sys::coreinit;

pub fn sleep(dur: Duration) {
    let time: SystemTime = dur.into();
    unsafe {
        coreinit::thread::sleep(time.into());
    }
}

#[repr(u32)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
pub enum Core {
    C0 = 0,
    C1 = 1,
    C2 = 2,
}

/// Gets the CPU core executing the current thread.
pub fn core() -> Core {
    match unsafe { coreinit::system::core_id() } {
        0 => Core::C0,
        1 => Core::C1,
        2 => Core::C2,
        _ => unreachable!(),
    }
}
