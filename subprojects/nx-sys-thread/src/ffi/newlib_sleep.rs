//! FFI bindings for sleep.c from libsysbase.
//!
//! Provides the `sleep` function that calls nanosleep internally.

use core::{ffi::c_uint, time::Duration};

use crate::thread_impl as sys;

/// Overrides the `sleep` function from the C standard library.
///
/// This function is declared in `<unistd.h>`.
/// Corresponds to libgloss/libsysbase/sleep.c
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread_newlib_sleep(seconds: c_uint) -> c_uint {
    sys::sleep(Duration::from_secs(seconds as u64));
    0
}
