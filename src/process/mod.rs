//! Process

use crate::prelude::*;

use std::ffi::c_void;
use sys::{coreinit, proc_ui};

/// Calling this function multiple times is UB.
pub fn init<F: FnMut() -> Result<(), ()> + 'static>(mut callback: F) {
    unsafe extern "C" fn trampoline<F: FnMut() -> Result<(), ()>>(f: *mut c_void) -> u32 {
        let callback = unsafe { &mut *(f as *mut F) };

        match callback() {
            Ok(_) => {
                unsafe {
                    coreinit::foreground::ready_to_release();
                }
                0
            }
            Err(_) => 1,
        }
    }

    unsafe {
        proc_ui::init_ex(trampoline::<F>, &mut callback as *mut F as *mut _);
    }
}

pub fn deinit() {
    unsafe {
        proc_ui::shutdown();
    }
}

pub use proc_ui::Status;

pub fn handle_system_messages() -> Status {
    unsafe { proc_ui::process_messages(1) }
}

/// Every application has an associated title ID.
///
/// Known title IDs are listed [here](https://wiiubrew.org/wiki/Title_database).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TitleID(u64);

impl TitleID {
    pub const MII_MAKER_JPN: Self = Self(0x00050010_1004A000);
    pub const MII_MAKER_USA: Self = Self(0x00050010_1004A100);
    pub const MII_MAKER_EUR: Self = Self(0x00050010_1004A200);

    #[inline]
    pub fn get() -> Self {
        Self(unsafe { coreinit::system::title_id() })
    }
}

impl From<u64> for TitleID {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Into<u64> for TitleID {
    fn into(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for TitleID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08X}-{:08X}", self.0 >> 32, self.0 & 0xFFFFFFFF)
    }
}
