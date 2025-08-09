//! FFI bindings for nanosleep.c from libsysbase.
//!
//! Provides the `nanosleep` function implementation.

use core::{
    ffi::{c_int, c_long},
    ptr,
    time::Duration,
};

use crate::thread_sleep;

// Error codes
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;

#[repr(C)]
#[derive(Default, Copy, Clone)]
struct TimeSpec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

/// Overrides the `nanosleep` function from the C standard library.
///
/// This function is declared in `<time.h>`.
/// Corresponds to libgloss/libsysbase/nanosleep.c
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_nanosleep(
    req: *const TimeSpec,
    rem: *mut TimeSpec,
) -> c_int {
    if req.is_null() {
        set_errno(EFAULT);
        return -1;
    }

    let request = unsafe { &*req };
    if request.tv_sec < 0 || request.tv_nsec < 0 || request.tv_nsec >= 1_000_000_000 {
        set_errno(EINVAL);
        return -1;
    }

    let duration = Duration::new(request.tv_sec as u64, request.tv_nsec as u32);
    thread_sleep::sleep(duration);

    if !rem.is_null() {
        unsafe { ptr::write(rem, TimeSpec::default()) };
    }

    0
}

/// Sets the thread-local `errno` value
#[inline]
fn set_errno(code: c_int) {
    unsafe extern "C" {
        // This is a newlib/libc function
        fn __errno() -> *mut c_int;
    }

    unsafe { *__errno() = code };
}
