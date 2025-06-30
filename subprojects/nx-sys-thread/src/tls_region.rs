//! # Thread-Local Storage (TLS) Region
//!
//! The thread-local storage (TLS) region is a 0x200-byte area.
//!
//! Its base address is loaded via the ARM thread ID register `tpidrro_el0`. Multiple threads store
//! their TLS regions in the same page, with the first TLS region typically located at `page + 0x200`, as the
//! first TLR spot is reserved for user-mode exception handling.
//!
//! In threads created by the Nintendo SDK, `tpidr_el0` is assigned to the `ThreadPointer` object
//! from the thread-local region.
//!
//! ## TLS layout overview
//! The **complete** 0x200-byte block, including the ABI-mandated *Thread-Control
//! Block* (TCB), looks like this:
//!
//! ```text
//! TLS base (TPIDRRO_EL0)
//! 0x000  ┌────────────────────────────┐
//!        │ IPC Message Buffer         │ 0x100 Bytes
//! 0x100  ├────────────────────────────┤
//!        │ <Unknown>                  │
//! 0x108  ├────────────────────────────┤  ╮  
//!        │  Dynamic TLS *slots*       │  │
//! 0x1E0  ├────────────────────────────┤  ├  User TLS region
//!        │  ThreadVars  (32 bytes)    │  │
//! 0x200  └────────────────────────────┘  ╯
//! ```
//!
//! The first 16 bytes are reserved for the *Thread-Control Block* (TCB) and
//! satisfy the AArch64 TLS ABI so that compiler-generated accesses to
//! `__aarch64_read_tp()` work. The final 32 bytes are reserved for `ThreadVars`.
//! Everything in between is available for user and linker-provided TLS data.
//!
//! ### User TLS region
//! From TLS base + 0x108 onward the block belongs entirely to user-mode code.
//! The layout is:
//!
//! ```text
//! TLS base (TPIDRRO_EL0) + 0x108
//! 0x108  ┌────────────────────────────┐
//!        │ Slot 0  (*mut c_void)      │ 8 Bytes
//! 0x110  ├────────────────────────────┤
//!        │ Slot 1  (*mut c_void)      │ 8 Bytes
//! 0x118  ├────────────────────────────┤
//!        ┆             …              ┆
//! 0x1D0  ├────────────────────────────┤
//!        │ Slot 25 (*mut c_void)      │ 8 Bytes
//! 0x1D8  ├────────────────────────────┤
//!        │ Slot 26 (*mut c_void)      │ 8 Bytes
//! 0x1E0  ╞════════════════════════════╡
//!        │ ThreadVars (0x20 bytes)    │
//! 0x200  └────────────────────────────┘
//! ```
//!
//! #### Dynamic TLS slots (`0x108` – `0x1DF`)
//!
//! Array of runtime-allocated pointers used by libnx to implement thread-local
//! storage that is *not* known at link-time (e.g. `pthread_key_create`, C
//! locale, etc.).  Each thread has its own copy; slot IDs are process-global.
//!
//! * 27 entries (`NUM_TLS_SLOTS`) of pointer-sized storage. Each slot can be
//!   claimed at runtime with `threadTlsAlloc()`/`threadTlsSet()` (see libnx C
//!   API) or—on the Rust side—via higher-level wrappers.
//! * A process-global bitmask tracks which slot IDs are in use; an optional
//!   *destructor* function may be registered so that per-thread cleanup runs
//!   automatically when the thread exits (`threadExit`).
//! * Access is purely arithmetic: `TPIDRRO_EL0 + 0x108 + slot_id *
//!   size_of::<*mut c_void>()`, no syscalls needed.
//! * Each entry is pointer-sized, so it can hold any `*mut T` or small integral
//!   value cast to `usize`.
//!
//! #### [`ThreadVars`] (`0x1E0` – `0x200`)
//!
//! A fixed 32-byte footer holding per-thread metadata that libnx needs
//! constantly: a *magic* value, the thread's kernel handle, a link to the
//! rust/C thread object, a pointer to the newlib re-entrancy state and a cached
//! copy of the thread-pointer (TP) value.
//!
//! ```text
//! TLS base + 0x1E0
//! 0x1E0 ┌────────────────────────────┐
//!       │ magic       (u32)          │
//! 0x1E4 ├────────────────────────────┤
//!       │ handle      (u32)          │
//! 0x1E8 ├────────────────────────────┤
//!       │ thread_ptr  (*mut c_void)  │
//! 0x1F0 ├────────────────────────────┤
//!       │ reent       (*mut c_void)  │
//! 0x1F8 ├────────────────────────────┤
//!       │ tls_tp      (*mut c_void)  │
//! 0x200 └────────────────────────────┘
//! ```
//!
//! ## References
//! - [Switchbrew Wiki: Thread Local Region](https://switchbrew.org/wiki/Thread_Local_Region)
//! - [switchbrew/libnx: tls.h](https://github.com/switchbrew/libnx/blob/master/nx/include/switch/arm/tls.h)
//! - [Fuchsia: Thread Local Storage (TLS)](https://fuchsia.dev/fuchsia-src/development/kernel/threads/tls)
//! - [Fuchsia: Thread Local Storage Implementation](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/docs/development/kernel/threads/tls.md)

use core::{
    ffi::c_void,
    ptr::{self, NonNull},
};

use nx_cpu::control_regs;
use nx_svc::thread::Handle;

/// Size of the Thread Local Storage (TLS) region.
pub const TLS_REGION_SIZE: usize = 0x200;

/// Start of the user-mode TLS region.
pub const USER_TLS_REGION_BEGIN: usize = 0x108;

/// End of the user-mode TLS region.
pub const USER_TLS_REGION_END: usize = TLS_REGION_SIZE - THREAD_VARS_SIZE;

/// The number of slots in the TLS region.
///
/// The TLS region is divided into slots of size `core::mem::size_of::<*mut c_void>()`.
///
/// The number of slots is calculated as the difference between the end and the beginning
/// of the user-mode TLS region, divided by the size of the slot.
pub const NUM_TLS_SLOTS: usize =
    (USER_TLS_REGION_END - USER_TLS_REGION_BEGIN) / size_of::<*mut c_void>();

/// Size of the ThreadVars structure  
///
/// The [`ThreadVars`] structure is exactly 32 bytes (0x20) long and is stored at the end
/// of the thread's TLS region.
pub const THREAD_VARS_SIZE: usize = 0x20;

/// Magic value used to verify that the [`ThreadVars`] structure is initialised.
///
/// The value corresponds to the ASCII string "!TV$".
pub const THREAD_VARS_MAGIC: u32 = 0x21545624;

/// Per-thread variables located at the end of the TLS area.
///
/// The struct occupies exactly [`THREAD_VARS_SIZE`] bytes (0x20) and matches the
/// layout used by the Horizon OS loader as documented on Switchbrew.
#[derive(Debug)]
#[repr(C)]
pub struct ThreadVars {
    /// Magic value used to check if the struct is initialised.
    pub magic: u32,
    /// Kernel handle identifying the thread.
    pub handle: Handle,
    /// Pointer to the current thread object (if any).
    pub thread_info_ptr: *mut c_void,
    /// Pointer to the thread's newlib reentrancy state.
    pub reent: *mut c_void,
    /// Pointer to this thread's thread-local segment (TP).
    ///
    /// ## AArch64 ABI Requirement
    ///
    /// This field **must** be located at exactly *TLS base + 0x1F8* to comply with the
    /// **AArch64 Procedure Call Standard (AAPCS64)** specification.
    ///
    /// ### Background
    ///
    /// The AArch64 ABI defines a standard mechanism for accessing thread-local variables
    /// (`__thread` in C, `thread_local` in C++). Compilers generate calls to a runtime
    /// function `__aarch64_read_tp()` to obtain a base pointer for thread-local variable
    /// access.
    ///
    /// ### Implementation
    ///
    /// The `__aarch64_read_tp()` function is implemented in assembly at
    /// `libnx/nx/source/runtime/readtp.s`:
    ///
    /// ```assembly
    /// __aarch64_read_tp:
    ///     mrs x0, tpidrro_el0     ; Read TLS base from system register
    ///     ldr x0, [x0, #0x1F8]    ; Load value from [TLS_base + 0x1F8]
    ///     ret                     ; Return the thread pointer
    /// ```
    ///
    /// A Rust implementation of this function can be found in [`ffi::aarch64`] module.
    ///
    /// ### Address Requirement
    ///
    /// - **Offset 0x1F8**: This places `tls_tp` at the last 8 bytes of the 0x200-byte TLS region
    /// - **Content**: Points to the TLS block start, typically TLS base
    /// - **Purpose**: Provides the base address that compiler-generated code uses
    ///   for thread-local variable access
    ///
    /// [`ffi::aarch64`]: crate::ffi::aarch64
    pub tls_ptr: *mut c_void,
}

/// Returns the base address of this thread's Thread-Local Storage (TLS) block as a plain
/// `usize`.
///
/// On AArch64 the per-thread TLS pointer is exposed to user-mode code via the
/// read-only system register `TPIDRRO_EL0`. Horizon OS initialises this register
/// during thread creation to point at the first byte of the 0x200-byte TLS
/// block described at the top of this module.
///
/// This function is nothing more than a thin, *safe* wrapper around a single
/// `mrs` instruction that reads that register. Because merely reading the
/// register cannot violate any safety guarantees the function is safe to call;
/// however any *use* of the returned address (e.g. by dereferencing it) must
/// observe the TLS layout documented in this file.
///
/// If you need a raw pointer instead of an integer address, use
/// [`get_ptr`] which performs the cast for you.
#[inline]
pub fn get_base_addr() -> usize {
    unsafe { control_regs::tpidrro_el0() }
}

/// Returns a raw pointer to the 512-byte Thread Local Storage (TLS) for the
/// current thread.
///
/// This is simply [`get_base_addr`] cast to a pointer, so obtaining the value
/// is completely safe. **Dereferencing** the pointer, however, requires `unsafe`
/// code and must respect the TLS layout documented in this module.
#[inline]
pub fn get_ptr() -> *mut c_void {
    get_base_addr() as *mut c_void
}

/// Returns a raw pointer to the [`ThreadVars`] for the current thread.
#[inline]
pub fn thread_vars_ptr() -> *mut ThreadVars {
    let tls_ptr = get_ptr();

    // SAFETY: The TLS area is 0x200 bytes in size, the [`ThreadVars`] sits at
    // the very end of it.
    unsafe { tls_ptr.add(TLS_REGION_SIZE - THREAD_VARS_SIZE) as *mut ThreadVars }
}

/// Initializes the current thread's [`ThreadVars`] TLS footer.
///
/// Internally the OS reserves the final 0x20 bytes of each 0x200-byte TLS
/// block for a small structure containing per-thread metadata that both the
/// kernel and userspace runtime consult frequently (see the module-level
/// description for the exact layout).  
///
/// This function must be invoked exactly **once** during thread start-up, after
/// a fresh TLS block has been allocated and before any user code attempts to
/// read thread metadata. The supplied values are copied verbatim into the
/// footer located at `TLS_base + 0x1E0`.
///
/// * `handle` – Kernel handle returned by `svcCreateThread`.
/// * `thread_info_ptr` – Language-specific thread object (e.g. a Rust `Thread`
///   struct) or `NULL` if not applicable.
/// * `reent` – Pointer to the thread's newlib re-entrancy state (`struct _reent`).
/// * `tls_tp` – Thread pointer value that compiler-generated code will obtain
///   via `__aarch64_read_tp()`. This is typically equal to `TLS_base` but other
///   layouts are possible.
///
/// # Safety
/// This routine mutates the TLS memory that the CPU is *actively* using for the
/// current thread. Callers must guarantee the following:
///
/// 1. The executing core is indeed running the thread whose TLS is being modified.
/// 2. No other code is concurrently accessing an uninitialised [`ThreadVars`].
///
/// Failing to uphold these requirements will lead to **undefined behaviour**, up to
/// and including memory corruption in unrelated threads.
#[inline]
pub unsafe fn init_thread_vars(
    handle: Handle,
    thread_info_ptr: *mut c_void,
    reent: *mut c_void,
    tls_ptr: *mut c_void,
) {
    let thread_vars = thread_vars_ptr();
    unsafe {
        thread_vars.write(ThreadVars {
            magic: THREAD_VARS_MAGIC,
            handle,
            thread_info_ptr,
            reent,
            tls_ptr,
        });
    }
}

/// Returns a raw pointer to the TLS dynamic slots array for the current thread.
///
/// The dynamic slots are an array of `NUM_TLS_SLOTS` (27) pointer-sized entries
/// located at TLS base + 0x108. Each slot can hold a `*mut c_void` value and
/// is used for runtime-allocated thread-local storage.
///
/// ```text
/// TLS base + 0x108 ──┐
///                    ├─ Slot 0  (*mut c_void)  ← returned pointer
///                    ├─ Slot 1  (*mut c_void)
///                    ├─ Slot 2  (*mut c_void)
///                    ┆        ...
///                    └─ Slot 26 (*mut c_void)
/// ```
///
/// # Safety
///
/// The returned pointer is valid for the lifetime of the current thread and
/// points to an array of `NUM_TLS_SLOTS` entries. Each entry is a `*mut c_void`.
#[inline]
pub fn slots_ptr() -> NonNull<*mut c_void> {
    let tls_ptr = get_ptr();

    // SAFETY: The TLS dynamic slots start at USER_TLS_BEGIN (0x108) from the
    // TLS base address. It is valid for the lifetime of the current thread.
    unsafe { NonNull::new_unchecked(tls_ptr.add(USER_TLS_REGION_BEGIN) as *mut *mut c_void) }
}

/// Returns the [`Handle`] of the current thread.
#[inline]
pub fn get_current_thread_handle() -> Handle {
    let tv = thread_vars_ptr();
    // SAFETY: `tv` points to a valid `ThreadVars` inside the current thread's
    // TLS block. The field access is performed with `read_volatile` to avoid
    // the compiler re-ordering or eliminating the read.
    unsafe { ptr::read_volatile(&raw const (*tv).handle) }
}

#[cfg(test)]
mod tests {
    use static_assertions::const_assert_eq;

    use super::{THREAD_VARS_SIZE, ThreadVars};

    // Ensure the layout stays consistent with Horizon expectations.
    const_assert_eq!(size_of::<ThreadVars>(), THREAD_VARS_SIZE);
}
