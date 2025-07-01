use core::{ffi::c_void, ptr};

use crate::slots;

/// Reads the raw pointer stored in the dynamic TLS slot `slot_id`.
///
/// Mirrors `threadTlsGet` in libnx's C API.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread_tls_get(slot_id: i32) -> *mut c_void {
    if slot_id < 0 {
        return ptr::null_mut();
    }

    match unsafe { slots::curr_thread_slot_get(slot_id as usize) } {
        Ok(value) => value,
        Err(_) => ptr::null_mut(),
    }
}

/// Writes `value` into dynamic TLS slot `slot_id`.
///
/// Mirrors `threadTlsSet` in libnx's C API.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread_tls_set(slot_id: i32, value: *mut c_void) {
    if slot_id < 0 {
        return; // Silently ignore invalid negative indices to match common C semantics.
    }

    let _ = unsafe { slots::curr_thread_slot_set(slot_id as usize, value) };
}

mod newlib {
    use core::{ffi::c_void, ptr};

    use crate::slots;

    /// POSIX constant for `EINVAL` (invalid argument).
    /// Mirrors the value used by newlib on Horizon (22).
    const EINVAL: i32 = 22;

    /// Rust implementation of `pthread_setspecific` that libgloss/newlib expects.
    ///
    /// # Safety
    /// The caller must ensure that `key` was previously obtained via a successful call to
    /// `pthread_key_create` (or equivalent) and therefore lies within the dynamic TLS slot
    /// range `[0, NUM_TLS_SLOTS)`.
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __nx_sys_thread_newlib_pthread_setspecific(
        key: u32,
        value: *const c_void,
    ) -> i32 {
        match unsafe { slots::curr_thread_slot_set(key as usize, value as *mut c_void) } {
            Ok(_) => 0,
            Err(_) => EINVAL,
        }
    }

    /// Rust implementation of `pthread_getspecific` that libgloss/newlib expects.
    ///
    /// Returns the stored pointer or NULL if `key` is out of range.
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __nx_sys_thread_newlib_pthread_getspecific(key: u32) -> *mut c_void {
        match unsafe { slots::curr_thread_slot_get(key as usize) } {
            Ok(value) => value,
            Err(_) => ptr::null_mut(),
        }
    }
}
