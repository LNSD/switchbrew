//! # Minimal **Thread-Local Storage (TLS)** utilities.
//!
//! The full TLS implementation – including the [`ThreadVars`] structure and
//! helper functions – lives in the [`nx-sys-thread`] crate.
//!
//! Unfortunately `nx-sys-thread` directly (and transitively) needs the low-level
//! mutex and condition-variable primitives that are provided by *this* crate,
//! which would cause a circular dependency. To avoid that cycle we replicate
//! just the tiny subset of functionality required by the sync algorithms:
//! obtaining the **handle of the current thread**.
//!
//! ## Libnx TLS layout
//! Every user-mode thread running on Horizon OS receives a 0x200-byte TLS
//! block. The base address of that block is available in the `TPIDRRO_EL0`
//! system register. The last 0x20 bytes are occupied by the so-called
//! `ThreadVars` structure.
//!
//! ```text
//!  TLS base (TPIDRRO_EL0)                  TLS + 0x200
//!  |                                           ^
//!  |                                           |
//!  +------------------------------+------------+
//!  |  user / compiler TLS slots   | ThreadVars |
//!  +------------------------------+------------+
//!                                   0x20 bytes
//!                                    ├── 0x00  magic (u32)
//!                                    ├── 0x04  handle (u32)
//!                                    ├── 0x08  thread_ptr (*mut c_void)
//!                                    ├── 0x10  reent (*mut c_void)
//!                                    └── 0x18  tls_tp (*mut c_void)
//! ```
//!
//! Because *ThreadVars* begins at `TLS + 0x200 − 0x20 = TLS + 0x1E0`, the
//! `handle` field resides at **absolute offset 0x1E4** within the TLS block.
//! That hard-coded constant is therefore used below to read the 32-bit thread
//! handle in a dependency-free manner. If the kernel ever changes the layout
//! the value would have to be adjusted in *both* this crate and
//! `nx-sys-thread`.
//!
//! ## Safety
//! All functions in this module are `#[inline]` and limited in scope. They do
//! not expose any mutable references into TLS, only raw pointers or plain data
//! values, mirroring libnx's C API. Callers must still treat the returned
//! pointer/value with the usual care required for low-level concurrency code.
//!
//! ## References
//! * Switchbrew – [Thread Local Region](https://switchbrew.org/wiki/Thread_Local_Region)
//! * libnx – [internal.h](https://github.com/switchbrew/libnx/blob/master/nx/source/internal.h)
//! * `nx-sys-thread::tls`
//!
//! [`ThreadVars`]: https://switchbrew.org/wiki/Thread_Local_Region
//! [`nx-sys-thread::tls`]: https://docs.rs/nx-sys-thread/latest/nx_sys_thread/tls/index.html

use core::{ffi::c_void, ptr};

use nx_cpu::control_regs;
use nx_svc::thread::Handle;

/// Offset of the current thread's kernel handle in the TLS block.
const TLS_HANDLE_OFFSET: usize = 0x1E4;

/// Returns a raw pointer to the **Thread-Local Storage (TLS) block** of the
/// currently running thread.
///
/// Internally the routine just reads the AArch64 register [`TPIDRRO_EL0`],
/// which the kernel sets up to point to the start of the 0x200-byte TLS region
/// described in the *module-level* documentation.  No additional checks are
/// performed.
///
/// Although the pointer itself is safe to obtain, **dereferencing it is `unsafe`**
/// because Rust has no knowledge about the memory layout beyond what is
/// documented in libnx.  Only low-level synchronisation code should touch the
/// raw bytes directly – higher-level code ought to rely on the richer helper
/// APIs provided by the `nx-sys-thread` crate.
///
/// Returns a raw, mutable pointer to the first byte of the 0x200-byte TLS block
/// that belongs to the current thread.
///
/// # Safety
/// The function itself is safe – the raw pointer is guaranteed to be valid for
/// *reading* 0x200 bytes. Any *dereference* the caller performs, however, must
/// uphold the usual Rust safety guarantees.
///
/// [`TPIDRRO_EL0`]: https://developer.arm.com/documentation/ddi0595/2021-12/AArch64-Registers/TPIDRRO-EL0--EL0-Read-Only-Thread-ID-Register
#[inline]
fn get_tls_ptr() -> *mut c_void {
    unsafe { control_regs::tpidrro_el0() as *mut c_void }
}

/// Fetches the kernel [`Handle`] of the **currently executing thread**.
///
/// The value is loaded from the `handle` field of the `ThreadVars` structure
/// that lives at TLS offset `0x1E4`.  A single `read_volatile` instruction is
/// used to prevent the compiler from tearing or caching the value across
/// multiple calls – important when the scheduler might reschedule the thread
/// to a different core between two loads.
///
/// The returned [`Handle`] *identifies* the thread to the kernel; it must never
/// be closed by user code.  It is safe to compare against other handles or pass
/// to SVC calls such as `ArbitrateLock`.
///
/// Returns the 32-bit kernel handle that uniquely describes the current thread.
#[inline]
pub fn get_current_thread_handle() -> Handle {
    let tls_ptr = get_tls_ptr();

    unsafe {
        // SAFETY: On Horizon each thread stores its kernel handle at TLS offset 0x1E4
        // inside the `ThreadVars` structure (see module-level docs). Dereferencing
        // the pointer with `read_volatile` prevents the compiler from folding or
        // reordering the load across inline assembly and atomic operations used by
        // the sync primitives.
        let raw_handle_ptr = tls_ptr.add(TLS_HANDLE_OFFSET) as *const Handle;

        // SAFETY: `raw_handle_ptr` points to the current thread's handle
        // located at offset 0x1E4 in TLS. The field access is performed with
        // `read_volatile` to avoid the compiler re-ordering or eliminating the
        // read.
        ptr::read_volatile(raw_handle_ptr)
    }
}
