use core::{ffi::c_void, ptr};

use nx_svc::thread::Handle;

use crate::tls;

/// Thread information structure
#[repr(C)]
pub struct Thread {
    /// The kernel thread handle
    pub handle: Handle,

    /// Whether the stack memory is owned by the thread.
    pub stack_mem_owned: bool,

    /// Alignment padding
    _align: [u8; 3],

    /// Stack memory information.
    pub stack_mem: ThreadStackMem,

    /// Pointer to the TLS slot array.
    pub tls_slot_array: *mut *mut c_void,

    /// Pointer to the next thread.
    pub next: *mut Thread,

    /// Pointer to the previous thread.
    pub prev_next: *mut *mut Thread,
}

/// Thread stack memory information
#[repr(C)]
pub struct ThreadStackMem {
    /// Pointer to stack memory
    pub mem: *mut c_void,

    /// Pointer to stack memory mirror
    pub mirror: *mut c_void,

    /// Stack memory size
    pub size: usize,
}

impl ThreadStackMem {
    /// Returns a pointer to the thread stack memory.
    pub fn memory_ptr(&self) -> *mut c_void {
        self.mem
    }

    /// Returns a pointer to the thread stack memory mirror.
    pub fn mirror_ptr(&self) -> *mut c_void {
        self.mirror
    }

    /// Returns the size of the thread stack memory.
    pub fn size(&self) -> usize {
        self.size
    }
}

/// Returns a raw pointer to the [`Thread`] information structure representing the
/// calling thread.
///
/// This is the Rust counterpart of libnx's `threadGetSelf` declared in
/// `switch/kernel/thread.h` and provides direct access to the per-thread data that
/// Horizon keeps in Thread Local Storage (TLS).
///
/// # Returns
/// A mutable raw pointer to the current thread's [`Thread`] structure. The
/// structure lives inside the TLS block of the running thread and remains valid
/// for the entire lifetime of that thread.
///
/// # Safety
/// * The returned pointer is only meaningful while the thread is alive; it must
///   not be dereferenced after the thread has exited.
/// * Using the pointer concurrently from multiple contexts without proper
///   synchronisation can lead to undefined behaviour because the kernel may
///   mutate some of the fields.
/// * The caller is responsible for ensuring that aliasing rules are not
///   violated when creating references from the raw pointer.
pub fn get_current_thread_info_ptr() -> *mut Thread {
    let tls_ptr = tls::thread_vars_ptr();

    // SAFETY: The current thread's information is stored in the TLS.
    // Use `read_volatile` to avoid the compiler re-ordering or eliminating the read.
    unsafe { ptr::read_volatile(&raw const (*tls_ptr).thread_info_ptr) as *mut Thread }
}

/// Returns the [`Handle`] of the calling thread.
///
/// This is the Rust counterpart of libnx's `threadGetCurHandle` declared in
/// `switch/kernel/thread.h` and provides direct access to the raw kernel
/// handle associated with the running thread.
///
/// # Returns
/// The [`Handle`] identifying the current thread. The handle is managed by the
/// kernel and remains valid for the entire lifetime of the thread.
///
/// # Safety
/// This function is intrinsically safe because it only reads the handle value
/// stored in the thread's TLS block and returns a copy. No shared mutable
/// state is accessed and no invariants can be violated.
pub fn get_current_thread_handle() -> Handle {
    let tls_ptr = tls::thread_vars_ptr();

    // SAFETY: The current thread's handle is stored in the TLS.
    // Use `read_volatile` to avoid the compiler re-ordering or eliminating the read.
    unsafe { ptr::read_volatile(&raw const (*tls_ptr).handle) }
}

#[cfg(test)]
mod tests {
    use static_assertions::const_assert;

    use super::{Thread, ThreadStackMem};

    // Assert that the size and alignment of the `Thread` struct is correct
    const_assert!(size_of::<Thread>() == 0x38);
    const_assert!(align_of::<Thread>() == 0x8);

    // Assert that the size and alignment of the `ThreadStackMem` struct is correct
    const_assert!(size_of::<ThreadStackMem>() == 0x18);
    const_assert!(align_of::<ThreadStackMem>() == 0x8);
}
