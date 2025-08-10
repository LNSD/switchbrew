//! FFI bindings for usleep.c from libsysbase.
//!
//! Provides the `usleep` function that calls nanosleep internally.

use core::{
    ffi::{c_int, c_uint},
    time::Duration,
};

use crate::thread_impl as sys;

/// Overrides the `usleep` function from the C standard library.
///
/// This function is declared in `<unistd.h>`.
/// Corresponds to libgloss/libsysbase/usleep.c
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread_newlib_usleep(useconds: c_uint) -> c_int {
    sys::sleep(Duration::from_micros(useconds as u64));
    0
}
