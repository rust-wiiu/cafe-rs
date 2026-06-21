//! Syncronization Primitives

pub mod mutex;

#[derive(Debug, thiserror::Error)]
pub enum LockError {
    #[error("The data is already locked elsewhere")]
    AlreadyLocked,
}

/// Locks the scheduler to the current thread and disables interrupts.
///
/// # Caution
/// Prefer to use normal primitives over this.
pub fn critical_section<F: FnOnce()>(f: F) {
    use crate::sys::coreinit::{interrupts, scheduler, thread};

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
