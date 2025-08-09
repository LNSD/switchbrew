//! # nx-alloc
#![no_std]

#[cfg(feature = "ffi")]
mod ffi;

pub mod global;
pub mod llffalloc;
mod sync;
