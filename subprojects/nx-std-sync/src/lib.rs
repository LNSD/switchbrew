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

/// #[panic_handler]
///
/// Use different panic handlers for debug and release builds.
/// - 'dev': halt on panic. Easier to debug panics; can put a breakpoint on `rust_begin_unwind`
/// - 'release': abort on panic. Minimal binary size.
///
/// See:
///  - <https://doc.rust-lang.org/nomicon/panic-handler.html>
///  - <https://docs.rust-embedded.org/book/start/panicking.html>
#[cfg(not(debug_assertions))]
#[allow(unused_imports)]
use panic_abort as _;
#[cfg(debug_assertions)]
#[allow(unused_imports)]
use panic_halt as _;
