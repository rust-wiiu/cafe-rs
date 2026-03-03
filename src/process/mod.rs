//! Process

use crate::prelude::*;

use std::{
    ffi::c_void,
    marker::PhantomData,
    ptr,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};
use sys::{coreinit, proc_ui};

static MAIN_CORE: AtomicU32 = AtomicU32::new(0);
static RUNNING: AtomicBool = AtomicBool::new(false);
// static HOMEBREW_LAUNCHER: AtomicBool = AtomicBool::new(false);

// PhantomData to impl !Send
pub struct Process(PhantomData<*const ()>);

impl Process {
    pub fn new() -> Self {
        if RUNNING.swap(true, Ordering::Relaxed) || unsafe { proc_ui::is_running() } != 0 {
            panic!("Process::new can only be called once.")
        }

        MAIN_CORE.store(cafe::thread::core().into(), Ordering::Relaxed);

        unsafe extern "C" fn save_callback(_context: *mut c_void) -> u32 {
            unsafe {
                coreinit::foreground::ready_to_release();
            }

            0
        }

        unsafe {
            proc_ui::init_ex(save_callback, ptr::null_mut());
        }

        Self(PhantomData)
    }

    pub fn running(&self) -> bool {
        let msg = unsafe { proc_ui::process_messages(1) };

        match msg {
            proc_ui::Status::Exit => RUNNING.store(false, Ordering::Release),
            proc_ui::Status::Releasing => unsafe { proc_ui::drawing_done() },
            _ => (),
        }

        RUNNING.load(Ordering::Acquire)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            proc_ui::shutdown();
        }
        RUNNING.store(false, Ordering::Relaxed);
    }
}

/// Can be used to check in arbitrary threads if the main thread / process is still running.
///
/// This does not handle the system calls to release the foreground. [Process::running] must be called within the main thread for ProcUI to work.
#[inline]
pub fn running() -> bool {
    RUNNING.load(Ordering::Acquire)
}

/// Returns the core the main thread is running on. This is set when [Process::new] is called.
#[inline]
pub fn main_core() -> u32 {
    MAIN_CORE.load(Ordering::Relaxed)
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

#[inline]
pub fn title_id() -> TitleID {
    TitleID(unsafe { coreinit::system::title_id() })
}
