//! Virtual memory management for Nintendo Switch
//!
//! This module provides C-compatible virtual memory management functions
//! that match the original libnx virtmem API.

mod core;
mod ffi;
pub mod sys;

// Re-export core types for convenience
pub use core::{MemRegion, RegionType}; 
