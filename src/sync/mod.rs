//! Syncronization Primitives

pub mod mutex;

use crate::sys::coreinit::{interrupts, scheduler, thread};

/// Locks the scheduler to the current thread and disables interrupts.
pub fn critical_section<F: FnOnce()>(f: F) {
    let (thread, state) = unsafe {
        let thread = thread::current();
        let state = interrupts::disable();
        scheduler::lock(thread.cast());
        (thread, state)
    };

    f();

    unsafe {
        scheduler::unlock(thread.cast());
        interrupts::restore(state);
    }
}
