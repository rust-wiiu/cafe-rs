//! Prelude
//!
//! # Example
//!
//! ```no_run
//! use cafe_rs::prelude::*;
//!
//! use std::sync::Mutex;
//! use sys::coreinit;
//! use cafe::net::Socket;
//! ```

pub use crate::std;

#[cfg(feature = "alloc")]
pub use alloc_crate::format;

pub use cafe_sys as sys;
pub use sys::UnsafeInit;

pub use crate as cafe;

pub use std::{boxed::Box, vec, vec::Vec};
