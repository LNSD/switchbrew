//! C FFI bindings for compatibility with existing C code
//!
//! This module provides `#[no_mangle]` C functions that follow the nx-thread
//! naming convention for internal virtmem operations.

use crate::virtmem::sys;

/// FFI-compatible reservation type
pub type VirtmemReservation = sys::VirtmemReservation;

/// Locks the virtual memory manager mutex
#[no_mangle]
pub extern "C" fn __nx_thread_virtmem_lock() {
    sys::virtmem_lock()
}

/// Unlocks the virtual memory manager mutex  
#[no_mangle]
pub extern "C" fn __nx_thread_virtmem_unlock() {
    sys::virtmem_unlock()
}

/// Finds a random slice of free general purpose address space
#[no_mangle]
pub unsafe extern "C" fn __nx_thread_virtmem_find_aslr(size: usize, guard_size: usize) -> *mut u8 {
    unsafe { sys::virtmem_find_aslr(size, guard_size) }
}

/// Finds a random slice of free stack address space
#[no_mangle]
pub unsafe extern "C" fn __nx_thread_virtmem_find_stack(size: usize, guard_size: usize) -> *mut u8 {
    unsafe { sys::virtmem_find_stack(size, guard_size) }
}

/// Finds a random slice of free code memory address space
#[no_mangle] 
pub unsafe extern "C" fn __nx_thread_virtmem_find_code_memory(size: usize, guard_size: usize) -> *mut u8 {
    unsafe { sys::virtmem_find_code_memory(size, guard_size) }
}

/// Reserves a range of memory address space
#[no_mangle]
pub unsafe extern "C" fn __nx_thread_virtmem_add_reservation(
    mem: *mut u8,
    size: usize,
) -> *mut VirtmemReservation {
    unsafe { sys::virtmem_add_reservation(mem, size) }
}

/// Releases a memory address space reservation
#[no_mangle]
pub unsafe extern "C" fn __nx_thread_virtmem_remove_reservation(reservation: *mut VirtmemReservation) {
    unsafe { sys::virtmem_remove_reservation(reservation) }
} 
