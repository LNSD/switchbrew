//! Synchronization primitives

use crate::{
    error::{KernelError as KError, ResultCode, ToRawResultCode},
    raw::{self, Handle},
    result::{Error, Result, raw::Result as RawResult},
};

/// Bitmask for the _waiters bitflag_ in mutex raw tag values.
///
/// When set in a mutex raw tag value, indicates that there are threads waiting to acquire the mutex.
/// The mutex raw tag value is expected to be `owner_thread_handle | HANDLE_WAIT_MASK` when threads
/// are waiting.
pub const HANDLE_WAIT_MASK: u32 = 0x40000000;

/// Arbitrates a mutex lock operation in userspace
///
/// Attempts to acquire a mutex by arbitrating the lock with the owner thread.
///
/// # Arguments
/// | Arg | Name | Description |
/// | --- | --- | --- |
/// | IN | _owner_thread_handle_ | The owner thread's kernel handle. Must be a valid thread handle. |
/// | IN | _mutex_ | Pointer to the mutex raw tag value in userspace memory. The mutex raw tag value must be `owner_thread_handle | [`HANDLE_WAIT_MASK`]`. |
/// | IN | _curr_thread_handle_ | The current thread's kernel handle requesting the lock. |
///
/// # Behavior
/// This function calls the [`__nx_svc_arbitrate_lock`] syscall with the provided arguments.
///
/// Then the kernel will:
/// 1. Validate the current thread's state and memory access
/// 2. Check if mutex value matches expected pattern (`owner_thread_handle | HANDLE_WAIT_MASK`)
/// 3. If matched, add current thread to owner's mutex waiter list
/// 4. Pause current thread execution until mutex is released
/// 5. Remove thread from waiter list upon wake-up
///
/// The current thread will be paused until either:
/// - The mutex is released by the owner
/// - The thread is terminated
/// - An error occurs (invalid handle, invalid memory state)
///
/// # Notes
/// - This is a blocking operation that will pause the current thread if the mutex is held
/// - The mutex must be properly initialized before calling this function
/// - Thread handles must belong to the same process
///
/// # Safety
/// This function is unsafe because it:
/// - Dereferences a raw pointer (`mutex`)
/// - Interacts directly with thread scheduling and kernel synchronization primitives
pub unsafe fn arbitrate_lock(
    owner_thread_handle: Handle,
    mutex: *mut u32,
    curr_thread_handle: Handle,
) -> Result<(), ArbitrateLockError> {
    let rc = unsafe { raw::arbitrate_lock(owner_thread_handle, mutex, curr_thread_handle) };
    RawResult::from_raw(rc).map((), |rc| match rc.description() {
        desc if KError::InvalidHandle == desc => ArbitrateLockError::InvalidHandle,
        desc if KError::InvalidAddress == desc => ArbitrateLockError::InvalidMemState,
        desc if KError::TerminationRequested == desc => ArbitrateLockError::ThreadTerminating,
        _ => ArbitrateLockError::Unknown(Error::from(rc)),
    })
}

/// Error type for [`arbitrate_lock`]
#[derive(Debug, thiserror::Error)]
pub enum ArbitrateLockError {
    /// The owner thread handle is invalid.
    #[error("Invalid handle")]
    InvalidHandle,
    /// The mutex memory address cannot be accessed.
    #[error("Invalid memory state")]
    InvalidMemState,
    /// The current thread is marked for termination.
    #[error("Thread terminating")]
    ThreadTerminating,
    /// An unknown error occurred.
    ///
    /// This variant is used when the error code is not recognized.
    #[error("Unknown error: {0}")]
    Unknown(Error),
}

/// Arbitrates a mutex unlock operation in userspace
///
/// Releases a mutex by arbitrating the unlock operation with waiting threads.
///
/// # Arguments
/// | Arg | Name | Description |
/// | --- | --- | --- |
/// | IN | _mutex_ | Pointer to the mutex tag value in userspace memory. |
///
/// # Behavior
/// This function calls the [`__nx_svc_arbitrate_unlock`] syscall with the provided arguments.
///
/// Then the kernel will:
/// 1. Validate the current thread's state and memory access
/// 2. Update the mutex value to release the lock
/// 3. If there are waiting threads:
///    - Select the next thread to own the mutex.
///    - Update the mutex value with the new owner
///    - Wake up the selected thread
///
/// ## Notes
/// - The current thread must be the owner of the mutex. Otherwise, this is a no-op
///
/// # Safety
/// This function is unsafe because it:
/// - Dereferences a raw pointer (`mutex`)
/// - Interacts directly with thread scheduling and kernel synchronization primitives
pub unsafe fn arbitrate_unlock(mutex: *mut u32) -> Result<(), ArbitrateUnlockError> {
    let rc = unsafe { raw::arbitrate_unlock(mutex) };
    RawResult::from_raw(rc).map((), |rc| match rc.description() {
        desc if KError::InvalidAddress == desc => ArbitrateUnlockError::InvalidMemState,
        _ => ArbitrateUnlockError::Unknown(Error::from(rc)),
    })
}

/// Error type for [`arbitrate_unlock`]
#[derive(Debug, thiserror::Error)]
pub enum ArbitrateUnlockError {
    /// The mutex memory address cannot be accessed.
    #[error("Invalid memory state")]
    InvalidMemState,
    /// An unknown error occurred.
    ///
    /// This variant is used when the error code is not recognized.
    #[error("Unknown error: {0}")]
    Unknown(Error),
}

/// Atomically releases a mutex and waits on a condition variable
///
/// Atomically releases the mutex and suspends the current thread until the condition variable is
/// signaled or a timeout occurs.
///
/// # Arguments
/// | Arg | Name | Description |
/// | --- | --- | --- |
/// | IN | _condvar_ | Pointer to the condition variable in userspace memory. |
/// | IN | _mutex_ | Pointer to the mutex raw tag value in userspace memory. |
/// | IN | _tag_ | The thread handle value associated with the mutex. |
/// | IN | _timeout_ns_ | Timeout in nanoseconds. Use 0 for no timeout, -1 for infinite wait. |
///
/// # Behavior
/// This function calls the [`__nx_svc_wait_process_wide_key_atomic`] syscall with the provided arguments.
///
/// Then the kernel will:
/// 1. Validate the current thread's state and memory access
/// 2. Release the mutex (updating mutex value and waking waiters)
/// 3. Add the current thread to the condition variable's waiter list
/// 4. Pause the current thread until either:
///    - The condition variable is signaled
///    - The timeout expires (if timeout > 0)
///    - The thread is terminated
/// 5. Remove thread from condition variable waiter list upon wake-up
/// 6. Re-acquire the mutex before returning
///
/// # Notes
/// - This is a blocking operation that will pause the current thread
/// - The mutex must be held by the current thread before calling this function
/// - The operation is atomic - no other thread can acquire the mutex between release and wait
/// - If timeout is 0, returns immediately after releasing mutex
/// - If timeout is -1, waits indefinitely
///
/// # Safety
/// This function is unsafe because it:
/// - Dereferences raw pointers (`mutex` and `condvar`)
/// - Interacts directly with thread scheduling and kernel synchronization primitives
pub unsafe fn wait_process_wide_key_atomic(
    condvar: *mut u32,
    mutex: *mut u32,
    tag: u32,
    timeout_ns: u64,
) -> Result<(), WaitProcessWideKeyError> {
    let res = unsafe { raw::wait_process_wide_key_atomic(mutex, condvar, tag, timeout_ns) };
    RawResult::from_raw(res).map((), |rc| match rc.description() {
        desc if KError::InvalidAddress == desc => WaitProcessWideKeyError::InvalidMemState,
        desc if KError::TerminationRequested == desc => WaitProcessWideKeyError::ThreadTerminating,
        desc if KError::TimedOut == desc => WaitProcessWideKeyError::TimedOut,
        _ => WaitProcessWideKeyError::Unknown(Error::from(rc)),
    })
}

/// Error type for [`wait_process_wide_key_atomic`]
#[derive(Debug, thiserror::Error)]
pub enum WaitProcessWideKeyError {
    /// The mutex or condvar memory address cannot be accessed.
    #[error("Invalid memory state")]
    InvalidMemState,
    /// The current thread is marked for termination.
    #[error("Thread terminating")]
    ThreadTerminating,
    /// The wait operation timed out.
    #[error("Operation timed out")]
    TimedOut,
    /// An unknown error occurred.
    ///
    /// This variant is used when the error code is not recognized.
    #[error("Unknown error: {0}")]
    Unknown(Error),
}

impl ToRawResultCode for WaitProcessWideKeyError {
    fn to_rc(self) -> ResultCode {
        match self {
            WaitProcessWideKeyError::InvalidMemState => KError::InvalidAddress.to_rc(),
            WaitProcessWideKeyError::ThreadTerminating => KError::TerminationRequested.to_rc(),
            WaitProcessWideKeyError::TimedOut => KError::TimedOut.to_rc(),
            WaitProcessWideKeyError::Unknown(err) => err.to_raw(),
        }
    }
}

/// Signals a condition variable to wake waiting threads
///
/// Wakes up one or more threads waiting on the specified condition variable.
///
/// # Arguments
/// | Arg | Name | Description |
/// | --- | --- | --- |
/// | IN | _condvar_ | Pointer to the condition variable in userspace memory. |
/// | IN | _count_ | Number of threads to wake. If greater than the number of waiting threads, all threads are woken. If less than or equal to 0, wakes all waiting threads. |
///
/// # Behavior
/// This function calls the [`__nx_svc_signal_process_wide_key`] syscall with the provided arguments.
///
/// Then the kernel will:
/// 1. Select threads to wake based on:
///    - Threads must be waiting on the specified condition variable
///    - Threads are ordered by their dynamic priority
///    - Up to _count_ threads are selected (or all threads if _count_ ≤ 0, e.g. -1)
/// 2. For each selected thread:
///    - Remove it from the condition variable's waiter list
///    - Attempt to re-acquire its associated mutex
/// 3. If no threads remain waiting:
///    - Reset the condition variable value to the default value
///
/// # Notes
/// - This is a non-blocking operation
/// - If no threads are waiting on the condition variable, this is effectively a no-op
/// - Woken threads will attempt to re-acquire their associated mutexes before resuming
/// - Thread selection is priority-aware, favoring threads with higher dynamic priority
///
/// # Safety
/// This function is unsafe because it:
/// - Dereferences a raw pointer (`condvar`)
/// - Interacts directly with thread scheduling and kernel synchronization primitives
pub unsafe fn signal_process_wide_key(condvar: *mut u32, count: i32) {
    unsafe { raw::signal_process_wide_key(condvar, count) };
}
