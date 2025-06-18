//! # Global allocator
//!
//! This module provides a global allocator that uses the linked list allocator.
//! It is used to allocate memory for the entire program.

#![cfg(feature = "global-allocator")]

use core::alloc::{GlobalAlloc, Layout};

use crate::llalloc::ALLOC;

#[global_allocator]
static GLOBAL_ALLOC: GlobalLlAllocator = GlobalLlAllocator;

/// A global allocator that uses the linked list allocator.
pub struct GlobalLlAllocator;

unsafe impl GlobalAlloc for GlobalLlAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut alloc = ALLOC.lock();
        unsafe { alloc.malloc(layout.size(), layout.align()) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut alloc = ALLOC.lock();
        unsafe { alloc.free(ptr, layout.size(), layout.align()) }
    }
}
