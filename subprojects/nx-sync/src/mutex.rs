//! # Mutex

use core::{arch::asm, ptr};

use nx_svc::{
    debug::break_event,
    raw::{BreakReason, INVALID_HANDLE},
    sync::{arbitrate_lock, arbitrate_unlock},
};

// TODO: Investigate this mask
const HANDLE_WAIT_MASK: u32 = 0x40000000;

/// Get [Mutex] tag.
///
/// The mutex tag corresponds to the current thread's kernel handle.
#[inline(always)]
fn get_tag() -> u32 {
    unsafe { nx_thread::raw::__nx_thread_get_current_thread_handle() }
}

/// Load-Exclusive (LDAXR) 32-bit value from the given pointer
///
/// ## References
/// - [ARM aarch64: LDAXR](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDAXR--Load-acquire-exclusive-register-?lang=en)
#[inline(always)]
fn load_exclusive(ptr: *const u32) -> u32 {
    let value: u32;
    unsafe {
        asm!(
        "ldaxr {val:w}, [{ptr:x}]", // Loads the 32-bit value from the memory location pointed to by ptr
        ptr = in(reg) ptr,          // Input: ptr to load from
        val = out(reg) value,       // Output: Capture thr result in value (via a register)
        options(nostack, preserves_flags)
        );
    }
    value
}

/// Store-Exclusive (STLXR) 32-bit value to the given pointer
///
/// ## References
/// - [ARM aarch64: STLXR](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/STLXR--Store-release-exclusive-register-?lang=en)
#[inline(always)]
fn store_exclusive(ptr: *mut u32, val: u32) -> Result<(), ()> {
    let mut res: u32;
    unsafe {
        asm!(
        "stlxr {res:w}, {val:w}, [{ptr:x}]", // Stores the 32-bit value to the memory location pointed to by ptr
        val = in(reg) val,                   // Input: Value to store
        ptr = in(reg) ptr,                   // Input: ptr to store to
        res = out(reg) res,                  // Output: Capture the result in res (via a register)
        options(nostack, preserves_flags)
        );
    }

    // If `res` is `0`, the operation updated memory, otherwise it failed
    if res == 0 { Ok(()) } else { Err(()) }
}

/// Clears the exclusive reservation using the `clrex` assembly instruction.
///
/// ## References
/// - [ARM aarch64: CLREX](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/CLREX--Clear-exclusive-?lang=en)
#[inline(always)]
fn clear_exclusive() {
    unsafe {
        asm!("clrex", options(nostack, preserves_flags));
    }
}

/// Initializes the mutex.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sync_mutex_init(mutex: *mut u32) {
    unsafe {
        *mutex = INVALID_HANDLE;
    }
}

/// Locks the mutex.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sync_mutex_lock(mutex: *mut u32) {
    // Get the current thread's handle (thread tag)
    let thread_tag = get_tag();

    let mut value = load_exclusive(mutex);
    loop {
        // If the mutex is unlocked, try to acquire it
        if value == INVALID_HANDLE {
            // Try to acquire the mutex
            match store_exclusive(mutex, thread_tag) {
                Ok(_) => break,
                Err(_) => {
                    // If failed, try again
                    value = load_exclusive(mutex);
                    continue;
                }
            }
        }

        // If the mutex doesn't have any waiters, try to register ourselves as the first waiter.
        if value & HANDLE_WAIT_MASK == 0
            && store_exclusive(mutex, value | HANDLE_WAIT_MASK).is_err()
        {
            // If failed, try again
            value = load_exclusive(mutex);
            continue;
        }

        // Ask the kernel to arbitrate the lock for us
        if arbitrate_lock(value & !HANDLE_WAIT_MASK, mutex, thread_tag).is_err() {
            // This should never happen
            break_event(BreakReason::Assert, ptr::null_mut(), 0);
        }

        // Reload the value, and check if we acquired the lock
        value = load_exclusive(mutex);
        if (value & !HANDLE_WAIT_MASK == thread_tag) {
            clear_exclusive();
            break;
        }
    }
}

/// Attempts to lock the mutex without waiting.
///
/// Returns `true` if the mutex was successfully locked, `false` otherwise.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sync_mutex_try_lock(mutex: *mut u32) -> bool {
    // Get the current thread's handle (thread tag)
    let thread_tag = get_tag();

    loop {
        // Check the mutex is not owned
        let value = load_exclusive(mutex);
        if value != INVALID_HANDLE {
            break;
        }

        if store_exclusive(mutex, thread_tag).is_ok() {
            return true;
        }
    }

    // Release our exclusive hold
    clear_exclusive();

    false
}

/// Unlocks the mutex.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sync_mutex_unlock(mutex: *mut u32) {
    // Get the current thread's handle (thread tag)
    let thread_tag = get_tag();

    let mut value = load_exclusive(mutex);
    loop {
        // If we have any listeners, we need to ask the kernel to arbitrate
        if value != thread_tag {
            clear_exclusive();
            break;
        }

        // Try to release the lock
        if store_exclusive(mutex, INVALID_HANDLE).is_ok() {
            break;
        }

        // Reload the value, and try again
        value = load_exclusive(mutex);
    }

    if (value & HANDLE_WAIT_MASK) != INVALID_HANDLE && arbitrate_unlock(mutex).is_err() {
        // This should never happen
        break_event(BreakReason::Assert, ptr::null_mut(), 0);
    }
}

/// Gets whether the mutex is locked by the current thread.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sync_mutex_is_locked_by_current_thread(mutex: *mut u32) -> bool {
    // Get the current thread's handle (thread tag)
    let thread_tag = get_tag();
    unsafe { *mutex & !HANDLE_WAIT_MASK == thread_tag }
}
