use crate::std::{
    ffi::c_void,
    ptr,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};
use crate::sys::{coreinit, proc_ui};

static MAIN_CORE: AtomicU32 = AtomicU32::new(0);
static RUNNING: AtomicBool = AtomicBool::new(false);

pub fn init() {
    MAIN_CORE.store(unsafe { coreinit::system::core_id() }, Ordering::Relaxed);
    RUNNING.store(true, Ordering::Relaxed);

    unsafe extern "C" fn save_callback(_context: *mut c_void) -> u32 {
        unsafe {
            proc_ui::ready_to_release();
        }
        0
    }

    unsafe {
        proc_ui::init_ex(save_callback, ptr::null_mut());
    }
}

pub fn deinit() {
    unsafe {
        proc_ui::shutdown();
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

// pub fn id() -> u64 {
//     todo!()
// }
