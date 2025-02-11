//! Condvar

use nx_svc::sync::{signal_process_wide_key, wait_process_wide_key_atomic};
use nx_thread::raw::Handle;

use crate::mutex::{__nx_sync_mutex_lock, Mutex};

/// Result type
type Result = u32;

/// A condition variable
#[repr(C)]
pub struct Condvar(u32);

/// Initializes a condition variable.
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sync_condvar_init(condvar: *mut Condvar) {
    unsafe {
        *condvar = Condvar(0);
    }
}

/// Waits on a condition variable with a timeout.
// TODO: Documentations
//  * @param[in] c Condition variable object.
//  * @param[in] m Mutex object to use inside the condition variable.
//  * @param[in] timeout Timeout in nanoseconds.
//  * @return Result code (0xEA01 on timeout).
//  * @remark On function return, the underlying mutex is acquired.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sync_condvar_wait_timeout(
    condvar: *mut Condvar,
    mutex: *mut Mutex,
    timeout: u64,
) -> Result {
    let curr_thread_handle = get_curr_thread_handle();

    let result = unsafe {
        wait_process_wide_key_atomic(
            condvar as *mut u32,
            mutex as *mut u32,
            curr_thread_handle,
            timeout,
        )
    };

    // On timeout, we need to acquire it manually.
    //
    // Masks out unused bits in a result code, retrieving the actual value for use in comparisons.
    // #define R_VALUE(res)       ((res)&0x3FFFFF)
    // if (R_VALUE(rc) == 0xEA01)
    if matches!(result, Err(err) if err & 0x3FFFFF == 0xEA01) {
        unsafe { __nx_sync_mutex_lock(mutex as *mut u32) };
    }

    match result {
        Ok(()) => 0,
        Err(rc) => rc,
    }
}

/// Waits on a condition variable.
// TODO: Documentations
//  * @param[in] c Condition variable object.
//  * @param[in] m Mutex object to use inside the condition variable.
//  * @return Result code.
//  * @remark On function return, the underlying mutex is acquired.
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sync_condvar_wait(
    condvar: *mut Condvar,
    mutex: *mut Mutex,
) -> Result {
    unsafe { __nx_sync_condvar_wait_timeout(condvar, mutex, u64::MAX) }
}

/// Wakes up to the specified number of threads waiting on a condition variable.
// TODO: Documentations
//  * @param[in] c Condition variable object.
//  * @param[in] num Maximum number of threads to wake up (or -1 to wake them all up).
//  * @return Result code.
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sync_condvar_wake(condvar: *mut Condvar, num: i32) -> Result {
    unsafe { signal_process_wide_key(condvar as *mut u32, num) };
    0
}

/// Wakes up a single thread waiting on a condition variable.
// TODO: Documentations
//  * @param[in] c Condition variable object.
//  * @return Result code.
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sync_condvar_wake_one(condvar: *mut Condvar) -> Result {
    unsafe { __nx_sync_condvar_wake(condvar, 1) }
}

/// Wakes up all thread waiting on a condition variable.
// TODO: Documentations
//  * @param[in] c Condition variable object.
//  * @return Result code.
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sync_condvar_wake_all(condvar: *mut Condvar) -> Result {
    unsafe { __nx_sync_condvar_wake(condvar, -1) }
}

/// Get the current thread's kernel handle.
#[inline(always)]
fn get_curr_thread_handle() -> Handle {
    unsafe { nx_thread::raw::__nx_thread_get_current_thread_handle() }
}
