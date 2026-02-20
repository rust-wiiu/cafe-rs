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

            log::info!("save_callback");

            0
        }

        unsafe {
            proc_ui::init_ex(save_callback, ptr::null_mut());
        }

        Self(PhantomData)
    }

    pub fn running(&self) -> bool {
        if MAIN_CORE.load(Ordering::Relaxed) != cafe::thread::core().into() {
            unsafe {
                proc_ui::sub_process_messages(1);
            }
            return RUNNING.load(Ordering::Acquire);
        }

        let msg = unsafe { proc_ui::process_messages(1) };

        log::info!("{:?}", msg);

        match msg {
            proc_ui::Status::Exit => {
                RUNNING.store(false, Ordering::Release);
            }
            proc_ui::Status::Releasing => unsafe { proc_ui::drawing_done() },
            _ => (),
        }

        let running = RUNNING.load(Ordering::Acquire);

        if !running {
            unsafe {
                proc_ui::shutdown();
            }
        }

        log::info!("{:?}", running);

        return running;
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            proc_ui::shutdown();
        }
    }
}

pub fn running() -> bool {
    todo!()
}

pub fn abort() {
    todo!()
}

pub fn exit() -> ! {
    todo!()
}

#[inline]
pub fn main_core() -> u32 {
    MAIN_CORE.load(Ordering::Relaxed)
}
