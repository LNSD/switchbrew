//! FFI bindings for sleep.c from libsysbase.
//!
//! Provides the `sleep` function that calls nanosleep internally.

use core::{ffi::c_uint, time::Duration};

use crate::thread_sleep;

/// Overrides the `sleep` function from the C standard library.
///
/// This function is declared in `<unistd.h>`.
/// Corresponds to libgloss/libsysbase/sleep.c
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_sleep(seconds: c_uint) -> c_uint {
    thread_sleep::sleep(Duration::from_secs(seconds as u64));
    0
}
