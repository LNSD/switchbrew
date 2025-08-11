use core::{ffi::c_void, ptr::NonNull};

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
