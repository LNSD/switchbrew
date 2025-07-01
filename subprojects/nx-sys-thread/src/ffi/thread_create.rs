//! FFI bindings for the thread creation API.
//!
//! This module exposes a C-compatible interface that mirrors the thread
//! creation/destruction functions from libnx. It delegates to the safe Rust
//! implementation in `crate::thread_impl::create` and handles type/error
//! conversions.

use core::{ffi::c_void, ptr::NonNull};

use nx_svc::error::ToRawResultCode;
use nx_sys_mem::stack as mem;

use crate::thread_impl as sys;

/// Creates a new thread.
///
/// This is the FFI-safe equivalent of `threadCreate` from libnx.
///
/// # Safety
/// * `t` must be a non-null, valid pointer to a `Thread` structure that can
///   be safely written to.
/// * If `stack_mem` is non-null, it must point to a valid, page-aligned
///   memory region of at least `stack_sz` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_create(
    t: *mut sys::Thread,
    entry: sys::ThreadFunc,
    arg: *mut c_void,
    stack_mem: *mut c_void,
    stack_sz: usize,
    prio: i32,
    cpuid: i32,
) -> u32 {
    // SAFETY: The caller guarantees that `t` is a valid pointer.
    let thread = unsafe { &mut *t };
    unsafe {
        sys::create(
            thread,
            entry,
            arg,
            NonNull::new(stack_mem),
            stack_sz,
            prio,
            cpuid,
        )
    }
    .map_or_else(
        |err| match err {
            sys::ThreadCreateError::InvalidStackSize => 0x4802,
            sys::ThreadCreateError::InvalidStackAlignment => 0x4802,
            sys::ThreadCreateError::OutOfMemory => 0x34A02,
            sys::ThreadCreateError::StackTooSmall => 0x4A02,
            sys::ThreadCreateError::StackAlloc(_) => 0x34A02, // Corresponds to LibnxError_OutOfMemory
            sys::ThreadCreateError::StackMap(err) => match err {
                mem::MapError::VirtAddressAllocFailed => 0x34A02, // Out of memory
                mem::MapError::Svc(svc_err) => svc_err.to_rc(),
            },
            sys::ThreadCreateError::SvcCreateThread(err) => err.to_rc(),
        },
        |_| 0,
    )
}

/// Frees up resources associated with a thread.
///
/// # Safety
/// The caller must ensure that `t` points to a valid [`Thread`] instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_close(t: *mut sys::Thread) -> u32 {
    let thread = unsafe { &mut *t };
    unsafe { sys::close(thread) }.map_or_else(
        |err| match err {
            sys::ThreadCloseError::ThreadNotExited => 0x3E02,
            sys::ThreadCloseError::BackingPointerMissing => 0x4402,
            sys::ThreadCloseError::UnmapError(err) => err.reason.to_rc(),
            sys::ThreadCloseError::CloseHandleError(err) => err.to_rc(),
        },
        |_| 0,
    )
}

/// Exits the current thread.
///
/// This is the FFI-safe equivalent of `threadExit` from libnx.
/// It retrieves the current thread's context and calls the underlying `exit`
/// implementation.
///
/// This function never returns.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_exit() -> ! {
    let thread = unsafe { &mut *sys::get_current_thread_info_ptr() };
    unsafe { sys::exit(thread) }
}
