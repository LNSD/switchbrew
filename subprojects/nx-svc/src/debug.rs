use core::ffi::c_void;

use super::raw::__nx_svc_break;
pub use super::raw::BreakReason;

// TODO: Review if it must be -> !
pub unsafe fn break_event(
    reason: BreakReason,
    address: *mut c_void, // TODO: Review uintptr_t
    size: usize,          // TODO: Review uintptr_t
) -> Result<(), ()> {
    let res = unsafe { __nx_svc_break(reason, address, size) };
    if res == 0 { Ok(()) } else { Err(()) }
}
