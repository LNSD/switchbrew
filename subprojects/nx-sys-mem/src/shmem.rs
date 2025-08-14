//! Shared memory management

#[cfg(feature = "ffi")]
mod ffi;

mod sys;

pub use sys::*;
