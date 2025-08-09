//! FFI bindings for thread-related functions.

use core::{ffi::c_void, ptr};

use nx_svc::{error::ToRawResultCode, raw::Handle as RawHandle};

use crate::{
    Thread, thread_context, thread_info, thread_pause, thread_resume, thread_start, thread_wait,
};

// ============================================================================
// Thread Information API
// ============================================================================

/// Retrieves a pointer to the calling thread's information block.
///
/// # Safety
/// 1. The returned pointer is only valid while the calling thread remains
///    alive; dereferencing it after the thread has exited results in undefined
///    behaviour.
/// 2. The caller must ensure that no mutable references coexist with shared
///    references derived from this pointer, upholding Rust's aliasing rules.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread_get_self() -> *mut Thread {
    thread_info::get_current_thread_info_ptr()
}

/// Retrieves the raw kernel handle associated with the calling thread.
///
/// This mirrors libnx's `threadGetCurHandle` and simply forwards the value
/// stored in the thread‐local storage.
///
/// # Safety
/// The returned handle is a plain value; dereferencing or otherwise using it
/// after the thread has exited is undefined behaviour, but callers typically
/// treat it as an opaque token and hand it to kernel services.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread_get_cur_handle() -> RawHandle {
    thread_info::get_current_thread_handle().to_raw()
}

// ============================================================================
// Thread Activity API
// ============================================================================

/// Starts the execution of a thread.
///
/// # Safety
///
/// The caller must ensure that `t` points to a valid [`Thread`] instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_start(t: *const Thread) -> u32 {
    // SAFETY: The caller must ensure that `t` is non-null.
    let thread = unsafe { &*t };

    thread_start::start(thread).map_or_else(|err| err.to_rc(), |_| 0)
}

/// Pauses the execution of a thread.
///
/// # Safety
///
/// The caller must ensure that `t` points to a valid [`Thread`] instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_pause(t: *const Thread) -> u32 {
    // SAFETY: The caller must ensure that `t` is non-null.
    let thread = unsafe { &*t };

    thread_pause::pause(thread).map_or_else(|err| err.to_rc(), |_| 0)
}

/// Resumes the execution of a previously paused thread.
///
/// # Safety
///
/// The caller must ensure that `t` points to a valid [`Thread`] instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_resume(t: *const Thread) -> u32 {
    // SAFETY: The caller must ensure that `t` is non-null.
    let thread = unsafe { &*t };

    thread_resume::resume(thread).map_or_else(|err| err.to_rc(), |_| 0)
}

// ============================================================================
// Thread Context API
// ============================================================================

/// CPU/FPU register dump for a paused thread.
#[repr(C)]
pub struct Context {
    /// General-purpose CPU registers X0..X28.
    pub cpu_gprs: [CpuRegister; 29],
    /// Frame pointer (X29).
    pub fp: u64,
    /// Link register (X30).
    pub lr: u64,
    /// Stack pointer.
    pub sp: u64,
    /// Program counter.
    pub pc: CpuRegister,
    /// Processor status register.
    pub psr: u32,
    /// NEON registers V0..V31.
    pub fpu_gprs: [FpuRegister; 32],
    /// Floating-point control register.
    pub fpcr: u32,
    /// Floating-point status register.
    pub fpsr: u32,
    /// EL0 Read/Write Software Thread ID Register.
    pub tpidr: u64,
}

/// 64/32-bit CPU register view as returned by `svcGetThreadContext3`.
#[repr(C)]
pub union CpuRegister {
    /// 64-bit AArch64 view (Xn)
    pub x: u64,
    /// 32-bit AArch64 view (Wn)
    pub w: u32,
    /// AArch32 view (Rn)
    pub r: u32,
}

/// 128/64/32-bit NEON register view.
#[repr(C)]
pub union FpuRegister {
    /// 128-bit vector (Vn)
    pub v: u128,
    /// 64-bit double-precision floating point (Dn)
    pub d: f64,
    /// 32-bit single-precision floating point (Sn)
    pub s: f32,
}

impl From<thread_context::Context> for Context {
    fn from(value: thread_context::Context) -> Self {
        // SAFETY: `CpuRegister`/`FpuRegister` are layout-identical to their
        // counterparts in `ctx`, therefore this transmute is sound.
        let cpu_gprs = unsafe { core::mem::transmute(value.cpu_gprs) };
        let fpu_gprs = unsafe { core::mem::transmute(value.fpu_gprs) };
        let pc = unsafe { core::mem::transmute(value.pc) };

        Self {
            cpu_gprs,
            fp: value.fp,
            lr: value.lr,
            sp: value.sp,
            pc,
            psr: value.psr,
            fpu_gprs,
            fpcr: value.fpcr,
            fpsr: value.fpsr,
            tpidr: value.tpidr,
        }
    }
}

/// Dumps the CPU/FPU context of a *paused* thread into `ctx`.
///
/// # Safety
/// * `t` must point to a valid [`Thread`] instance.
/// * `ctx` must be non-null and point to writable memory large enough to hold
///   a [`Context`] structure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_dump_context(ctx: *mut Context, t: *const Thread) -> u32 {
    // SAFETY: The caller guarantees that the pointers are valid.
    let thread = unsafe { &*t };

    match thread_context::dump_context(&thread) {
        Ok(sys_ctx) => {
            // Write the converted context back to the caller-provided buffer.
            unsafe { ctx.write(sys_ctx.into()) };
            0
        }
        Err(err) => err.to_rc(),
    }
}

// ============================================================================
// Thread Wait API
// ============================================================================

/// Blocks the caller until the target thread has terminated.
///
/// Mirrors libnx's `threadWaitForExit` function.
///
/// # Safety
/// * `t` must be non-null and point to a valid [`Thread`] instance.
/// * The pointed-to thread must outlive this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_sys_thread_wait_for_exit(t: *const Thread) -> u32 {
    // SAFETY: The caller is responsible for ensuring `t` is non-null and valid.
    let thread = unsafe { &*t };

    thread_wait::wait_thread_exit(thread).map_or_else(|err| err.to_rc(), |_| 0)
}
// ============================================================================
// Dynamic TLS Slots API
// ============================================================================

/// Reads the raw pointer stored in the dynamic TLS slot `slot_id`.
///
/// Mirrors `threadTlsGet` in libnx's C API.
///
/// TODO: Add support for dynamic TLS slots
/// Currently returns NULL for all slot IDs as dynamic slots are not implemented.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread_tls_get(_slot_id: i32) -> *mut c_void {
    // Dynamic TLS slots not supported - always return NULL
    ptr::null_mut()
}

/// Writes `value` into dynamic TLS slot `slot_id`.
///
/// Mirrors `threadTlsSet` in libnx's C API.
///
/// TODO: Add support for dynamic TLS slots
/// Currently does nothing as dynamic slots are not implemented.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_sys_thread_tls_set(_slot_id: i32, _value: *mut c_void) {
    // Dynamic TLS slots not supported - silently ignore
}
