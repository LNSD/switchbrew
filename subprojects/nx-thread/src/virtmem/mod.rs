//! Virtual memory management for Nintendo Switch
//!
//! This module provides C-compatible virtual memory management functions
//! that match the original libnx virtmem API.
//!
//! ## C-Compatible Usage
//! ```rust
//! use nx_thread::virtmem::sys;
//!
//! unsafe fn c_style_allocation() {
//!     sys::virtmem_lock();
//!     
//!     let addr = sys::virtmem_find_aslr(0x10000, 0x1000);
//!     if !addr.is_null() {
//!         let reservation = sys::virtmem_add_reservation(addr, 0x10000);
//!         if !reservation.is_null() {
//!             // Use the memory...
//!             sys::virtmem_remove_reservation(reservation);
//!         }
//!     }
//!     
//!     sys::virtmem_unlock();
//! }
//! ```

mod core;
mod ffi;

pub mod sys;

// Re-export core types for convenience
pub use core::{MemRegion, RegionType};

/// RAII guard for virtual memory operations
/// 
/// This guard ensures the virtual memory mutex is held for the duration
/// of its lifetime, providing thread-safe access to virtual memory operations.
pub struct VirtmemGuard {
    _private: (),
}

impl VirtmemGuard {
    /// Acquire the virtual memory lock
    /// 
    /// Returns a guard that provides exclusive access to virtual memory operations.
    /// The mutex is automatically released when the guard is dropped.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use nx_thread::virtmem::VirtmemGuard;
    /// 
    /// let guard = VirtmemGuard::lock();
    /// // Virtual memory operations are now safe
    /// let addr = guard.find_aslr(0x10000, 0x1000)?;
    /// // Mutex automatically released when guard drops
    /// ```
    pub fn lock() -> Self {
        sys::virtmem_lock();
        Self { _private: () }
    }

    /// Find random address space in ASLR region
    /// 
    /// # Arguments
    /// 
    /// * `size` - Desired size of the slice (will be page-aligned)
    /// * `guard_size` - Desired size of unmapped guard areas (will be page-aligned)
    /// 
    /// # Returns
    /// 
    /// * `Ok(NonNull<u8>)` - Valid address space pointer
    /// * `Err(Error::NoAddressFound)` - No suitable address found
    /// * `Err(Error::SizeTooBig)` - Requested size exceeds region
    pub fn find_aslr(&self, size: usize, guard_size: usize) -> Result<NonNull<u8>> {
        let ptr = unsafe { sys::virtmem_find_aslr(size, guard_size) };
        NonNull::new(ptr).ok_or(Error::NoAddressFound)
    }

    /// Find random address space in stack region
    /// 
    /// # Arguments
    /// 
    /// * `size` - Desired size of the slice (will be page-aligned)
    /// * `guard_size` - Desired size of unmapped guard areas (will be page-aligned)
    /// 
    /// # Returns
    /// 
    /// * `Ok(NonNull<u8>)` - Valid address space pointer
    /// * `Err(Error::NoAddressFound)` - No suitable address found
    /// * `Err(Error::SizeTooBig)` - Requested size exceeds region
    pub fn find_stack(&self, size: usize, guard_size: usize) -> Result<NonNull<u8>> {
        let ptr = unsafe { sys::virtmem_find_stack(size, guard_size) };
        NonNull::new(ptr).ok_or(Error::NoAddressFound)
    }

    /// Find random address space for code memory
    /// 
    /// # Arguments
    /// 
    /// * `size` - Desired size of the slice (will be page-aligned)
    /// * `guard_size` - Desired size of unmapped guard areas (will be page-aligned)
    /// 
    /// # Returns
    /// 
    /// * `Ok(NonNull<u8>)` - Valid address space pointer  
    /// * `Err(Error::NoAddressFound)` - No suitable address found
    /// * `Err(Error::SizeTooBig)` - Requested size exceeds region
    pub fn find_code_memory(&self, size: usize, guard_size: usize) -> Result<NonNull<u8>> {
        let ptr = unsafe { sys::virtmem_find_code_memory(size, guard_size) };
        NonNull::new(ptr).ok_or(Error::NoAddressFound)
    }

    /// Reserve address space range
    /// 
    /// Creates a reservation that prevents other allocations from overlapping
    /// with the specified memory range.
    /// 
    /// # Arguments
    /// 
    /// * `mem` - Starting address of the reservation
    /// * `size` - Size of the reservation
    /// 
    /// # Returns
    /// 
    /// * `Ok(Reservation)` - Successfully created reservation
    /// * `Err(Error::OutOfMemory)` - Failed to allocate reservation tracking
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let guard = VirtmemGuard::lock();
    /// let addr = guard.find_aslr(0x10000, 0x1000)?;
    /// let reservation = guard.add_reservation(addr, 0x10000)?;
    /// // Address space is now reserved
    /// drop(reservation); // Automatically removes reservation
    /// ```
    pub fn add_reservation(&self, mem: NonNull<u8>, size: usize) -> Result<Reservation> {
        let ptr = unsafe { sys::virtmem_add_reservation(mem.as_ptr(), size) };
        let inner = NonNull::new(ptr as *mut core::ReservationNode)
            .ok_or(Error::OutOfMemory)?;
        Ok(Reservation { inner })
    }

    /// Release address space reservation
    /// 
    /// Manually removes a reservation, making the address space available
    /// for future allocations.
    /// 
    /// # Arguments
    /// 
    /// * `reservation` - Reservation to remove
    /// 
    /// # Note
    /// 
    /// Reservations are automatically removed when dropped, so this method
    /// is only needed if you want to release the reservation before the
    /// `Reservation` object goes out of scope.
    pub fn remove_reservation(&self, reservation: Reservation) {
        unsafe { 
            sys::virtmem_remove_reservation(reservation.inner.as_ptr() as *mut sys::VirtmemReservation) 
        };
        core::mem::forget(reservation); // Prevent double-free in Drop
    }
}

impl Drop for VirtmemGuard {
    fn drop(&mut self) {
        sys::virtmem_unlock();
    }
}

/// Address space reservation handle
/// 
/// Represents a reserved range of virtual address space that prevents
/// other allocations from overlapping with it. The reservation is
/// automatically removed when this handle is dropped.
pub struct Reservation {
    inner: NonNull<core::ReservationNode>,
}

impl Reservation {
    /// Get the memory region covered by this reservation
    pub fn region(&self) -> MemRegion {
        unsafe { self.inner.as_ref().region }
    }
}

impl Drop for Reservation {
    fn drop(&mut self) {
        // Auto-remove reservation if not manually removed
        if core::virtmem_is_locked_by_current_thread() {
            unsafe { 
                sys::virtmem_remove_reservation(self.inner.as_ptr() as *mut sys::VirtmemReservation) 
            };
        }
        // Note: If mutex not held, we can't safely remove the reservation
        // This could be a programming error - consider debug assertion
        #[cfg(debug_assertions)]
        if !core::virtmem_is_locked_by_current_thread() {
            panic!("Reservation dropped without holding virtmem mutex - potential memory leak");
        }
    }
} 
