//! Virtual memory management for Nintendo Switch
//!
//! This module provides C-compatible virtual memory management functions
//! that match the original libnx virtmem API.

use core::{cell::UnsafeCell, ffi::c_void, ptr};

use nx_sync::sys::switch::Mutex;
use nx_rand::sys;

/// Constants
const RANDOM_MAX_ATTEMPTS: usize = 0x200;
const PAGE_SIZE: usize = 0x1000;
const PAGE_MASK: usize = PAGE_SIZE - 1;
const MAX_RESERVATIONS: usize = 64; // Fixed-size reservation pool

/// Global virtual memory manager mutex
static VIRTMEM_MUTEX: Mutex = Mutex::new();

/// Global virtual memory state (protected by mutex)
static mut VIRTMEM_STATE: UnsafeCell<Option<VirtmemState>> = UnsafeCell::new(None);

/// Opaque reservation handle for C compatibility
///
/// This is an opaque type that represents a memory reservation.
/// It should only be created by `virtmem_add_reservation()` and
/// destroyed by `virtmem_remove_reservation()`.
#[repr(C)]
pub struct VirtmemReservation {
    _private: [u8; 0],
}

/// Memory region bounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemRegion {
    pub start: usize,
    pub end: usize,
}

/// Virtual memory region types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    /// General purpose ASLR region
    Aslr,
    /// Stack region
    Stack,
    /// Code memory region (version-dependent)
    CodeMemory,
}

/// Reservation entry in the fixed-size pool
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ReservationEntry {
    in_use: bool,
    region: MemRegion,
}

/// Virtual memory manager state
#[derive(Debug)]
struct VirtmemState {
    alias_region: MemRegion,
    heap_region: MemRegion,
    aslr_region: MemRegion,
    stack_region: MemRegion,
    reservations: [ReservationEntry; MAX_RESERVATIONS],
    is_legacy_kernel: bool,
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
pub fn lock() {
    VIRTMEM_MUTEX.lock();
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
pub fn unlock() {
    VIRTMEM_MUTEX.unlock();
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
pub unsafe fn find_aslr(size: usize, guard_size: usize) -> *mut c_void {
    if !VIRTMEM_MUTEX.is_locked_by_current_thread() {
        return ptr::null_mut();
    }

    if !unsafe { ensure_initialized() } {
        return ptr::null_mut();
    }

    let state = unsafe { get_virtmem_state().unwrap() };
    unsafe { memregion_find_random(&state.aslr_region, size, guard_size) }
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
pub unsafe fn find_stack(size: usize, guard_size: usize) -> *mut c_void {
    if !VIRTMEM_MUTEX.is_locked_by_current_thread() {
        return ptr::null_mut();
    }

    if !unsafe { ensure_initialized() } {
        return ptr::null_mut();
    }

    let state = unsafe { get_virtmem_state().unwrap() };
    unsafe { memregion_find_random(&state.stack_region, size, guard_size) }
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
pub unsafe fn find_code_memory(size: usize, guard_size: usize) -> *mut c_void {
    if !VIRTMEM_MUTEX.is_locked_by_current_thread() {
        return ptr::null_mut();
    }

    if !unsafe { ensure_initialized() } {
        return ptr::null_mut();
    }

    let state = unsafe { get_virtmem_state().unwrap() };
    let region = if state.is_legacy_kernel {
        &state.stack_region
    } else {
        &state.aslr_region
    };
    unsafe { memregion_find_random(region, size, guard_size) }
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
pub unsafe fn add_reservation(mem: *mut c_void, size: usize) -> *mut VirtmemReservation {
    if !VIRTMEM_MUTEX.is_locked_by_current_thread() {
        return ptr::null_mut();
    }

    if !unsafe { ensure_initialized() } {
        return ptr::null_mut();
    }

    let state = unsafe { get_virtmem_state_mut().unwrap() };

    // Find an unused slot in the reservation pool
    for reservation in &mut state.reservations {
        if !reservation.in_use {
            reservation.in_use = true;
            reservation.region.start = mem as usize;
            reservation.region.end = mem as usize + size;
            return reservation as *mut _ as *mut VirtmemReservation;
        }
    }

    ptr::null_mut()
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
pub unsafe fn remove_reservation(reservation: *mut VirtmemReservation) {
    if !VIRTMEM_MUTEX.is_locked_by_current_thread() {
        return;
    }

    if reservation.is_null() {
        return;
    }

    if !unsafe { ensure_initialized() } {
        return;
    }

    // Mark the reservation as unused
    let entry = &mut *(reservation as *mut ReservationEntry);
    entry.in_use = false;
    entry.region.start = 0;
    entry.region.end = 0;
}

// Internal helper functions

/// Page align a size value
#[inline]
fn page_align(size: usize) -> usize {
    (size + PAGE_MASK) & !PAGE_MASK
}

/// Check if two memory regions overlap
#[inline]
fn memregion_overlaps(region: &MemRegion, start: usize, end: usize) -> bool {
    start < region.end && region.start < end
}

/// Check if memory range is mapped by querying the kernel
fn memregion_is_mapped(start: usize, end: usize, guard: usize) -> bool {
    // Adjust start/end by the desired guard size
    let _query_start = start.saturating_sub(guard);
    let _query_end = end + guard;

    // For now, assume unmapped (would need nx-svc query_memory implementation)
    // TODO: Implement proper memory query via nx-svc when available
    todo!()
}

/// Check if memory range overlaps with any reservations
unsafe fn memregion_is_reserved(start: usize, end: usize, guard: usize) -> bool {
    // Adjust start/end by the desired guard size
    let query_start = start.saturating_sub(guard);
    let query_end = end + guard;

    let state = unsafe { get_virtmem_state().unwrap() };

    for reservation in &state.reservations {
        if reservation.in_use && memregion_overlaps(&reservation.region, query_start, query_end) {
            return true;
        }
    }

    false
}

/// Find random address in a memory region
unsafe fn memregion_find_random(region: &MemRegion, size: usize, guard_size: usize) -> *mut c_void {
    // Page align the sizes
    let size = page_align(size);
    let guard_size = page_align(guard_size);

    // Ensure the requested size isn't greater than the memory region itself
    let region_size = region.end - region.start;
    if size > region_size {
        return ptr::null_mut();
    }

    let state = unsafe { get_virtmem_state().unwrap() };

    // Main allocation loop
    let aslr_max_page_offset = (region_size - size) >> 12;
    for _i in 0..RANDOM_MAX_ATTEMPTS {
        // Calculate a random memory range outside reserved areas
        let mut cur_addr;
        loop {
            // Use nx-rand for random number generation
            let page_offset = (sys::next_u64() as usize) % (aslr_max_page_offset + 1);
            cur_addr = region.start + (page_offset << 12);

            // Avoid mapping within the alias region
            if memregion_overlaps(&state.alias_region, cur_addr, cur_addr + size) {
                continue;
            }

            // Avoid mapping within the heap region
            if memregion_overlaps(&state.heap_region, cur_addr, cur_addr + size) {
                continue;
            }

            // Found a candidate address
            break;
        }

        // Check that there isn't anything mapped at the desired memory range
        if memregion_is_mapped(cur_addr, cur_addr + size, guard_size) {
            continue;
        }

        // Check that the desired memory range doesn't overlap any reservations
        if unsafe { memregion_is_reserved(cur_addr, cur_addr + size, guard_size) } {
            continue;
        }

        // We found a suitable address!
        return cur_addr as *mut c_void;
    }

    ptr::null_mut()
}

/// Initialize virtual memory regions
unsafe fn virtmem_setup() -> VirtmemState {
    // TODO: Replace with actual kernel info queries when nx-svc is complete
    todo!()
}

/// Ensure virtual memory state is initialized
unsafe fn ensure_initialized() -> bool {
    if unsafe { (*VIRTMEM_STATE.get()).is_none() } {
        unsafe { *VIRTMEM_STATE.get() = Some(virtmem_setup()) };
    }
    true
}

/// Helper to safely access VIRTMEM_STATE
unsafe fn get_virtmem_state() -> Option<&'static VirtmemState> {
    // SAFETY: We only access this when the mutex is held
    unsafe { (*VIRTMEM_STATE.get()).as_ref() }
}

/// Helper to safely access VIRTMEM_STATE mutably
unsafe fn get_virtmem_state_mut() -> Option<&'static mut VirtmemState> {
    // SAFETY: We only access this when the mutex is held
    unsafe { (*VIRTMEM_STATE.get()).as_mut() }
}
