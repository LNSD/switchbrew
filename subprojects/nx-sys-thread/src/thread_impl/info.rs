use core::{
    ffi::c_void,
    ptr::{self, NonNull},
};

use nx_svc::thread::Handle;

use crate::{
    slots::{SlotError, Slots},
    tls_region,
};

/// Thread information structure
pub struct Thread {
    /// The kernel thread handle
    pub handle: Handle,

    /// Stack memory information.
    pub stack_mem: ThreadStackMem,

    /// TLS slots.
    pub tls_slots: Option<Slots>,
}

impl Thread {
    /// Gets a TLS slot value.
    pub fn slot_get(&self, mod_id: usize) -> Result<*mut c_void, SlotGetError> {
        let Some(slots) = self.tls_slots.as_ref() else {
            return Err(SlotGetError::ThreadNotInitialized);
        };

        match slots.get(mod_id) {
            Ok(value) => Ok(value),
            Err(SlotError::OutOfBounds(mod_id)) => Err(SlotGetError::OutOfBounds(mod_id)),
        }
    }

    /// Sets a TLS slot to the given value.
    pub fn slot_set(&mut self, mod_id: usize, value: *mut c_void) -> Result<(), SlotSetError> {
        let Some(slots) = self.tls_slots.as_mut() else {
            return Err(SlotSetError::ThreadNotInitialized);
        };

        match slots.set(mod_id, value) {
            Ok(()) => Ok(()),
            Err(SlotError::OutOfBounds(mod_id)) => Err(SlotSetError::OutOfBounds(mod_id)),
        }
    }
}
/// Thread stack memory information
pub enum ThreadStackMem {
    /// The stack memory is owned by the thread.
    Owned {
        /// Pointer to stack memory
        mem: NonNull<c_void>,

        /// Pointer to stack memory mirror
        mirror: NonNull<c_void>,

        /// Stack memory size
        size: usize,
    },

    /// The stack memory is not owned by the thread.
    Provided {
        /// Pointer to stack memory
        mirror: NonNull<c_void>,

        /// Stack memory size
        size: usize,
    },
}

impl ThreadStackMem {
    /// Creates a new owned thread stack memory.
    pub fn new_owned(mem: NonNull<c_void>, mirror: NonNull<c_void>, size: usize) -> Self {
        Self::Owned { mem, mirror, size }
    }

    /// Creates a new thread stack memory from a provided stack memory.
    pub fn new_provided(mirror: NonNull<c_void>, size: usize) -> Self {
        Self::Provided { mirror, size }
    }

    /// Returns true if the stack memory is owned by the thread.
    pub fn is_owned(&self) -> bool {
        matches!(self, ThreadStackMem::Owned { .. })
    }

    /// Returns a pointer to the thread stack memory.
    pub fn memory_ptr(&self) -> Option<NonNull<c_void>> {
        match self {
            ThreadStackMem::Owned { mem, .. } => Some(*mem),
            ThreadStackMem::Provided { .. } => None,
        }
    }

    /// Returns a pointer to the thread stack memory mirror.
    pub fn mirror_ptr(&self) -> NonNull<c_void> {
        match self {
            ThreadStackMem::Owned { mirror, .. } => *mirror,
            ThreadStackMem::Provided { mirror, .. } => *mirror,
        }
    }

    /// Returns the size of the thread stack memory.
    pub fn size(&self) -> usize {
        match self {
            ThreadStackMem::Owned { size, .. } => *size,
            ThreadStackMem::Provided { size, .. } => *size,
        }
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
    let tv_ptr = tls_region::thread_vars_ptr();

    // SAFETY: The current thread's information is stored in the TLS.
    // Use `read_volatile` to avoid the compiler re-ordering or eliminating the read.
    unsafe { ptr::read_volatile(&raw const (*tv_ptr).thread_info_ptr) as *mut Thread }
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
    let tv_ptr = tls_region::thread_vars_ptr();

    // SAFETY: The current thread's handle is stored in the TLS.
    // Use `read_volatile` to avoid the compiler re-ordering or eliminating the read.
    unsafe { ptr::read_volatile(&raw const (*tv_ptr).handle) }
}

/// Error type for getting a thread's TLS slot value.
#[derive(Debug, thiserror::Error)]
pub enum SlotGetError {
    #[error("Thread not initialized")]
    ThreadNotInitialized,

    #[error("Mod index out of bounds: {0}")]
    OutOfBounds(usize),
}

/// Error type for setting a thread's TLS slot value.
#[derive(Debug, thiserror::Error)]
pub enum SlotSetError {
    #[error("Thread not initialized")]
    ThreadNotInitialized,

    #[error("Mod index out of bounds: {0}")]
    OutOfBounds(usize),
}
