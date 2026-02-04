//! cafe-rs

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc as alloc_crate;

#[cfg(feature = "rt")]
pub use cafe_rt as rt;

pub use cafe_sys as sys;

#[cfg(feature = "alloc")]
pub mod allocator;
pub mod prelude;
pub mod std;

pub mod alloc;
pub mod controller;
pub mod datetime;
pub mod graphics;
pub mod mem;
pub mod net;
pub mod process;
pub mod rrc;
pub mod sync;
pub mod thread;
pub mod time;

// ==== ONLY DURING DEV STAGES ====

#[derive(Debug, Clone)]
pub enum Error {
    Any(&'static str),
    Integer(i32),
}

pub type Result<T> = core::result::Result<T, Error>;
