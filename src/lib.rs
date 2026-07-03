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
pub mod font;
pub mod gamepad;
pub mod graphics;
pub mod io;
pub mod mem;
pub mod net;
pub mod process;
pub mod sync;
pub mod thread;
pub mod time;
