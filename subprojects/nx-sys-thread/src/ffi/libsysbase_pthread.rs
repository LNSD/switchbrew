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
// Thread syscalls
//-----------------------------------------------------------------------------

/// Opaque pthread handle type.
/// Corresponds to struct __pthread_t in newlib.c
type PthreadT = *mut c_void;

/// Creates a new thread with the specified entry function and stack.
///
/// Allocates thread structure, configures CPU affinity, and starts execution.
/// Corresponds to libnx/nx/source/runtime/newlib.c:__syscall_thread_create
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_syscall_thread_create(
    _thread: *mut PthreadT,
    _func: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    _arg: *mut c_void,
    _stack_addr: *mut c_void,
    _stack_size: usize,
) -> c_int {
    // TODO: Implement thread creation
    ENOSYS
}

/// Waits for a thread to terminate and retrieves its exit value.
///
/// Blocks until the specified thread exits, then cleans up its resources.
/// Corresponds to libnx/nx/source/runtime/newlib.c:__syscall_thread_join
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_syscall_thread_join(
    _thread: PthreadT,
) -> *mut c_void {
    // TODO: Implement thread join
    ptr::null_mut()
}

/// Exits the current thread with the specified return value.
///
/// Terminates thread execution and makes the value available to joiners.
/// Corresponds to libnx/nx/source/runtime/newlib.c:__syscall_thread_exit
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_syscall_thread_exit(_value: *mut c_void) {
    // TODO: Implement thread exit
}

/// Returns a handle to the currently executing thread.
///
/// Gets the current thread's handle for use in other thread operations.
/// Corresponds to libnx/nx/source/runtime/newlib.c:__syscall_thread_self
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_syscall_thread_self() -> PthreadT {
    // TODO: Implement get current thread
    ptr::null_mut()
}

/// Detaches a thread so it cleans up automatically on exit.
///
/// Currently unsupported - always returns ENOSYS.
/// Corresponds to libnx/nx/source/runtime/newlib.c:__syscall_thread_detach
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_syscall_thread_detach(_thread: PthreadT) -> c_int {
    // Unsupported operation
    ENOSYS
}

//-----------------------------------------------------------------------------
// Thread-specific keys (TLS) syscalls
//-----------------------------------------------------------------------------

/// Creates a thread-local storage key with optional destructor.
///
/// Allocates a TLS slot that can store thread-specific data.
/// Corresponds to libnx/nx/source/runtime/newlib.c:__syscall_tls_create
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_syscall_tls_create(
    _key: *mut u32,
    _destructor: Option<unsafe extern "C" fn(*mut c_void)>,
) -> c_int {
    // TODO: Implement TLS key creation
    ENOSYS
}

/// Sets the value for a thread-local storage key.
///
/// Stores a thread-specific value associated with the TLS key.
/// Corresponds to libnx/nx/source/runtime/newlib.c:__syscall_tls_set
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_syscall_tls_set(
    _key: u32,
    _value: *const c_void,
) -> c_int {
    // TODO: Implement TLS set
    ENOSYS
}

/// Gets the value for a thread-local storage key.
///
/// Retrieves the thread-specific value associated with the TLS key.
/// Corresponds to libnx/nx/source/runtime/newlib.c:__syscall_tls_get
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_syscall_tls_get(_key: u32) -> *mut c_void {
    // TODO: Implement TLS get
    ptr::null_mut()
}

/// Deletes a thread-local storage key.
///
/// Frees the TLS slot and calls destructors if needed.
/// Corresponds to libnx/nx/source/runtime/newlib.c:__syscall_tls_delete
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread__libsysbase_syscall_tls_delete(_key: u32) -> c_int {
    // TODO: Implement TLS delete
    ENOSYS
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
