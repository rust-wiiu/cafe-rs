//! std
//!
//! A subset of all cafe-rs features which follow the Rust std APIs. It is highly recommended to use as many symbols from the `cafe_rs::std::` module as possible and only use `cafe_rs::` symbols when they are not present in std.

pub use core::any;
pub use core::array;
pub use core::cell;
pub use core::char;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::future;
pub use core::hint;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::num;
pub use core::ops;
pub use core::option;
pub use core::pin;
pub use core::ptr;

pub use alloc_crate::alloc;
pub use alloc_crate::borrow;
pub use alloc_crate::boxed;
pub use alloc_crate::format;
pub use alloc_crate::slice;
pub use alloc_crate::str;
pub use alloc_crate::string;
pub use alloc_crate::vec;

pub mod ffi {
    pub use alloc_crate::ffi::c_str::*;
    pub use core::ffi::*;
}

pub mod sync {
    pub use crate::sync::mutex::Mutex;
    pub use alloc_crate::sync::*;
    pub use core::sync::*;
}

pub mod time {
    pub use crate::time::SystemTime;
    pub use core::time::*;
}

pub mod fmt {
    pub use alloc_crate::fmt::*;
    pub use core::fmt::*;
}

pub mod net {
    pub use core::net::*;
}

pub mod thread {
    pub use crate::thread::*;
}
