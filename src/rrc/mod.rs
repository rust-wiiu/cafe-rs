//! Resource Reference Counting
//!
//! Save handling of resources which need (de)initialization.

use crate::prelude::*;

use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

/// A reference counter that calls init/deinit hooks on 0->1 and 1->0 transitions.
#[derive(Debug)]
pub struct Rrc {
    /// Guards access to `counter` and the execution of init/deinit
    lock: AtomicBool,
    /// The reference count. Wrapped in UnsafeCell because we mutate it
    /// behind a shared reference (&self) under lock protection.
    counter: UnsafeCell<usize>,
    /// Called when counter transitions from 0 to 1
    init: fn(),
    /// Called when counter transitions from 1 to 0
    deinit: fn(),
}

// SAFETY: Rrc handles its own synchronization via the spinlock.
//
// All access to `counter` is protected by the lock with proper Acquire/Release ordering.
unsafe impl Sync for Rrc {}

impl Rrc {
    /// Creates a new resource reference counter.
    ///
    /// # Arguments
    /// * `init` - Function called when the first reference is acquired
    /// * `deinit` - Function called when the last reference is released
    ///
    /// The provided functions should be as simple and short as possible as well as have a static lifetime.
    pub const fn new(init: fn(), deinit: fn()) -> Self {
        Self {
            lock: AtomicBool::new(false),
            counter: UnsafeCell::new(0),
            init,
            deinit,
        }
    }

    /// Acquires a reference to the resource.
    ///
    /// If this is the first reference (counter was 0), calls the `init` function before incrementing the counter.
    pub fn acquire(&'static self) -> Resource {
        self.spin_lock();

        // SAFETY: We hold the lock, ensuring exclusive access to counter.
        unsafe {
            let count = &mut *self.counter.get();

            if *count == 0 {
                (self.init)();
            }

            *count = count.checked_add(1).expect("reference count overflow");
        }

        self.unlock();

        Resource(self)
    }

    /// Releases a reference to the resource.
    ///
    /// If this was the last reference (counter becomes 0), calls the `deinit` function after decrementing the counter.
    fn release(&'static self) {
        self.spin_lock();

        // SAFETY: We hold the lock, ensuring exclusive access to counter.
        unsafe {
            let count = &mut *self.counter.get();

            *count = count.checked_sub(1).expect("reference count underflow");

            if *count == 0 {
                (self.deinit)();
            }
        }

        self.unlock();
    }

    /// Acquires the spinlock using a test-and-test-and-set pattern.
    ///
    /// This reduces cache line contention by spinning on a read-only operation before attempting the compare_exchange.
    #[inline]
    fn spin_lock(&self) {
        // Fast path: try to acquire immediately
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            return;
        }

        // Slow path: spin with backoff
        loop {
            // Spin on reads to avoid cache line bouncing
            while self.lock.load(Ordering::Relaxed) {
                core::hint::spin_loop();
            }

            // Try to acquire
            if self
                .lock
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return;
            }
        }
    }

    /// Releases the spinlock.
    #[inline]
    fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}

#[derive(Debug)]
pub struct Resource(&'static Rrc);

impl Clone for Resource {
    fn clone(&self) -> Self {
        self.0.acquire()
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        self.0.release();
    }
}
