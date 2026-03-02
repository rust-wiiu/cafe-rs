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

pub use crate as cafe;
pub use crate::std;
pub use cafe_sys as sys;

pub use alloc_crate::format;
pub use std::{boxed::Box, fmt::Debug, vec, vec::Vec};
pub use sys::UnsafeInit;
