//! # Virtual Memory Management

#[cfg(feature = "ffi")]
mod ffi;

mod sys;

pub use sys::*;
