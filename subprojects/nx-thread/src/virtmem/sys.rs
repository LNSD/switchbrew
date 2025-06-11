//! C-compatible raw API for FFI interoperability
//!
//! This module provides C-compatible functions that match the original libnx virtmem API.
//! All functions are unsafe and require manual mutex management and pointer validation.

use core::ptr;
use crate::virtmem::core::*;

/// Opaque reservation handle for C compatibility
/// 
/// This is an opaque type that represents a memory reservation.
/// It should only be created by `virtmem_add_reservation()` and
/// destroyed by `virtmem_remove_reservation()`.
#[repr(C)]
pub struct VirtmemReservation {
    _private: [u8; 0],
}

/// Locks the virtual memory manager mutex.
/// 
/// This function provides exclusive access to virtual memory operations
/// across the entire process. All virtual memory find and reservation
/// operations require the mutex to be held.
/// 
/// # Safety
/// 
/// This function is thread-safe but provides no memory safety guarantees.
/// Caller must ensure proper pairing with `virtmem_unlock()`.
/// 
/// # Note
/// 
/// This function is equivalent to the C `virtmemLock()` function.
pub fn virtmem_lock() {
    virtmem_lock_raw();
}

/// Unlocks the virtual memory manager mutex.
/// 
/// This function releases exclusive access to virtual memory operations,
/// allowing other threads to perform virtual memory operations.
/// 
/// # Safety
/// 
/// This function is thread-safe but provides no memory safety guarantees.
/// Caller must ensure proper pairing with `virtmem_lock()`.
/// 
/// # Note
/// 
/// This function is equivalent to the C `virtmemUnlock()` function.
pub fn virtmem_unlock() {
    virtmem_unlock_raw();
}

/// Finds a random slice of free general purpose address space.
/// 
/// This function searches the ASLR region for a suitable address range
/// that can accommodate the requested size plus guard areas.
/// 
/// # Arguments
/// 
/// * `size` - Desired size of the slice (rounded up to page alignment)
/// * `guard_size` - Desired size of unmapped guard areas (rounded up to page alignment)
/// 
/// # Returns
/// 
/// Pointer to the slice of address space, or null on failure.
/// 
/// # Safety
/// 
/// * The virtual memory manager mutex must be held during the find-and-map process
/// * Returned pointer is not guaranteed to be valid beyond the mutex scope
/// * Caller must validate the returned pointer is not null
/// * Caller must ensure the returned address range is properly mapped before use
/// 
/// # Note
/// 
/// This function is equivalent to the C `virtmemFindAslr()` function.
pub unsafe fn virtmem_find_aslr(size: usize, guard_size: usize) -> *mut u8 {
    unsafe { virtmem_find_random_raw(RegionType::Aslr, size, guard_size) }
}

/// Finds a random slice of free stack address space.
/// 
/// This function searches the stack region for a suitable address range
/// that can accommodate the requested size plus guard areas.
/// 
/// # Arguments
/// 
/// * `size` - Desired size of the slice (rounded up to page alignment)
/// * `guard_size` - Desired size of unmapped guard areas (rounded up to page alignment)
/// 
/// # Returns
/// 
/// Pointer to the slice of address space, or null on failure.
/// 
/// # Safety
/// 
/// * The virtual memory manager mutex must be held during the find-and-map process
/// * Returned pointer is not guaranteed to be valid beyond the mutex scope
/// * Caller must validate the returned pointer is not null
/// * Caller must ensure the returned address range is properly mapped before use
/// 
/// # Note
/// 
/// This function is equivalent to the C `virtmemFindStack()` function.
pub unsafe fn virtmem_find_stack(size: usize, guard_size: usize) -> *mut u8 {
    unsafe { virtmem_find_random_raw(RegionType::Stack, size, guard_size) }
}

/// Finds a random slice of free code memory address space.
/// 
/// This function searches the appropriate region for code memory allocation.
/// On legacy kernels (1.0.0), code memory must be allocated in the stack region.
/// On newer kernels, code memory can be allocated in the ASLR region.
/// 
/// # Arguments
/// 
/// * `size` - Desired size of the slice (rounded up to page alignment)  
/// * `guard_size` - Desired size of unmapped guard areas (rounded up to page alignment)
/// 
/// # Returns
/// 
/// Pointer to the slice of address space, or null on failure.
/// 
/// # Safety
/// 
/// * The virtual memory manager mutex must be held during the find-and-map process
/// * Returned pointer is not guaranteed to be valid beyond the mutex scope
/// * Caller must validate the returned pointer is not null
/// * Caller must ensure the returned address range is properly mapped before use
/// 
/// # Note
/// 
/// This function is equivalent to the C `virtmemFindCodeMemory()` function.
pub unsafe fn virtmem_find_code_memory(size: usize, guard_size: usize) -> *mut u8 {
    unsafe { virtmem_find_random_raw(RegionType::CodeMemory, size, guard_size) }
}

/// Reserves a range of memory address space.
/// 
/// This function creates a reservation that prevents other virtual memory
/// allocation operations from returning overlapping address ranges.
/// The reservation must be explicitly removed when no longer needed.
/// 
/// # Arguments
/// 
/// * `mem` - Pointer to the address space slice to reserve
/// * `size` - Size of the slice to reserve
/// 
/// # Returns
/// 
/// Pointer to a reservation object, or null on failure.
/// 
/// # Safety
/// 
/// * The virtual memory manager mutex must be held during the find-and-reserve process
/// * `mem` must be a valid pointer to addressable memory (may be unmapped)
/// * Returned reservation must be freed with `virtmem_remove_reservation()`
/// * Reservation pointer becomes invalid after `virtmem_remove_reservation()` call
/// * Double-free of the same reservation is undefined behavior
/// 
/// # Note
/// 
/// This function is equivalent to the C `virtmemAddReservation()` function.
/// The returned reservation is intended to be used in lieu of a memory map
/// operation when the memory won't be mapped straight away.
pub unsafe fn virtmem_add_reservation(
    mem: *mut u8,
    size: usize,
) -> *mut VirtmemReservation {
    let entry = unsafe { virtmem_add_reservation_raw(mem, size) };
    entry as *mut VirtmemReservation
}

/// Releases a memory address space reservation.
/// 
/// This function removes a previously created reservation, making the
/// associated address space available for future allocation operations.
/// 
/// # Arguments
/// 
/// * `reservation` - Reservation to release (must be non-null)
/// 
/// # Safety
/// 
/// * The virtual memory manager mutex must be held before calling this function
/// * `reservation` must be a valid reservation returned by `virtmem_add_reservation()`
/// * `reservation` pointer becomes invalid after this call
/// * Double-free of the same reservation is undefined behavior
/// * Calling with a null pointer is safe but has no effect
/// 
/// # Note
/// 
/// This function is equivalent to the C `virtmemRemoveReservation()` function.
pub unsafe fn virtmem_remove_reservation(reservation: *mut VirtmemReservation) {
    if !reservation.is_null() {
        unsafe { virtmem_remove_reservation_raw(reservation as *mut ReservationEntry) };
    }
} 
