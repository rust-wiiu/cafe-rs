//! cafe-rs

#![no_std]

// always require alloc crate. alloc feature is for default GlobalAllocator.
extern crate alloc as alloc_crate;

#[cfg(feature = "rt")]
pub use cafe_rt as rt;

pub use cafe_sys as sys;

pub mod prelude;
pub mod std;

pub mod alloc;
pub mod datetime;
pub mod gamepad;
pub mod graphics;
pub mod io;
pub mod mem;
pub mod net;
pub mod process;
pub mod rrc;
pub mod sync;
pub mod thread;
pub mod time;

/// What to do when an allocation fails?
///
/// Use this for now to unify OOM behavior.
macro_rules! OOM {
    () => {
        panic!("Out of memory!");
    };
}
pub(crate) use OOM;

// ==== ONLY DURING DEV STAGES ====

#[derive(Debug, Clone)]
pub enum Error {
    Any(&'static str),
    Integer(i32),
}

pub type Result<T> = core::result::Result<T, Error>;
