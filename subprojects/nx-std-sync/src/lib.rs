//! # nx-std-sync
#![no_std]

// The `alloc` crate enables memory allocation.
extern crate alloc;
// The `nx-alloc` crate exposes the `#[global_allocator]` for the dependent crates.
extern crate nx_alloc;

#[cfg(feature = "ffi")]
mod ffi;

pub mod barrier;
pub mod condvar;
pub mod mutex;
pub mod once_lock;
pub mod oneshot;
mod result;
pub mod rwlock;
pub mod semaphore;
