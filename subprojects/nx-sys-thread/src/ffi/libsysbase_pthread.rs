//! FFI bindings for pthread.c from libsysbase.
//!
//! Provides pthread-related function implementations.

use core::{
    ffi::{c_int, c_void},
    ptr,
};

use crate::thread_impl as sys;

/// POSIX constant for `ENOSYS` (function not implemented).
/// Standard POSIX error code (38).
const ENOSYS: c_int = 38;

//-----------------------------------------------------------------------------
// Thread-specific keys (TLS)
//-----------------------------------------------------------------------------

/// Rust implementation of `pthread_setspecific` that libgloss/newlib expects.
///
/// TODO: Add support for pthread TLS functions
/// Currently returns ENOSYS as dynamic slots are not implemented.
/// Corresponds to libgloss/libsysbase/pthread.c:559
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_pthread_setspecific(
    _key: u32,
    _value: *const c_void,
) -> c_int {
    // Dynamic TLS slots not supported - return ENOSYS (function not implemented)
    ENOSYS
}

/// Rust implementation of `pthread_getspecific` that libgloss/newlib expects.
///
/// TODO: Add support for pthread TLS functions
/// Currently returns NULL as dynamic slots are not implemented.
/// Corresponds to libgloss/libsysbase/pthread.c:567
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_pthread_getspecific(_key: u32) -> *mut c_void {
    // Dynamic TLS slots not supported - always return NULL
    ptr::null_mut()
}

//-----------------------------------------------------------------------------
// sched.h
//-----------------------------------------------------------------------------

/// Overrides the `sched_yield` function from the C standard library.
///
/// This function is declared in `<sched.h>`.
/// Corresponds to libgloss/libsysbase/pthread.c:1072
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_sched_yield() -> c_int {
    sys::yield_with_migration();
    0
}

/// Overrides the `sched_getcpu` function from the C standard library.
///
/// This function is declared in `<sched.h>`.
/// Corresponds to libgloss/libsysbase/pthread.c:1079
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_sched_getcpu() -> c_int {
    sys::get_current_cpu() as c_int
}
