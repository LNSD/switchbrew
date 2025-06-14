use super::raw::__nx_svc_break;
// TODO: Do not re-export the raw type
pub use super::raw::BreakReason;

// TODO: Review if it must be -> !
pub unsafe fn break_event(reason: BreakReason, address: usize, size: usize) -> Result<(), ()> {
    let res = unsafe { __nx_svc_break(reason, address, size) };
    if res == 0 { Ok(()) } else { Err(()) }
}
