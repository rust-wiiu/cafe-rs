//! Resource Reference Counting
//!
//! Save handling of resources which need (de)initialization.

use crate::std::sync::atomic::{AtomicUsize, Ordering};
use crate::sync::critical_section;

/// Resource Reference Counter
#[derive(Debug)]
pub struct Rrc {
    counter: AtomicUsize,
    init: fn(),
    deinit: fn(),
}

impl Rrc {
    pub const fn new(init: fn(), deinit: fn()) -> Self {
        Self {
            counter: AtomicUsize::new(0),
            init,
            deinit,
        }
    }

    pub fn acquire(&'static self) -> Resource {
        let prev = self.counter.fetch_add(1, Ordering::AcqRel);

        if prev == 0 {
            critical_section(|| {
                if self.counter.load(Ordering::Acquire) == 1 {
                    (self.init)();
                }
            });
        }

        Resource(self)
    }

    fn release(&'static self) {
        let prev = self.counter.fetch_sub(1, Ordering::AcqRel);

        if prev == 1 {
            critical_section(|| {
                if self.counter.load(Ordering::Acquire) == 0 {
                    (self.deinit)();
                }
            });
        }
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
