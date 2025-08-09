//! FFI bindings for pthread.c from libsysbase.
//!
//! Provides pthread TLS functions and scheduling functions.

use core::{
    ffi::{c_int, c_uint, c_void},
    ptr,
};

use crate::thread_sleep;

// Error codes
const ENOSYS: c_int = 38;

// ============================================================================
// Scheduling Functions (weak symbols in pthread.c)
// ============================================================================

/// Overrides the `sched_yield` function from the C standard library.
///
/// This function is declared in `<sched.h>`.
/// Corresponds to weak symbol in libgloss/libsysbase/pthread.c:1071-1076
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_sched_yield() -> c_int {
    thread_sleep::yield_with_migration();
    0
}

/// Overrides the `sched_getcpu` function from the C standard library.
///
/// This function is declared in `<sched.h>`.
/// Corresponds to weak symbol in libgloss/libsysbase/pthread.c:1078-1083
/// TODO: Implement CPU affinity tracking
/// Currently returns -1 with ENOSYS as CPU tracking is not implemented.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_sched_getcpu() -> c_int {
    set_errno(ENOSYS);
    -1
}

/// Sets the thread-local `errno` value
// ============================================================================
// Pthread Mutex Functions
// ============================================================================

/// pthread_mutex_init function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_mutex_init(
    _mutex: *mut c_void,
    _attr: *const c_void,
) -> c_int {
    todo!()
}

/// pthread_mutex_destroy function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_mutex_destroy(
    _mutex: *mut c_void,
) -> c_int {
    todo!()
}

/// pthread_mutex_lock function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_mutex_lock(_mutex: *mut c_void) -> c_int {
    todo!()
}

/// pthread_mutex_unlock function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_mutex_unlock(_mutex: *mut c_void) -> c_int {
    todo!()
}

/// pthread_mutex_trylock function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_mutex_trylock(
    _mutex: *mut c_void,
) -> c_int {
    todo!()
}

// ============================================================================
// Pthread Condition Variable Functions
// ============================================================================

/// pthread_cond_init function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_cond_init(
    _cond: *mut c_void,
    _attr: *const c_void,
) -> c_int {
    todo!()
}

/// pthread_cond_destroy function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_cond_destroy(_cond: *mut c_void) -> c_int {
    todo!()
}

/// pthread_cond_wait function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_cond_wait(
    _cond: *mut c_void,
    _mutex: *mut c_void,
) -> c_int {
    todo!()
}

/// pthread_cond_timedwait function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_cond_timedwait(
    _cond: *mut c_void,
    _mutex: *mut c_void,
    _abstime: *const c_void,
) -> c_int {
    todo!()
}

/// pthread_cond_signal function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_cond_signal(_cond: *mut c_void) -> c_int {
    todo!()
}

/// pthread_cond_broadcast function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_cond_broadcast(
    _cond: *mut c_void,
) -> c_int {
    todo!()
}

// ============================================================================
// Pthread Thread Management Functions
// ============================================================================

/// pthread_create function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_create(
    _thread: *mut c_void,
    _attr: *const c_void,
    _start_routine: *mut c_void,
    _arg: *mut c_void,
) -> c_int {
    todo!()
}

/// pthread_join function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_join(
    _thread: c_void,
    _retval: *mut *mut c_void,
) -> c_int {
    todo!()
}

/// pthread_detach function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_detach(_thread: c_void) -> c_int {
    todo!()
}

/// pthread_self function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_self() -> c_void {
    todo!()
}

/// pthread_exit function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_exit(_retval: *mut c_void) {
    todo!()
}

/// pthread_equal function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_equal(_t1: c_void, _t2: c_void) -> c_int {
    todo!()
}

// ============================================================================
// Pthread Attribute Functions
// ============================================================================

/// pthread_attr_init function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_attr_init(_attr: *mut c_void) -> c_int {
    todo!()
}

/// pthread_attr_destroy function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_attr_destroy(_attr: *mut c_void) -> c_int {
    todo!()
}

/// pthread_attr_setstacksize function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_attr_setstacksize(
    _attr: *mut c_void,
    _stacksize: usize,
) -> c_int {
    todo!()
}

/// pthread_attr_getstacksize function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_attr_getstacksize(
    _attr: *const c_void,
    _stacksize: *mut usize,
) -> c_int {
    todo!()
}

/// pthread_attr_setdetachstate function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_attr_setdetachstate(
    _attr: *mut c_void,
    _detachstate: c_int,
) -> c_int {
    todo!()
}

/// pthread_attr_getdetachstate function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_attr_getdetachstate(
    _attr: *const c_void,
    _detachstate: *mut c_int,
) -> c_int {
    todo!()
}

// ============================================================================
// Pthread Read-Write Lock Functions
// ============================================================================

/// pthread_rwlock_init function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_rwlock_init(
    _rwlock: *mut c_void,
    _attr: *const c_void,
) -> c_int {
    todo!()
}

/// pthread_rwlock_destroy function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_rwlock_destroy(
    _rwlock: *mut c_void,
) -> c_int {
    todo!()
}

/// pthread_rwlock_rdlock function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_rwlock_rdlock(
    _rwlock: *mut c_void,
) -> c_int {
    todo!()
}

/// pthread_rwlock_wrlock function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_rwlock_wrlock(
    _rwlock: *mut c_void,
) -> c_int {
    todo!()
}

/// pthread_rwlock_unlock function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_rwlock_unlock(
    _rwlock: *mut c_void,
) -> c_int {
    todo!()
}

/// pthread_rwlock_tryrdlock function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_rwlock_tryrdlock(
    _rwlock: *mut c_void,
) -> c_int {
    todo!()
}

/// pthread_rwlock_trywrlock function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_rwlock_trywrlock(
    _rwlock: *mut c_void,
) -> c_int {
    todo!()
}

// ============================================================================
// Semaphore Functions
// ============================================================================

/// sem_init function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_sem_init(
    _sem: *mut c_void,
    _pshared: c_int,
    _value: c_uint,
) -> c_int {
    todo!()
}

/// sem_destroy function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_sem_destroy(_sem: *mut c_void) -> c_int {
    todo!()
}

/// sem_wait function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_sem_wait(_sem: *mut c_void) -> c_int {
    todo!()
}

/// sem_post function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_sem_post(_sem: *mut c_void) -> c_int {
    todo!()
}

/// sem_trywait function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_sem_trywait(_sem: *mut c_void) -> c_int {
    todo!()
}

// ============================================================================
// Pthread Barrier Functions
// ============================================================================

/// pthread_barrier_init function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_barrier_init(
    _barrier: *mut c_void,
    _attr: *const c_void,
    _count: c_uint,
) -> c_int {
    todo!()
}

/// pthread_barrier_destroy function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_barrier_destroy(
    _barrier: *mut c_void,
) -> c_int {
    todo!()
}

/// pthread_barrier_wait function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_barrier_wait(
    _barrier: *mut c_void,
) -> c_int {
    todo!()
}

// ============================================================================
// Pthread Once Functions
// ============================================================================

/// pthread_once function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_once(
    _once_control: *mut c_void,
    _init_routine: unsafe extern "C" fn(),
) -> c_int {
    todo!()
}

// ============================================================================
// Pthread Key Functions
// ============================================================================

/// pthread_key_create function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_key_create(
    _key: *mut u32,
    _destructor: Option<unsafe extern "C" fn(*mut c_void)>,
) -> c_int {
    todo!()
}

/// pthread_key_delete function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_key_delete(_key: u32) -> c_int {
    todo!()
}

/// Rust implementation of `pthread_setspecific` that libgloss/newlib expects.
///
/// TODO: Add support for pthread TLS functions
/// Currently returns ENOSYS as dynamic slots are not implemented.
/// Corresponds to libgloss/libsysbase/pthread.c:559
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_setspecific(
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
pub unsafe extern "C" fn __nx_sys_thread_newlib_pthread_getspecific(_key: u32) -> *mut c_void {
    // Dynamic TLS slots not supported - always return NULL
    ptr::null_mut()
}

#[inline]
fn set_errno(code: c_int) {
    unsafe extern "C" {
        // This is a newlib/libc function
        fn __errno() -> *mut c_int;
    }

    unsafe { *__errno() = code };
}
