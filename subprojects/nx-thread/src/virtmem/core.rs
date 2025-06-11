//! Core virtual memory management implementation
//!
//! This module contains the shared logic and state management for the C-compatible sys API.

use core::ptr;
use nx_sync::sys::switch::Mutex;
use nx_svc::ResultCode;

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
pub struct ReservationEntry {
    pub in_use: bool,
    pub region: MemRegion,
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

/// Global virtual memory manager mutex
static VIRTMEM_MUTEX: Mutex = Mutex::new();

/// Global virtual memory state (protected by mutex)
static mut VIRTMEM_STATE: Option<VirtmemState> = None;

/// Constants
const RANDOM_MAX_ATTEMPTS: usize = 0x200;
const PAGE_SIZE: usize = 0x1000;
const PAGE_MASK: usize = PAGE_SIZE - 1;
const MAX_RESERVATIONS: usize = 64; // Fixed-size reservation pool

/// Page align a size value
#[inline]
pub fn page_align(size: usize) -> usize {
    (size + PAGE_MASK) & !PAGE_MASK
}

/// Core mutex operations
pub fn virtmem_lock_raw() {
    VIRTMEM_MUTEX.lock();
}

pub fn virtmem_unlock_raw() {
    VIRTMEM_MUTEX.unlock();
}

pub fn virtmem_is_locked_by_current_thread() -> bool {
    VIRTMEM_MUTEX.is_locked_by_current_thread()
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
    false
}

/// Check if memory range overlaps with any reservations
unsafe fn memregion_is_reserved(start: usize, end: usize, guard: usize) -> bool {
    // Adjust start/end by the desired guard size
    let query_start = start.saturating_sub(guard);
    let query_end = end + guard;

    let state = VIRTMEM_STATE.as_ref().unwrap();
    
    for reservation in &state.reservations {
        if reservation.in_use && memregion_overlaps(&reservation.region, query_start, query_end) {
            return true;
        }
    }

    false
}

/// Find random address in a memory region
unsafe fn memregion_find_random(region: &MemRegion, size: usize, guard_size: usize) -> *mut u8 {
    // Page align the sizes
    let size = page_align(size);
    let guard_size = page_align(guard_size);

    // Ensure the requested size isn't greater than the memory region itself
    let region_size = region.end - region.start;
    if size > region_size {
        return ptr::null_mut();
    }

    let state = VIRTMEM_STATE.as_ref().unwrap();
    
    // Main allocation loop
    let aslr_max_page_offset = (region_size - size) >> 12;
    for _i in 0..RANDOM_MAX_ATTEMPTS {
        // Calculate a random memory range outside reserved areas
        let mut cur_addr;
        loop {
            // Use a simple PRNG for now
            let page_offset = simple_rng() % (aslr_max_page_offset + 1);
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
        if memregion_is_reserved(cur_addr, cur_addr + size, guard_size) {
            continue;
        }

        // We found a suitable address!
        return cur_addr as *mut u8;
    }

    ptr::null_mut()
}

/// Simple PRNG for address randomization
static mut SIMPLE_RNG_STATE: u64 = 1;

fn simple_rng() -> usize {
    unsafe {
        SIMPLE_RNG_STATE = SIMPLE_RNG_STATE.wrapping_mul(1103515245).wrapping_add(12345);
        (SIMPLE_RNG_STATE / 65536) as usize
    }
}

/// Initialize virtual memory regions
/// TODO: Replace with actual kernel info queries when nx-svc is complete
unsafe fn virtmem_setup() -> VirtmemState {
    // Use hardcoded values for now - these would come from kernel info queries
    VirtmemState {
        alias_region: MemRegion { start: 0x8000000, end: 0x80000000 },
        heap_region: MemRegion { start: 0x80000000, end: 0x100000000 },
        aslr_region: MemRegion { start: 0x8000000, end: 0x1000000000 },
        stack_region: MemRegion { start: 0x8000000, end: 0x80000000 },
        reservations: [ReservationEntry { in_use: false, region: MemRegion { start: 0, end: 0 } }; MAX_RESERVATIONS],
        is_legacy_kernel: false,
    }
}

/// Ensure virtual memory state is initialized
unsafe fn ensure_initialized() -> bool {
    if VIRTMEM_STATE.is_none() {
        VIRTMEM_STATE = Some(virtmem_setup());
    }
    true
}

/// Core algorithm implementations

/// Find random address space in specified region type
pub unsafe fn virtmem_find_random_raw(
    region_type: RegionType,
    size: usize,
    guard_size: usize,
) -> *mut u8 {
    if !virtmem_is_locked_by_current_thread() {
        return ptr::null_mut();
    }

    if !ensure_initialized() {
        return ptr::null_mut();
    }

    let state = VIRTMEM_STATE.as_ref().unwrap();
    let region = match region_type {
        RegionType::Aslr => &state.aslr_region,
        RegionType::Stack => &state.stack_region,
        RegionType::CodeMemory => {
            // [1.0.0] requires CodeMemory to be mapped within the stack region
            if state.is_legacy_kernel {
                &state.stack_region
            } else {
                &state.aslr_region
            }
        }
    };

    memregion_find_random(region, size, guard_size)
}

/// Add a reservation to the tracking pool
pub unsafe fn virtmem_add_reservation_raw(
    mem: *mut u8,
    size: usize,
) -> *mut ReservationEntry {
    if !virtmem_is_locked_by_current_thread() {
        return ptr::null_mut();
    }

    if !ensure_initialized() {
        return ptr::null_mut();
    }

    let state = VIRTMEM_STATE.as_mut().unwrap();
    
    // Find an unused slot in the reservation pool
    for reservation in &mut state.reservations {
        if !reservation.in_use {
            reservation.in_use = true;
            reservation.region.start = mem as usize;
            reservation.region.end = mem as usize + size;
            return reservation as *mut ReservationEntry;
        }
    }

    // No free slots available
    ptr::null_mut()
}

/// Remove a reservation from the tracking pool
pub unsafe fn virtmem_remove_reservation_raw(reservation: *mut ReservationEntry) {
    if !virtmem_is_locked_by_current_thread() {
        return;
    }

    if reservation.is_null() {
        return;
    }

    if VIRTMEM_STATE.is_none() {
        return;
    }

    // Mark the reservation as unused
    let entry = &mut *reservation;
    entry.in_use = false;
    entry.region.start = 0;
    entry.region.end = 0;
} 
